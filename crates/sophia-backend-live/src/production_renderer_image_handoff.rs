use sophia_renderer_live::LiveRendererImageId;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveRendererImageHandoffAdmission {
    Ready,
    Missing,
    InvalidIdentity,
    DuplicateIdentity,
    CoverageMismatch,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LiveRendererImageResumePhase {
    #[default]
    AwaitingOutputOwner,
    AwaitingImageRestore,
    Ready,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveRendererImageResumeObservation {
    OutputOwnerInitialized,
    ImagesRestored,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveRendererImageResumeTransition {
    Advanced(LiveRendererImageResumePhase),
    Rejected,
}

pub const fn reduce_live_renderer_image_resume_observation(
    phase: LiveRendererImageResumePhase,
    observation: LiveRendererImageResumeObservation,
) -> LiveRendererImageResumeTransition {
    use LiveRendererImageResumeObservation::{ImagesRestored, OutputOwnerInitialized};
    use LiveRendererImageResumePhase::{AwaitingImageRestore, AwaitingOutputOwner, Ready};

    match (phase, observation) {
        (AwaitingOutputOwner, OutputOwnerInitialized) => {
            LiveRendererImageResumeTransition::Advanced(AwaitingImageRestore)
        }
        (AwaitingImageRestore, ImagesRestored) => {
            LiveRendererImageResumeTransition::Advanced(Ready)
        }
        _ => LiveRendererImageResumeTransition::Rejected,
    }
}

pub fn reduce_live_renderer_image_handoff_admission(
    retained: &[LiveRendererImageId],
    handoff: Option<&[LiveRendererImageId]>,
) -> LiveRendererImageHandoffAdmission {
    if retained.iter().any(|image| !image.is_valid())
        || handoff.into_iter().flatten().any(|image| !image.is_valid())
    {
        return LiveRendererImageHandoffAdmission::InvalidIdentity;
    }

    let retained_set = retained.iter().copied().collect::<BTreeSet<_>>();
    if retained_set.len() != retained.len() {
        return LiveRendererImageHandoffAdmission::DuplicateIdentity;
    }

    let Some(handoff) = handoff else {
        return if retained.is_empty() {
            LiveRendererImageHandoffAdmission::Ready
        } else {
            LiveRendererImageHandoffAdmission::Missing
        };
    };
    let handoff_set = handoff.iter().copied().collect::<BTreeSet<_>>();
    if handoff_set.len() != handoff.len() {
        return LiveRendererImageHandoffAdmission::DuplicateIdentity;
    }
    if retained_set == handoff_set {
        LiveRendererImageHandoffAdmission::Ready
    } else {
        LiveRendererImageHandoffAdmission::CoverageMismatch
    }
}

/// Collect opaque snapshots while the native owner is quiescent. Stores are
/// sparse: a head need never have drawn the pixels retained on another head.
/// Absence in one store is ordinary; absence across the complete owner set is
/// a lost scene obligation. Failed exports still abort the whole handoff.
pub fn collect_live_renderer_image_handoff<T>(
    retained: &[LiveRendererImageId],
    owner_count: usize,
    mut export: impl FnMut(usize, LiveRendererImageId) -> Result<Option<T>, Box<dyn std::error::Error>>,
) -> Result<Vec<Vec<T>>, Box<dyn std::error::Error>> {
    if reduce_live_renderer_image_handoff_admission(retained, Some(retained))
        != LiveRendererImageHandoffAdmission::Ready
    {
        return Err("renderer-image handoff has invalid retained identities".into());
    }
    if owner_count == 0 {
        return Err("renderer-image handoff has no renderer owners".into());
    }
    let mut covered = BTreeSet::new();
    let mut owners = Vec::with_capacity(owner_count);
    for owner in 0..owner_count {
        let mut snapshots = Vec::new();
        for image in retained {
            if let Some(snapshot) = export(owner, *image)? {
                covered.insert(*image);
                snapshots.push(snapshot);
            }
        }
        owners.push(snapshots);
    }
    if covered.len() != retained.len() {
        return Err("retained scene refers to an unavailable promoted renderer image".into());
    }
    Ok(owners)
}

/// Return snapshot indices to import at each head. Owner keys identify image
/// stores, so heads sharing a worker import their overlapping images once.
pub fn plan_live_renderer_image_restore(
    owners: &[(usize, Vec<LiveRendererImageId>)],
) -> Result<Vec<Vec<usize>>, LiveRendererImageHandoffAdmission> {
    let mut imported = BTreeSet::new();
    let mut plan = Vec::with_capacity(owners.len());
    for (owner, images) in owners {
        let admission = reduce_live_renderer_image_handoff_admission(images, Some(images));
        if admission != LiveRendererImageHandoffAdmission::Ready {
            return Err(admission);
        }
        plan.push(
            images
                .iter()
                .enumerate()
                .filter_map(|(index, image)| imported.insert((*owner, *image)).then_some(index))
                .collect(),
        );
    }
    Ok(plan)
}
