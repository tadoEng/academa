use crate::PackageError;

/// Security limits applied before and during in-memory package revalidation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackageLimits {
    /// Maximum accepted ZIP byte length.
    pub max_archive_bytes: u64,
    /// Maximum accepted number of ZIP file entries.
    pub max_files: usize,
    /// Maximum accepted uncompressed length for one entry.
    pub max_file_bytes: u64,
    /// Maximum accepted aggregate uncompressed length.
    pub max_total_uncompressed_bytes: u64,
}

impl Default for PackageLimits {
    fn default() -> Self {
        Self {
            max_archive_bytes: 64 * 1024 * 1024,
            max_files: 256,
            max_file_bytes: 32 * 1024 * 1024,
            max_total_uncompressed_bytes: 64 * 1024 * 1024,
        }
    }
}

impl PackageLimits {
    pub(crate) fn validate(self) -> Result<Self, PackageError> {
        for (field, value) in [
            ("max_archive_bytes", self.max_archive_bytes),
            ("max_file_bytes", self.max_file_bytes),
            (
                "max_total_uncompressed_bytes",
                self.max_total_uncompressed_bytes,
            ),
        ] {
            if value == 0 {
                return Err(PackageError::InvalidLimits { field });
            }
        }
        if self.max_files == 0 {
            return Err(PackageError::InvalidLimits { field: "max_files" });
        }
        Ok(self)
    }
}
