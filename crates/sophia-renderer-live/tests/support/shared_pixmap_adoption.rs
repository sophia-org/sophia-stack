fn allocation_for_adoption() -> LiveSharedBufferAllocation {
    LiveSharedBufferAllocation {
        descriptor: DmaBufDescriptor {
            handle: BufferHandle::from_raw(17),
            size: Size {
                width: 1,
                height: 1,
            },
            format: sophia_protocol::DRM_FORMAT_ARGB8888,
            modifier: 0,
            plane_count: 1,
            planes: [
                Some(DmaBufPlaneDescriptor {
                    offset: 0,
                    stride: 4,
                }),
                None,
                None,
                None,
            ],
        },
        plane_fds: Vec::new(),
    }
}

#[test]
fn a_timed_out_receiver_cannot_orphan_a_buffered_allocation_reply() {
    let (reply, receipt) = allocation_reply();
    let expired = Instant::now();
    assert!(matches!(
        receipt.receive(expired),
        Err(LiveSharedPixmapError::Unavailable)
    ));

    // Keep the receiver alive: send succeeds even though its caller timed out.
    assert_eq!(
        reply.deliver(Ok(allocation_for_adoption()), expired),
        AllocationDisposition::Unclaimed,
    );
    assert!(receipt.receiver.try_recv().is_ok());
    assert!(!receipt.adoption.adopt());
}

#[test]
fn an_expired_worker_cannot_hand_an_allocation_to_a_late_receiver() {
    let (reply, receipt) = allocation_reply();
    assert_eq!(
        reply.deliver(Ok(allocation_for_adoption()), Instant::now()),
        AllocationDisposition::Unclaimed,
    );
    assert!(matches!(
        receipt.receive(Instant::now() + Duration::from_secs(10)),
        Err(LiveSharedPixmapError::Unavailable),
    ));
}

#[test]
fn an_already_buffered_reply_does_not_extend_the_adoption_deadline() {
    let (reply, receipt) = allocation_reply();
    reply.sender.send(Ok(allocation_for_adoption())).unwrap();
    assert!(matches!(
        receipt.receive(Instant::now()),
        Err(LiveSharedPixmapError::Unavailable),
    ));
    assert_eq!(
        receipt.adoption.0.load(Ordering::Acquire),
        ALLOCATION_CANCELLED,
    );
}

#[test]
fn receiving_an_allocation_adopts_it_before_the_worker_returns() {
    let (reply, receipt) = allocation_reply();
    let deadline = Instant::now() + Duration::from_secs(10);
    let worker = std::thread::spawn(move || reply.deliver(Ok(allocation_for_adoption()), deadline));
    let allocation = receipt.receive(deadline).unwrap();
    assert_eq!(allocation.descriptor.handle, BufferHandle::from_raw(17));
    assert_eq!(worker.join().unwrap(), AllocationDisposition::Adopted);
    receipt.adoption.cancel();
    assert_eq!(
        receipt.adoption.0.load(Ordering::Acquire),
        ALLOCATION_ADOPTED
    );
}

#[test]
fn a_disconnected_receiver_leaves_its_allocation_unclaimed() {
    let (reply, receipt) = allocation_reply();
    drop(receipt);
    assert_eq!(
        reply.deliver(
            Ok(allocation_for_adoption()),
            Instant::now() + Duration::from_secs(10),
        ),
        AllocationDisposition::Unclaimed,
    );
}

#[test]
fn an_allocation_failure_does_not_release_an_existing_backing() {
    let (reply, receipt) = allocation_reply();
    let deadline = Instant::now() + Duration::from_secs(10);
    assert_eq!(
        reply.deliver(Err(LiveSharedPixmapError::IdentityInUse), deadline),
        AllocationDisposition::NoAllocation,
    );
    assert!(matches!(
        receipt.receive(deadline),
        Err(LiveSharedPixmapError::IdentityInUse)
    ));
}
