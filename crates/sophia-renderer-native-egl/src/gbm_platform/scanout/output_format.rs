use super::{NativeCompositionFormatRequest, NativeGbmScanoutBufferExportDetail};

/// A format preference can be abandoned only before rendering starts. The
/// decision survives target recovery, so a retry cannot reopen admission.
pub(super) struct CompositionFormatAdmission {
    format: Option<u32>,
    may_relax: bool,
}

impl CompositionFormatAdmission {
    pub(super) fn new(request: Option<NativeCompositionFormatRequest>) -> Self {
        Self {
            format: request.map(NativeCompositionFormatRequest::fourcc),
            may_relax: matches!(request, Some(NativeCompositionFormatRequest::Preferred(_))),
        }
    }

    pub(super) const fn format(&self) -> Option<u32> {
        self.format
    }

    pub(super) fn drawing_started(&mut self) {
        self.may_relax = false;
    }

    pub(super) fn target_refused(&mut self, detail: NativeGbmScanoutBufferExportDetail) {
        if !matches!(
            detail,
            NativeGbmScanoutBufferExportDetail::EglConfigUnavailable
                | NativeGbmScanoutBufferExportDetail::GbmSurfaceUnavailable
                | NativeGbmScanoutBufferExportDetail::EglSurfaceUnavailable
        ) {
            self.may_relax = false;
        }
    }

    pub(super) fn relax(&mut self) -> bool {
        if !self.may_relax {
            return false;
        }
        self.may_relax = false;
        self.format = None;
        true
    }
}
