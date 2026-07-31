use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use academa_content::{ContractError, LessonContract, LessonManifest, PublicLesson};
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
    let contract = LessonContract::load(&fixture_root()).expect("valid fixture should load");

    contract
        .validate()
        .expect("valid fixture should satisfy the package-facing contract");
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
    let mut contract = LessonContract::load(&fixture_root()).expect("valid fixture should load");
    contract.protected_answers = read_json(&invalid_fixture(
        "broken-reference",
        "protected-answers.json",
    ));

    let error = contract
        .validate()
        .expect_err("unknown question reference must be rejected");

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
    let mut contract = LessonContract::load(&fixture_root()).expect("valid fixture should load");
    contract.manifest =
        read_json::<LessonManifest>(&invalid_fixture("unsupported-version", "manifest.json"));

    let error = contract
        .validate()
        .expect_err("unsupported schema version must be rejected");

    assert!(matches!(
        error,
        ContractError::UnsupportedSchemaVersion { found: 2 }
    ));
}

#[test]
fn serialized_public_lesson_contains_no_protected_keys() {
    let contract = LessonContract::load(&fixture_root()).expect("valid fixture should load");
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
    let original = LessonContract::load(&fixture_root()).expect("valid fixture should load");
    let copied_fixture = TempFixture::copy_from(&fixture_root());
    let copied = LessonContract::load(&copied_fixture.root).expect("copied fixture should load");

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
    let original = LessonContract::load(&fixture_root()).expect("valid fixture should load");
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
