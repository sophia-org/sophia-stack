use sophia_protocol::*;

fn request(target: Option<SurfaceId>) -> PolicyProjectionRequest {
    PolicyProjectionRequest {
        connection_epoch: 1,
        request_id: 2,
        scene_generation: 3,
        policy_generation: 4,
        affected_outputs: vec![OutputId::from_raw(1), OutputId::from_raw(2)],
        cause: PolicyRequestCause::PointerFocus {
            output: OutputId::from_raw(2),
            target,
        },
    }
}

#[test]
fn pointer_focus_wire_preserves_empty_and_window_targets_without_coordinates() {
    for target in [None, Some(SurfaceId::new(0, 1)), Some(SurfaceId::new(7, 3))] {
        let original = request(target);
        let wire = encode_wm_v1_policy_projection_request(&original).unwrap();
        assert_eq!(wire.cause_kind, 4);
        assert_eq!(wire.action, 2);
        assert_eq!(wire.activation_serial, 0);
        assert_eq!(
            decode_wm_v1_policy_projection_request(&wire).unwrap(),
            original
        );
    }
}

#[test]
fn pointer_focus_wire_rejects_ambiguous_targets_and_foreign_fields() {
    let valid = encode_wm_v1_policy_projection_request(&request(None)).unwrap();
    for mutation in 0..12 {
        let mut bad = valid.clone();
        match mutation {
            0 => bad.action = 0,
            1 => bad.action = 3,
            2 => bad.target_index = 7,
            3 => {
                bad.target_index = u32::MAX;
                bad.target_generation = 1;
            }
            4 => bad.activation_serial = 1,
            5 => bad.interaction_phase = 1,
            6 => bad.interaction_kind = 1,
            7 => bad.interaction_axis = 1,
            8 => bad.interaction_x = 1,
            9 => bad.interaction_y = 1,
            10 => bad.interaction_width = 1,
            _ => bad.interaction_height = 1,
        }
        assert!(
            decode_wm_v1_policy_projection_request(&bad).is_err(),
            "mutation {mutation}"
        );
    }
    let mut invalid = request(None);
    invalid.affected_outputs = vec![OutputId::from_raw(1)];
    assert!(encode_wm_v1_policy_projection_request(&invalid).is_err());
}
