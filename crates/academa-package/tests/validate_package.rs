use std::collections::BTreeMap;
use std::fs;
use std::io::{Cursor, Read, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use academa_content::{
    AssetDeclaration, AssetDeliveryClassification, AssetId, LessonContract, LessonManifest,
    ProtectedAnswers, PublicLesson,
};
use academa_package::{
    ChecksumInventory, PackageError, PackageLimits, compile_directory, compute_package_digest,
    validate_package,
};
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../contracts/fixtures/daily-reading-v1/valid")
}

fn compiled_bytes() -> Vec<u8> {
    compile_directory(&fixture_root())
        .expect("sanitized fixture should compile")
        .bytes
}

fn compiled_asset_bytes() -> Vec<u8> {
    let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "academa-package-validate-{}-{serial}",
        std::process::id()
    ));
    fs::create_dir(&root).expect("temporary asset fixture should be created");
    for name in ["manifest.json", "lesson.json", "protected-answers.json"] {
        fs::copy(fixture_root().join(name), root.join(name))
            .expect("sanitized fixture file should copy");
    }
    let read = |name: &str| fs::read(root.join(name)).expect("fixture JSON should read");
    let mut contract = LessonContract {
        manifest: serde_json::from_slice::<LessonManifest>(&read("manifest.json"))
            .expect("manifest should decode"),
        lesson: serde_json::from_slice::<PublicLesson>(&read("lesson.json"))
            .expect("lesson should decode"),
        protected_answers: serde_json::from_slice::<ProtectedAnswers>(&read(
            "protected-answers.json",
        ))
        .expect("protected answers should decode"),
    };
    let asset_bytes = b"reviewed-asset-bytes";
    contract.lesson.assets.push(AssetDeclaration {
        id: AssetId::new("asset-reading-audio").expect("asset ID should be valid"),
        package_path: "assets/reading.bin".to_string(),
        media_type: "application/octet-stream".to_string(),
        byte_length: asset_bytes.len() as u64,
        sha256: compute_package_digest(asset_bytes),
        delivery_classification: AssetDeliveryClassification::LearnerSafe,
    });
    contract.manifest.content_digest = contract
        .computed_content_digest()
        .expect("asset contract should digest");
    let mut lesson = serde_json::to_vec_pretty(&contract.lesson).expect("lesson should encode");
    lesson.push(b'\n');
    fs::write(root.join("lesson.json"), lesson).expect("lesson should write");
    let mut manifest =
        serde_json::to_vec_pretty(&contract.manifest).expect("manifest should encode");
    manifest.push(b'\n');
    fs::write(root.join("manifest.json"), manifest).expect("manifest should write");
    fs::create_dir(root.join("assets")).expect("assets directory should be created");
    fs::write(root.join("assets/reading.bin"), asset_bytes).expect("asset should write");
    let package = compile_directory(&root).expect("asset fixture should compile");
    fs::remove_dir_all(&root).expect("temporary asset fixture should be removed");
    package.bytes
}

fn archive_entries(bytes: &[u8]) -> BTreeMap<String, Vec<u8>> {
    let mut archive = ZipArchive::new(Cursor::new(bytes)).expect("test archive should open");
    let mut entries = BTreeMap::new();
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).expect("test entry should open");
        let name = entry.name().to_string();
        let mut body = Vec::new();
        entry
            .read_to_end(&mut body)
            .expect("test entry should read");
        entries.insert(name, body);
    }
    entries
}

fn build_archive(entries: impl IntoIterator<Item = (String, Vec<u8>)>) -> Vec<u8> {
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    for (name, body) in entries {
        writer
            .start_file(name, options)
            .expect("test ZIP entry should start");
        writer
            .write_all(&body)
            .expect("test ZIP entry should write");
    }
    writer
        .finish()
        .expect("test ZIP should finish")
        .into_inner()
}

fn end_of_central_directory(bytes: &[u8]) -> usize {
    bytes
        .windows(4)
        .rposition(|window| window == b"PK\x05\x06")
        .expect("test ZIP should contain an end-of-central-directory record")
}

fn set_standard_declared_entry_count(bytes: &mut [u8], entry_count: u16) {
    set_standard_declared_entry_counts(bytes, entry_count, entry_count);
}

fn set_standard_declared_entry_counts(bytes: &mut [u8], on_disk: u16, total: u16) {
    let eocd = end_of_central_directory(bytes);
    bytes[eocd + 8..eocd + 10].copy_from_slice(&on_disk.to_le_bytes());
    bytes[eocd + 10..eocd + 12].copy_from_slice(&total.to_le_bytes());
}

fn add_zip64_declared_entry_count(bytes: &mut Vec<u8>, entry_count: u64) {
    let eocd = end_of_central_directory(bytes);
    let central_size = u32::from_le_bytes(
        bytes[eocd + 12..eocd + 16]
            .try_into()
            .expect("central size should be present"),
    );
    let central_offset = u32::from_le_bytes(
        bytes[eocd + 16..eocd + 20]
            .try_into()
            .expect("central offset should be present"),
    );
    let mut standard_eocd = bytes[eocd..].to_vec();
    standard_eocd[8..12].fill(0xff);
    standard_eocd[12..16].fill(0xff);
    standard_eocd[16..20].fill(0xff);

    bytes.truncate(eocd);
    bytes.extend_from_slice(b"PK\x06\x06");
    bytes.extend_from_slice(&44_u64.to_le_bytes());
    bytes.extend_from_slice(&45_u16.to_le_bytes());
    bytes.extend_from_slice(&45_u16.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&entry_count.to_le_bytes());
    bytes.extend_from_slice(&entry_count.to_le_bytes());
    bytes.extend_from_slice(&u64::from(central_size).to_le_bytes());
    bytes.extend_from_slice(&u64::from(central_offset).to_le_bytes());

    bytes.extend_from_slice(b"PK\x06\x07");
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&(eocd as u64).to_le_bytes());
    bytes.extend_from_slice(&1_u32.to_le_bytes());
    bytes.extend_from_slice(&standard_eocd);
}

fn append_rejected_eocd_decoy(bytes: &mut Vec<u8>, entry_count: u16) {
    let decoy_offset = u32::try_from(bytes.len()).expect("test archive should fit ZIP32 offsets");
    bytes.extend_from_slice(b"PK\x05\x06");
    bytes.extend_from_slice(&0_u16.to_le_bytes());
    bytes.extend_from_slice(&0_u16.to_le_bytes());
    bytes.extend_from_slice(&entry_count.to_le_bytes());
    bytes.extend_from_slice(&entry_count.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&decoy_offset.to_le_bytes());
    bytes.extend_from_slice(&0_u16.to_le_bytes());
}

fn append_rejected_zip64_multidisk_decoy(bytes: &mut Vec<u8>) {
    let zip64_offset = u64::try_from(bytes.len()).expect("test archive should fit ZIP64 offsets");
    bytes.extend_from_slice(b"PK\x06\x06");
    bytes.extend_from_slice(&44_u64.to_le_bytes());
    bytes.extend_from_slice(&45_u16.to_le_bytes());
    bytes.extend_from_slice(&45_u16.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&1_u64.to_le_bytes());
    bytes.extend_from_slice(&1_u64.to_le_bytes());
    bytes.extend_from_slice(&0_u64.to_le_bytes());
    bytes.extend_from_slice(&zip64_offset.to_le_bytes());

    bytes.extend_from_slice(b"PK\x06\x07");
    bytes.extend_from_slice(&0_u32.to_le_bytes());
    bytes.extend_from_slice(&zip64_offset.to_le_bytes());
    bytes.extend_from_slice(&2_u32.to_le_bytes());

    bytes.extend_from_slice(b"PK\x05\x06");
    bytes.extend_from_slice(&0_u16.to_le_bytes());
    bytes.extend_from_slice(&0_u16.to_le_bytes());
    bytes.extend_from_slice(&u16::MAX.to_le_bytes());
    bytes.extend_from_slice(&u16::MAX.to_le_bytes());
    bytes.extend_from_slice(&u32::MAX.to_le_bytes());
    bytes.extend_from_slice(&u32::MAX.to_le_bytes());
    bytes.extend_from_slice(&0_u16.to_le_bytes());
}

fn rewrite_inventory(
    entries: &mut BTreeMap<String, Vec<u8>>,
    edit: impl FnOnce(&mut ChecksumInventory),
) {
    let mut inventory: ChecksumInventory = serde_json::from_slice(
        entries
            .get("checksums.json")
            .expect("test inventory should exist"),
    )
    .expect("test inventory should decode");
    edit(&mut inventory);
    let mut bytes = serde_json::to_vec_pretty(&inventory).expect("inventory should encode");
    bytes.push(b'\n');
    entries.insert("checksums.json".to_string(), bytes);
}

fn metadata_totals(bytes: &[u8]) -> (usize, u64, u64) {
    let mut archive = ZipArchive::new(Cursor::new(bytes)).expect("test archive should open");
    let mut maximum = 0;
    let mut total = 0;
    for index in 0..archive.len() {
        let entry = archive
            .by_index_raw(index)
            .expect("raw test entry should open");
        maximum = maximum.max(entry.size());
        total += entry.size();
    }
    (archive.len(), maximum, total)
}

#[test]
fn validate_package_recovers_the_typed_contract_and_transport_digest() {
    let bytes = compiled_bytes();

    let validated = validate_package(&bytes, PackageLimits::default())
        .expect("compiled fixture should revalidate");

    assert_eq!(validated.package_digest, compute_package_digest(&bytes));
    assert_eq!(
        validated.contract.manifest.content_digest,
        "sha256:5919e958c3f2466713267cd6a65b7648e5cf02707624b8bd62661cf3eb3a910f"
    );
}

#[test]
fn payload_mutation_changes_transport_digest_and_fails_checksum_validation() {
    let original = compiled_bytes();
    let mut entries = archive_entries(&original);
    entries
        .get_mut("lesson.json")
        .expect("lesson entry should exist")
        .push(b' ');
    let mutated = build_archive(entries);

    let error = validate_package(&mutated, PackageLimits::default())
        .expect_err("mutated payload must fail");

    assert_ne!(
        compute_package_digest(&original),
        compute_package_digest(&mutated)
    );
    assert!(matches!(
        error,
        PackageError::LengthMismatch { ref path, .. } if path == "lesson.json"
    ));
}

#[test]
fn unsafe_raw_archive_paths_are_rejected() {
    for path in [
        "../lesson.json",
        "/lesson.json",
        "C:/lesson.json",
        "assets\\lesson.json",
        "assets/./lesson.json",
        "assets//lesson.json",
    ] {
        let bytes = build_archive([(path.to_string(), Vec::new())]);
        let error = validate_package(&bytes, PackageLimits::default())
            .expect_err("unsafe raw path must fail");
        assert!(
            matches!(error, PackageError::UnsafePath { .. }),
            "unexpected error for {path}: {error}"
        );
    }
}

#[test]
fn duplicate_archive_paths_are_rejected() {
    let mut bytes = build_archive([
        ("lesson-a.json".to_string(), b"first".to_vec()),
        ("lesson-b.json".to_string(), b"second".to_vec()),
    ]);
    for index in 0..=bytes.len() - b"lesson-b.json".len() {
        if bytes[index..].starts_with(b"lesson-b.json") {
            bytes[index + 7] = b'a';
        }
    }

    let error =
        validate_package(&bytes, PackageLimits::default()).expect_err("duplicate path must fail");

    assert!(matches!(
        error,
        PackageError::DuplicatePath { ref path } if path == "lesson-a.json"
    ));
}

#[test]
fn directory_archive_entries_are_rejected() {
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    writer
        .add_directory("assets/", SimpleFileOptions::default())
        .expect("test directory entry should write");
    let bytes = writer
        .finish()
        .expect("test ZIP should finish")
        .into_inner();

    let error =
        validate_package(&bytes, PackageLimits::default()).expect_err("directory entry must fail");

    assert!(matches!(error, PackageError::DirectoryEntry { .. }));
}

#[test]
fn stored_only_policy_rejects_deflated_metadata() {
    let mut bytes = build_archive([("lesson.json".to_string(), b"body".to_vec())]);
    for index in 0..bytes.len().saturating_sub(12) {
        if bytes[index..].starts_with(b"PK\x03\x04") {
            bytes[index + 8] = 8;
            bytes[index + 9] = 0;
        } else if bytes[index..].starts_with(b"PK\x01\x02") {
            bytes[index + 10] = 8;
            bytes[index + 11] = 0;
        }
    }

    let error =
        validate_package(&bytes, PackageLimits::default()).expect_err("deflated entry must fail");

    assert!(matches!(
        error,
        PackageError::UnsupportedCompression { ref path } if path == "lesson.json"
    ));
}

#[test]
fn encrypted_entry_metadata_is_rejected_before_body_read() {
    let mut bytes = build_archive([("lesson.json".to_string(), b"body".to_vec())]);
    for index in 0..bytes.len().saturating_sub(10) {
        if bytes[index..].starts_with(b"PK\x03\x04") {
            bytes[index + 6] |= 1;
        } else if bytes[index..].starts_with(b"PK\x01\x02") {
            bytes[index + 8] |= 1;
        }
    }

    let error =
        validate_package(&bytes, PackageLimits::default()).expect_err("encrypted entry must fail");

    assert!(matches!(
        error,
        PackageError::EncryptedEntry { ref path } if path == "lesson.json"
    ));
}

#[test]
fn missing_fixed_file_is_rejected() {
    let mut entries = archive_entries(&compiled_bytes());
    entries.remove("lesson.json");
    let bytes = build_archive(entries);

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("missing fixed file must fail");

    assert!(matches!(
        error,
        PackageError::MissingFile { ref path } if path == "lesson.json"
    ));
}

#[test]
fn undeclared_extra_file_is_rejected() {
    let mut entries = archive_entries(&compiled_bytes());
    entries.insert("assets/extra.bin".to_string(), b"extra".to_vec());
    rewrite_inventory(&mut entries, |inventory| {
        inventory.files.push(academa_package::FileChecksum {
            path: "assets/extra.bin".to_string(),
            byte_length: 5,
            sha256: compute_package_digest(b"extra"),
        });
        inventory
            .files
            .sort_by(|left, right| left.path.cmp(&right.path));
    });
    let bytes = build_archive(entries);

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("undeclared archive file must fail");

    assert!(matches!(
        error,
        PackageError::ExtraFile { ref path } if path == "assets/extra.bin"
    ));
}

#[test]
fn unsupported_inventory_version_is_rejected() {
    let mut entries = archive_entries(&compiled_bytes());
    rewrite_inventory(&mut entries, |inventory| inventory.schema_version = 2);
    let bytes = build_archive(entries);

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("inventory version must fail");

    assert!(matches!(
        error,
        PackageError::InvalidInventory {
            reason: "unsupported schema version"
        }
    ));
}

#[test]
fn unsorted_inventory_is_rejected() {
    let mut entries = archive_entries(&compiled_bytes());
    rewrite_inventory(&mut entries, |inventory| inventory.files.reverse());
    let bytes = build_archive(entries);

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("unsorted inventory must fail");

    assert!(matches!(
        error,
        PackageError::InvalidInventory {
            reason: "files are not in strict path order"
        }
    ));
}

#[test]
fn duplicate_inventory_path_is_rejected() {
    let mut entries = archive_entries(&compiled_bytes());
    rewrite_inventory(&mut entries, |inventory| {
        inventory.files.push(inventory.files[0].clone());
        inventory
            .files
            .sort_by(|left, right| left.path.cmp(&right.path));
    });
    let bytes = build_archive(entries);

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("duplicate inventory path must fail");

    assert!(matches!(
        error,
        PackageError::InvalidInventory {
            reason: "duplicate file path"
        }
    ));
}

#[test]
fn inventory_missing_archive_payload_is_rejected() {
    let mut entries = archive_entries(&compiled_bytes());
    rewrite_inventory(&mut entries, |inventory| {
        inventory.files.retain(|file| file.path != "lesson.json");
    });
    let bytes = build_archive(entries);

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("missing inventory payload must fail");

    assert!(matches!(
        error,
        PackageError::InvalidInventory {
            reason: "inventory is missing an archive payload"
        }
    ));
}

#[test]
fn inventory_extra_missing_payload_is_rejected() {
    let mut entries = archive_entries(&compiled_bytes());
    rewrite_inventory(&mut entries, |inventory| {
        inventory.files.push(academa_package::FileChecksum {
            path: "assets/ghost.bin".to_string(),
            byte_length: 1,
            sha256: compute_package_digest(b"x"),
        });
        inventory
            .files
            .sort_by(|left, right| left.path.cmp(&right.path));
    });
    let bytes = build_archive(entries);

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("inventory path without payload must fail");

    assert!(matches!(
        error,
        PackageError::InvalidInventory {
            reason: "inventory names a missing archive payload"
        }
    ));
}

#[test]
fn inventory_wrong_byte_length_is_rejected() {
    let mut entries = archive_entries(&compiled_bytes());
    rewrite_inventory(&mut entries, |inventory| {
        inventory.files[0].byte_length += 1;
    });
    let bytes = build_archive(entries);

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("wrong inventory length must fail");

    assert!(matches!(error, PackageError::LengthMismatch { .. }));
}

#[test]
fn inventory_self_entry_is_rejected() {
    let mut entries = archive_entries(&compiled_bytes());
    rewrite_inventory(&mut entries, |inventory| {
        inventory.files.push(academa_package::FileChecksum {
            path: "checksums.json".to_string(),
            byte_length: 0,
            sha256: compute_package_digest(&[]),
        });
    });
    let bytes = build_archive(entries);

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("inventory self-entry must fail");

    assert!(matches!(
        error,
        PackageError::InvalidInventory {
            reason: "inventory must exclude checksums.json"
        }
    ));
}

#[test]
fn inventory_wrong_digest_is_rejected() {
    let mut entries = archive_entries(&compiled_bytes());
    rewrite_inventory(&mut entries, |inventory| {
        inventory.files[0].sha256 = compute_package_digest(b"wrong");
    });
    let bytes = build_archive(entries);

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("wrong inventory digest must fail");

    assert!(matches!(error, PackageError::ChecksumMismatch { .. }));
}

#[test]
fn declared_asset_package_revalidates_successfully() {
    let bytes = compiled_asset_bytes();

    let validated = validate_package(&bytes, PackageLimits::default())
        .expect("declared asset package should validate");

    assert_eq!(validated.contract.lesson.assets.len(), 1);
}

#[test]
fn missing_declared_asset_is_rejected_after_inventory_closure() {
    let mut entries = archive_entries(&compiled_asset_bytes());
    entries.remove("assets/reading.bin");
    rewrite_inventory(&mut entries, |inventory| {
        inventory
            .files
            .retain(|file| file.path != "assets/reading.bin");
    });
    let bytes = build_archive(entries);

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("missing declared asset must fail");

    assert!(matches!(
        error,
        PackageError::MissingFile { ref path } if path == "assets/reading.bin"
    ));
}

#[test]
fn declared_asset_length_is_revalidated_after_inventory() {
    let mut entries = archive_entries(&compiled_asset_bytes());
    entries
        .get_mut("assets/reading.bin")
        .expect("asset entry should exist")
        .push(b'!');
    let asset = entries["assets/reading.bin"].clone();
    rewrite_inventory(&mut entries, |inventory| {
        let checksum = inventory
            .files
            .iter_mut()
            .find(|file| file.path == "assets/reading.bin")
            .expect("asset checksum should exist");
        checksum.byte_length = asset.len() as u64;
        checksum.sha256 = compute_package_digest(&asset);
    });
    let bytes = build_archive(entries);

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("asset declaration length must be rechecked");

    assert!(matches!(
        error,
        PackageError::LengthMismatch { ref path, .. } if path == "assets/reading.bin"
    ));
}

#[test]
fn declared_asset_hash_is_revalidated_after_inventory() {
    let mut entries = archive_entries(&compiled_asset_bytes());
    entries
        .get_mut("assets/reading.bin")
        .expect("asset entry should exist")[0] ^= 1;
    let asset = entries["assets/reading.bin"].clone();
    rewrite_inventory(&mut entries, |inventory| {
        let checksum = inventory
            .files
            .iter_mut()
            .find(|file| file.path == "assets/reading.bin")
            .expect("asset checksum should exist");
        checksum.sha256 = compute_package_digest(&asset);
    });
    let bytes = build_archive(entries);

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("asset declaration hash must be rechecked");

    assert!(matches!(
        error,
        PackageError::ChecksumMismatch { ref path, .. } if path == "assets/reading.bin"
    ));
}

#[test]
fn invalid_zero_limit_profiles_are_rejected() {
    for (field, limits) in [
        (
            "max_archive_bytes",
            PackageLimits {
                max_archive_bytes: 0,
                ..PackageLimits::default()
            },
        ),
        (
            "max_files",
            PackageLimits {
                max_files: 0,
                ..PackageLimits::default()
            },
        ),
        (
            "max_file_bytes",
            PackageLimits {
                max_file_bytes: 0,
                ..PackageLimits::default()
            },
        ),
        (
            "max_total_uncompressed_bytes",
            PackageLimits {
                max_total_uncompressed_bytes: 0,
                ..PackageLimits::default()
            },
        ),
    ] {
        let error = validate_package(&[], limits).expect_err("zero limit must fail");
        assert!(
            matches!(error, PackageError::InvalidLimits { field: found } if found == field),
            "unexpected zero-limit error for {field}: {error}"
        );
    }
}

#[test]
fn archive_byte_limit_fails_at_the_first_byte_over_limit() {
    let bytes = compiled_bytes();
    let limits = PackageLimits {
        max_archive_bytes: bytes.len() as u64 - 1,
        ..PackageLimits::default()
    };

    let error = validate_package(&bytes, limits).expect_err("archive limit must fail");

    assert!(matches!(error, PackageError::ArchiveTooLarge { .. }));
}

#[test]
fn file_count_limit_fails_at_the_first_file_over_limit() {
    let bytes = compiled_bytes();
    let (files, _, _) = metadata_totals(&bytes);
    let limits = PackageLimits {
        max_files: files - 1,
        ..PackageLimits::default()
    };

    let error = validate_package(&bytes, limits).expect_err("file count limit must fail");

    assert!(matches!(error, PackageError::TooManyFiles { .. }));
}

#[test]
fn standard_declared_file_count_is_rejected_before_zip_parsing() {
    let mut bytes = build_archive([("manifest.json".to_string(), b"not JSON".to_vec())]);
    set_standard_declared_entry_count(&mut bytes, 2);
    let limits = PackageLimits {
        max_files: 1,
        ..PackageLimits::default()
    };

    let error = validate_package(&bytes, limits).expect_err("declared count must fail first");

    assert!(
        matches!(
            error,
            PackageError::TooManyFiles {
                limit: 1,
                actual: 2
            }
        ),
        "unexpected error: {error:?}"
    );
}

#[test]
fn standard_on_disk_count_is_enforced_when_total_count_is_smaller() {
    let mut bytes = build_archive([("manifest.json".to_string(), b"not JSON".to_vec())]);
    set_standard_declared_entry_counts(&mut bytes, 2, 1);
    let limits = PackageLimits {
        max_files: 1,
        ..PackageLimits::default()
    };

    let error = validate_package(&bytes, limits).expect_err("on-disk count must fail first");

    assert!(matches!(
        error,
        PackageError::TooManyFiles {
            limit: 1,
            actual: 2
        }
    ));
}

#[test]
fn unresolvable_zip64_size_sentinel_is_rejected_before_zip_parsing() {
    let mut bytes = build_archive([("manifest.json".to_string(), b"not JSON".to_vec())]);
    let eocd = end_of_central_directory(&bytes);
    bytes[eocd + 12..eocd + 16].copy_from_slice(&u32::MAX.to_le_bytes());

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("unresolvable ZIP64 marker must fail closed");

    assert!(matches!(error, PackageError::Zip(_)));
}

#[test]
fn zip64_declared_file_count_is_rejected_before_zip_parsing() {
    let mut bytes = build_archive([("manifest.json".to_string(), b"not JSON".to_vec())]);
    add_zip64_declared_entry_count(&mut bytes, 2);
    let limits = PackageLimits {
        max_files: 1,
        ..PackageLimits::default()
    };

    let error = validate_package(&bytes, limits).expect_err("ZIP64 count must fail first");

    assert!(
        matches!(
            error,
            PackageError::TooManyFiles {
                limit: 1,
                actual: 2
            }
        ),
        "unexpected error: {error:?}"
    );
}

#[test]
fn declared_file_count_before_trailing_junk_is_rejected_before_zip_parsing() {
    let mut bytes = build_archive([
        ("first.bin".to_string(), vec![1]),
        ("second.bin".to_string(), vec![2]),
    ]);
    bytes.extend_from_slice(b"trailing bytes accepted by zip");
    let limits = PackageLimits {
        max_files: 1,
        ..PackageLimits::default()
    };
    assert_eq!(
        ZipArchive::new(Cursor::new(&bytes))
            .expect("zip should accept trailing bytes")
            .len(),
        2
    );

    let error = validate_package(&bytes, limits).expect_err("real count before junk must fail");

    assert!(matches!(
        error,
        PackageError::TooManyFiles {
            limit: 1,
            actual: 2
        }
    ));
}

#[test]
fn rejected_eocd_decoy_does_not_hide_earlier_file_count() {
    let mut bytes = build_archive([
        ("first.bin".to_string(), vec![1]),
        ("second.bin".to_string(), vec![2]),
    ]);
    append_rejected_eocd_decoy(&mut bytes, 1);
    let limits = PackageLimits {
        max_files: 1,
        ..PackageLimits::default()
    };
    assert_eq!(
        ZipArchive::new(Cursor::new(&bytes))
            .expect("zip should reject the decoy and accept the earlier EOCD")
            .len(),
        2
    );

    let error = validate_package(&bytes, limits).expect_err("fallback count must fail");

    assert!(matches!(
        error,
        PackageError::TooManyFiles {
            limit: 1,
            actual: 2
        }
    ));
}

#[test]
fn rejected_zip64_multidisk_decoy_does_not_hide_earlier_file_count() {
    let mut bytes = build_archive([
        ("first.bin".to_string(), vec![1]),
        ("second.bin".to_string(), vec![2]),
    ]);
    append_rejected_zip64_multidisk_decoy(&mut bytes);
    let limits = PackageLimits {
        max_files: 1,
        ..PackageLimits::default()
    };
    assert_eq!(
        ZipArchive::new(Cursor::new(&bytes))
            .expect("zip should reject the ZIP64 decoy and accept the earlier EOCD")
            .len(),
        2
    );

    let error = validate_package(&bytes, limits).expect_err("ZIP64 fallback count must fail");

    assert!(matches!(
        error,
        PackageError::TooManyFiles {
            limit: 1,
            actual: 2
        }
    ));
}

#[test]
fn rejected_eocd_decoy_does_not_hide_earlier_duplicate_name() {
    let mut bytes = build_archive([
        ("lesson-a.json".to_string(), vec![1]),
        ("lesson-b.json".to_string(), vec![2]),
    ]);
    for index in 0..=bytes.len() - b"lesson-b.json".len() {
        if bytes[index..].starts_with(b"lesson-b.json") {
            bytes[index + 7] = b'a';
        }
    }
    append_rejected_eocd_decoy(&mut bytes, 1);

    let error = validate_package(&bytes, PackageLimits::default())
        .expect_err("fallback duplicate must fail");

    assert!(matches!(
        error,
        PackageError::DuplicatePath { path } if path == "lesson-a.json"
    ));
}

#[test]
fn eocd_signature_inside_archive_comment_is_not_a_false_match() {
    let mut bytes = compiled_bytes();
    let eocd = end_of_central_directory(&bytes);
    let comment = b"PK\x05\x06not-the-real-eocd-record";
    let comment_length = u16::try_from(comment.len()).expect("test comment should fit");
    bytes[eocd + 20..eocd + 22].copy_from_slice(&comment_length.to_le_bytes());
    bytes.extend_from_slice(comment);

    validate_package(&bytes, PackageLimits::default())
        .expect("EOCD-like bytes in a ZIP comment should not hide the real record");
}

#[test]
fn malformed_archive_without_a_locatable_eocd_is_delegated_to_zip_parser() {
    let error = validate_package(b"not a ZIP archive", PackageLimits::default())
        .expect_err("malformed archive must fail");

    assert!(
        matches!(error, PackageError::Zip(_)),
        "unexpected error: {error:?}"
    );
}

#[test]
fn per_file_limit_fails_at_the_first_byte_over_limit() {
    let bytes = compiled_bytes();
    let (_, maximum, _) = metadata_totals(&bytes);
    let limits = PackageLimits {
        max_file_bytes: maximum - 1,
        ..PackageLimits::default()
    };

    let error = validate_package(&bytes, limits).expect_err("per-file limit must fail");

    assert!(matches!(error, PackageError::FileTooLarge { .. }));
}

#[test]
fn total_uncompressed_limit_fails_at_the_first_byte_over_limit() {
    let bytes = compiled_bytes();
    let (_, _, total) = metadata_totals(&bytes);
    let limits = PackageLimits {
        max_total_uncompressed_bytes: total - 1,
        ..PackageLimits::default()
    };

    let error = validate_package(&bytes, limits).expect_err("total limit must fail");

    assert!(matches!(
        error,
        PackageError::TotalUncompressedTooLarge { .. }
    ));
}
