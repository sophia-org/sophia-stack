// Session command actions occupy a disjoint range. The generation is part of
// the action itself, so an activation cannot acquire a replacement slot's command.
const SESSION_COMMAND_ACTION_BASE: u64 = 0xffff_ff00_0000_0000;
const SESSION_COMMAND_GENERATION_MAX: u64 = u32::MAX as u64 - 1;

#[derive(Clone, Debug)]
struct SessionCommandRegistry {
    generation: u64,
    local_roles: bool,
    roles: BTreeMap<SessionApplicationId, WmActionId>,
    commands: BTreeMap<String, (WmActionId, std::sync::Arc<SessionApplicationSpec>)>,
}

impl SessionCommandRegistry {
    fn prepare(
        generation: u64,
        applications: &SessionApplicationConfig,
    ) -> Result<Self, &'static str> {
        if generation == 0 || generation > SESSION_COMMAND_GENERATION_MAX {
            return Err("session command generation exhausted");
        }
        if applications.command_names.len() > 32 {
            return Err("session command registry exceeds capacity");
        }
        let mut commands = BTreeMap::new();
        for (slot, name) in applications.command_names.iter().enumerate() {
            let command = applications
                .named_command(name)
                .ok_or("session command has no resolved application")?;
            let action =
                WmActionId::from_raw(SESSION_COMMAND_ACTION_BASE | (generation << 8) | slot as u64);
            commands.insert(name.clone(), (action, std::sync::Arc::new(command.clone())));
        }
        let mut roles = BTreeMap::new();
        for (id, name) in [
            (TERMINAL_APPLICATION_ID, applications.terminal.as_ref()),
            (BROWSER_APPLICATION_ID, applications.browser.as_ref()),
        ] {
            if let Some(command) = name.and_then(|name| applications.applications.get(name)) {
                let action = WmActionId::from_raw(
                    SESSION_COMMAND_ACTION_BASE | (generation << 8) | commands.len() as u64,
                );
                roles.insert(id, action);
                commands.insert(
                    format!("@role:{}", id.raw()),
                    (action, std::sync::Arc::new(command.clone())),
                );
            }
        }
        Ok(Self {
            generation,
            commands,
            roles,
            local_roles: true,
        })
    }

    fn with_policy_launch_roles(mut self, enabled: bool) -> Self {
        if enabled {
            self.roles.clear();
            self.local_roles = false;
        }
        self
    }

    fn action(&self, name: &str) -> Option<WmActionId> {
        self.commands.get(name).map(|(action, _)| *action)
    }

    fn command(&self, action: WmActionId) -> Option<std::sync::Arc<SessionApplicationSpec>> {
        self.commands
            .values()
            .find(|(id, _)| *id == action)
            .map(|(_, command)| std::sync::Arc::clone(command))
    }
}

const fn is_reserved_session_action(action: WmActionId) -> bool {
    action.raw() >= SESSION_COMMAND_ACTION_BASE
}

impl LiveWmSession {
    fn enqueue_command_shortcut(
        &mut self,
        action: WmActionId,
        launches: &mut SessionLaunchQueue,
        active_children: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let Some(command) = self.command_registry.command(action) else {
            crate::session_eprintln!(
                "sophia_session_app schema=3 status=rejected source=shortcut reason=stale_command"
            );
            return Ok(());
        };
        let transaction = self
            .public
            .as_mut()
            .ok_or("session command has no transaction owner")?
            .mint_transaction()?;
        let intent = SessionLaunchIntent {
            transaction,
            application: sophia_protocol::SessionApplicationId::from_raw(action.raw()),
            placement_classification: command.placement_classification,
        };
        let outcome = launches.enqueue_command(intent, command, active_children);
        crate::session_println!(
            "sophia_session_app schema=3 source=shortcut generation={} transaction={} outcome={outcome:?}",
            self.command_registry.generation,
            transaction.raw(),
        );
        Ok(())
    }
}
