use std::fs::Metadata;
use std::path::{Path, PathBuf};

use crate::PackageError;

pub(crate) fn validate_package_path(path: &str) -> Result<(), PackageError> {
    let valid = !path.is_empty()
        && !path.starts_with('/')
        && !path.ends_with('/')
        && !path.contains('\\')
        && !path.contains(':')
        && !path.chars().any(char::is_control)
        && path
            .split('/')
            .all(|segment| !segment.is_empty() && segment != "." && segment != "..");
    if valid {
        Ok(())
    } else {
        Err(PackageError::UnsafePath {
            path: path.to_string(),
        })
    }
}

pub(crate) fn source_path(root: &Path, package_path: &str) -> PathBuf {
    package_path
        .split('/')
        .fold(root.to_path_buf(), |path, segment| path.join(segment))
}

pub(crate) fn reject_link_or_reparse(
    package_path: &str,
    metadata: &Metadata,
) -> Result<(), PackageError> {
    if metadata.file_type().is_symlink() || is_reparse_point(metadata) {
        Err(PackageError::UnsafePath {
            path: package_path.to_string(),
        })
    } else {
        Ok(())
    }
}

#[cfg(windows)]
fn is_reparse_point(metadata: &Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(windows))]
fn is_reparse_point(_: &Metadata) -> bool {
    false
}
