#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod archive;
mod canonical_json;
mod checksums;
mod compiler;
mod digest;
mod error;
mod limits;
mod path;

pub use archive::{ValidatedPackage, validate_package};
pub use checksums::{ChecksumInventory, FileChecksum};
pub use compiler::{CompiledPackage, compile_directory};
pub use digest::compute_package_digest;
pub use error::PackageError;
pub use limits::PackageLimits;
