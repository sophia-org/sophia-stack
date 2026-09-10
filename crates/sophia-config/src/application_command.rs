use std::path::{Path, PathBuf};

use kdl::KdlNode;

use crate::{DesktopProfileError, DesktopValueProvenance};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DesktopApplicationCommand {
    Exec {
        executable: PathBuf,
        arguments: Vec<String>,
    },
    UseCore(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DesktopApplication {
    pub name: String,
    pub command: DesktopApplicationCommand,
    pub provenance: DesktopValueProvenance,
}

fn schema(message: &str) -> DesktopProfileError {
    DesktopProfileError::Schema(format!("application command: {message}"))
}

pub(crate) fn application_identity(value: &str) -> Result<&str, DesktopProfileError> {
    if value.is_empty()
        || value.len() > crate::DESKTOP_SESSION_MAX_APPLICATION_NAME_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(schema("application identity is invalid"));
    }
    Ok(value)
}

pub(crate) fn single_application_identity(node: &KdlNode) -> Result<&str, DesktopProfileError> {
    if node.ty().is_some() || node.entries().len() != 1 {
        return Err(schema("one untyped application identity is required"));
    }
    let entry = &node.entries()[0];
    if entry.name().is_some() || entry.ty().is_some() {
        return Err(schema(
            "application identity must be positional and untyped",
        ));
    }
    application_identity(
        entry
            .value()
            .as_string()
            .ok_or_else(|| schema("application identity must be a string"))?,
    )
}

pub(crate) fn parse_application_command(
    node: &KdlNode,
) -> Result<DesktopApplicationCommand, DesktopProfileError> {
    if node.name().value() == "use-core" {
        if node.children().is_some() {
            return Err(schema("use-core cannot contain children"));
        }
        return Ok(DesktopApplicationCommand::UseCore(
            single_application_identity(node)?.to_owned(),
        ));
    }
    if node.name().value() != "exec"
        || node.ty().is_some()
        || node.children().is_some()
        || node.entries().is_empty()
        || node.entries().len() > crate::SOPHIA_CONFIG_MAX_ARGUMENTS + 1
    {
        return Err(schema(
            "exec requires an executable and at most 32 literal arguments",
        ));
    }
    let mut arguments = Vec::with_capacity(node.entries().len());
    for entry in node.entries() {
        let value = entry
            .value()
            .as_string()
            .filter(|value| {
                value.len() <= crate::SOPHIA_CONFIG_MAX_ARGUMENT_BYTES
                    && !value.chars().any(char::is_control)
            })
            .ok_or_else(|| schema("argv requires bounded strings without control characters"))?;
        if entry.ty().is_some() || entry.name().is_some() {
            return Err(schema("argv must be positional and untyped"));
        }
        arguments.push(value.to_owned());
    }
    let executable = arguments.remove(0);
    if executable.is_empty()
        || matches!(executable.as_str(), "." | "..")
        || (!Path::new(&executable).is_absolute() && executable.contains('/'))
    {
        return Err(schema("executable must be a PATH name or an absolute path"));
    }
    Ok(DesktopApplicationCommand::Exec {
        executable: executable.into(),
        arguments,
    })
}

pub(crate) fn parse_application(
    node: &KdlNode,
    provenance: &DesktopValueProvenance,
) -> Result<DesktopApplication, DesktopProfileError> {
    let name = single_application_identity(node)?.to_owned();
    let children = node
        .children()
        .filter(|children| children.nodes().len() == 1)
        .ok_or_else(|| schema("application requires exactly one exec or use-core command"))?;
    Ok(DesktopApplication {
        name,
        command: parse_application_command(&children.nodes()[0])?,
        provenance: provenance.clone(),
    })
}
