#![cfg(feature = "gbm-platform")]

use sophia_renderer_native_egl::{NativeDmaBufCapabilityError, NativeDmaBufImportFormat};

#[path = "../src/gbm_platform/dmabuf_capabilities/collector.rs"]
mod collector;
use collector::{CapabilityQuery, collect_formats, query_result, require_query_support};

#[path = "support/dmabuf_capabilities.rs"]
mod queries;
