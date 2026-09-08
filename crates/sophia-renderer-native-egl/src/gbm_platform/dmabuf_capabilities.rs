use std::os::fd::AsFd;
use std::ptr;

use gbm::AsRaw;

mod collector;
use collector::{CapabilityQuery, collect_formats, query_result, require_query_support};

/// Explicit modifier combinations queryable for non-external texture import.
/// These facts do not establish allocation or render-target support.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NativeDmaBufImportFormat {
    pub format: u32,
    pub modifiers: Vec<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeDmaBufCapabilityError {
    Unavailable,
    DeviceUnavailable,
    DisplayUnavailable,
    InitializeFailed,
    QueryFailed,
    InvalidResponse,
    LimitExceeded,
}

impl std::fmt::Display for NativeDmaBufCapabilityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for NativeDmaBufCapabilityError {}

type Egl = khronos_egl::DynamicInstance<khronos_egl::EGL1_5>;
type FormatsQuery =
    unsafe extern "system" fn(khronos_egl::EGLDisplay, i32, *mut i32, *mut i32) -> u32;
type ModifiersQuery = unsafe extern "system" fn(
    khronos_egl::EGLDisplay,
    i32,
    i32,
    *mut u64,
    *mut u32,
    *mut i32,
) -> u32;

struct DisplayGuard<'a> {
    egl: &'a Egl,
    display: khronos_egl::Display,
}

impl Drop for DisplayGuard<'_> {
    fn drop(&mut self) {
        let _ = self.egl.terminate(self.display);
    }
}

/// Queries the supplied device without creating a GL context or changing the current one.
pub fn query_native_dmabuf_import_formats<T: AsFd>(
    device: T,
) -> Result<Vec<NativeDmaBufImportFormat>, NativeDmaBufCapabilityError> {
    use NativeDmaBufCapabilityError as E;
    let device = gbm::Device::new(device).map_err(|_| E::DeviceUnavailable)?;
    // The dynamically loaded EGL functions and GBM device outlive the display guard.
    let egl = unsafe { Egl::load_required() }.map_err(|_| E::Unavailable)?;
    let display = unsafe {
        egl.get_platform_display(
            super::EGL_PLATFORM_GBM_KHR,
            device.as_raw() as khronos_egl::NativeDisplayType,
            &[khronos_egl::ATTRIB_NONE],
        )
    }
    .map_err(|_| E::DisplayUnavailable)?;
    let initialized = DisplayGuard { egl: &egl, display };
    egl.initialize(display).map_err(|_| E::InitializeFailed)?;
    let extensions = egl
        .query_string(Some(display), khronos_egl::EXTENSIONS)
        .map_err(|_| E::QueryFailed)?
        .to_str()
        .map_err(|_| E::InvalidResponse)?;
    let formats = egl.get_proc_address("eglQueryDmaBufFormatsEXT");
    let modifiers = egl.get_proc_address("eglQueryDmaBufModifiersEXT");
    require_query_support(extensions, formats.is_some(), modifiers.is_some())?;
    // Both extension entry points were checked and retain the advertised ABI.
    let query = unsafe {
        DriverQuery {
            display: initialized.display,
            formats: std::mem::transmute::<extern "system" fn(), FormatsQuery>(
                formats.ok_or(E::Unavailable)?,
            ),
            modifiers: std::mem::transmute::<extern "system" fn(), ModifiersQuery>(
                modifiers.ok_or(E::Unavailable)?,
            ),
        }
    };
    collect_formats(&query)
}

struct DriverQuery {
    display: khronos_egl::Display,
    formats: FormatsQuery,
    modifiers: ModifiersQuery,
}

impl CapabilityQuery for DriverQuery {
    fn formats(&self, formats: &mut [i32]) -> Result<i32, NativeDmaBufCapabilityError> {
        let mut count = -1;
        // Slice length bounds the writable storage. Empty slices request only the count.
        let result = unsafe {
            (self.formats)(
                self.display.as_ptr(),
                i32::try_from(formats.len())
                    .map_err(|_| NativeDmaBufCapabilityError::LimitExceeded)?,
                if formats.is_empty() {
                    ptr::null_mut()
                } else {
                    formats.as_mut_ptr()
                },
                &mut count,
            )
        };
        query_result(result, count)
    }

    fn modifiers(
        &self,
        format: i32,
        modifiers: &mut [u64],
        external: &mut [u32],
    ) -> Result<i32, NativeDmaBufCapabilityError> {
        let mut count = -1;
        if modifiers.len() != external.len() {
            return Err(NativeDmaBufCapabilityError::InvalidResponse);
        }
        // The paired arrays have identical bounded capacity and remain alive through the call.
        let result = unsafe {
            (self.modifiers)(
                self.display.as_ptr(),
                format,
                i32::try_from(modifiers.len())
                    .map_err(|_| NativeDmaBufCapabilityError::LimitExceeded)?,
                if modifiers.is_empty() {
                    ptr::null_mut()
                } else {
                    modifiers.as_mut_ptr()
                },
                if external.is_empty() {
                    ptr::null_mut()
                } else {
                    external.as_mut_ptr()
                },
                &mut count,
            )
        };
        query_result(result, count)
    }
}
