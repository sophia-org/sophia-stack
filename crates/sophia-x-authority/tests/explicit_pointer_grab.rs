use std::num::NonZeroUsize;
use std::thread;

use sophia_protocol::{
    ClientAdmissionContext, ClientAdmissionId, ClientAuthProvenance, ClientAuthenticationMethod,
    NamespaceCapabilities, NamespaceContext, NamespaceId, NamespaceProfile,
};
use sophia_x_authority::{
    XAuthorityExplicitPointerGrabAnchor, XAuthorityExplicitPointerGrabBridgeError,
    XAuthorityExplicitPointerGrabRequestKind, XAuthorityExplicitPointerGrabResponse,
    x_authority_explicit_pointer_grab_bridge,
};

fn admission() -> ClientAdmissionContext {
    ClientAdmissionContext::new(
        ClientAdmissionId::from_raw(4),
        NamespaceContext::new(
            NamespaceId::from_raw(3),
            NamespaceProfile::Confined,
            NamespaceCapabilities::NONE,
        )
        .unwrap(),
        ClientAuthProvenance::new(ClientAuthenticationMethod::PeerCredentials, 9).unwrap(),
    )
    .unwrap()
}

#[test]
fn explicit_pointer_grab_bridge_rejects_queue_saturation() {
    let (client, owner) = x_authority_explicit_pointer_grab_bridge(NonZeroUsize::new(1).unwrap());
    let first_client = client.clone();
    let worker = thread::spawn(move || {
        first_client.request(
            admission(),
            XAuthorityExplicitPointerGrabRequestKind::Prepare {
                anchor: XAuthorityExplicitPointerGrabAnchor::AdmissionDefault,
                replaces: None,
                after_observation: None,
                control_epoch: 1,
            },
        )
    });
    while owner.pending() == 0 {
        thread::yield_now();
    }

    assert_eq!(
        client.request(
            admission(),
            XAuthorityExplicitPointerGrabRequestKind::Prepare {
                anchor: XAuthorityExplicitPointerGrabAnchor::AdmissionDefault,
                replaces: None,
                after_observation: None,
                control_epoch: 1,
            },
        ),
        Err(XAuthorityExplicitPointerGrabBridgeError::Capacity),
    );

    let request = owner.try_recv().unwrap();
    owner
        .respond(
            request.id,
            XAuthorityExplicitPointerGrabResponse::Rejected(
                sophia_x_authority::XAuthorityExplicitPointerGrabRejection::AlreadyOwned,
            ),
        )
        .unwrap();
    assert!(worker.join().unwrap().is_ok());
}

#[test]
fn explicit_pointer_grab_bridge_fails_closed_when_owner_disconnects() {
    let (client, owner) = x_authority_explicit_pointer_grab_bridge(NonZeroUsize::new(1).unwrap());
    drop(owner);

    assert_eq!(
        client.request(
            admission(),
            XAuthorityExplicitPointerGrabRequestKind::Prepare {
                anchor: XAuthorityExplicitPointerGrabAnchor::AdmissionDefault,
                replaces: None,
                after_observation: None,
                control_epoch: 1,
            },
        ),
        Err(XAuthorityExplicitPointerGrabBridgeError::Disconnected),
    );
}

#[test]
fn explicit_pointer_grab_bridge_correlates_bounded_passive_records() {
    let (client, owner) = x_authority_explicit_pointer_grab_bridge(NonZeroUsize::new(2).unwrap());
    let worker = thread::spawn(move || {
        client.request(
            admission(),
            XAuthorityExplicitPointerGrabRequestKind::Prepare {
                anchor: XAuthorityExplicitPointerGrabAnchor::AdmissionDefault,
                replaces: None,
                after_observation: None,
                control_epoch: 1,
            },
        )
    });
    let request = loop {
        if let Ok(request) = owner.try_recv() {
            break request;
        }
        thread::yield_now();
    };
    assert_eq!(request.admission, admission());
    owner
        .respond(
            request.id,
            XAuthorityExplicitPointerGrabResponse::Rejected(
                sophia_x_authority::XAuthorityExplicitPointerGrabRejection::NotViewable,
            ),
        )
        .unwrap();
    assert_eq!(
        worker.join().unwrap().unwrap(),
        XAuthorityExplicitPointerGrabResponse::Rejected(
            sophia_x_authority::XAuthorityExplicitPointerGrabRejection::NotViewable,
        )
    );
}

fn wait_request(
    owner: &sophia_x_authority::XAuthorityExplicitPointerGrabOwner,
) -> sophia_x_authority::XAuthorityExplicitPointerGrabRequest {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    loop {
        if let Ok(request) = owner.try_recv() {
            return request;
        }
        assert!(std::time::Instant::now() < deadline);
        thread::yield_now();
    }
}

fn prepare() -> XAuthorityExplicitPointerGrabRequestKind {
    XAuthorityExplicitPointerGrabRequestKind::Prepare {
        anchor: XAuthorityExplicitPointerGrabAnchor::AdmissionDefault,
        replaces: None,
        after_observation: Some(sophia_protocol::TransactionId::from_raw(41)),
        control_epoch: 7,
    }
}

#[test]
fn a_deferred_prepare_keeps_its_credit_and_does_not_block_termination() {
    let (client, owner) = x_authority_explicit_pointer_grab_bridge(NonZeroUsize::new(1).unwrap());
    let first = client.clone();
    let worker = thread::spawn(move || first.request(admission(), prepare()));
    let pending = wait_request(&owner);
    assert_eq!(owner.pending(), 1, "dequeue is not completion");
    assert_eq!(pending.kind, prepare());
    assert_eq!(
        client.request(admission(), prepare()),
        Err(XAuthorityExplicitPointerGrabBridgeError::Capacity)
    );
    let identity = sophia_protocol::ApplicationRouteLeaseIdentity {
        id: sophia_protocol::ApplicationRouteLeaseId::from_raw(3),
        seat: sophia_protocol::SeatId::from_raw(1),
        frontend_sequence: 3,
        control_epoch: 7,
    };
    let termination = thread::spawn(move || {
        client.request(
            admission(),
            XAuthorityExplicitPointerGrabRequestKind::Abort { identity },
        )
    });
    let abort = wait_request(&owner);
    assert_eq!(owner.pending(), 2);
    assert_eq!(
        owner
            .respond(abort.id, XAuthorityExplicitPointerGrabResponse::Aborted)
            .unwrap(),
        sophia_x_authority::XAuthorityExplicitPointerGrabResponseDisposition::Delivered
    );
    assert_eq!(
        termination.join().unwrap().unwrap(),
        XAuthorityExplicitPointerGrabResponse::Aborted
    );
    owner
        .respond(
            pending.id,
            XAuthorityExplicitPointerGrabResponse::Rejected(
                sophia_x_authority::XAuthorityExplicitPointerGrabRejection::Stale,
            ),
        )
        .unwrap();
    assert!(worker.join().unwrap().is_ok());
    assert_eq!(owner.pending(), 0);
    assert_eq!(
        owner
            .respond(pending.id, XAuthorityExplicitPointerGrabResponse::Aborted)
            .unwrap(),
        sophia_x_authority::XAuthorityExplicitPointerGrabResponseDisposition::Cancelled
    );
    assert_eq!(
        owner.pending(),
        0,
        "duplicate completion cannot spend credit twice"
    );
}

#[test]
fn a_timed_out_prepare_reports_late_success_as_cancelled() {
    let (client, owner) = x_authority_explicit_pointer_grab_bridge(NonZeroUsize::new(1).unwrap());
    let worker = thread::spawn(move || client.request(admission(), prepare()));
    let request = wait_request(&owner);
    assert!(!owner.is_cancelled(request.id).unwrap());
    assert_eq!(
        worker.join().unwrap(),
        Err(XAuthorityExplicitPointerGrabBridgeError::Timeout)
    );
    assert!(std::time::Instant::now() >= request.deadline);
    assert!(owner.is_cancelled(request.id).unwrap());
    assert_eq!(owner.pending(), 1, "owner still owes exact cleanup");
    assert_eq!(
        owner
            .respond(request.id, XAuthorityExplicitPointerGrabResponse::Activated)
            .unwrap(),
        sophia_x_authority::XAuthorityExplicitPointerGrabResponseDisposition::Cancelled
    );
    assert_eq!(owner.pending(), 0);
}
