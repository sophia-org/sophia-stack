use std::cell::RefCell;
use std::collections::VecDeque;

use super::*;
use NativeDmaBufCapabilityError as E;

const MODIFIER_INVALID: u64 = 0x00ff_ffff_ffff_ffff;
const EXTENSIONS: &str = "EGL_EXT_image_dma_buf_import EGL_EXT_image_dma_buf_import_modifiers";

enum Call {
    Formats {
        capacity: usize,
        values: Vec<i32>,
        reported: i32,
    },
    Modifiers {
        format: i32,
        capacity: usize,
        values: Vec<u64>,
        external: Vec<u32>,
        reported: i32,
    },
    Fail(E),
}
struct Script(RefCell<VecDeque<Call>>);
impl Script {
    fn new(calls: impl IntoIterator<Item = Call>) -> Self {
        Self(RefCell::new(calls.into_iter().collect()))
    }
    fn done(&self) {
        assert!(
            self.0.borrow().is_empty(),
            "expected query was never issued"
        );
    }
}
impl CapabilityQuery for Script {
    fn formats(&self, output: &mut [i32]) -> Result<i32, E> {
        match self.0.borrow_mut().pop_front().expect("unexpected query") {
            Call::Formats {
                capacity,
                values,
                reported,
            } => {
                assert_eq!(output.len(), capacity);
                output[..values.len()].copy_from_slice(&values);
                Ok(reported)
            }
            Call::Fail(error) => Err(error),
            Call::Modifiers { .. } => panic!("formats query replaced modifiers query"),
        }
    }
    fn modifiers(
        &self,
        actual_format: i32,
        output: &mut [u64],
        external_output: &mut [u32],
    ) -> Result<i32, E> {
        match self.0.borrow_mut().pop_front().expect("unexpected query") {
            Call::Modifiers {
                format,
                capacity,
                values,
                external,
                reported,
            } => {
                assert_eq!(actual_format, format);
                assert_eq!(output.len(), capacity);
                assert_eq!(external_output.len(), capacity);
                output[..values.len()].copy_from_slice(&values);
                external_output[..external.len()].copy_from_slice(&external);
                Ok(reported)
            }
            Call::Fail(error) => Err(error),
            Call::Formats { .. } => panic!("modifiers query replaced formats query"),
        }
    }
}
fn format_count(count: i32) -> Call {
    Call::Formats {
        capacity: 0,
        values: vec![],
        reported: count,
    }
}
fn format_values(values: &[i32]) -> Call {
    Call::Formats {
        capacity: values.len(),
        values: values.to_vec(),
        reported: values.len() as i32,
    }
}
fn modifier_count(format: i32, count: i32) -> Call {
    Call::Modifiers {
        format,
        capacity: 0,
        values: vec![],
        external: vec![],
        reported: count,
    }
}
fn modifier_values(format: i32, values: &[u64], external: &[u32]) -> Call {
    Call::Modifiers {
        format,
        capacity: values.len(),
        values: values.to_vec(),
        external: external.to_vec(),
        reported: values.len() as i32,
    }
}

#[test]
fn advertised_extension_tokens_and_both_procedures_are_required() {
    assert_eq!(require_query_support(EXTENSIONS, true, true), Ok(()));
    for (extensions, formats, modifiers) in [
        ("", true, true),
        ("EGL_EXT_image_dma_buf_import", true, true),
        ("EGL_EXT_image_dma_buf_import_modifiers", true, true),
        (
            "EGL_EXT_image_dma_buf_import EGL_EXT_image_dma_buf_import_modifiers_extra",
            true,
            true,
        ),
        (EXTENSIONS, false, true),
        (EXTENSIONS, true, false),
    ] {
        assert_eq!(
            require_query_support(extensions, formats, modifiers),
            Err(E::Unavailable)
        );
    }
}

#[test]
fn capabilities_are_canonical_and_exclude_external_only_and_implicit_tokens() {
    let query = Script::new([
        format_count(4),
        format_values(&[20, 10, 20, i32::MIN]),
        modifier_count(10, 7),
        modifier_values(
            10,
            &[7, 0, 5, 7, MODIFIER_INVALID, 9, u64::MAX],
            &[0, 0, 1, 0, 0, 0, 0],
        ),
        modifier_count(20, 2),
        modifier_values(20, &[0, 8], &[1, 1]),
        modifier_count(i32::MIN, 1),
        modifier_values(i32::MIN, &[3], &[0]),
    ]);
    assert_eq!(
        collect_formats(&query),
        Ok(vec![
            NativeDmaBufImportFormat {
                format: 10,
                modifiers: vec![0, 7, 9]
            },
            NativeDmaBufImportFormat {
                format: 1 << 31,
                modifiers: vec![3]
            },
        ])
    );
    query.done();
}

#[test]
fn successful_empty_queries_do_not_invent_linear_support() {
    let empty = Script::new([format_count(0)]);
    assert_eq!(collect_formats(&empty), Ok(vec![]));
    empty.done();
    let implicit = Script::new([format_count(1), format_values(&[10]), modifier_count(10, 0)]);
    assert_eq!(collect_formats(&implicit), Ok(vec![]));
    implicit.done();
}

#[test]
fn driver_failures_and_invalid_boolean_returns_are_distinct() {
    assert_eq!(query_result(khronos_egl::TRUE, 3), Ok(3));
    assert_eq!(query_result(khronos_egl::FALSE, 3), Err(E::QueryFailed));
    assert_eq!(query_result(2, 3), Err(E::InvalidResponse));
    for calls in [
        vec![Call::Fail(E::QueryFailed)],
        vec![format_count(1), Call::Fail(E::QueryFailed)],
        vec![
            format_count(1),
            format_values(&[10]),
            Call::Fail(E::QueryFailed),
        ],
        vec![
            format_count(1),
            format_values(&[10]),
            modifier_count(10, 1),
            Call::Fail(E::QueryFailed),
        ],
    ] {
        let query = Script::new(calls);
        assert_eq!(collect_formats(&query), Err(E::QueryFailed));
        query.done();
    }
}

#[test]
fn driver_counts_are_bounded_before_allocating_or_refilling() {
    for (count, error) in [
        (-1, E::InvalidResponse),
        (513, E::LimitExceeded),
        (i32::MAX, E::LimitExceeded),
    ] {
        let query = Script::new([format_count(count)]);
        assert_eq!(collect_formats(&query), Err(error));
        query.done();
    }
    for (count, error) in [
        (-1, E::InvalidResponse),
        (4097, E::LimitExceeded),
        (i32::MAX, E::LimitExceeded),
    ] {
        let query = Script::new([
            format_count(1),
            format_values(&[10]),
            modifier_count(10, count),
        ]);
        assert_eq!(collect_formats(&query), Err(error));
        query.done();
    }
}

#[test]
fn changed_counts_and_unwritten_storage_are_not_partial_success() {
    for reported in [-1, 0, 2] {
        let query = Script::new([
            format_count(1),
            Call::Formats {
                capacity: 1,
                values: vec![10],
                reported,
            },
        ]);
        assert_eq!(collect_formats(&query), Err(E::InvalidResponse));
        query.done();
        let query = Script::new([
            format_count(1),
            format_values(&[10]),
            modifier_count(10, 1),
            Call::Modifiers {
                format: 10,
                capacity: 1,
                values: vec![0],
                external: vec![0],
                reported,
            },
        ]);
        assert_eq!(collect_formats(&query), Err(E::InvalidResponse));
        query.done();
    }
    let no_format = Script::new([
        format_count(1),
        Call::Formats {
            capacity: 1,
            values: vec![],
            reported: 1,
        },
    ]);
    assert_eq!(collect_formats(&no_format), Err(E::InvalidResponse));
    let no_external = Script::new([
        format_count(1),
        format_values(&[10]),
        modifier_count(10, 1),
        Call::Modifiers {
            format: 10,
            capacity: 1,
            values: vec![0],
            external: vec![],
            reported: 1,
        },
    ]);
    assert_eq!(collect_formats(&no_external), Err(E::InvalidResponse));
}

#[test]
fn contradictory_external_flags_cannot_advertise_a_combination() {
    for (values, external) in [(vec![7], vec![2]), (vec![7, 7], vec![0, 1])] {
        let query = Script::new([
            format_count(1),
            format_values(&[10]),
            modifier_count(10, values.len() as i32),
            modifier_values(10, &values, &external),
        ]);
        assert_eq!(collect_formats(&query), Err(E::InvalidResponse));
        query.done();
    }
}

#[test]
fn total_modifier_budget_counts_results_before_filtering_and_deduplication() {
    for overflow in [false, true] {
        let formats: Vec<i32> = (1..=if overflow { 5 } else { 4 }).collect();
        let mut calls = vec![format_count(formats.len() as i32), format_values(&formats)];
        for format in 1..=4 {
            calls.push(modifier_count(format, 4096));
            calls.push(modifier_values(format, &vec![0; 4096], &vec![1; 4096]));
        }
        if overflow {
            calls.push(modifier_count(5, 1));
        }
        let query = Script::new(calls);
        assert_eq!(
            collect_formats(&query),
            if overflow {
                Err(E::LimitExceeded)
            } else {
                Ok(vec![])
            }
        );
        query.done();
    }
}

#[test]
fn maximum_format_count_is_supported_without_extra_queries() {
    let formats: Vec<i32> = (1..=512).collect();
    let mut calls = vec![format_count(512), format_values(&formats)];
    calls.extend(formats.iter().map(|format| modifier_count(*format, 0)));
    let query = Script::new(calls);
    assert_eq!(collect_formats(&query), Ok(vec![]));
    query.done();
}
