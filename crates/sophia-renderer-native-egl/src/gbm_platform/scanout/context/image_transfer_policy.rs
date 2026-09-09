#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum BridgeSelection {
    Reuse(usize),
    Replace(usize),
    Allocate,
    Deferred,
}

/// A matching busy slot remains unavailable until GPU completion is observed.
pub(super) fn select_bridge(
    slots: impl Iterator<Item = (bool, bool)>,
    capacity: usize,
) -> BridgeSelection {
    let mut count = 0;
    let mut idle = None;
    for (index, (completed, matches)) in slots.enumerate() {
        count += 1;
        if completed {
            if matches {
                return BridgeSelection::Reuse(index);
            }
            idle.get_or_insert(index);
        }
    }
    if let Some(index) = idle {
        BridgeSelection::Replace(index)
    } else if count < capacity {
        BridgeSelection::Allocate
    } else {
        BridgeSelection::Deferred
    }
}

/// A remembered slot only reorders the bounded inventory; every attempt imports.
pub(super) fn source_attempt_order(
    count: usize,
    preferred: Option<usize>,
) -> impl Iterator<Item = usize> {
    preferred
        .into_iter()
        .filter(move |index| *index < count)
        .chain((0..count).filter(move |index| Some(*index) != preferred))
}

/// Only an actual import refusal enters fallback. Allocation, context and
/// lifecycle failures retain their original outcome.
pub(super) fn capture_direct_or_transfer<S, T, E: Copy>(
    state: &mut S,
    direct: impl FnOnce(&mut S) -> Result<T, E>,
    is_import_failure: impl FnOnce(E) -> bool,
    transfer: impl FnOnce(&mut S, E) -> Result<T, E>,
) -> Result<T, E> {
    match direct(state) {
        Ok(captured) => Ok(captured),
        Err(error) if is_import_failure(error) => transfer(state, error),
        Err(error) => Err(error),
    }
}
