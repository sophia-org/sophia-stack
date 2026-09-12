# Source mapping and future trace boundary

| Model action | Implementation boundary |
| --- | --- |
| Admit | `routing/recovery.rs` `InputRecovery::admit`, before `XAuthorityRoutedInputSender` publishes; owner `InputDeliveryState::track` before next service |
| Bind | `InputRecovery::bind`, after `route_engine_input` resolves X grabs, before private queue publication |
| StartWrite | `connection/writers/input.rs` checks `delivery_active`, then acquires output serialization and rechecks |
| WriteReturns | successful `write_all` and `flush`, before acquiring delivery ledger for finish |
| Finish | `InputRecovery::finish` / `terminal_locked`, under the ledger mutex |
| CancelUnresolved | `InputRecovery::recover`, `client=None`, under the ledger mutex |
| Disconnect | `disconnect_locked`, socket shutdown before terminal outcomes; independent socket clone, no output mutex |
| Observe | `InputDeliveryPhase::drain_at` authenticates through `observe_delivery`, then `settle_input_delivery` removes exact pending/barrier IDs |
| DispatchControl | `SessionControlQueue::service_when`, only after barrier empty |
| RequestVt/Handoff | `owner_loop/lifecycle.rs`, 500 ms grace then recover all, drain receipts, then advance input security epoch/switch |
| Tick | monotonic `Instant`, deadlines never reset by subsequent input |

For emitted trace validation, capture each transition while holding its actual
ledger/owner lock: event name, monotonic delivery ID, actual receiver or explicit
unresolved, surface ID+generation, seat, control epoch, original admission age,
phase, terminal outcome, pending membership and barrier membership. Socket
shutdown and terminal publication are ordered but may only be represented as
one action when their mutex exclusion is retained. Never log event bytes,
keycodes, text, passwords, titles, clipboard or arbitrary Display messages.
The current recorder intentionally emits recovery/terminal summaries rather than
all these transitions, so those summaries alone cannot validate the full model.
