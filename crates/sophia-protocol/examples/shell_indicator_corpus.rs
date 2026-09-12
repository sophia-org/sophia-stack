use sophia_protocol::*;

fn main() {
    let transaction = TransactionId::from_raw(8);
    // Two outputs, and the second is active while holding no indicator at all.
    // That is the shape this vocabulary exists to carry, so the golden bytes
    // pin it rather than a case where focus could be inferred from an entry.
    let snapshot = ShellIndicatorSnapshot {
        connection_epoch: 5,
        generation: 6,
        active_output: Some(OutputId::from_raw(2)),
        statuses: vec![
            ShellOutputStatus {
                output: OutputId::from_raw(1),
                focus_bits: 0,
                layout: "Scroller".to_owned(),
            },
            ShellOutputStatus {
                output: OutputId::from_raw(2),
                focus_bits: 1,
                layout: "Scroller".to_owned(),
            },
        ],
        indicators: vec![
            ShellIndicator {
                output: OutputId::from_raw(1),
                indicator: 11,
                action: 41,
                slot: 0,
                state_bits: 1,
                label: "web".to_owned(),
            },
            ShellIndicator {
                output: OutputId::from_raw(1),
                indicator: 12,
                action: 0,
                slot: 1,
                state_bits: 0,
                label: "code".to_owned(),
            },
        ],
    };
    for (index, frame) in encode_shell_indicator_snapshot(transaction, &snapshot)
        .unwrap()
        .iter()
        .enumerate()
    {
        print_frame(&format!("snapshot{index}"), frame);
    }

    let empty = ShellIndicatorSnapshot {
        connection_epoch: 5,
        generation: 7,
        active_output: None,
        statuses: Vec::new(),
        indicators: Vec::new(),
    };
    for (index, frame) in encode_shell_indicator_snapshot(transaction, &empty)
        .unwrap()
        .iter()
        .enumerate()
    {
        print_frame(&format!("empty{index}"), frame);
    }

    print_frame(
        "activate",
        &encode_shell_indicator_activation(
            transaction,
            &ShellIndicatorActivation {
                connection_epoch: 5,
                snapshot_generation: 6,
                output: OutputId::from_raw(1),
                indicator: 11,
                action: 41,
                event_id: 77,
            },
        )
        .unwrap(),
    );
    for (name, status) in [
        ("accepted", ShellIndicatorActivationStatus::Accepted),
        ("stale", ShellIndicatorActivationStatus::Stale),
        ("unknown", ShellIndicatorActivationStatus::Unknown),
        ("unauthorized", ShellIndicatorActivationStatus::Unauthorized),
    ] {
        print_frame(
            &format!("outcome_{name}"),
            &encode_shell_indicator_activation_outcome(
                transaction,
                &ShellIndicatorActivationOutcome {
                    connection_epoch: 5,
                    snapshot_generation: 6,
                    event_id: 77,
                    status,
                    reason: 0,
                },
            )
            .unwrap(),
        );
    }
}

fn print_frame(kind: &str, bytes: &[u8]) {
    println!(
        "{kind}|{}",
        bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()
    );
}
