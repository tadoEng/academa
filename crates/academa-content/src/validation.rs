use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use thiserror::Error;

use crate::{Activity, LessonContract};

const SUPPORTED_SCHEMA_VERSION: u32 = 1;
const SUPPORTED_PACKAGE_VERSION: u32 = 1;

/// A deterministic package-facing contract failure.
#[derive(Debug, Error)]
pub enum ContractError {
    #[error("cannot read contract file {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("cannot decode contract file {path}: {source}")]
    DecodeJson {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("schema version {found} is unsupported; expected 1")]
    UnsupportedSchemaVersion { found: u32 },
    #[error("package version {found} is unsupported; expected 1")]
    UnsupportedPackageVersion { found: u32 },
    #[error("{field} does not match across contract files")]
    IdentityMismatch { field: &'static str },
    #[error("{field} must not be empty")]
    EmptyCollection { field: &'static str },
    #[error("{field} must contain at least {minimum} items")]
    TooFewItems { field: &'static str, minimum: usize },
    #[error("duplicate {kind} `{id}`")]
    DuplicateId { kind: &'static str, id: String },
    #[error("{source_kind} `{source_id}` references missing {target_kind} `{target_id}`")]
    MissingReference {
        source_kind: &'static str,
        source_id: String,
        target_kind: &'static str,
        target_id: String,
    },
    #[error("question `{question_id}` must have exactly one protected answer policy")]
    AnswerPolicyCardinality { question_id: String },
    #[error("scoring policy `{answer_policy_id}` must award at least one point")]
    InvalidScoringPolicy { answer_policy_id: String },
    #[error("reviewed canonical content requires human approval")]
    HumanReviewRequired,
    #[error("invalid SHA-256 value for {field}: `{value}`")]
    InvalidDigest { field: &'static str, value: String },
    #[error("cannot serialize semantic content for digest: {0}")]
    DigestSerialization(serde_json::Error),
    #[error("content digest mismatch: declared `{declared}`, computed `{computed}`")]
    DigestMismatch { declared: String, computed: String },
}

pub(crate) fn validate(contract: &LessonContract) -> Result<(), ContractError> {
    validate_versions(contract)?;
    validate_identity(contract)?;
    validate_required_content(contract)?;
    validate_references(contract)?;
    validate_assets(contract)?;
    validate_digest(contract)
}

fn validate_versions(contract: &LessonContract) -> Result<(), ContractError> {
    for version in [
        contract.manifest.schema_version,
        contract.lesson.schema_version,
        contract.protected_answers.schema_version,
    ] {
        if version != SUPPORTED_SCHEMA_VERSION {
            return Err(ContractError::UnsupportedSchemaVersion { found: version });
        }
    }
    if contract.manifest.package_version != SUPPORTED_PACKAGE_VERSION {
        return Err(ContractError::UnsupportedPackageVersion {
            found: contract.manifest.package_version,
        });
    }
    Ok(())
}

fn validate_identity(contract: &LessonContract) -> Result<(), ContractError> {
    if contract.manifest.course_id != contract.lesson.course_id {
        return Err(ContractError::IdentityMismatch { field: "course_id" });
    }
    if contract.manifest.module_id != contract.lesson.module_id {
        return Err(ContractError::IdentityMismatch { field: "module_id" });
    }
    if contract.manifest.lesson_id != contract.lesson.lesson_id
        || contract.manifest.lesson_id != contract.protected_answers.lesson_id
    {
        return Err(ContractError::IdentityMismatch { field: "lesson_id" });
    }
    Ok(())
}

fn validate_required_content(contract: &LessonContract) -> Result<(), ContractError> {
    require_nonempty("objectives", contract.lesson.objectives.len())?;
    require_nonempty(
        "reading_passage.paragraphs",
        contract.lesson.reading_passage.paragraphs.len(),
    )?;
    require_nonempty("activities", contract.lesson.activities.len())?;
    require_nonempty("vocabulary", contract.lesson.vocabulary.len())?;
    require_nonempty(
        "production_prompts",
        contract.lesson.production_prompts.len(),
    )?;
    require_nonempty("mistake_tags", contract.lesson.mistake_tags.len())?;
    require_nonempty("review_policies", contract.lesson.review_policies.len())?;
    require_nonempty(
        "protected_answers.answer_policies",
        contract.protected_answers.answer_policies.len(),
    )?;
    if !contract.lesson.provenance.human_reviewed {
        return Err(ContractError::HumanReviewRequired);
    }

    ensure_unique(
        "objective ID",
        contract
            .lesson
            .objectives
            .iter()
            .map(|objective| objective.id.as_str()),
    )?;
    ensure_unique(
        "paragraph ID",
        contract
            .lesson
            .reading_passage
            .paragraphs
            .iter()
            .map(|paragraph| paragraph.id.as_str()),
    )?;
    ensure_unique(
        "vocabulary entry ID",
        contract
            .lesson
            .vocabulary
            .iter()
            .map(|entry| entry.id.as_str()),
    )?;
    ensure_unique(
        "production prompt ID",
        contract
            .lesson
            .production_prompts
            .iter()
            .map(|prompt| prompt.id.as_str()),
    )?;
    ensure_unique(
        "mistake tag ID",
        contract
            .lesson
            .mistake_tags
            .iter()
            .map(|tag| tag.id.as_str()),
    )?;
    ensure_unique(
        "review policy ID",
        contract
            .lesson
            .review_policies
            .iter()
            .map(|policy| policy.id.as_str()),
    )?;
    ensure_unique(
        "asset ID",
        contract.lesson.assets.iter().map(|asset| asset.id.as_str()),
    )?;
    ensure_unique(
        "answer policy ID",
        contract
            .protected_answers
            .answer_policies
            .iter()
            .map(|policy| policy.id.as_str()),
    )
}

fn validate_references(contract: &LessonContract) -> Result<(), ContractError> {
    let objective_ids: HashSet<_> = contract
        .lesson
        .objectives
        .iter()
        .map(|objective| objective.id.as_str())
        .collect();
    let paragraph_ids: HashSet<_> = contract
        .lesson
        .reading_passage
        .paragraphs
        .iter()
        .map(|paragraph| paragraph.id.as_str())
        .collect();
    let mistake_tag_ids: HashSet<_> = contract
        .lesson
        .mistake_tags
        .iter()
        .map(|tag| tag.id.as_str())
        .collect();
    let review_policy_ids: HashSet<_> = contract
        .lesson
        .review_policies
        .iter()
        .map(|policy| policy.id.as_str())
        .collect();

    let mut question_options = HashMap::<&str, HashSet<&str>>::new();
    let mut activity_ids = HashSet::new();
    for activity in &contract.lesson.activities {
        let Activity::ObjectiveQuestion {
            id,
            objective_id,
            question_id,
            options,
            ..
        } = activity;
        insert_unique("activity ID", &mut activity_ids, id.as_str())?;
        ensure_reference(
            "activity",
            id.as_str(),
            "objective",
            objective_id.as_str(),
            &objective_ids,
        )?;
        if options.len() < 2 {
            return Err(ContractError::TooFewItems {
                field: "activity.options",
                minimum: 2,
            });
        }
        ensure_unique("option ID", options.iter().map(|option| option.id.as_str()))?;
        let option_ids: HashSet<_> = options.iter().map(|option| option.id.as_str()).collect();
        if question_options
            .insert(question_id.as_str(), option_ids)
            .is_some()
        {
            return Err(ContractError::DuplicateId {
                kind: "question ID",
                id: question_id.to_string(),
            });
        }
    }

    for prompt in &contract.lesson.production_prompts {
        ensure_reference(
            "production prompt",
            prompt.id.as_str(),
            "objective",
            prompt.objective_id.as_str(),
            &objective_ids,
        )?;
    }

    let mut policy_count = HashMap::<&str, usize>::new();
    for policy in &contract.protected_answers.answer_policies {
        let Some(option_ids) = question_options.get(policy.question_id.as_str()) else {
            return Err(ContractError::MissingReference {
                source_kind: "answer policy",
                source_id: policy.id.to_string(),
                target_kind: "question",
                target_id: policy.question_id.to_string(),
            });
        };
        *policy_count.entry(policy.question_id.as_str()).or_default() += 1;
        require_nonempty(
            "answer_policy.accepted_option_ids",
            policy.accepted_option_ids.len(),
        )?;
        ensure_unique(
            "accepted option ID",
            policy
                .accepted_option_ids
                .iter()
                .map(|option_id| option_id.as_str()),
        )?;
        require_nonempty(
            "answer_policy.feedback.evidence_references",
            policy.feedback.evidence_references.len(),
        )?;
        for accepted_option_id in &policy.accepted_option_ids {
            ensure_reference(
                "answer policy",
                policy.id.as_str(),
                "answer option",
                accepted_option_id.as_str(),
                option_ids,
            )?;
        }
        ensure_reference(
            "answer policy",
            policy.id.as_str(),
            "mistake tag",
            policy.mistake_tag_id.as_str(),
            &mistake_tag_ids,
        )?;
        ensure_reference(
            "answer policy",
            policy.id.as_str(),
            "review policy",
            policy.review_policy_id.as_str(),
            &review_policy_ids,
        )?;
        for evidence in &policy.feedback.evidence_references {
            ensure_reference(
                "answer policy",
                policy.id.as_str(),
                "paragraph",
                evidence.paragraph_id.as_str(),
                &paragraph_ids,
            )?;
        }
        if policy.scoring.max_score == 0 {
            return Err(ContractError::InvalidScoringPolicy {
                answer_policy_id: policy.id.to_string(),
            });
        }
    }

    for question_id in question_options.keys() {
        if policy_count.get(question_id).copied() != Some(1) {
            return Err(ContractError::AnswerPolicyCardinality {
                question_id: (*question_id).to_string(),
            });
        }
    }
    Ok(())
}

fn validate_assets(contract: &LessonContract) -> Result<(), ContractError> {
    for asset in &contract.lesson.assets {
        validate_sha256("asset.sha256", &asset.sha256)?;
    }
    Ok(())
}

fn validate_digest(contract: &LessonContract) -> Result<(), ContractError> {
    validate_sha256("manifest.content_digest", &contract.manifest.content_digest)?;
    let computed = contract.computed_content_digest()?;
    if contract.manifest.content_digest != computed {
        return Err(ContractError::DigestMismatch {
            declared: contract.manifest.content_digest.clone(),
            computed,
        });
    }
    Ok(())
}

fn validate_sha256(field: &'static str, value: &str) -> Result<(), ContractError> {
    let valid = value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if valid {
        Ok(())
    } else {
        Err(ContractError::InvalidDigest {
            field,
            value: value.to_string(),
        })
    }
}

fn require_nonempty(field: &'static str, length: usize) -> Result<(), ContractError> {
    if length == 0 {
        Err(ContractError::EmptyCollection { field })
    } else {
        Ok(())
    }
}

fn ensure_unique<'a>(
    kind: &'static str,
    ids: impl IntoIterator<Item = &'a str>,
) -> Result<(), ContractError> {
    let mut seen = HashSet::new();
    for id in ids {
        insert_unique(kind, &mut seen, id)?;
    }
    Ok(())
}

fn insert_unique<'a>(
    kind: &'static str,
    seen: &mut HashSet<&'a str>,
    id: &'a str,
) -> Result<(), ContractError> {
    if seen.insert(id) {
        Ok(())
    } else {
        Err(ContractError::DuplicateId {
            kind,
            id: id.to_string(),
        })
    }
}

fn ensure_reference(
    source_kind: &'static str,
    source_id: &str,
    target_kind: &'static str,
    target_id: &str,
    available: &HashSet<&str>,
) -> Result<(), ContractError> {
    if available.contains(target_id) {
        Ok(())
    } else {
        Err(ContractError::MissingReference {
            source_kind,
            source_id: source_id.to_string(),
            target_kind,
            target_id: target_id.to_string(),
        })
    }
}
