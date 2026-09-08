use super::{NativeDmaBufCapabilityError, NativeDmaBufImportFormat};

const MAX_FORMATS: usize = 512;
const MAX_MODIFIERS_PER_FORMAT: usize = 4096;
const MAX_MODIFIERS_TOTAL: usize = 16_384;
const MODIFIER_INVALID: u64 = 0x00ff_ffff_ffff_ffff;

pub(super) fn require_query_support(
    extensions: &str,
    formats: bool,
    modifiers: bool,
) -> Result<(), NativeDmaBufCapabilityError> {
    let advertised = |name| {
        extensions
            .split_ascii_whitespace()
            .any(|extension| extension == name)
    };
    if !formats
        || !modifiers
        || !advertised("EGL_EXT_image_dma_buf_import")
        || !advertised("EGL_EXT_image_dma_buf_import_modifiers")
    {
        return Err(NativeDmaBufCapabilityError::Unavailable);
    }
    Ok(())
}

pub(super) trait CapabilityQuery {
    fn formats(&self, formats: &mut [i32]) -> Result<i32, NativeDmaBufCapabilityError>;
    fn modifiers(
        &self,
        format: i32,
        modifiers: &mut [u64],
        external: &mut [u32],
    ) -> Result<i32, NativeDmaBufCapabilityError>;
}

pub(super) fn query_result(result: u32, count: i32) -> Result<i32, NativeDmaBufCapabilityError> {
    match result {
        khronos_egl::TRUE => Ok(count),
        khronos_egl::FALSE => Err(NativeDmaBufCapabilityError::QueryFailed),
        _ => Err(NativeDmaBufCapabilityError::InvalidResponse),
    }
}

fn bounded_count(count: i32, maximum: usize) -> Result<usize, NativeDmaBufCapabilityError> {
    let count = usize::try_from(count).map_err(|_| NativeDmaBufCapabilityError::InvalidResponse)?;
    if count > maximum {
        return Err(NativeDmaBufCapabilityError::LimitExceeded);
    }
    Ok(count)
}

pub(super) fn collect_formats(
    query: &impl CapabilityQuery,
) -> Result<Vec<NativeDmaBufImportFormat>, NativeDmaBufCapabilityError> {
    use NativeDmaBufCapabilityError as E;
    let count = bounded_count(query.formats(&mut [])?, MAX_FORMATS)?;
    if count == 0 {
        return Ok(Vec::new());
    }
    let mut formats = vec![0; count];
    if bounded_count(query.formats(&mut formats)?, MAX_FORMATS)? != count || formats.contains(&0) {
        return Err(E::InvalidResponse);
    }
    formats.sort_unstable_by_key(|format| *format as u32);
    formats.dedup();
    let mut result = Vec::new();
    let mut total = 0;
    for format in formats {
        let count = bounded_count(
            query.modifiers(format, &mut [], &mut [])?,
            MAX_MODIFIERS_PER_FORMAT,
        )?;
        total += count;
        if total > MAX_MODIFIERS_TOTAL {
            return Err(E::LimitExceeded);
        }
        if count == 0 {
            continue;
        }
        let mut modifiers = vec![MODIFIER_INVALID; count];
        let mut external = vec![u32::MAX; count];
        if bounded_count(
            query.modifiers(format, &mut modifiers, &mut external)?,
            MAX_MODIFIERS_PER_FORMAT,
        )? != count
        {
            return Err(E::InvalidResponse);
        }
        if external
            .iter()
            .any(|value| !matches!(*value, khronos_egl::FALSE | khronos_egl::TRUE))
        {
            return Err(E::InvalidResponse);
        }
        let mut combinations: Vec<_> = modifiers.into_iter().zip(external).collect();
        combinations.sort_unstable_by_key(|(modifier, _)| *modifier);
        if combinations
            .windows(2)
            .any(|pair| pair[0].0 == pair[1].0 && pair[0].1 != pair[1].1)
        {
            return Err(E::InvalidResponse);
        }
        let mut modifiers: Vec<_> = combinations
            .into_iter()
            .filter_map(|(modifier, external)| {
                (external == khronos_egl::FALSE
                    && modifier != MODIFIER_INVALID
                    && modifier != u64::MAX)
                    .then_some(modifier)
            })
            .collect();
        modifiers.dedup();
        if !modifiers.is_empty() {
            result.push(NativeDmaBufImportFormat {
                format: format as u32,
                modifiers,
            });
        }
    }
    Ok(result)
}
