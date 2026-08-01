use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{Cursor, Write};
use std::path::Path;

use academa_content::{LessonContract, LessonManifest, ProtectedAnswers, PublicLesson};
use serde::de::DeserializeOwned;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, DateTime, System, ZipWriter};

use crate::canonical_json::to_canonical_json;
use crate::digest::sha256_digest;
use crate::path::{reject_link_or_reparse, source_path, validate_package_path};
use crate::{ChecksumInventory, FileChecksum, PackageError, PackageLimits};

const MANIFEST: &str = "manifest.json";
const LESSON: &str = "lesson.json";
const PROTECTED_ANSWERS: &str = "protected-answers.json";
const CHECKSUMS: &str = "checksums.json";

/// Deterministic bytes and identities produced from one reviewed lesson directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledPackage {
    /// Exact deterministic version-1 ZIP bytes.
    pub bytes: Vec<u8>,
    /// Semantic digest declared by the validated lesson manifest.
    pub content_digest: String,
    /// Transport digest computed over the exact ZIP bytes.
    pub package_digest: String,
}

/// Compiles one reviewed canonical lesson directory into deterministic ZIP bytes.
///
/// The directory must contain the three fixed JSON files and exactly the assets
/// declared by the decoded learner-safe lesson. Filesystem links and reparse points
/// are rejected and never followed.
///
/// # Errors
///
/// Returns [`PackageError`] when source I/O, JSON decoding, contract validation,
/// asset closure, checksums, deterministic ZIP writing, or package limits fail.
pub fn compile_directory(root: &Path) -> Result<CompiledPackage, PackageError> {
    let limits = PackageLimits::default().validate()?;
    let root_metadata = fs::symlink_metadata(root).map_err(|source| PackageError::Io {
        path: root.to_path_buf(),
        source,
    })?;
    reject_link_or_reparse(".", &root_metadata)?;
    if !root_metadata.is_dir() {
        return Err(PackageError::UnsafePath {
            path: root.display().to_string(),
        });
    }

    let manifest: LessonManifest = read_json(root, MANIFEST, limits)?;
    let lesson: PublicLesson = read_json(root, LESSON, limits)?;
    let protected_answers: ProtectedAnswers = read_json(root, PROTECTED_ANSWERS, limits)?;
    let contract = LessonContract {
        manifest,
        lesson,
        protected_answers,
    };
    academa_validation::validate(&contract)?;
    let file_count = contract.lesson.assets.len() + 4;
    if file_count > limits.max_files {
        return Err(PackageError::TooManyFiles {
            limit: limits.max_files,
            actual: file_count,
        });
    }

    let mut payloads = BTreeMap::new();
    payloads.insert(
        MANIFEST.to_string(),
        to_canonical_json(&contract.manifest, MANIFEST)?,
    );
    payloads.insert(
        LESSON.to_string(),
        to_canonical_json(&contract.lesson, LESSON)?,
    );
    payloads.insert(
        PROTECTED_ANSWERS.to_string(),
        to_canonical_json(&contract.protected_answers, PROTECTED_ANSWERS)?,
    );
    let mut payload_total = payloads
        .values()
        .map(|bytes| bytes.len() as u64)
        .sum::<u64>();
    if payload_total > limits.max_total_uncompressed_bytes {
        return Err(PackageError::TotalUncompressedTooLarge {
            limit: limits.max_total_uncompressed_bytes,
            actual: payload_total,
        });
    }

    let declared_paths: BTreeSet<_> = contract
        .lesson
        .assets
        .iter()
        .map(|asset| asset.package_path.as_str())
        .collect();
    for asset in &contract.lesson.assets {
        validate_package_path(&asset.package_path)?;
        let bytes = read_source_with_total(root, &asset.package_path, limits, payload_total)?;
        let actual_length = bytes.len() as u64;
        payload_total = payload_total.saturating_add(actual_length);
        if actual_length != asset.byte_length {
            return Err(PackageError::LengthMismatch {
                path: asset.package_path.clone(),
                expected: asset.byte_length,
                actual: actual_length,
            });
        }
        let actual_digest = sha256_digest(&bytes);
        if actual_digest != asset.sha256 {
            return Err(PackageError::ChecksumMismatch {
                path: asset.package_path.clone(),
                expected: asset.sha256.clone(),
                actual: actual_digest,
            });
        }
        payloads.insert(asset.package_path.clone(), bytes);
    }
    verify_source_asset_closure(root, &declared_paths)?;
    let inventory = ChecksumInventory {
        schema_version: 1,
        files: payloads
            .iter()
            .map(|(path, bytes)| FileChecksum {
                path: path.clone(),
                byte_length: bytes.len() as u64,
                sha256: sha256_digest(bytes),
            })
            .collect(),
    };
    let checksum_bytes = to_canonical_json(&inventory, CHECKSUMS)?;
    enforce_payload_limits(&payloads, &checksum_bytes, limits)?;
    let bytes = write_archive(&payloads, &checksum_bytes)?;
    if bytes.len() as u64 > limits.max_archive_bytes {
        return Err(PackageError::ArchiveTooLarge {
            limit: limits.max_archive_bytes,
            actual: bytes.len() as u64,
        });
    }

    Ok(CompiledPackage {
        package_digest: sha256_digest(&bytes),
        bytes,
        content_digest: contract.manifest.content_digest,
    })
}

fn read_json<T: DeserializeOwned>(
    root: &Path,
    file: &'static str,
    limits: PackageLimits,
) -> Result<T, PackageError> {
    let bytes = read_source(root, file, limits)?;
    serde_json::from_slice(&bytes).map_err(|source| PackageError::Json { file, source })
}

fn read_source(
    root: &Path,
    package_path: &str,
    limits: PackageLimits,
) -> Result<Vec<u8>, PackageError> {
    let (path, _) = source_metadata(root, package_path, limits)?;
    fs::read(&path).map_err(|source| PackageError::Io { path, source })
}

fn read_source_with_total(
    root: &Path,
    package_path: &str,
    limits: PackageLimits,
    current_total: u64,
) -> Result<Vec<u8>, PackageError> {
    let (path, metadata) = source_metadata(root, package_path, limits)?;
    let total = current_total.saturating_add(metadata.len());
    if total > limits.max_total_uncompressed_bytes {
        return Err(PackageError::TotalUncompressedTooLarge {
            limit: limits.max_total_uncompressed_bytes,
            actual: total,
        });
    }
    fs::read(&path).map_err(|source| PackageError::Io { path, source })
}

fn source_metadata(
    root: &Path,
    package_path: &str,
    limits: PackageLimits,
) -> Result<(std::path::PathBuf, fs::Metadata), PackageError> {
    validate_package_path(package_path)?;
    let path = source_path(root, package_path);
    let mut current = root.to_path_buf();
    let segments: Vec<_> = package_path.split('/').collect();
    let mut metadata = None;
    for (index, segment) in segments.iter().enumerate() {
        current.push(segment);
        let current_metadata = match fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                return Err(PackageError::MissingFile {
                    path: package_path.to_string(),
                });
            }
            Err(source) => {
                return Err(PackageError::Io {
                    path: current,
                    source,
                });
            }
        };
        reject_link_or_reparse(package_path, &current_metadata)?;
        if index + 1 < segments.len() && !current_metadata.is_dir() {
            return Err(PackageError::UnsafePath {
                path: package_path.to_string(),
            });
        }
        metadata = Some(current_metadata);
    }
    let metadata = metadata.ok_or_else(|| PackageError::UnsafePath {
        path: package_path.to_string(),
    })?;
    if !metadata.is_file() {
        return Err(PackageError::UnsafePath {
            path: package_path.to_string(),
        });
    }
    if metadata.len() > limits.max_file_bytes {
        return Err(PackageError::FileTooLarge {
            path: package_path.to_string(),
            limit: limits.max_file_bytes,
            actual: metadata.len(),
        });
    }
    Ok((path, metadata))
}

fn verify_source_asset_closure(
    root: &Path,
    declared_paths: &BTreeSet<&str>,
) -> Result<(), PackageError> {
    let assets = root.join("assets");
    let metadata = match fs::symlink_metadata(&assets) {
        Ok(metadata) => metadata,
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(source) => {
            return Err(PackageError::Io {
                path: assets,
                source,
            });
        }
    };
    reject_link_or_reparse("assets", &metadata)?;
    if !metadata.is_dir() {
        return Err(PackageError::UnsafePath {
            path: "assets".to_string(),
        });
    }
    visit_asset_directory(root, &assets, declared_paths)
}

fn visit_asset_directory(
    root: &Path,
    directory: &Path,
    declared_paths: &BTreeSet<&str>,
) -> Result<(), PackageError> {
    let entries = fs::read_dir(directory).map_err(|source| PackageError::Io {
        path: directory.to_path_buf(),
        source,
    })?;
    for entry in entries {
        let entry = entry.map_err(|source| PackageError::Io {
            path: directory.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        let package_path = path
            .strip_prefix(root)
            .map_err(|_| PackageError::UnsafePath {
                path: path.display().to_string(),
            })?
            .components()
            .map(|component| component.as_os_str().to_str())
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| PackageError::UnsafePath {
                path: path.display().to_string(),
            })?
            .join("/");
        validate_package_path(&package_path)?;
        let metadata = fs::symlink_metadata(&path).map_err(|source| PackageError::Io {
            path: path.clone(),
            source,
        })?;
        reject_link_or_reparse(&package_path, &metadata)?;
        if metadata.is_dir() {
            visit_asset_directory(root, &path, declared_paths)?;
        } else if metadata.is_file() {
            if !declared_paths.contains(package_path.as_str()) {
                return Err(PackageError::ExtraFile { path: package_path });
            }
        } else {
            return Err(PackageError::UnsafePath { path: package_path });
        }
    }
    Ok(())
}

fn enforce_payload_limits(
    payloads: &BTreeMap<String, Vec<u8>>,
    checksum_bytes: &[u8],
    limits: PackageLimits,
) -> Result<(), PackageError> {
    let mut total = 0_u64;
    for (path, bytes) in payloads {
        let length = bytes.len() as u64;
        if length > limits.max_file_bytes {
            return Err(PackageError::FileTooLarge {
                path: path.clone(),
                limit: limits.max_file_bytes,
                actual: length,
            });
        }
        total = total.saturating_add(length);
        if total > limits.max_total_uncompressed_bytes {
            return Err(PackageError::TotalUncompressedTooLarge {
                limit: limits.max_total_uncompressed_bytes,
                actual: total,
            });
        }
    }
    let checksum_length = checksum_bytes.len() as u64;
    if checksum_length > limits.max_file_bytes {
        return Err(PackageError::FileTooLarge {
            path: CHECKSUMS.to_string(),
            limit: limits.max_file_bytes,
            actual: checksum_length,
        });
    }
    total = total.saturating_add(checksum_length);
    if total > limits.max_total_uncompressed_bytes {
        return Err(PackageError::TotalUncompressedTooLarge {
            limit: limits.max_total_uncompressed_bytes,
            actual: total,
        });
    }
    Ok(())
}

fn write_archive(
    payloads: &BTreeMap<String, Vec<u8>>,
    checksum_bytes: &[u8],
) -> Result<Vec<u8>, PackageError> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .last_modified_time(DateTime::default())
        .system(System::Unix)
        .unix_permissions(0o644);
    for (path, bytes) in payloads {
        writer.start_file(path, options)?;
        writer.write_all(bytes)?;
    }
    writer.start_file(CHECKSUMS, options)?;
    writer.write_all(checksum_bytes)?;
    let cursor = writer.finish()?;
    Ok(cursor.into_inner())
}
