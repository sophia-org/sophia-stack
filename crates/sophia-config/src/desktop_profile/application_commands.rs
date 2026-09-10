use kdl::{KdlDocument, KdlNode};

use super::{DesktopAuthority, DesktopProfileError, DesktopProfileGeneration, DesktopProfileValue};

fn schema(message: &str) -> DesktopProfileError {
    DesktopProfileError::Schema(format!("application lowering: {message}"))
}

pub(super) fn node(value: &DesktopProfileValue) -> Result<KdlNode, DesktopProfileError> {
    let document = KdlDocument::parse_v2(&value.encoded)
        .map_err(|error| schema(&format!("invalid value: {error}")))?;
    if document.nodes().len() != 1 {
        return Err(schema("a value requires exactly one node"));
    }
    Ok(document.nodes()[0].clone())
}

pub(super) fn validate_source_names(
    authority: DesktopAuthority,
    node: &KdlNode,
) -> Result<(), DesktopProfileError> {
    let reserved = |name: &str| name.starts_with("__shortcut_");
    if authority == DesktopAuthority::Session
        && node.name().value() == "application"
        && node
            .get(0)
            .and_then(|value| value.as_string())
            .is_some_and(reserved)
    {
        return Err(schema(
            "application names beginning __shortcut_ are reserved",
        ));
    }
    if authority == DesktopAuthority::Shortcut
        && matches!(node.name().value(), "bind" | "pointer-bind")
    {
        let target = node.get(1).and_then(|value| value.as_string());
        let reserved_target = target
            .and_then(|target| target.strip_prefix("application:"))
            .is_some_and(reserved);
        let reserved_launch = node.children().is_some_and(|children| {
            children.nodes().iter().any(|child| {
                child.name().value() == "launch"
                    && child
                        .get(0)
                        .and_then(|value| value.as_string())
                        .is_some_and(reserved)
            })
        });
        if reserved_target || reserved_launch {
            return Err(schema(
                "source shortcuts cannot reference reserved generated commands",
            ));
        }
    }
    Ok(())
}

/// Moves literal shortcut commands to Session without changing source identity.
pub(crate) fn lower_application_commands(
    profile: &DesktopProfileGeneration,
) -> Result<DesktopProfileGeneration, DesktopProfileError> {
    let mut lowered = profile.clone();
    let session = lowered
        .candidates
        .get(&DesktopAuthority::Session)
        .ok_or_else(|| schema("missing session candidate"))?;
    let mut names = std::collections::BTreeSet::new();
    for value in &session.values {
        let application = node(value)?;
        if application.name().value() == "application" {
            let name = crate::application_command::single_application_identity(&application)?;
            if !names.insert(name.to_owned()) {
                return Err(schema("duplicate application identity"));
            }
        }
    }
    let mut applications = Vec::new();
    let shortcut = lowered
        .candidates
        .get_mut(&DesktopAuthority::Shortcut)
        .ok_or_else(|| schema("missing shortcut candidate"))?;
    for (ordinal, value) in shortcut.values.iter_mut().enumerate() {
        let mut binding = node(value)?;
        let Some(children) = binding.children() else {
            continue;
        };
        if binding.name().value() != "bind"
            || binding.ty().is_some()
            || binding.entries().iter().any(|entry| entry.ty().is_some())
            || binding
                .entries()
                .iter()
                .filter(|entry| entry.name().is_none())
                .count()
                != 1
            || children.nodes().len() != 1
        {
            return Err(schema(
                "command binding requires one trigger and one launch or exec child",
            ));
        }
        let command = &children.nodes()[0];
        let name = match command.name().value() {
            "launch" if command.children().is_none() => {
                crate::application_command::single_application_identity(command)?.to_owned()
            }
            "exec" => {
                crate::application_command::parse_application_command(command)?;
                let name = format!("__shortcut_{ordinal}");
                if !names.insert(name.clone()) {
                    return Err(schema(
                        "generated command identity collides with an application",
                    ));
                }
                let mut application = KdlNode::new("application");
                application.push(name.clone());
                let mut contents = KdlDocument::new();
                contents.nodes_mut().push(command.clone());
                application.set_children(contents);
                applications.push(DesktopProfileValue {
                    key: format!("session.application.{name}"),
                    encoded: application.to_string().trim().to_owned(),
                    provenance: value.provenance.clone(),
                });
                name
            }
            _ => {
                return Err(schema(
                    "command binding supports only launch or literal exec",
                ));
            }
        };
        *binding.children_mut() = None;
        binding.push(format!("application:{name}"));
        value.encoded = binding.to_string().trim().to_owned();
    }
    if names.len() > crate::SOPHIA_CONFIG_MAX_APPLICATIONS {
        return Err(schema("application registry exceeds 32 entries"));
    }
    lowered
        .candidates
        .get_mut(&DesktopAuthority::Session)
        .expect("session candidate was checked")
        .values
        .extend(applications);
    Ok(lowered)
}
