use super::{
    LibdrmNativeAtomicCommitFlagsReport, LibdrmNativeAtomicCommitRequestScope,
    LibdrmNativeAtomicCommitSubmitStatus, LibdrmNativeAtomicRequestEvidence,
};

/// One actual TEST_ONLY result. Errors describe the ioctl, not a layout cause;
/// flags describe the test rather than the subsequent committing request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LibdrmNativeAtomicTestReport {
    pub status: LibdrmNativeAtomicCommitSubmitStatus,
    pub request: Option<LibdrmNativeAtomicRequestEvidence>,
    pub error_kind: Option<std::io::ErrorKind>,
    pub raw_os_error: Option<i32>,
    pub request_scope: LibdrmNativeAtomicCommitRequestScope,
    pub commit_flags: LibdrmNativeAtomicCommitFlagsReport,
}
