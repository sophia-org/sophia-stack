#![cfg(all(feature = "libdrm-events", feature = "gbm-probe"))]

use sophia_backend_live::{
    LiveRendererImageHandoffAdmission, LiveRendererImageResumeObservation,
    LiveRendererImageResumePhase, LiveRendererImageResumeTransition,
    collect_live_renderer_image_handoff, plan_live_renderer_image_restore,
    reduce_live_renderer_image_handoff_admission, reduce_live_renderer_image_resume_observation,
};
use sophia_renderer_live::LiveRendererImageId;

fn image(raw: u64) -> LiveRendererImageId {
    LiveRendererImageId::from_raw(raw)
}

#[test]
fn renderer_handoff_requires_exact_unique_retained_image_coverage() {
    use LiveRendererImageHandoffAdmission::{
        CoverageMismatch, DuplicateIdentity, InvalidIdentity, Missing, Ready,
    };

    assert_eq!(
        reduce_live_renderer_image_handoff_admission(&[], None),
        Ready
    );
    assert_eq!(
        reduce_live_renderer_image_handoff_admission(
            &[image(2), image(1)],
            Some(&[image(1), image(2)]),
        ),
        Ready
    );
    assert_eq!(
        reduce_live_renderer_image_handoff_admission(&[image(1)], None),
        Missing
    );
    assert_eq!(
        reduce_live_renderer_image_handoff_admission(&[image(1), image(2)], Some(&[image(1)]),),
        CoverageMismatch
    );
    assert_eq!(
        reduce_live_renderer_image_handoff_admission(&[image(1)], Some(&[image(1), image(2)]),),
        CoverageMismatch
    );
    assert_eq!(
        reduce_live_renderer_image_handoff_admission(&[image(1), image(1)], Some(&[image(1)]),),
        DuplicateIdentity
    );
    assert_eq!(
        reduce_live_renderer_image_handoff_admission(&[image(1)], Some(&[image(1), image(1)]),),
        DuplicateIdentity
    );
    assert_eq!(
        reduce_live_renderer_image_handoff_admission(&[image(0)], Some(&[image(0)])),
        InvalidIdentity
    );
}

#[test]
fn renderer_handoff_resume_initializes_the_output_owner_before_restoring_images() {
    use LiveRendererImageResumeObservation::{ImagesRestored, OutputOwnerInitialized};
    use LiveRendererImageResumePhase::{AwaitingImageRestore, AwaitingOutputOwner, Ready};
    use LiveRendererImageResumeTransition::{Advanced, Rejected};

    assert_eq!(
        reduce_live_renderer_image_resume_observation(AwaitingOutputOwner, ImagesRestored),
        Rejected
    );
    assert_eq!(
        reduce_live_renderer_image_resume_observation(AwaitingOutputOwner, OutputOwnerInitialized,),
        Advanced(AwaitingImageRestore)
    );
    assert_eq!(
        reduce_live_renderer_image_resume_observation(AwaitingImageRestore, OutputOwnerInitialized,),
        Rejected
    );
    assert_eq!(
        reduce_live_renderer_image_resume_observation(AwaitingImageRestore, ImagesRestored),
        Advanced(Ready)
    );
    assert_eq!(
        reduce_live_renderer_image_resume_observation(Ready, ImagesRestored),
        Rejected
    );
}

#[test]
fn two_outputs_handoff_the_union_of_their_sparse_renderer_stores() {
    // A panel on each output and a terminal on only the first. The old
    // first-output-only export demanded image 3 in owner 0 and aborted.
    let stores = [vec![image(1), image(2)], vec![image(3)], vec![]];
    let retained = [image(1), image(2), image(3)];
    let mut visited = Vec::new();
    let captured = collect_live_renderer_image_handoff(&retained, stores.len(), |owner, id| {
        visited.push((owner, id));
        Ok(stores[owner].contains(&id).then_some(id))
    })
    .unwrap();
    assert_eq!(captured, stores);
    assert_eq!(visited.len(), 9);
    let plan =
        plan_live_renderer_image_restore(&captured.into_iter().enumerate().collect::<Vec<_>>())
            .unwrap();
    assert_eq!(plan, [vec![0, 1], vec![0], vec![]]);
}

#[test]
fn mirrored_images_restore_once_per_store_including_shared_workers() {
    let images = vec![image(1), image(2)];
    assert_eq!(
        plan_live_renderer_image_restore(&[(0, images.clone()), (1, images.clone())]).unwrap(),
        [vec![0, 1], vec![0, 1]],
    );
    // The first two heads share a worker; a third is on a different device.
    assert_eq!(
        plan_live_renderer_image_restore(&[(7, vec![image(1)]), (7, images.clone()), (8, images),])
            .unwrap(),
        [vec![0], vec![1], vec![0, 1]],
    );
}

#[test]
fn sparse_handoff_still_rejects_missing_images_and_invalid_identities() {
    let missing = collect_live_renderer_image_handoff(&[image(1), image(2)], 2, |_, id| {
        Ok((id == image(1)).then_some(id))
    })
    .unwrap_err();
    assert_eq!(
        missing.to_string(),
        "retained scene refers to an unavailable promoted renderer image"
    );
    for retained in [vec![image(0)], vec![image(1), image(1)]] {
        assert!(
            collect_live_renderer_image_handoff::<()>(&retained, 2, |_, _| {
                panic!("invalid retention must fail before exporting")
            })
            .is_err()
        );
    }
    assert!(collect_live_renderer_image_handoff::<()>(&[], 0, |_, _| unreachable!()).is_err());
    assert_eq!(
        plan_live_renderer_image_restore(&[(0, vec![image(1), image(1)])]),
        Err(LiveRendererImageHandoffAdmission::DuplicateIdentity),
    );
}

#[test]
fn partial_handoff_drops_its_snapshots_and_preserves_export_failure() {
    use sophia_renderer_live::LiveRendererScanoutBufferExportDetail as Detail;
    use std::cell::Cell;
    use std::rc::Rc;

    struct Snapshot(Rc<Cell<usize>>);
    impl Drop for Snapshot {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }
    let released = Rc::new(Cell::new(0));
    let result = collect_live_renderer_image_handoff(&[image(1)], 2, |owner, _| {
        if owner == 0 {
            Ok(Some(Snapshot(released.clone())))
        } else {
            Err(Detail::WorkerDisconnected.into())
        }
    });
    let error = result.err().expect("an export failure must abort capture");
    assert_eq!(
        error.downcast_ref::<Detail>(),
        Some(&Detail::WorkerDisconnected)
    );
    assert_eq!(released.get(), 1);
}
