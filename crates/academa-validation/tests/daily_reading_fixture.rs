use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use academa_content::{
    Activity, AssetDeclaration, AssetDeliveryClassification, AssetId, LessonContract,
    LessonManifest, MistakeTagId, MistakeTagReference, ProtectedAnswers, PublicLesson,
    ReviewPolicyId, ReviewPolicyReference, compute_content_digest,
};
use academa_validation::{ContractError, validate};
use serde::de::DeserializeOwned;
use serde_json::Value;

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../contracts/fixtures/daily-reading-v1/valid")
}

fn invalid_fixture(name: &str, file: &str) -> PathBuf {
    fixture_root()
        .parent()
        .expect("valid fixture should have a parent")
        .join("invalid")
        .join(name)
        .join(file)
}

fn read_json<T: DeserializeOwned>(path: &Path) -> T {
    let bytes = std::fs::read(path).expect("fixture file should be readable");
    serde_json::from_slice(&bytes).expect("fixture file should decode")
}

fn load_contract(root: &Path) -> LessonContract {
    LessonContract {
        manifest: read_json(&root.join("manifest.json")),
        lesson: read_json(&root.join("lesson.json")),
        protected_answers: read_json(&root.join("protected-answers.json")),
    }
}

fn asset(id: &str, package_path: &str, sha256: &str) -> AssetDeclaration {
    AssetDeclaration {
        id: AssetId::new(id).expect("test asset ID should be valid"),
        package_path: package_path.to_string(),
        media_type: "audio/mpeg".to_string(),
        byte_length: 128,
        sha256: sha256.to_string(),
        delivery_classification: AssetDeliveryClassification::LearnerSafe,
    }
}

struct TempFixture {
    root: PathBuf,
}

impl TempFixture {
    fn copy_from(source: &Path) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after the Unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "academa-daily-reading-contract-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir(&root).expect("temporary fixture directory should be created");
        for file in ["manifest.json", "lesson.json", "protected-answers.json"] {
            std::fs::copy(source.join(file), root.join(file))
                .expect("fixture file should be copied");
        }
        Self { root }
    }
}

impl Drop for TempFixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

#[test]
fn valid_daily_reading_fixture_loads_and_validates() {
    let contract = load_contract(&fixture_root());

    validate(&contract).expect("valid fixture should satisfy the package-facing contract");
}

#[test]
fn public_lesson_rejects_answer_bearing_field() {
    let path = invalid_fixture("answer-leak", "lesson.json");
    let bytes = std::fs::read(&path).expect("answer-leak fixture should be readable");
    let error = serde_json::from_slice::<PublicLesson>(&bytes)
        .expect_err("answer-bearing public lesson must be rejected");

    assert!(
        error.to_string().contains("accepted_option_ids"),
        "unexpected decode error: {error}"
    );
}

#[test]
fn protected_answers_reject_unknown_question_reference() {
    let mut contract = load_contract(&fixture_root());
    contract.protected_answers = read_json(&invalid_fixture(
        "broken-reference",
        "protected-answers.json",
    ));

    let error = validate(&contract).expect_err("unknown question reference must be rejected");

    assert!(matches!(
        error,
        ContractError::MissingReference {
            source_kind: "answer policy",
            target_kind: "question",
            ref target_id,
            ..
        } if target_id == "question-does-not-exist"
    ));
}

#[test]
fn manifest_rejects_unsupported_schema_version() {
    let mut contract = load_contract(&fixture_root());
    contract.manifest =
        read_json::<LessonManifest>(&invalid_fixture("unsupported-version", "manifest.json"));

    let error = validate(&contract).expect_err("unsupported schema version must be rejected");

    assert!(matches!(
        error,
        ContractError::UnsupportedSchemaVersion { found: 2 }
    ));
}

#[test]
fn provenance_reference_rejects_machine_local_path() {
    let mut lesson_json = serde_json::to_value(load_contract(&fixture_root()).lesson)
        .expect("lesson should serialize");
    lesson_json["provenance"]["source_reference"] =
        Value::String(r"C:\private-source\article.pdf".to_string());

    let error = serde_json::from_value::<PublicLesson>(lesson_json)
        .expect_err("machine-local provenance path must be rejected");

    assert!(error.to_string().contains("provenance reference"));
}

#[test]
fn unsafe_asset_package_path_is_rejected() {
    let mut contract = load_contract(&fixture_root());
    let mut lesson_json = serde_json::to_value(&contract.lesson).expect("lesson should serialize");
    lesson_json["assets"] = serde_json::json!([{
        "id": "asset-unsafe",
        "package_path": "assets/../protected-answers.json",
        "media_type": "text/plain",
        "byte_length": 12,
        "sha256": "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "delivery_classification": "learner_safe"
    }]);
    contract.lesson = serde_json::from_value(lesson_json).expect("asset should decode");

    let error = validate(&contract).expect_err("unsafe asset path must be rejected");

    assert!(matches!(error, ContractError::InvalidAssetPath { .. }));
}

#[test]
fn duplicate_asset_package_path_is_rejected() {
    let mut contract = load_contract(&fixture_root());
    contract.lesson.assets = vec![
        asset(
            "asset-audio-a",
            "assets/reading.mp3",
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ),
        asset(
            "asset-audio-b",
            "assets/reading.mp3",
            "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        ),
    ];

    let error = validate(&contract).expect_err("duplicate asset path must be rejected");

    assert!(matches!(
        error,
        ContractError::DuplicateAssetPath { ref path } if path == "assets/reading.mp3"
    ));
}

#[test]
fn answer_policy_mistake_tag_must_appear_in_classification_activity() {
    let mut contract = load_contract(&fixture_root());
    let alternate_id =
        MistakeTagId::new("mistake-alternate").expect("alternate mistake-tag ID should be valid");
    contract.lesson.mistake_tags.push(MistakeTagReference {
        id: alternate_id.clone(),
        label: "Alternate valid tag".to_string(),
    });
    contract.protected_answers.answer_policies[0].mistake_tag_id = alternate_id;

    let error = validate(&contract)
        .expect_err("answer-policy mistake tag absent from activity must be rejected");

    assert!(matches!(
        error,
        ContractError::PolicyActivityMismatch {
            reference_kind: "mistake tag",
            ref reference_id,
            ..
        } if reference_id == "mistake-alternate"
    ));
}

#[test]
fn answer_policy_review_policy_must_appear_in_scheduled_review_activity() {
    let mut contract = load_contract(&fixture_root());
    let alternate_id = ReviewPolicyId::new("review-alternate")
        .expect("alternate review-policy ID should be valid");
    contract.lesson.review_policies.push(ReviewPolicyReference {
        id: alternate_id.clone(),
        label: "Alternate valid review".to_string(),
    });
    contract.protected_answers.answer_policies[0].review_policy_id = alternate_id;

    let error = validate(&contract)
        .expect_err("answer-policy review absent from activity must be rejected");

    assert!(matches!(
        error,
        ContractError::PolicyActivityMismatch {
            reference_kind: "review policy",
            ref reference_id,
            ..
        } if reference_id == "review-alternate"
    ));
}

#[test]
fn missing_answer_policy_error_order_is_deterministic() {
    let mut contract = load_contract(&fixture_root());
    let Activity::ObjectiveQuestion { .. } = &contract.lesson.activities[1] else {
        panic!("second fixture activity should be the objective question");
    };
    let template = serde_json::to_value(&contract.lesson.activities[1])
        .expect("question activity should serialize");
    for (activity_id, question_id) in [
        ("activity-question-zeta", "question-zeta"),
        ("activity-question-alpha", "question-alpha"),
    ] {
        let mut additional = template.clone();
        additional["id"] = Value::String(activity_id.to_string());
        additional["question_id"] = Value::String(question_id.to_string());
        contract
            .lesson
            .activities
            .push(serde_json::from_value(additional).expect("additional question should decode"));
    }

    let error = validate(&contract).expect_err("missing answer policy must be rejected");

    assert!(matches!(
        error,
        ContractError::AnswerPolicyCardinality { ref question_id }
            if question_id == "question-alpha"
    ));
}

#[test]
fn serialized_public_lesson_contains_no_protected_keys() {
    let contract = load_contract(&fixture_root());
    let public_json =
        serde_json::to_value(&contract.lesson).expect("public lesson should serialize");
    let mut forbidden_paths = Vec::new();

    find_forbidden_keys(&public_json, "$", &mut forbidden_paths);

    assert!(
        forbidden_paths.is_empty(),
        "protected keys leaked into public lesson: {forbidden_paths:?}"
    );
}

#[test]
fn semantic_digest_is_independent_of_fixture_path() {
    let original = load_contract(&fixture_root());
    let copied_fixture = TempFixture::copy_from(&fixture_root());
    let copied = load_contract(&copied_fixture.root);

    assert_eq!(
        serde_json::to_vec(&original.lesson).expect("original public lesson should serialize"),
        serde_json::to_vec(&copied.lesson).expect("copied public lesson should serialize")
    );
    assert_eq!(
        serde_json::to_vec(&original.protected_answers)
            .expect("original protected answers should serialize"),
        serde_json::to_vec(&copied.protected_answers)
            .expect("copied protected answers should serialize")
    );
    assert_eq!(
        original
            .computed_content_digest()
            .expect("original fixture should digest"),
        copied
            .computed_content_digest()
            .expect("copied fixture should digest")
    );
}

#[test]
fn semantic_digest_changes_when_protected_feedback_changes() {
    let original = load_contract(&fixture_root());
    let mut changed = original.clone();
    changed.protected_answers.answer_policies[0]
        .feedback
        .explanation
        .push_str(" Changed after review.");

    assert_ne!(
        original
            .computed_content_digest()
            .expect("original fixture should digest"),
        changed
            .computed_content_digest()
            .expect("changed fixture should digest")
    );
}

#[test]
fn semantic_digest_ignores_asset_package_path_rename() {
    let original = load_contract(&fixture_root());
    let mut first_path = original.clone();
    first_path.lesson.assets.push(asset(
        "asset-reading-audio",
        "assets/reading-a.mp3",
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    ));
    let mut renamed_path = first_path.clone();
    renamed_path.lesson.assets[0].package_path = "assets/reading-b.mp3".to_string();

    assert_eq!(
        first_path
            .computed_content_digest()
            .expect("first asset path should digest"),
        renamed_path
            .computed_content_digest()
            .expect("renamed asset path should digest")
    );
}

#[test]
fn semantic_digest_changes_when_asset_hash_changes() {
    let original = load_contract(&fixture_root());
    let mut first_hash = original.clone();
    first_hash.lesson.assets.push(asset(
        "asset-reading-audio",
        "assets/reading.mp3",
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    ));
    let mut changed_hash = first_hash.clone();
    changed_hash.lesson.assets[0].sha256 =
        "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string();

    assert_ne!(
        first_hash
            .computed_content_digest()
            .expect("first asset hash should digest"),
        changed_hash
            .computed_content_digest()
            .expect("changed asset hash should digest")
    );
}

#[test]
fn fixture_files_round_trip_through_serde_without_semantic_change() {
    let contract = load_contract(&fixture_root());

    let manifest: LessonManifest = serde_json::from_slice(
        &serde_json::to_vec(&contract.manifest).expect("manifest should serialize"),
    )
    .expect("manifest should deserialize");
    let lesson: PublicLesson = serde_json::from_slice(
        &serde_json::to_vec(&contract.lesson).expect("lesson should serialize"),
    )
    .expect("lesson should deserialize");
    let protected_answers: ProtectedAnswers = serde_json::from_slice(
        &serde_json::to_vec(&contract.protected_answers)
            .expect("protected answers should serialize"),
    )
    .expect("protected answers should deserialize");

    assert_eq!(
        contract,
        LessonContract {
            manifest,
            lesson,
            protected_answers,
        }
    );
}

#[test]
fn semantic_digest_is_independent_of_json_formatting_and_field_order() {
    let contract = load_contract(&fixture_root());
    let original_bytes = std::fs::read(fixture_root().join("lesson.json"))
        .expect("fixture lesson should be readable");
    let value: Value =
        serde_json::from_slice(&original_bytes).expect("fixture lesson should decode as JSON");
    let reordered_bytes = serde_json::to_vec(&value).expect("JSON value should serialize");
    let reordered_text = std::str::from_utf8(&reordered_bytes).expect("JSON should be UTF-8");
    assert!(
        reordered_text.starts_with("{\"activities\":"),
        "serde_json's ordered map should move activities before schema_version"
    );
    let reordered_lesson: PublicLesson =
        serde_json::from_slice(&reordered_bytes).expect("reordered lesson should decode");

    assert_eq!(
        contract
            .computed_content_digest()
            .expect("original contract should digest"),
        compute_content_digest(&reordered_lesson, &contract.protected_answers)
            .expect("reordered contract should digest")
    );
}

#[test]
fn blank_required_semantic_text_is_rejected_before_digest_comparison() {
    let mut contract = load_contract(&fixture_root());
    contract.lesson.title = "   ".to_string();

    let error = validate(&contract).expect_err("blank lesson title must be rejected");

    assert!(matches!(
        error,
        ContractError::BlankText {
            field: "lesson.title"
        }
    ));
}

fn find_forbidden_keys(value: &Value, path: &str, found: &mut Vec<String>) {
    const FORBIDDEN: [&str; 7] = [
        "answer_policies",
        "accepted_option_ids",
        "scoring",
        "feedback",
        "explanation",
        "evidence_references",
        "release_policy",
    ];

    match value {
        Value::Object(object) => {
            for (key, child) in object {
                let child_path = format!("{path}.{key}");
                if FORBIDDEN.contains(&key.as_str()) {
                    found.push(child_path.clone());
                }
                find_forbidden_keys(child, &child_path, found);
            }
        }
        Value::Array(values) => {
            for (index, child) in values.iter().enumerate() {
                find_forbidden_keys(child, &format!("{path}[{index}]"), found);
            }
        }
        _ => {}
    }
}
