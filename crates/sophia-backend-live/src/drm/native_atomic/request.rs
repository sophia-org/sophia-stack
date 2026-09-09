mod properties;
use properties::CanonicalAtomicProperties;
pub use properties::LibdrmNativeAtomicProperty;

/// Canonical built request state. Equality does not explain a driver refusal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LibdrmNativeAtomicRequestEvidence {
    properties: CanonicalAtomicProperties,
    primary_framebuffer: (u32, u32),
    pub scope: LibdrmNativeAtomicCommitRequestScope,
    pub flags: LibdrmNativeAtomicCommitFlagsReport,
}

impl LibdrmNativeAtomicRequestEvidence {
    pub fn properties(&self) -> &[LibdrmNativeAtomicProperty] {
        self.properties.as_slice()
    }

    pub const fn primary_framebuffer(&self) -> (u32, u32) {
        self.primary_framebuffer
    }

    /// Requires a different primary FB value and identical remaining state.
    pub fn equivalent_except_primary_framebuffer(&self, other: &Self) -> bool {
        if self.scope != other.scope
            || self.flags != other.flags
            || self.primary_framebuffer != other.primary_framebuffer
            || self.properties().len() != other.properties().len()
        {
            return false;
        }
        let mut different_framebuffer = false;
        for (left, right) in self.properties().iter().zip(other.properties()) {
            if (left.object, left.property) != (right.object, right.property) {
                return false;
            }
            if (left.object, left.property) == self.primary_framebuffer {
                different_framebuffer = left.value != right.value;
            } else if left.value != right.value {
                return false;
            }
        }
        different_framebuffer
    }
}

/// Lowers every property once, sharing the raw values with the evidence collector.
#[derive(Default)]
pub(crate) struct LibdrmNativeRecordedAtomicRequest {
    request: drm::control::atomic::AtomicModeReq,
    properties: Option<CanonicalAtomicProperties>,
}

impl LibdrmNativeRecordedAtomicRequest {
    pub(crate) fn new() -> Self {
        Self {
            request: drm::control::atomic::AtomicModeReq::new(),
            properties: Some(CanonicalAtomicProperties::new()),
        }
    }

    pub(crate) fn add_property<H: drm::control::ResourceHandle>(
        &mut self,
        object: H,
        property: drm::control::property::Handle,
        value: drm::control::property::Value<'_>,
    ) {
        let object: drm::control::RawResourceHandle = object.into();
        let value = value.into();
        self.request.add_raw_property(object, property, value);
        if self.properties.as_mut().is_some_and(|properties| {
            !properties.insert(LibdrmNativeAtomicProperty {
                object: object.into(),
                property: property.into(),
                value,
            })
        }) {
            self.properties = None;
        }
    }

    pub(crate) fn finish(
        self,
        scope: LibdrmNativeAtomicCommitRequestScope,
        primary_framebuffer: (u32, u32),
    ) -> LibdrmNativeAtomicCommitRequest {
        let mut request = match scope {
            LibdrmNativeAtomicCommitRequestScope::PageFlip => {
                LibdrmNativeAtomicCommitRequest::new(self.request)
            }
            LibdrmNativeAtomicCommitRequestScope::Modeset => {
                LibdrmNativeAtomicCommitRequest::modeset(self.request)
            }
        };
        request.properties = self.properties;
        request.primary_framebuffer = Some(primary_framebuffer);
        request
    }
}

/// Cloneable so one built request can be asked about and then performed.
///
/// A validating commit and the flip that follows it must describe the exact
/// same framebuffer, or the driver's answer was about something else. Cloning
/// the request is what makes that literal rather than approximate: the
/// alternative is building a second framebuffer and trusting that identical
/// inputs produce an identically scannable one.
#[derive(Clone, Debug)]
pub struct LibdrmNativeAtomicCommitRequest {
    request: drm::control::atomic::AtomicModeReq,
    properties: Option<CanonicalAtomicProperties>,
    primary_framebuffer: Option<(u32, u32)>,
    scope: LibdrmNativeAtomicCommitRequestScope,
    page_flip_event: bool,
    nonblocking: bool,
    allow_modeset: bool,
    test_only: bool,
}

impl LibdrmNativeAtomicCommitRequest {
    pub const fn new(request: drm::control::atomic::AtomicModeReq) -> Self {
        Self {
            request,
            properties: None,
            primary_framebuffer: None,
            scope: LibdrmNativeAtomicCommitRequestScope::PageFlip,
            page_flip_event: true,
            nonblocking: true,
            allow_modeset: false,
            test_only: false,
        }
    }

    pub const fn modeset(request: drm::control::atomic::AtomicModeReq) -> Self {
        Self {
            request,
            properties: None,
            primary_framebuffer: None,
            scope: LibdrmNativeAtomicCommitRequestScope::Modeset,
            page_flip_event: true,
            nonblocking: true,
            allow_modeset: false,
            test_only: false,
        }
    }

    pub const fn without_page_flip_event(mut self) -> Self {
        self.page_flip_event = false;
        self
    }

    pub const fn blocking(mut self) -> Self {
        self.nonblocking = false;
        self
    }

    pub const fn allow_modeset(mut self) -> Self {
        self.allow_modeset = true;
        self
    }

    pub const fn test_only(mut self) -> Self {
        self.test_only = true;
        self
    }

    /// A validation-only commit never reaches scanout, so it never completes with
    /// an event. The kernel does not merely ignore the combination: it rejects
    /// `TEST_ONLY` together with `PAGE_FLIP_EVENT` with `EINVAL` before inspecting
    /// a single property, so leaving both set makes every validation look like a
    /// refused topology. Deriving the flag instead of storing it keeps that
    /// unrepresentable whatever order a caller sets things in.
    const fn effective_page_flip_event(&self) -> bool {
        self.page_flip_event && !self.test_only
    }

    pub const fn reduced_flags(&self) -> LibdrmNativeAtomicCommitFlagsReport {
        LibdrmNativeAtomicCommitFlagsReport {
            page_flip_event: self.effective_page_flip_event(),
            nonblocking: self.nonblocking,
            allow_modeset: self.allow_modeset,
            test_only: self.test_only,
        }
    }

    pub fn evidence(&self) -> Option<LibdrmNativeAtomicRequestEvidence> {
        Some(LibdrmNativeAtomicRequestEvidence {
            properties: self.properties?,
            primary_framebuffer: self.primary_framebuffer?,
            scope: self.scope,
            flags: self.reduced_flags(),
        })
    }

    pub const fn reduced_scope(&self) -> LibdrmNativeAtomicCommitRequestScope {
        self.scope
    }

    pub(crate) fn into_native(
        self,
    ) -> (
        drm::control::AtomicCommitFlags,
        drm::control::atomic::AtomicModeReq,
    ) {
        let mut flags = drm::control::AtomicCommitFlags::empty();
        if self.effective_page_flip_event() {
            flags |= drm::control::AtomicCommitFlags::PAGE_FLIP_EVENT;
        }
        if self.nonblocking {
            flags |= drm::control::AtomicCommitFlags::NONBLOCK;
        }
        if self.allow_modeset {
            flags |= drm::control::AtomicCommitFlags::ALLOW_MODESET;
        }
        if self.test_only {
            flags |= drm::control::AtomicCommitFlags::TEST_ONLY;
        }
        (flags, self.request)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LibdrmNativeAtomicCommitRequestScope {
    PageFlip,
    Modeset,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LibdrmNativeAtomicCommitFlagsReport {
    pub page_flip_event: bool,
    pub nonblocking: bool,
    pub allow_modeset: bool,
    pub test_only: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LibdrmNativeAtomicCommitSubmitReport {
    pub status: LibdrmNativeAtomicCommitSubmitStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LibdrmNativeAtomicCommitSubmitStatus {
    Submitted,
    WouldBlock,
    Rejected,
}
