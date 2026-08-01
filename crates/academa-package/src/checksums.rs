use serde::{Deserialize, Serialize};

/// Checksum metadata for one version-1 package payload file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileChecksum {
    /// Canonical package-relative payload path.
    pub path: String,
    /// Exact uncompressed payload length in bytes.
    pub byte_length: u64,
    /// Lowercase `sha256:`-prefixed digest of the payload bytes.
    pub sha256: String,
}

/// Versioned checksum inventory covering every package payload except itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChecksumInventory {
    /// Checksum inventory schema version. Version 1 is the only supported value.
    pub schema_version: u32,
    /// Payload checksums sorted by canonical package path.
    pub files: Vec<FileChecksum>,
}
