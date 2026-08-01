use serde::Serialize;

use crate::PackageError;

pub(crate) fn to_canonical_json<T: Serialize>(
    value: &T,
    file: &'static str,
) -> Result<Vec<u8>, PackageError> {
    let mut bytes =
        serde_json::to_vec_pretty(value).map_err(|source| PackageError::Json { file, source })?;
    bytes.push(b'\n');
    Ok(bytes)
}
