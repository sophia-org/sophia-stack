use std::ffi::OsStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TopologyEventSource {
    Kernel,
    Processed,
}

fn is_device_node(name: &OsStr) -> bool {
    let Some(name) = name.to_str() else {
        return false;
    };
    ["card", "renderD"].into_iter().any(|prefix| {
        name.strip_prefix(prefix).is_some_and(|suffix| {
            !suffix.is_empty() && suffix.bytes().all(|byte| byte.is_ascii_digit())
        })
    })
}

pub(super) fn topology_event_requires_rescan(
    source: TopologyEventSource,
    action: udev::EventType,
    name: &OsStr,
    hotplug: bool,
) -> bool {
    match source {
        TopologyEventSource::Kernel => {
            (action == udev::EventType::Change && hotplug)
                || (is_device_node(name)
                    && matches!(action, udev::EventType::Remove | udev::EventType::Unbind))
        }
        TopologyEventSource::Processed => {
            is_device_node(name)
                && matches!(
                    action,
                    udev::EventType::Add | udev::EventType::Bind | udev::EventType::Change
                )
        }
    }
}
