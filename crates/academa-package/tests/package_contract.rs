use academa_package::{ChecksumInventory, FileChecksum, PackageLimits, compute_package_digest};

#[test]
fn checksum_inventory_serializes_to_the_versioned_wire_shape() {
    let inventory = ChecksumInventory {
        schema_version: 1,
        files: vec![FileChecksum {
            path: "lesson.json".to_string(),
            byte_length: 3,
            sha256: "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                .to_string(),
        }],
    };

    let value = serde_json::to_value(inventory).expect("inventory should serialize");

    assert_eq!(
        value,
        serde_json::json!({
            "schema_version": 1,
            "files": [{
                "path": "lesson.json",
                "byte_length": 3,
                "sha256": "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            }]
        })
    );
}

#[test]
fn checksum_inventory_rejects_unknown_fields() {
    let error = serde_json::from_value::<ChecksumInventory>(serde_json::json!({
        "schema_version": 1,
        "files": [],
        "unexpected": true
    }))
    .expect_err("unknown inventory fields must be rejected");

    assert!(error.to_string().contains("unexpected"));
}

#[test]
fn package_digest_is_lowercase_prefixed_sha256() {
    let digest = compute_package_digest(b"abc");

    assert_eq!(
        digest,
        "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn default_limits_match_the_phase_zero_upload_profile() {
    let limits = PackageLimits::default();

    assert_eq!(
        limits,
        PackageLimits {
            max_archive_bytes: 64 * 1024 * 1024,
            max_files: 256,
            max_file_bytes: 32 * 1024 * 1024,
            max_total_uncompressed_bytes: 64 * 1024 * 1024,
        }
    );
}
