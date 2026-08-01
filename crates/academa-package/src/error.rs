use std::path::PathBuf;

use thiserror::Error;

/// A deterministic package compilation or revalidation failure.
#[derive(Debug, Error)]
pub enum PackageError {
    /// A caller supplied a zero-valued security limit.
    #[error("package limit `{field}` must be greater than zero")]
    InvalidLimits {
        /// Name of the invalid limit field.
        field: &'static str,
    },
    /// A package source file could not be read.
    #[error("cannot read package source `{path}`: {source}")]
    Io {
        /// Filesystem path that could not be read.
        path: PathBuf,
        /// Underlying filesystem failure.
        #[source]
        source: std::io::Error,
    },
    /// An in-memory archive read or write failed.
    #[error("cannot read or write in-memory package bytes: {0}")]
    ArchiveIo(#[from] std::io::Error),
    /// A fixed package JSON surface could not be decoded or encoded.
    #[error("invalid package JSON `{file}`: {source}")]
    Json {
        /// Stable package filename or generated JSON surface.
        file: &'static str,
        /// Underlying JSON failure.
        #[source]
        source: serde_json::Error,
    },
    /// The decoded lesson contract failed deterministic cross-file validation.
    #[error("invalid lesson contract: {0}")]
    Contract(#[from] academa_validation::ContractError),
    /// A raw archive or declared asset path is unsafe or noncanonical.
    #[error("unsafe package path `{path}`")]
    UnsafePath {
        /// Rejected raw path.
        path: String,
    },
    /// A package path appears more than once.
    #[error("duplicate package path `{path}`")]
    DuplicatePath {
        /// Repeated package path.
        path: String,
    },
    /// A required package file is absent.
    #[error("required package file `{path}` is missing")]
    MissingFile {
        /// Missing fixed or declared path.
        path: String,
    },
    /// An archive or source assets tree contains an undeclared file.
    #[error("undeclared package file `{path}`")]
    ExtraFile {
        /// Undeclared path.
        path: String,
    },
    /// A ZIP directory entry is forbidden by package version 1.
    #[error("directory ZIP entry `{path}` is not supported")]
    DirectoryEntry {
        /// Directory entry path.
        path: String,
    },
    /// A ZIP entry uses unsupported compression.
    #[error("ZIP entry `{path}` must use stored compression")]
    UnsupportedCompression {
        /// Entry path using an unsupported compression method.
        path: String,
    },
    /// A ZIP entry uses unsupported encryption.
    #[error("encrypted ZIP entry `{path}` is not supported")]
    EncryptedEntry {
        /// Encrypted entry path.
        path: String,
    },
    /// A ZIP entry represents a file type unsupported by package version 1.
    #[error("unsupported ZIP entry `{path}`: {kind}")]
    UnsupportedEntry {
        /// Unsupported entry path.
        path: String,
        /// Stable unsupported entry kind.
        kind: &'static str,
    },
    /// The checksum inventory shape, version, ordering, or closure is invalid.
    #[error("invalid checksum inventory: {reason}")]
    InvalidInventory {
        /// Stable reason for rejecting the inventory.
        reason: &'static str,
    },
    /// A JSON entry is semantically valid but not in canonical package encoding.
    #[error("package JSON `{file}` is not canonical")]
    NonCanonicalJson {
        /// Noncanonical fixed JSON filename.
        file: &'static str,
    },
    /// A payload length differs from its checksum inventory or asset declaration.
    #[error("byte length mismatch for `{path}`: expected {expected}, found {actual}")]
    LengthMismatch {
        /// Mismatched payload path.
        path: String,
        /// Declared byte length.
        expected: u64,
        /// Actual byte length.
        actual: u64,
    },
    /// A payload digest differs from its checksum inventory or asset declaration.
    #[error("SHA-256 mismatch for `{path}`: expected `{expected}`, found `{actual}`")]
    ChecksumMismatch {
        /// Mismatched payload path.
        path: String,
        /// Declared lowercase SHA-256 digest.
        expected: String,
        /// Computed lowercase SHA-256 digest.
        actual: String,
    },
    /// The archive exceeds the caller's byte limit.
    #[error("archive has {actual} bytes; limit is {limit}")]
    ArchiveTooLarge {
        /// Allowed archive byte length.
        limit: u64,
        /// Actual archive byte length.
        actual: u64,
    },
    /// The archive exceeds the caller's file-count limit.
    #[error("archive has {actual} files; limit is {limit}")]
    TooManyFiles {
        /// Allowed file count.
        limit: usize,
        /// Actual file count.
        actual: usize,
    },
    /// One archive entry exceeds the caller's per-file byte limit.
    #[error("archive entry `{path}` has {actual} bytes; per-file limit is {limit}")]
    FileTooLarge {
        /// Oversized entry path.
        path: String,
        /// Allowed uncompressed byte length.
        limit: u64,
        /// Declared uncompressed byte length.
        actual: u64,
    },
    /// Archive entries exceed the caller's aggregate uncompressed byte limit.
    #[error("archive declares {actual} uncompressed bytes; total limit is {limit}")]
    TotalUncompressedTooLarge {
        /// Allowed aggregate uncompressed byte length.
        limit: u64,
        /// Declared aggregate uncompressed byte length.
        actual: u64,
    },
    /// ZIP parsing or writing failed.
    #[error("invalid ZIP package: {0}")]
    Zip(#[from] zip::result::ZipError),
}
