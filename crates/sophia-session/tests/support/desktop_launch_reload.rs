#![cfg(test)]

use super::*;
use crate::live_session::*;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[path = "policy_active_focus.rs"]
mod policy_active_focus;

struct ReloadFixture {
    // Fragments and their directory must be released before the fixture root.
    wm: LiveWmSession,
    source: ConfigFixture,
}

impl ReloadFixture {
    fn new() -> Self {
        Self::from_config_fixture(ConfigFixture::new(&[]))
    }

    fn from_config_fixture(mut source: ConfigFixture) -> Self {
        source.config.wm_socket_path = source.directory.join("wm.sock");
        activate_session_profile(&mut source.config);
        let config = &source.config;
        let PreparedPublicPolicyLaunch {
            profile_fragments,
            directory,
            policy_profile,
            shell_profile,
            shortcut_profile_slot,
            broker_profile,
        } = PreparedPublicPolicyLaunch::new(config).unwrap();
        let key = sophia_config::DesktopProfileActivationKey::from(&config.desktop_profile);
        let socket_path = directory.endpoint_path().join("wm.sock");
        let checkpoint_path = directory.checkpoint_path();
        let output_socket = directory.path().join("output-endpoint/output.sock");
        // Exercise exact launch-spec preservation without acquiring a socket or
        // starting the supervised process or output authority.
        let spec = public_policy_launch_spec(
            config,
            "/bin/true",
            &socket_path,
            &checkpoint_path,
            profile_fragments.path(sophia_config::DesktopAuthority::Policy),
            true,
            Some(&output_socket),
        )
        .unwrap();
        let output = sophia_engine::HeadlessOutput::deterministic();
        let (session_operations, operation_actions) = public_session_operations(config);
        let scene =
            LivePublicPolicyState::initial_scene(&[output], output.id, session_operations.clone());
        let mut reducer = sophia_engine::PolicyProjectionReducer::new(scene).unwrap();
        reducer.connect(1).unwrap();
        let configuration = sophia_protocol::PolicyConfiguration {
            connection_epoch: 1,
            generation: key.generation().raw(),
            actions: Vec::new(),
            chrome: sophia_protocol::WmChromePolicy::default(),
        };
        let commands = SessionCommandRegistry::prepare(1, &config.applications).unwrap();
        let registry = resolve_public_shortcuts(
            &config.shortcut_profile_candidate,
            &configuration,
            configuration.generation,
            &commands,
        )
        .unwrap();
        let bounds = wm_output_bounds(&[output])
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        let public = LivePublicPolicyState {
            control_generation: 1,
            control_catalog_serial: 1,
            control_tickets: BTreeMap::new(),
            worker: None,
            output_service: None,
            output_authority: None,
            output_effect_dispatched: false,
            output_topology_reload_pending: false,
            startup_output_transaction: None,
            output_cancel_requested: None,
            output_pending_connection_epoch: None,
            next_output_snapshot_transaction: 2,
            output_capabilities: Vec::new(),
            _profile_fragments: profile_fragments,
            _profile_slot: policy_profile,
            profile_key: Some(key),
            checkpoint_path,
            directory,
            reducer,
            connection_epoch: 1,
            next_connection_epoch: 2,
            next_transaction: 3,
            configured: true,
            negotiated: true,
            selected_capabilities: 0,
            cycle_submitted: false,
            transport_ready: true,
            queue: VecDeque::new(),
            pending_dirty_outputs: BTreeSet::new(),
            in_flight_source: None,
            in_flight_request: None,
            staged: None,
            prepared: None,
            shortcut_profile_slot,
            actions: Vec::new(),
            accepted_configuration: Some(configuration),
            launch_classifications: BTreeMap::new(),
            outputs: vec![output],
            output_bounds: bounds.clone(),
            output_generations: BTreeMap::from([(output.id, 1)]),
            live_output_ids: BTreeSet::from([output.id]),
            work_areas: bounds,
            session_operations,
            operation_actions,
            expected_operation_slot: None,
            pending_operation: None,
            active_output: output.id,
            deferred_command: None,
            transport_unavailable: false,
            proof_fault_after: None,
            proof_fault_triggered: false,
            proof_restart_after_action: None,
            proof_restart_checkpoint_before: None,
            proof_restart_triggered: false,
        };
        let wm = LiveWmSession {
            supervisor: ProcessSupervisor::new(SupervisedProcessKind::WindowManager, spec),
            supervisor_state: sophia_runtime::SupervisorState::new(
                SupervisedProcessKind::WindowManager,
            ),
            restart_policy: RestartPolicy::default(),
            socket_path,
            public: Some(public),
            _shell_profile: Some(shell_profile),
            _broker_profile: Some(broker_profile),
            requests: 0,
            request_peak_depth: 0,
            request_rejections: 0,
            action_requests_ordered: 0,
            stale_responses: 0,
            work_area_relayout_required: false,
            shell_reservation_bands: Vec::new(),
            shortcuts: Some(WmShortcutRouter::new(registry)),
            command_registry: commands,
            desktop_reload: None,
            _other_authority_fragments: None,
            pending_policy_launch_spec: None,
            pending_policy_configuration: None,
            wm_chrome_supported: true,
            chrome: sophia_protocol::WmChromePolicy::default(),
            fallback_chrome: config.surface_chrome_style,
            visual_chrome: config.surface_chrome_style,
            pending_visual_chrome: None,
            force_transport_restart: false,
            committed: 1,
            last_committed_at: None,
            max_request: Duration::ZERO,
            max_queue_dwell: Duration::ZERO,
            restarts: 0,
            degraded: false,
            control_restart: None,
            control_lifetime: None,
        };
        Self { wm, source }
    }

    fn save(&self, executable: &str, policy: Option<&str>) {
        let policy = policy.map_or(String::new(), |layout| {
            format!("policy {{ layout \"{layout}\"; }}\n")
        });
        let text = format!(
            "schema 1\n{policy}shell {{ enabled #false; }}\nsession {{\n\
             terminal \"terminal\"\n browser \"brave-origin\"\n startup \"panel\"\n\
             application \"demo\" {{ exec \"{executable}\" \"literal argument\"; }}\n\
             }}\nshortcut {{ profile \"reload\"; bind \"Super+b\" {{ launch \"demo\"; }} }}\n",
        );
        std::fs::write(
            self.source.config.desktop_profile_source.as_ref().unwrap(),
            text,
        )
        .unwrap();
        sophia_config::load_prepared_desktop_profile(
            self.source.config.desktop_profile_source.as_deref(),
            sophia_config::ConfigGeneration::from_raw(
                self.source.config.desktop_profile.generation.raw() + 1,
            ),
        )
        .unwrap();
    }

    fn reload(&mut self) -> DesktopProfileReloadOutcome {
        self.wm
            .reload_desktop_profile(&mut self.source.config)
            .unwrap()
    }

    fn stage_configuration(
        &mut self,
        configuration: &sophia_protocol::PolicyConfiguration,
    ) -> sophia_protocol::PolicyProjectionOutcome {
        let mut public = self.wm.public.take().unwrap();
        let result = self
            .wm
            .stage_policy_configuration(&mut public, configuration)
            .unwrap();
        self.wm.public = Some(public);
        result
    }

    fn configuration(&self) -> sophia_protocol::PolicyConfiguration {
        let public = self.wm.public.as_ref().unwrap();
        sophia_protocol::PolicyConfiguration {
            connection_epoch: public.connection_epoch,
            // Hagia starts its action catalog at 1 for each new connection,
            // independently of the activated desktop profile generation.
            generation: 1,
            actions: Vec::new(),
            chrome: sophia_protocol::WmChromePolicy::default(),
        }
    }

    fn replacement_started(&mut self) {
        let spec = self
            .wm
            .desktop_reload
            .as_mut()
            .unwrap()
            .replacement_spec
            .take()
            .unwrap();
        let previous_output = self
            .wm
            .supervisor
            .launch_spec()
            .environment
            .iter()
            .find(|(name, _)| name == sophia_runtime::SOPHIA_OUTPUT_SOCKET_ENV)
            .unwrap()
            .clone();
        assert!(spec.environment.contains(&previous_output));
        let public = self.wm.public.as_mut().unwrap();
        assert!(spec.environment.iter().any(|(name, path)| {
            name == "HAGIA_POLICY_CANDIDATE"
                && Path::new(path)
                    == public
                        ._profile_fragments
                        .path(sophia_config::DesktopAuthority::Policy)
        }));
        self.wm.supervisor = ProcessSupervisor::new(SupervisedProcessKind::WindowManager, spec);
        public.connection_epoch += 1;
        public.next_connection_epoch = public.connection_epoch + 1;
        public.configured = false;
        self.wm.force_transport_restart = false;
    }

    fn settle(&mut self, idle: bool) {
        assert_eq!(
            idle,
            self.wm.shortcuts.as_ref().unwrap().shortcut_idle(),
            "publication readiness must match the actual key ledger",
        );
        self.wm
            .settle_desktop_reload(&mut self.source.config, idle)
            .unwrap();
    }

    fn policy_path(&self) -> PathBuf {
        self.wm
            .public
            .as_ref()
            .unwrap()
            ._profile_fragments
            .path(sophia_config::DesktopAuthority::Policy)
            .to_path_buf()
    }
}

#[test]
fn command_only_reload_changes_the_real_router_and_registry_without_a_policy_restart() {
    let mut fixture = ReloadFixture::new();
    let old_policy_path = fixture.policy_path();
    let old_spec = fixture.wm.supervisor.launch_spec().clone();
    let startup_participant = fixture.source.config.session_profile.slot().clone();
    fixture.save("/first/command", None);
    assert_eq!(fixture.reload(), DesktopProfileReloadOutcome::Applied);
    let first = fixture.wm.command_registry.action("demo").unwrap();
    let retained = fixture.wm.command_registry.command(first).unwrap();
    let seat = SeatId::from_raw(1);
    let router = fixture.wm.shortcuts.as_mut().unwrap();
    assert!(router.route_key(seat, 125, true).action.is_none());
    assert_eq!(router.route_key(seat, 48, true).action, Some(first));
    router.route_key(seat, 48, false);
    router.route_key(seat, 125, false);
    assert!(router.shortcut_idle());
    fixture.save("/second/command", None);
    assert_eq!(fixture.reload(), DesktopProfileReloadOutcome::Applied);
    let second = fixture.wm.command_registry.action("demo").unwrap();
    assert_ne!(first, second);
    assert!(fixture.wm.command_registry.command(first).is_none());
    assert_eq!(retained.executable, Path::new("/first/command"));
    assert_eq!(
        fixture
            .wm
            .command_registry
            .command(second)
            .unwrap()
            .executable,
        Path::new("/second/command")
    );
    assert_eq!(fixture.wm.supervisor.launch_spec(), &old_spec);
    assert_eq!(fixture.policy_path(), old_policy_path);
    assert_eq!(
        fixture.source.config.session_profile.slot(),
        &startup_participant
    );
    let launch_profile = fixture
        .source
        .config
        .active_launch_profile
        .as_ref()
        .unwrap();
    assert_eq!(
        launch_profile.generation,
        fixture.source.config.desktop_profile.generation
    );
    assert_eq!(
        launch_profile.digest,
        fixture.source.config.desktop_profile.digest
    );
    assert!(!fixture.wm.force_transport_restart);
    assert!(!fixture.wm.desktop_reload_pending());
    assert_eq!(fixture.source.config.applications.startup, ["panel"]);
}

#[test]
fn removing_a_startup_command_publishes_its_replacement_without_replaying_startup() {
    let mut source = ConfigFixture::new(&[]);
    let desktop = source.directory.join("desktop.kdl");
    std::fs::write(
        &desktop,
        r#"schema 1
shell { enabled #false; }
session {
    application "startup-a" { exec "/first/startup-command"; }
    startup "startup-a"
}
shortcut {
    profile "reload"
    bind "Super+b" { launch "startup-a"; }
}
"#,
    )
    .unwrap();
    source.config = PersistentXtermSessionConfig::from_args(&[
        format!("--config={}", source.directory.join("core.kdl").display()),
        format!("--desktop-profile={}", desktop.display()),
        "--no-input".to_owned(),
        "--session-mode=normal".to_owned(),
    ])
    .unwrap();
    let mut fixture = ReloadFixture::from_config_fixture(source);
    let startup_participant = fixture.source.config.session_profile.slot().clone();
    let old_spec = fixture.wm.supervisor.launch_spec().clone();
    let old_policy_path = fixture.policy_path();
    let old_action = fixture.wm.command_registry.action("startup-a").unwrap();
    assert_eq!(fixture.source.config.applications.startup, ["startup-a"]);
    assert!(
        fixture
            .source
            .config
            .applications
            .applications
            .contains_key("startup-a")
    );
    std::fs::write(
        &desktop,
        r#"schema 1
shell { enabled #false; }
session {
    application "replacement-b" { exec "/second/command" "literal argument"; }
    startup
}
shortcut {
    profile "reload"
    bind "Super+b" { launch "replacement-b"; }
}
"#,
    )
    .unwrap();
    assert_eq!(fixture.reload(), DesktopProfileReloadOutcome::Applied);
    assert_eq!(fixture.source.config.applications.startup, ["startup-a"]);
    assert_eq!(
        fixture.source.config.session_profile.slot(),
        &startup_participant
    );
    assert!(
        !fixture
            .source
            .config
            .applications
            .applications
            .contains_key("startup-a")
    );
    assert!(fixture.wm.command_registry.command(old_action).is_none());
    let replacement = fixture.wm.command_registry.action("replacement-b").unwrap();
    let command = fixture.wm.command_registry.command(replacement).unwrap();
    assert_eq!(command.executable, Path::new("/second/command"));
    assert_eq!(command.arguments, ["literal argument"]);
    let launch_profile = fixture
        .source
        .config
        .active_launch_profile
        .as_ref()
        .unwrap();
    assert_eq!(launch_profile.startup.as_deref(), Some([].as_slice()));
    let router = fixture.wm.shortcuts.as_mut().unwrap();
    let seat = SeatId::from_raw(1);
    router.route_key(seat, 125, true);
    assert_eq!(router.route_key(seat, 48, true).action, Some(replacement));
    router.route_key(seat, 48, false);
    router.route_key(seat, 125, false);
    assert!(router.shortcut_idle());
    assert_eq!(fixture.wm.supervisor.launch_spec(), &old_spec);
    assert_eq!(fixture.policy_path(), old_policy_path);
    assert!(!fixture.wm.force_transport_restart);
    assert!(!fixture.wm.desktop_reload_pending());
}

#[test]
fn policy_acceptance_keeps_active_commands_and_key_ledger_until_input_is_idle() {
    let mut fixture = ReloadFixture::new();
    fixture.save("/old/command", None);
    assert_eq!(fixture.reload(), DesktopProfileReloadOutcome::Applied);
    let old_action = fixture.wm.command_registry.action("demo").unwrap();
    let old_applications = fixture.source.config.applications.clone();
    fixture.save("/new/command", Some("grid"));
    assert_eq!(
        fixture.reload(),
        DesktopProfileReloadOutcome::RestartRequired
    );
    fixture.replacement_started();
    assert!(fixture.wm.public.as_ref().unwrap().profile_key.unwrap().generation().raw() > 1);
    let mut stale = fixture.configuration();
    stale.connection_epoch -= 1;
    assert_eq!(
        fixture.stage_configuration(&stale),
        sophia_protocol::PolicyProjectionOutcome::RejectedInvalid
    );
    let seat = SeatId::from_raw(1);
    fixture
        .wm
        .shortcuts
        .as_mut()
        .unwrap()
        .route_key(seat, 125, true);
    let held = fixture.wm.shortcuts.clone();
    assert_eq!(
        fixture.stage_configuration(&fixture.configuration()),
        sophia_protocol::PolicyProjectionOutcome::Committed
    );
    fixture.settle(false);
    assert_eq!(fixture.wm.shortcuts, held);
    assert_eq!(fixture.wm.command_registry.action("demo"), Some(old_action));
    assert_eq!(fixture.source.config.applications, old_applications);
    assert!(!fixture.wm.public.as_ref().unwrap().configured);
    fixture
        .wm
        .shortcuts
        .as_mut()
        .unwrap()
        .route_key(seat, 125, false);
    fixture.settle(true);
    assert!(fixture.wm.public.as_ref().unwrap().configured);
    assert!(fixture.wm.shortcuts.as_ref().unwrap().shortcut_idle());
    let action = fixture.wm.command_registry.action("demo").unwrap();
    assert_ne!(action, old_action);
    assert_eq!(
        fixture
            .wm
            .command_registry
            .command(action)
            .unwrap()
            .executable,
        Path::new("/new/command")
    );
}

#[test]
fn rejected_policy_restores_the_exact_spec_fragments_and_commands() {
    let mut fixture = ReloadFixture::new();
    fixture.save("/old/command", None);
    assert_eq!(fixture.reload(), DesktopProfileReloadOutcome::Applied);
    let old_path = fixture.policy_path();
    let old_bytes = std::fs::read(&old_path).unwrap();
    let old_spec = fixture.wm.supervisor.launch_spec().clone();
    let old_apps = fixture.source.config.applications.clone();
    let old_router = fixture.wm.shortcuts.clone();
    fixture.save("/rejected/command", Some("grid"));
    assert_eq!(
        fixture.reload(),
        DesktopProfileReloadOutcome::RestartRequired
    );
    let rejected_path = fixture.policy_path();
    fixture.replacement_started();
    let mut invalid = fixture.configuration();
    invalid
        .actions
        .push(sophia_protocol::PolicyActionRegistration {
            action: WmActionId::from_raw(1),
            name: "unavailable".to_owned(),
            session_operation_slot: Some(999),
        });
    assert_eq!(
        fixture.stage_configuration(&invalid),
        sophia_protocol::PolicyProjectionOutcome::RejectedInvalid
    );
    let restored = fixture.wm.rollback_desktop_reload().unwrap();
    assert_eq!(restored, old_spec);
    assert_eq!(fixture.policy_path(), old_path);
    assert_eq!(std::fs::read(old_path).unwrap(), old_bytes);
    assert!(!rejected_path.exists());
    assert_eq!(fixture.source.config.applications, old_apps);
    assert_eq!(fixture.wm.shortcuts, old_router);
    assert!(!fixture.wm.desktop_reload_pending());
}

#[test]
fn restored_policy_configuration_preserves_a_held_key_until_its_release() {
    let mut fixture = ReloadFixture::new();
    fixture.save("/old/command", None);
    assert_eq!(fixture.reload(), DesktopProfileReloadOutcome::Applied);
    fixture.save("/rejected/command", Some("grid"));
    assert_eq!(
        fixture.reload(),
        DesktopProfileReloadOutcome::RestartRequired
    );
    fixture.replacement_started();
    let restored = fixture.wm.rollback_desktop_reload().unwrap();
    fixture.wm.supervisor = ProcessSupervisor::new(SupervisedProcessKind::WindowManager, restored);
    fixture.wm.force_transport_restart = false;
    let seat = SeatId::from_raw(1);
    fixture
        .wm
        .shortcuts
        .as_mut()
        .unwrap()
        .route_key(seat, 125, true);
    let held = fixture.wm.shortcuts.clone();
    assert_eq!(
        fixture.stage_configuration(&fixture.configuration()),
        sophia_protocol::PolicyProjectionOutcome::Committed
    );
    fixture.settle(false);
    assert_eq!(fixture.wm.shortcuts, held);
    fixture
        .wm
        .shortcuts
        .as_mut()
        .unwrap()
        .route_key(seat, 125, false);
    fixture.settle(true);
    assert!(fixture.wm.shortcuts.as_ref().unwrap().shortcut_idle());
    assert!(fixture.wm.public.as_ref().unwrap().configured);
}

#[test]
fn an_accepted_candidate_cannot_publish_against_a_replaced_core_snapshot() {
    let mut fixture = ReloadFixture::new();
    fixture.save("/new/command", Some("grid"));
    let old_path = fixture.policy_path();
    let old_apps = fixture.source.config.applications.clone();
    assert_eq!(
        fixture.reload(),
        DesktopProfileReloadOutcome::RestartRequired
    );
    fixture.replacement_started();
    assert_eq!(
        fixture.stage_configuration(&fixture.configuration()),
        sophia_protocol::PolicyProjectionOutcome::Committed
    );
    let changed = format!("{CORE}diagnostics verbose=#true\n");
    fixture
        .source
        .config
        .core_config_state
        .reload(changed.as_bytes())
        .unwrap();
    fixture.settle(true);
    assert_eq!(fixture.source.config.applications, old_apps);
    assert_eq!(fixture.policy_path(), old_path);
    assert!(fixture.wm.pending_policy_launch_spec.is_some());
    assert!(fixture.wm.force_transport_restart);
    assert!(!fixture.wm.desktop_reload_pending());
}

#[test]
fn policy_timeout_restores_the_active_generation_without_waiting_for_key_release() {
    let mut fixture = ReloadFixture::new();
    let old_path = fixture.policy_path();
    let old_apps = fixture.source.config.applications.clone();
    fixture.save("/new/command", Some("grid"));
    assert_eq!(
        fixture.reload(),
        DesktopProfileReloadOutcome::RestartRequired
    );
    fixture.replacement_started();
    fixture.wm.desktop_reload.as_mut().unwrap().deadline = Instant::now();
    fixture
        .wm
        .shortcuts
        .as_mut()
        .unwrap()
        .route_key(SeatId::from_raw(1), 125, true);
    assert!(!fixture.wm.shortcuts.as_ref().unwrap().shortcut_idle());
    let held = fixture.wm.shortcuts.clone();
    fixture.settle(false);
    assert_eq!(fixture.policy_path(), old_path);
    assert_eq!(fixture.source.config.applications, old_apps);
    assert!(fixture.wm.pending_policy_launch_spec.is_some());
    assert!(fixture.wm.force_transport_restart);
    assert!(!fixture.wm.desktop_reload_pending());
    assert_eq!(fixture.wm.shortcuts, held);
}

#[test]
fn initial_authority_fragments_survive_multiple_committed_policy_replacements() {
    let mut fixture = ReloadFixture::new();
    let initial = fixture.policy_path();
    let shell = fixture
        .wm
        .public
        .as_ref()
        .unwrap()
        ._profile_fragments
        .path(sophia_config::DesktopAuthority::Shell)
        .to_path_buf();
    let initial_bytes = std::fs::read(&shell).unwrap();
    fixture.save("/first/command", Some("grid"));
    assert_eq!(
        fixture.reload(),
        DesktopProfileReloadOutcome::RestartRequired
    );
    fixture.replacement_started();
    assert_eq!(
        fixture.stage_configuration(&fixture.configuration()),
        sophia_protocol::PolicyProjectionOutcome::Committed
    );
    fixture.settle(true);
    let previous_policy = fixture.policy_path();
    assert!(initial.exists());
    assert!(previous_policy.exists());
    fixture.save("/second/command", Some("tile"));
    assert_eq!(
        fixture.reload(),
        DesktopProfileReloadOutcome::RestartRequired
    );
    fixture.replacement_started();
    assert_eq!(
        fixture.stage_configuration(&fixture.configuration()),
        sophia_protocol::PolicyProjectionOutcome::Committed
    );
    fixture.settle(true);
    assert!(initial.exists());
    assert_eq!(std::fs::read(&shell).unwrap(), initial_bytes);
    assert!(!previous_policy.exists());
    assert!(fixture.policy_path().exists());
    assert_eq!(
        fixture
            .wm
            ._other_authority_fragments
            .as_ref()
            .unwrap()
            .path(sophia_config::DesktopAuthority::Policy),
        initial
    );
}

#[test]
fn output_reload_is_delivered_once_after_the_active_candidate_and_cancellation_settle() {
    use sophia_backend_live::{
        LibdrmNativeOutputCapability, LibdrmNativeOutputTiming,
        LibdrmNativeVrrPropertyDiscoveryStatus, project_live_output_authority_snapshot,
    };
    use sophia_protocol::{
        OutputHeadTargetProposal, OutputLogicalGroupProposal, OutputTopologyCandidate,
        OutputTopologyIntent, OutputTransform, OutputVrrPolicy,
    };
    let mut fixture = ReloadFixture::new();
    let public = fixture.wm.public.as_mut().unwrap();
    let output = public.outputs[0];
    let timing = LibdrmNativeOutputTiming::new(
        u32::try_from(output.size.width).unwrap(),
        u32::try_from(output.size.height).unwrap(),
        60_000,
    );
    let capability = LibdrmNativeOutputCapability::new(
        output.id,
        11,
        "DP-1",
        [timing],
        Some(timing),
        timing,
        LibdrmNativeVrrPropertyDiscoveryStatus::Discovered,
    )
    .unwrap()
    .bind_head(sophia_engine::RenderHeadId::from_raw(11))
    .unwrap();
    let snapshot =
        project_live_output_authority_snapshot(std::slice::from_ref(&capability), &[output], 7)
            .unwrap();
    let head = &snapshot.heads[0];
    let group = &snapshot.groups[0];
    let candidate = OutputTopologyCandidate {
        base_topology_epoch: snapshot.topology_epoch,
        intent: OutputTopologyIntent::Apply,
        primary_group_index: 0,
        heads: vec![OutputHeadTargetProposal {
            head: head.head,
            head_generation: head.generation,
            mode: head.current_mode.unwrap(),
            transform: OutputTransform::Normal,
            vrr: OutputVrrPolicy::Disabled,
        }],
        groups: vec![OutputLogicalGroupProposal {
            output: group.output,
            logical: group.logical,
            members: group.members.clone(),
        }],
    };
    public.output_authority = Some(
        crate::live_output_authority::LiveOutputAuthorityOwner::new(
            public.connection_epoch,
            snapshot.clone(),
        )
        .unwrap(),
    );
    public.output_capabilities = vec![capability];
    assert!(public.admit_reloaded_output_topology(candidate).unwrap());
    let transaction = public
        .output_authority
        .as_ref()
        .unwrap()
        .active_transaction()
        .unwrap();
    public.output_topology_reload_pending = true;
    assert!(!public.take_output_topology_reload_request());
    assert!(public.output_topology_reload_pending);
    assert_eq!(
        public.take_output_topology_effect().unwrap().transaction,
        transaction
    );
    public
        .request_output_candidate_cancellation("test peer disconnected".to_owned(), None)
        .unwrap();
    assert!(!public.take_output_topology_reload_request());
    let authority = public.output_authority.as_mut().unwrap();
    authority
        .fail(sophia_engine::OutputTopologyTransactionFailure::Stale)
        .unwrap();
    let settlement = authority.settle_terminal().unwrap();
    assert!(authority.active_transaction().is_none());
    assert!(public.output_cancel_requested.is_some());
    // The authority is terminal, but Session still owns cancellation settlement.
    assert!(!public.take_output_topology_reload_request());
    assert!(public.output_topology_reload_pending);
    public.finish_output_settlement(settlement).unwrap();
    assert!(!public.output_candidate_active());
    assert_eq!(public.published_output_snapshot(), Some(snapshot));
    assert!(public.take_output_topology_reload_request());
    assert!(!public.take_output_topology_reload_request());
}
