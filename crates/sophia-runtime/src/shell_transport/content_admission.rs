use sophia_protocol::{
    ContentAdmissionRefused, SOPHIA_SHELL_CAPABILITY_CONTENT_DISCRETE_INPUT,
    SOPHIA_SHELL_CAPABILITY_CONTENT_SURFACE, SOPHIA_SHELL_CONTENT_REVISION,
};

const PERMISSION_DENIED: u16 = 1;
const UNAVAILABLE: u16 = 4;

/// Operator and implementation decision for the revision-5 content workflow.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ShellContentAdmissionPolicy {
    /// The host has no complete content service. This is the production default
    /// until the owner loop and renderer lifecycle are wired end to end.
    #[default]
    Unavailable,
    /// The implementation exists but the effective operator profile denies it.
    Denied,
    /// CPU content is admitted. Discrete input remains an independent grant.
    Granted { discrete_input: bool },
}

pub(super) enum ContentAdmissionDecision {
    NotRequested,
    Granted(u64),
    Refused(ContentAdmissionRefused),
}

pub(super) fn decide(
    revision: u16,
    requested: u64,
    policy: ShellContentAdmissionPolicy,
) -> ContentAdmissionDecision {
    if requested == 0 || revision < SOPHIA_SHELL_CONTENT_REVISION {
        return ContentAdmissionDecision::NotRequested;
    }
    let refused = |reason, denied_capabilities| {
        ContentAdmissionDecision::Refused(ContentAdmissionRefused {
            reason,
            denied_capabilities,
        })
    };
    match policy {
        ShellContentAdmissionPolicy::Unavailable => refused(UNAVAILABLE, requested),
        ShellContentAdmissionPolicy::Denied => refused(PERMISSION_DENIED, requested),
        ShellContentAdmissionPolicy::Granted { discrete_input } => {
            let denied_input =
                requested & SOPHIA_SHELL_CAPABILITY_CONTENT_DISCRETE_INPUT != 0 && !discrete_input;
            if denied_input {
                refused(
                    PERMISSION_DENIED,
                    SOPHIA_SHELL_CAPABILITY_CONTENT_DISCRETE_INPUT,
                )
            } else {
                ContentAdmissionDecision::Granted(
                    SOPHIA_SHELL_CAPABILITY_CONTENT_SURFACE
                        | (requested & SOPHIA_SHELL_CAPABILITY_CONTENT_DISCRETE_INPUT),
                )
            }
        }
    }
}
