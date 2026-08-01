use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use academa_content::{
    AssetDeclaration, AssetDeliveryClassification, AssetId, LessonContract, LessonManifest,
    ProtectedAnswers, PublicLesson,
};
use academa_package::{PackageError, compile_directory, compute_package_digest};
use serde::Serialize;
use serde::de::DeserializeOwned;

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../contracts/fixtures/daily-reading-v1/valid")
}

struct TempFixture {
    root: PathBuf,
}

impl TempFixture {
    fn copy_valid() -> Self {
        let serial = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "academa-package-compile-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir(&root).expect("temporary fixture root should be created");
        for name in ["manifest.json", "lesson.json", "protected-answers.json"] {
            fs::copy(fixture_root().join(name), root.join(name))
                .expect("sanitized fixture file should copy");
        }
        Self { root }
    }

    fn load_contract(&self) -> LessonContract {
        LessonContract {
            manifest: read_json(&self.root.join("manifest.json")),
            lesson: read_json(&self.root.join("lesson.json")),
            protected_answers: read_json(&self.root.join("protected-answers.json")),
        }
    }

    fn declare_asset(&self, bytes: &[u8], declared_length: u64, declared_hash: String) {
        self.declare_asset_at("assets/reading.bin", bytes, declared_length, declared_hash);
    }

    fn declare_asset_at(
        &self,
        package_path: &str,
        bytes: &[u8],
        declared_length: u64,
        declared_hash: String,
    ) {
        let mut contract = self.load_contract();
        contract.lesson.assets.push(AssetDeclaration {
            id: AssetId::new("asset-reading-audio").expect("test asset ID should be valid"),
            package_path: package_path.to_string(),
            media_type: "application/octet-stream".to_string(),
            byte_length: declared_length,
            sha256: declared_hash,
            delivery_classification: AssetDeliveryClassification::LearnerSafe,
        });
        contract.manifest.content_digest = contract
            .computed_content_digest()
            .expect("test contract should digest");
        write_json(&self.root.join("lesson.json"), &contract.lesson);
        write_json(&self.root.join("manifest.json"), &contract.manifest);
        if !bytes.is_empty() {
            let asset_path = package_path
                .split('/')
                .fold(self.root.clone(), |path, segment| path.join(segment));
            fs::create_dir_all(
                asset_path
                    .parent()
                    .expect("asset path should have a parent"),
            )
            .expect("test asset directory should be created");
            fs::write(asset_path, bytes).expect("test asset should be written");
        }
    }
}

impl Drop for TempFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn read_json<T: DeserializeOwned>(path: &Path) -> T {
    let bytes = fs::read(path).expect("test JSON should be readable");
    serde_json::from_slice(&bytes).expect("test JSON should decode")
}

fn write_json<T: Serialize>(path: &Path, value: &T) {
    let mut bytes = serde_json::to_vec_pretty(value).expect("test JSON should encode");
    bytes.push(b'\n');
    fs::write(path, bytes).expect("test JSON should be written");
}

#[test]
fn compile_directory_is_byte_identical_across_clean_roots() {
    let first = TempFixture::copy_valid();
    let second = TempFixture::copy_valid();

    let first_package = compile_directory(&first.root).expect("first fixture should compile");
    let second_package = compile_directory(&second.root).expect("second fixture should compile");

    assert_eq!(first_package.bytes, second_package.bytes);
    assert_eq!(first_package.content_digest, second_package.content_digest);
    assert_eq!(first_package.package_digest, second_package.package_digest);
}

#[test]
fn compile_directory_canonicalizes_json_whitespace_and_object_order() {
    let original = TempFixture::copy_valid();
    let rewritten = TempFixture::copy_valid();
    for name in ["manifest.json", "lesson.json", "protected-answers.json"] {
        let value: serde_json::Value = read_json(&rewritten.root.join(name));
        let compact = serde_json::to_vec(&value).expect("JSON value should serialize");
        fs::write(rewritten.root.join(name), compact).expect("rewritten JSON should be written");
    }

    let original_package = compile_directory(&original.root).expect("fixture should compile");
    let rewritten_package =
        compile_directory(&rewritten.root).expect("rewritten fixture should compile");

    assert_eq!(original_package.bytes, rewritten_package.bytes);
}

#[test]
fn compile_directory_keeps_protected_answers_out_of_lesson_json() {
    let fixture = TempFixture::copy_valid();
    let package = compile_directory(&fixture.root).expect("fixture should compile");
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(package.bytes))
        .expect("compiled bytes should be ZIP");
    let mut lesson = String::new();
    std::io::Read::read_to_string(
        &mut archive
            .by_name("lesson.json")
            .expect("lesson entry should exist"),
        &mut lesson,
    )
    .expect("lesson entry should be readable");

    assert!(!lesson.contains("accepted_option_ids"));
}

#[test]
fn compile_directory_writes_explicit_fixed_zip_metadata() {
    let fixture = TempFixture::copy_valid();
    let package = compile_directory(&fixture.root).expect("fixture should compile");
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(&package.bytes))
        .expect("compiled bytes should be ZIP");
    for index in 0..archive.len() {
        let entry = archive.by_index_raw(index).expect("entry should open");
        assert_eq!(entry.compression(), zip::CompressionMethod::Stored);
        assert_eq!(entry.last_modified(), Some(zip::DateTime::default()));
        assert_eq!(entry.unix_mode().map(|mode| mode & 0o777), Some(0o644));
    }
    let unix_central_entries = package
        .bytes
        .windows(6)
        .filter(|window| window.starts_with(b"PK\x01\x02"))
        .all(|window| window[5] == 3);

    assert!(
        unix_central_entries,
        "all central entries must declare Unix metadata"
    );
}

#[test]
fn compile_directory_rejects_missing_declared_asset() {
    let fixture = TempFixture::copy_valid();
    fixture.declare_asset(
        &[],
        3,
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
    );

    let error = compile_directory(&fixture.root).expect_err("missing asset must fail");

    assert!(matches!(
        error,
        PackageError::MissingFile { ref path } if path == "assets/reading.bin"
    ));
}

#[test]
fn compile_directory_rejects_undeclared_asset() {
    let fixture = TempFixture::copy_valid();
    fs::create_dir(fixture.root.join("assets")).expect("assets directory should be created");
    fs::write(fixture.root.join("assets/extra.bin"), b"extra")
        .expect("extra asset should be written");

    let error = compile_directory(&fixture.root).expect_err("extra asset must fail");

    assert!(matches!(
        error,
        PackageError::ExtraFile { ref path } if path == "assets/extra.bin"
    ));
}

#[test]
fn compile_directory_rejects_wrong_asset_length() {
    let fixture = TempFixture::copy_valid();
    let bytes = b"asset-bytes";
    fixture.declare_asset(bytes, 1, compute_package_digest(bytes));

    let error = compile_directory(&fixture.root).expect_err("wrong length must fail");

    assert!(matches!(
        error,
        PackageError::LengthMismatch {
            ref path,
            expected: 1,
            actual: 11
        } if path == "assets/reading.bin"
    ));
}

#[test]
fn compile_directory_rejects_wrong_asset_hash() {
    let fixture = TempFixture::copy_valid();
    let bytes = b"asset-bytes";
    fixture.declare_asset(
        bytes,
        bytes.len() as u64,
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
    );

    let error = compile_directory(&fixture.root).expect_err("wrong hash must fail");

    assert!(matches!(
        error,
        PackageError::ChecksumMismatch { ref path, .. } if path == "assets/reading.bin"
    ));
}

#[test]
fn asset_path_rename_changes_package_bytes_without_changing_semantic_digest() {
    let first = TempFixture::copy_valid();
    let second = TempFixture::copy_valid();
    let bytes = b"asset-bytes";
    let digest = compute_package_digest(bytes);
    first.declare_asset(bytes, bytes.len() as u64, digest.clone());
    second.declare_asset(bytes, bytes.len() as u64, digest);

    let mut second_contract = second.load_contract();
    second_contract.lesson.assets[0].package_path = "assets/renamed.bin".to_string();
    write_json(&second.root.join("lesson.json"), &second_contract.lesson);
    fs::rename(
        second.root.join("assets/reading.bin"),
        second.root.join("assets/renamed.bin"),
    )
    .expect("test asset should be renamed");

    let first_package = compile_directory(&first.root).expect("first asset should compile");
    let second_package = compile_directory(&second.root).expect("renamed asset should compile");

    assert_ne!(first_package.bytes, second_package.bytes);
    assert_eq!(first_package.content_digest, second_package.content_digest);
}

#[cfg(unix)]
#[test]
fn compile_directory_rejects_symlinked_asset() {
    use std::os::unix::fs::symlink;

    let fixture = TempFixture::copy_valid();
    let bytes = b"asset-bytes";
    fixture.declare_asset(bytes, bytes.len() as u64, compute_package_digest(bytes));
    fs::remove_file(fixture.root.join("assets/reading.bin"))
        .expect("real test asset should be removed");
    symlink(
        fixture.root.join("lesson.json"),
        fixture.root.join("assets/reading.bin"),
    )
    .expect("test symlink should be created");

    let error = compile_directory(&fixture.root).expect_err("symlink must fail");

    assert!(matches!(error, PackageError::UnsafePath { .. }));
}

#[cfg(windows)]
#[test]
fn compile_directory_rejects_reparse_point_asset_when_creation_is_permitted() {
    use std::os::windows::fs::symlink_file;

    let fixture = TempFixture::copy_valid();
    let bytes = b"asset-bytes";
    fixture.declare_asset(bytes, bytes.len() as u64, compute_package_digest(bytes));
    fs::remove_file(fixture.root.join("assets/reading.bin"))
        .expect("real test asset should be removed");
    if let Err(error) = symlink_file(
        fixture.root.join("lesson.json"),
        fixture.root.join("assets/reading.bin"),
    ) {
        assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
        return;
    }

    let error = compile_directory(&fixture.root).expect_err("reparse point must fail");

    assert!(matches!(error, PackageError::UnsafePath { .. }));
}

#[cfg(unix)]
#[test]
fn compile_directory_rejects_nested_symlink_directory_before_reading_asset() {
    use std::os::unix::fs::symlink;

    let fixture = TempFixture::copy_valid();
    let bytes = b"nested-asset";
    fixture.declare_asset_at(
        "assets/nested/reading.bin",
        bytes,
        bytes.len() as u64,
        compute_package_digest(bytes),
    );
    fs::create_dir(fixture.root.join("outside")).expect("outside directory should be created");
    fs::rename(
        fixture.root.join("assets/nested/reading.bin"),
        fixture.root.join("outside/reading.bin"),
    )
    .expect("asset should move outside declared tree");
    fs::remove_dir(fixture.root.join("assets/nested"))
        .expect("empty nested directory should be removed");
    symlink(
        fixture.root.join("outside"),
        fixture.root.join("assets/nested"),
    )
    .expect("nested symlink should be created");

    let error = compile_directory(&fixture.root).expect_err("nested symlink must fail");

    assert!(matches!(error, PackageError::UnsafePath { .. }));
}

#[cfg(windows)]
#[test]
fn compile_directory_rejects_nested_reparse_directory_when_creation_is_permitted() {
    use std::os::windows::fs::symlink_dir;

    let fixture = TempFixture::copy_valid();
    let bytes = b"nested-asset";
    fixture.declare_asset_at(
        "assets/nested/reading.bin",
        bytes,
        bytes.len() as u64,
        compute_package_digest(bytes),
    );
    fs::create_dir(fixture.root.join("outside")).expect("outside directory should be created");
    fs::rename(
        fixture.root.join("assets/nested/reading.bin"),
        fixture.root.join("outside/reading.bin"),
    )
    .expect("asset should move outside declared tree");
    fs::remove_dir(fixture.root.join("assets/nested"))
        .expect("empty nested directory should be removed");
    if let Err(error) = symlink_dir(
        fixture.root.join("outside"),
        fixture.root.join("assets/nested"),
    ) {
        assert_eq!(error.kind(), std::io::ErrorKind::PermissionDenied);
        return;
    }

    let error = compile_directory(&fixture.root).expect_err("nested reparse point must fail");

    assert!(matches!(error, PackageError::UnsafePath { .. }));
}

#[allow(dead_code)]
fn _type_surface(_: LessonManifest, _: PublicLesson, _: ProtectedAnswers) {}
