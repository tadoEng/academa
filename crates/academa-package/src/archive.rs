use std::collections::{BTreeMap, BTreeSet};
use std::io::{Cursor, Read};

use academa_content::{LessonContract, LessonManifest, ProtectedAnswers, PublicLesson};
use serde::de::DeserializeOwned;
use zip::result::ZipError;
use zip::{CompressionMethod, ZipArchive};

use crate::canonical_json::to_canonical_json;
use crate::digest::sha256_digest;
use crate::path::validate_package_path;
use crate::{ChecksumInventory, PackageError, PackageLimits};

const MANIFEST: &str = "manifest.json";
const LESSON: &str = "lesson.json";
const PROTECTED_ANSWERS: &str = "protected-answers.json";
const CHECKSUMS: &str = "checksums.json";

/// A decoded lesson contract accepted from exact package bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedPackage {
    /// Validated package-facing lesson contract.
    pub contract: LessonContract,
    /// Lowercase SHA-256 digest of the exact accepted ZIP bytes.
    pub package_digest: String,
}

/// Revalidates untrusted package bytes entirely in memory under explicit limits.
///
/// The function never extracts files. It rejects unsafe raw names, links,
/// directories, encryption, compression, duplicate paths, size violations,
/// noncanonical JSON, checksum failures, and declared-asset closure mismatches.
///
/// # Errors
///
/// Returns the first deterministic [`PackageError`] in the package validation order.
pub fn validate_package(
    package_bytes: &[u8],
    limits: PackageLimits,
) -> Result<ValidatedPackage, PackageError> {
    let limits = limits.validate()?;
    let archive_length = package_bytes.len() as u64;
    if archive_length > limits.max_archive_bytes {
        return Err(PackageError::ArchiveTooLarge {
            limit: limits.max_archive_bytes,
            actual: archive_length,
        });
    }
    preflight_central_directory(package_bytes, limits.max_files)?;

    let cursor = Cursor::new(package_bytes);
    let mut archive = ZipArchive::new(cursor)?;
    if archive.len() > limits.max_files {
        return Err(PackageError::TooManyFiles {
            limit: limits.max_files,
            actual: archive.len(),
        });
    }
    let entry_metadata = validate_entry_metadata(&mut archive, limits)?;
    let entries = read_entry_bodies(&mut archive, &entry_metadata)?;

    for required in [MANIFEST, LESSON, PROTECTED_ANSWERS, CHECKSUMS] {
        if !entries.contains_key(required) {
            return Err(PackageError::MissingFile {
                path: required.to_string(),
            });
        }
    }

    let manifest: LessonManifest = decode_entry(&entries, MANIFEST)?;
    let lesson: PublicLesson = decode_entry(&entries, LESSON)?;
    let protected_answers: ProtectedAnswers = decode_entry(&entries, PROTECTED_ANSWERS)?;
    let contract = LessonContract {
        manifest,
        lesson,
        protected_answers,
    };
    academa_validation::validate(&contract)?;

    let inventory: ChecksumInventory = decode_entry(&entries, CHECKSUMS)?;
    validate_inventory(&inventory, &entries)?;
    validate_asset_closure(&contract, &entries)?;
    validate_canonical_json(&contract, &inventory, &entries)?;

    Ok(ValidatedPackage {
        contract,
        package_digest: sha256_digest(package_bytes),
    })
}

fn preflight_central_directory(package_bytes: &[u8], max_files: usize) -> Result<(), PackageError> {
    let work_budget = package_bytes.len().saturating_mul(4);
    preflight_central_directory_with_budget(package_bytes, max_files, work_budget)
}

fn preflight_central_directory_with_budget(
    package_bytes: &[u8],
    max_files: usize,
    mut work_budget: usize,
) -> Result<(), PackageError> {
    const EOCD_SIGNATURE: &[u8; 4] = b"PK\x05\x06";
    const CENTRAL_SIGNATURE: &[u8; 4] = b"PK\x01\x02";
    const EOCD_SIZE: usize = 22;
    const CENTRAL_SIZE: usize = 46;

    for (eocd, signature) in package_bytes
        .windows(EOCD_SIGNATURE.len())
        .enumerate()
        .rev()
    {
        if signature != EOCD_SIGNATURE {
            continue;
        }
        let Some(comment_length) = read_u16(package_bytes, eocd + 20).map(usize::from) else {
            continue;
        };
        if eocd
            .checked_add(EOCD_SIZE)
            .and_then(|end| end.checked_add(comment_length))
            .is_none_or(|comment_end| comment_end > package_bytes.len())
        {
            continue;
        }
        let Some(candidate) = central_directory_candidate(package_bytes, eocd)? else {
            continue;
        };
        if candidate.disk_number != candidate.disk_with_central_directory {
            continue;
        }
        if candidate.entry_count == 0 {
            continue;
        }
        let Some(central_offset) = find_central_header(
            package_bytes,
            candidate.relative_central_offset,
            candidate.central_end,
            CENTRAL_SIGNATURE,
            &mut work_budget,
        )?
        else {
            continue;
        };

        let max_files_u64 = u64::try_from(max_files).unwrap_or(u64::MAX);
        if candidate.entry_count > max_files_u64 {
            return Err(PackageError::TooManyFiles {
                limit: max_files,
                actual: usize::try_from(candidate.entry_count).unwrap_or(usize::MAX),
            });
        }
        let Ok(entry_count) = usize::try_from(candidate.entry_count) else {
            return Err(PackageError::TooManyFiles {
                limit: max_files,
                actual: usize::MAX,
            });
        };
        // Keep walking older candidates even when this directory is structurally
        // complete. `zip` may reject details inside a central header and retry an
        // earlier EOCD; every directory it could reach must satisfy this preflight.
        scan_central_directory(
            package_bytes,
            central_offset,
            entry_count,
            candidate.central_end,
            CENTRAL_SIZE,
            CENTRAL_SIGNATURE,
            &mut work_budget,
        )?;
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct CentralDirectoryCandidate {
    entry_count: u64,
    relative_central_offset: usize,
    central_end: usize,
    disk_number: u32,
    disk_with_central_directory: u32,
}

fn central_directory_candidate(
    package_bytes: &[u8],
    eocd: usize,
) -> Result<Option<CentralDirectoryCandidate>, PackageError> {
    const ZIP64_EOCD_SIGNATURE: &[u8; 4] = b"PK\x06\x06";
    const ZIP64_LOCATOR_SIGNATURE: &[u8; 4] = b"PK\x06\x07";

    let Some(entry_count_32) = read_u16(package_bytes, eocd + 10) else {
        return Ok(None);
    };
    let Some(central_size_32) = read_u32(package_bytes, eocd + 12) else {
        return Ok(None);
    };
    let Some(central_offset_32) = read_u32(package_bytes, eocd + 16) else {
        return Ok(None);
    };
    let may_be_zip64 =
        entry_count_32 == u16::MAX || central_size_32 == u32::MAX || central_offset_32 == u32::MAX;
    if !may_be_zip64 {
        return Ok(standard_directory_candidate(package_bytes, eocd));
    }
    let unresolved_zip64 = || {
        PackageError::from(ZipError::InvalidArchive(
            "unresolvable ZIP64 central-directory preflight".into(),
        ))
    };
    let zip64 = {
        let Some(locator) = eocd.checked_sub(20) else {
            return Err(unresolved_zip64());
        };
        if !package_bytes[locator..].starts_with(ZIP64_LOCATOR_SIGNATURE) {
            return Err(unresolved_zip64());
        }
        let Some(zip64_offset) =
            read_u64(package_bytes, locator + 8).and_then(|value| usize::try_from(value).ok())
        else {
            return Err(unresolved_zip64());
        };
        if package_bytes
            .get(zip64_offset..)
            .is_none_or(|bytes| !bytes.starts_with(ZIP64_EOCD_SIGNATURE))
        {
            return Err(unresolved_zip64());
        }
        let Some(zip64_record_size) = zip64_offset
            .checked_add(4)
            .and_then(|offset| read_u64(package_bytes, offset))
            .and_then(|value| usize::try_from(value).ok())
        else {
            return Err(unresolved_zip64());
        };
        if zip64_record_size < 44
            || zip64_offset
                .checked_add(12)
                .and_then(|end| end.checked_add(zip64_record_size))
                != Some(locator)
        {
            return Err(unresolved_zip64());
        }
        let entry_count = zip64_offset
            .checked_add(32)
            .and_then(|offset| read_u64(package_bytes, offset))
            .ok_or_else(unresolved_zip64)?;
        let relative_central_offset = zip64_offset
            .checked_add(48)
            .and_then(|offset| read_u64(package_bytes, offset))
            .and_then(|value| usize::try_from(value).ok())
            .ok_or_else(unresolved_zip64)?;
        CentralDirectoryCandidate {
            entry_count,
            relative_central_offset,
            central_end: zip64_offset,
            disk_number: zip64_offset
                .checked_add(16)
                .and_then(|offset| read_u32(package_bytes, offset))
                .ok_or_else(unresolved_zip64)?,
            disk_with_central_directory: zip64_offset
                .checked_add(20)
                .and_then(|offset| read_u32(package_bytes, offset))
                .ok_or_else(unresolved_zip64)?,
        }
    };
    Ok(Some(zip64))
}

fn standard_directory_candidate(
    package_bytes: &[u8],
    eocd: usize,
) -> Option<CentralDirectoryCandidate> {
    Some(CentralDirectoryCandidate {
        entry_count: u64::from(read_u16(package_bytes, eocd + 8)?),
        relative_central_offset: usize::try_from(read_u32(package_bytes, eocd + 16)?).ok()?,
        central_end: eocd,
        disk_number: u32::from(read_u16(package_bytes, eocd + 4)?),
        disk_with_central_directory: u32::from(read_u16(package_bytes, eocd + 6)?),
    })
}

fn find_central_header(
    package_bytes: &[u8],
    relative_offset: usize,
    central_end: usize,
    signature: &[u8; 4],
    search_budget: &mut usize,
) -> Result<Option<usize>, PackageError> {
    if relative_offset >= central_end {
        return Ok(None);
    }
    let Some(search_bytes) = package_bytes.get(relative_offset..central_end) else {
        return Ok(None);
    };
    if search_bytes.starts_with(signature) {
        consume_preflight_work(search_budget, signature.len())?;
        return Ok(Some(relative_offset));
    }
    consume_preflight_work(search_budget, search_bytes.len())?;
    Ok(search_bytes
        .windows(signature.len())
        .position(|window| window == signature)
        .and_then(|offset| relative_offset.checked_add(offset)))
}

fn scan_central_directory(
    package_bytes: &[u8],
    mut offset: usize,
    entry_count: usize,
    central_end: usize,
    fixed_size: usize,
    signature: &[u8; 4],
    work_budget: &mut usize,
) -> Result<(), PackageError> {
    let mut seen = BTreeSet::new();
    for _ in 0..entry_count {
        consume_preflight_work(work_budget, fixed_size)?;
        if offset
            .checked_add(fixed_size)
            .is_none_or(|end| end > central_end)
            || package_bytes
                .get(offset..)
                .is_none_or(|bytes| !bytes.starts_with(signature))
        {
            return Ok(());
        }
        let Some(name_length) = read_u16(package_bytes, offset + 28).map(usize::from) else {
            return Ok(());
        };
        let Some(extra_length) = read_u16(package_bytes, offset + 30).map(usize::from) else {
            return Ok(());
        };
        let Some(comment_length) = read_u16(package_bytes, offset + 32).map(usize::from) else {
            return Ok(());
        };
        let Some(variable_length) = name_length
            .checked_add(extra_length)
            .and_then(|value| value.checked_add(comment_length))
        else {
            return Ok(());
        };
        consume_preflight_work(work_budget, variable_length)?;
        let name_start = offset + fixed_size;
        let Some(name_end) = name_start.checked_add(name_length) else {
            return Ok(());
        };
        if name_end > central_end {
            return Ok(());
        }
        let name = package_bytes[name_start..name_end].to_vec();
        if seen.contains(&name) {
            return Err(PackageError::DuplicatePath {
                path: String::from_utf8_lossy(&name).into_owned(),
            });
        }
        seen.insert(name);
        let Some(next) = name_end
            .checked_add(extra_length)
            .and_then(|value| value.checked_add(comment_length))
        else {
            return Ok(());
        };
        if next > central_end {
            return Ok(());
        }
        offset = next;
    }
    Ok(())
}

fn consume_preflight_work(work_budget: &mut usize, amount: usize) -> Result<(), PackageError> {
    let Some(remaining) = work_budget.checked_sub(amount) else {
        return Err(ZipError::InvalidArchive(
            "central-directory preflight work limit exceeded".into(),
        )
        .into());
    };
    *work_budget = remaining;
    Ok(())
}

fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    let end = offset.checked_add(2)?;
    let value: [u8; 2] = bytes.get(offset..end)?.try_into().ok()?;
    Some(u16::from_le_bytes(value))
}

fn read_u32(bytes: &[u8], offset: usize) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let value: [u8; 4] = bytes.get(offset..end)?.try_into().ok()?;
    Some(u32::from_le_bytes(value))
}

fn read_u64(bytes: &[u8], offset: usize) -> Option<u64> {
    let end = offset.checked_add(8)?;
    let value: [u8; 8] = bytes.get(offset..end)?.try_into().ok()?;
    Some(u64::from_le_bytes(value))
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Write};

    use zip::ZipWriter;
    use zip::write::SimpleFileOptions;

    use super::{PackageError, ZipError, preflight_central_directory_with_budget};

    #[test]
    fn aggregate_budget_bounds_repeated_shared_central_directory_scans() {
        let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
        writer
            .start_file("one.bin", SimpleFileOptions::default())
            .expect("test entry should start");
        writer.write_all(b"one").expect("test entry should write");
        let mut bytes = writer
            .finish()
            .expect("test archive should finish")
            .into_inner();
        let real_eocd = bytes
            .windows(4)
            .rposition(|window| window == b"PK\x05\x06")
            .expect("test archive should contain an EOCD");
        let central_size: [u8; 4] = bytes[real_eocd + 12..real_eocd + 16]
            .try_into()
            .expect("central size should be present");
        let central_offset: [u8; 4] = bytes[real_eocd + 16..real_eocd + 20]
            .try_into()
            .expect("central offset should be present");

        for _ in 0..8 {
            bytes.extend_from_slice(b"PK\x05\x06");
            bytes.extend_from_slice(&0_u16.to_le_bytes());
            bytes.extend_from_slice(&0_u16.to_le_bytes());
            bytes.extend_from_slice(&1_u16.to_le_bytes());
            bytes.extend_from_slice(&1_u16.to_le_bytes());
            bytes.extend_from_slice(&central_size);
            bytes.extend_from_slice(&central_offset);
            bytes.extend_from_slice(&0_u16.to_le_bytes());
        }

        let error = preflight_central_directory_with_budget(&bytes, 256, 100)
            .expect_err("shared-directory rescans must exhaust one aggregate budget");

        assert!(
            matches!(
                error,
                PackageError::Zip(ZipError::InvalidArchive(ref message))
                    if message.contains("preflight work limit")
            ),
            "unexpected error: {error:?}"
        );
    }
}

#[derive(Debug)]
struct EntryMetadata {
    index: usize,
    path: String,
    size: u64,
}

fn validate_entry_metadata(
    archive: &mut ZipArchive<Cursor<&[u8]>>,
    limits: PackageLimits,
) -> Result<Vec<EntryMetadata>, PackageError> {
    let mut seen = BTreeSet::new();
    let mut metadata = Vec::with_capacity(archive.len());
    let mut total = 0_u64;
    for index in 0..archive.len() {
        let entry = archive.by_index_raw(index)?;
        let raw_path =
            std::str::from_utf8(entry.name_raw()).map_err(|_| PackageError::UnsafePath {
                path: String::from_utf8_lossy(entry.name_raw()).into_owned(),
            })?;
        if entry.is_dir() {
            return Err(PackageError::DirectoryEntry {
                path: raw_path.to_string(),
            });
        }
        validate_package_path(raw_path)?;
        if entry.name() != raw_path {
            return Err(PackageError::UnsafePath {
                path: raw_path.to_string(),
            });
        }
        if entry.is_symlink() {
            return Err(PackageError::UnsupportedEntry {
                path: raw_path.to_string(),
                kind: "symbolic link",
            });
        }
        if entry.encrypted() {
            return Err(PackageError::EncryptedEntry {
                path: raw_path.to_string(),
            });
        }
        if entry.compression() != CompressionMethod::Stored {
            return Err(PackageError::UnsupportedCompression {
                path: raw_path.to_string(),
            });
        }
        if !seen.insert(raw_path.to_string()) {
            return Err(PackageError::DuplicatePath {
                path: raw_path.to_string(),
            });
        }
        let size = entry.size();
        if size > limits.max_file_bytes {
            return Err(PackageError::FileTooLarge {
                path: raw_path.to_string(),
                limit: limits.max_file_bytes,
                actual: size,
            });
        }
        total = total.saturating_add(size);
        if total > limits.max_total_uncompressed_bytes {
            return Err(PackageError::TotalUncompressedTooLarge {
                limit: limits.max_total_uncompressed_bytes,
                actual: total,
            });
        }
        metadata.push(EntryMetadata {
            index,
            path: raw_path.to_string(),
            size,
        });
    }
    Ok(metadata)
}

fn read_entry_bodies(
    archive: &mut ZipArchive<Cursor<&[u8]>>,
    metadata: &[EntryMetadata],
) -> Result<BTreeMap<String, Vec<u8>>, PackageError> {
    let mut entries = BTreeMap::new();
    for expected in metadata {
        let mut entry = archive.by_index(expected.index)?;
        let mut body = Vec::new();
        entry
            .by_ref()
            .take(expected.size.saturating_add(1))
            .read_to_end(&mut body)?;
        let actual = body.len() as u64;
        if actual != expected.size {
            return Err(PackageError::LengthMismatch {
                path: expected.path.clone(),
                expected: expected.size,
                actual,
            });
        }
        entries.insert(expected.path.clone(), body);
    }
    Ok(entries)
}

fn decode_entry<T: DeserializeOwned>(
    entries: &BTreeMap<String, Vec<u8>>,
    file: &'static str,
) -> Result<T, PackageError> {
    let bytes = entries.get(file).ok_or_else(|| PackageError::MissingFile {
        path: file.to_string(),
    })?;
    serde_json::from_slice(bytes).map_err(|source| PackageError::Json { file, source })
}

fn validate_inventory(
    inventory: &ChecksumInventory,
    entries: &BTreeMap<String, Vec<u8>>,
) -> Result<(), PackageError> {
    if inventory.schema_version != 1 {
        return Err(PackageError::InvalidInventory {
            reason: "unsupported schema version",
        });
    }
    if inventory.files.iter().any(|file| file.path == CHECKSUMS) {
        return Err(PackageError::InvalidInventory {
            reason: "inventory must exclude checksums.json",
        });
    }
    let mut seen = BTreeSet::new();
    for file in &inventory.files {
        validate_package_path(&file.path)?;
        if !seen.insert(file.path.as_str()) {
            return Err(PackageError::InvalidInventory {
                reason: "duplicate file path",
            });
        }
        if !is_sha256(&file.sha256) {
            return Err(PackageError::InvalidInventory {
                reason: "noncanonical SHA-256 digest",
            });
        }
    }
    if inventory
        .files
        .windows(2)
        .any(|pair| pair[0].path >= pair[1].path)
    {
        return Err(PackageError::InvalidInventory {
            reason: "files are not in strict path order",
        });
    }
    let expected_paths: BTreeSet<_> = entries
        .keys()
        .filter(|path| path.as_str() != CHECKSUMS)
        .map(String::as_str)
        .collect();
    if expected_paths.difference(&seen).next().is_some() {
        return Err(PackageError::InvalidInventory {
            reason: "inventory is missing an archive payload",
        });
    }
    if seen.difference(&expected_paths).next().is_some() {
        return Err(PackageError::InvalidInventory {
            reason: "inventory names a missing archive payload",
        });
    }
    for file in &inventory.files {
        let bytes = &entries[&file.path];
        let actual_length = bytes.len() as u64;
        if actual_length != file.byte_length {
            return Err(PackageError::LengthMismatch {
                path: file.path.clone(),
                expected: file.byte_length,
                actual: actual_length,
            });
        }
        let actual_digest = sha256_digest(bytes);
        if actual_digest != file.sha256 {
            return Err(PackageError::ChecksumMismatch {
                path: file.path.clone(),
                expected: file.sha256.clone(),
                actual: actual_digest,
            });
        }
    }
    Ok(())
}

fn validate_asset_closure(
    contract: &LessonContract,
    entries: &BTreeMap<String, Vec<u8>>,
) -> Result<(), PackageError> {
    let mut allowed: BTreeSet<&str> = [MANIFEST, LESSON, PROTECTED_ANSWERS, CHECKSUMS]
        .into_iter()
        .collect();
    for asset in &contract.lesson.assets {
        allowed.insert(asset.package_path.as_str());
        let bytes = entries
            .get(&asset.package_path)
            .ok_or_else(|| PackageError::MissingFile {
                path: asset.package_path.clone(),
            })?;
        let actual_length = bytes.len() as u64;
        if actual_length != asset.byte_length {
            return Err(PackageError::LengthMismatch {
                path: asset.package_path.clone(),
                expected: asset.byte_length,
                actual: actual_length,
            });
        }
        let actual_digest = sha256_digest(bytes);
        if actual_digest != asset.sha256 {
            return Err(PackageError::ChecksumMismatch {
                path: asset.package_path.clone(),
                expected: asset.sha256.clone(),
                actual: actual_digest,
            });
        }
    }
    if let Some(path) = entries.keys().find(|path| !allowed.contains(path.as_str())) {
        return Err(PackageError::ExtraFile { path: path.clone() });
    }
    Ok(())
}

fn validate_canonical_json(
    contract: &LessonContract,
    inventory: &ChecksumInventory,
    entries: &BTreeMap<String, Vec<u8>>,
) -> Result<(), PackageError> {
    for (file, canonical) in [
        (MANIFEST, to_canonical_json(&contract.manifest, MANIFEST)?),
        (LESSON, to_canonical_json(&contract.lesson, LESSON)?),
        (
            PROTECTED_ANSWERS,
            to_canonical_json(&contract.protected_answers, PROTECTED_ANSWERS)?,
        ),
        (CHECKSUMS, to_canonical_json(inventory, CHECKSUMS)?),
    ] {
        if entries[file] != canonical {
            return Err(PackageError::NonCanonicalJson { file });
        }
    }
    Ok(())
}

fn is_sha256(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
