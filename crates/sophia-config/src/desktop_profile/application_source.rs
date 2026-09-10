use std::collections::BTreeMap;

use kdl::{KdlDocument, KdlNode};

use super::application_commands::{lower_application_commands, node};
use super::{DesktopAuthority, DesktopProfileError, DesktopProfileGeneration};

fn schema() -> DesktopProfileError {
    DesktopProfileError::Schema(
        "generated application cannot be restored to its source binding".into(),
    )
}

/// Renders source syntax while retaining literal commands and named applications.
pub fn render_desktop_profile_source(
    profile: &DesktopProfileGeneration,
) -> Result<String, DesktopProfileError> {
    let profile = lower_application_commands(profile)?;
    super::prepare_desktop_profile_candidates(&profile)?;
    let mut commands = BTreeMap::new();
    for value in &profile.candidates[&DesktopAuthority::Session].values {
        let application = node(value)?;
        if application.name().value() != "application" {
            continue;
        }
        let name = crate::application_command::single_application_identity(&application)?;
        if name.starts_with("__shortcut_") {
            let children = application.children().ok_or_else(schema)?;
            let command = children.nodes().first().ok_or_else(schema)?;
            if command.name().value() != "exec"
                || commands.insert(name.to_owned(), command.clone()).is_some()
            {
                return Err(schema());
            }
        }
    }
    let mut document = KdlDocument::new();
    let mut version = KdlNode::new("schema");
    version.push(1);
    document.nodes_mut().push(version);
    for authority in DesktopAuthority::ALL {
        let mut contents = KdlDocument::new();
        for (ordinal, value) in profile.candidates[&authority].values.iter().enumerate() {
            let mut setting = node(value)?;
            if authority == DesktopAuthority::Session
                && setting.name().value() == "application"
                && crate::application_command::single_application_identity(&setting)?
                    .starts_with("__shortcut_")
            {
                continue;
            }
            if authority == DesktopAuthority::Shortcut && setting.name().value() == "bind" {
                let generated = setting
                    .get(1)
                    .and_then(|value| value.as_string())
                    .and_then(|target| target.strip_prefix("application:"))
                    .filter(|name| name.starts_with("__shortcut_"));
                if let Some(name) = generated {
                    if name != format!("__shortcut_{ordinal}") {
                        return Err(schema());
                    }
                    let command = commands.remove(name).ok_or_else(schema)?;
                    let target = setting
                        .entries()
                        .iter()
                        .enumerate()
                        .filter(|(_, entry)| entry.name().is_none())
                        .nth(1)
                        .map(|(index, _)| index)
                        .ok_or_else(schema)?;
                    setting.entries_mut().remove(target);
                    let mut children = KdlDocument::new();
                    children.nodes_mut().push(command);
                    setting.set_children(children);
                }
            }
            contents.nodes_mut().push(setting);
        }
        let mut section = KdlNode::new(authority.name());
        section.set_children(contents);
        document.nodes_mut().push(section);
    }
    if !commands.is_empty() {
        return Err(schema());
    }
    document.autoformat();
    Ok(document.to_string())
}
