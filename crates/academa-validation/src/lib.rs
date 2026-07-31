#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

use std::collections::{BTreeMap, HashSet};
use thiserror::Error;

use academa_content::{Activity, LessonContract};

const SUPPORTED_SCHEMA_VERSION: u32 = 1;
const SUPPORTED_PACKAGE_VERSION: u32 = 1;

/// A deterministic package-facing contract failure.
#[derive(Debug, Error)]
pub enum ContractError {
    /// A contract surface declares an unsupported schema version.
    #[error("schema version {found} is unsupported; expected 1")]
    UnsupportedSchemaVersion {
        /// Unsupported version found in the contract.
        found: u32,
    },
    /// The manifest declares an unsupported package version.
    #[error("package version {found} is unsupported; expected 1")]
    UnsupportedPackageVersion {
        /// Unsupported package version.
        found: u32,
    },
    /// A stable identity differs across contract surfaces.
    #[error("{field} does not match across contract files")]
    IdentityMismatch {
        /// Name of the inconsistent identity field.
        field: &'static str,
    },
    /// A required collection is empty.
    #[error("{field} must not be empty")]
    EmptyCollection {
        /// Name of the empty collection.
        field: &'static str,
    },
    /// A required semantic text value is blank.
    #[error("{field} must contain non-whitespace text")]
    BlankText {
        /// Name of the blank field.
        field: &'static str,
    },
    /// A collection contains fewer than the required number of items.
    #[error("{field} must contain at least {minimum} items")]
    TooFewItems {
        /// Name of the undersized collection.
        field: &'static str,
        /// Minimum accepted item count.
        minimum: usize,
    },
    /// A stable identity appears more than once in one namespace.
    #[error("duplicate {kind} `{id}`")]
    DuplicateId {
        /// Identity namespace.
        kind: &'static str,
        /// Repeated identity.
        id: String,
    },
    /// A logical reference has no declared target.
    #[error("{source_kind} `{source_id}` references missing {target_kind} `{target_id}`")]
    MissingReference {
        /// Kind of object containing the reference.
        source_kind: &'static str,
        /// Identity of the object containing the reference.
        source_id: String,
        /// Required target namespace.
        target_kind: &'static str,
        /// Missing target identity.
        target_id: String,
    },
    /// A scored question does not have exactly one answer policy.
    #[error("question `{question_id}` must have exactly one protected answer policy")]
    AnswerPolicyCardinality {
        /// Question whose policy count is invalid.
        question_id: String,
    },
    /// A scoring policy awards no points.
    #[error("scoring policy `{answer_policy_id}` must award at least one point")]
    InvalidScoringPolicy {
        /// Invalid answer-policy identity.
        answer_policy_id: String,
    },
    /// The ordered flow omits a required Daily Reading activity family.
    #[error("required activity family `{family}` is missing")]
    MissingActivityFamily {
        /// Missing wire-level activity family.
        family: &'static str,
    },
    /// A timed-reading recommendation is zero seconds.
    #[error("timed-reading activity `{activity_id}` must recommend at least one second")]
    InvalidTimer {
        /// Invalid timed-reading activity identity.
        activity_id: String,
    },
    /// An asset path escapes or does not begin below `assets/`.
    #[error("invalid package-relative asset path `{path}`")]
    InvalidAssetPath {
        /// Rejected package path.
        path: String,
    },
    /// An asset declares an empty byte stream.
    #[error("asset `{asset_id}` must contain at least one byte")]
    InvalidAssetLength {
        /// Empty asset identity.
        asset_id: String,
    },
    /// Canonical content lacks explicit human approval.
    #[error("reviewed canonical content requires human approval")]
    HumanReviewRequired,
    /// A SHA-256 field does not use the canonical lowercase wire format.
    #[error("invalid SHA-256 value for {field}: `{value}`")]
    InvalidDigest {
        /// Invalid digest field.
        field: &'static str,
        /// Rejected digest value.
        value: String,
    },
    /// Typed semantic content could not be serialized for hashing.
    #[error("cannot serialize semantic content for digest: {0}")]
    DigestSerialization(serde_json::Error),
    /// Declared and computed semantic digests differ.
    #[error("content digest mismatch: declared `{declared}`, computed `{computed}`")]
    DigestMismatch {
        /// Manifest digest.
        declared: String,
        /// Digest computed from typed semantic content.
        computed: String,
    },
}

/// Validates schema versions, identities, semantic content, cross-file references, assets, and the declared digest.
///
/// # Errors
///
/// Returns the first deterministic [`ContractError`] in the fixed validation order.
pub fn validate(contract: &LessonContract) -> Result<(), ContractError> {
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
    require_text("lesson.title", &contract.lesson.title)?;
    require_text(
        "reading_passage.title",
        &contract.lesson.reading_passage.title,
    )?;
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

    for objective in &contract.lesson.objectives {
        require_text("objective.description", &objective.description)?;
    }
    for paragraph in &contract.lesson.reading_passage.paragraphs {
        require_text("reading_passage.paragraph.text", &paragraph.text)?;
    }
    for entry in &contract.lesson.vocabulary {
        require_text("vocabulary.term", &entry.term)?;
        require_text("vocabulary.definition", &entry.definition)?;
        require_text("vocabulary.source_sentence", &entry.source_sentence)?;
    }
    for prompt in &contract.lesson.production_prompts {
        require_text("production_prompt.prompt", &prompt.prompt)?;
    }
    for tag in &contract.lesson.mistake_tags {
        require_text("mistake_tag.label", &tag.label)?;
    }
    for policy in &contract.lesson.review_policies {
        require_text("review_policy.label", &policy.label)?;
    }
    for policy in &contract.protected_answers.answer_policies {
        require_text(
            "answer_policy.feedback.explanation",
            &policy.feedback.explanation,
        )?;
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
    let vocabulary_entry_ids: HashSet<_> = contract
        .lesson
        .vocabulary
        .iter()
        .map(|entry| entry.id.as_str())
        .collect();
    let production_prompt_ids: HashSet<_> = contract
        .lesson
        .production_prompts
        .iter()
        .map(|prompt| prompt.id.as_str())
        .collect();

    let mut question_options = BTreeMap::<&str, HashSet<&str>>::new();
    let mut activity_ids = HashSet::new();
    let mut has_timed_reading = false;
    let mut has_objective_question = false;
    let mut has_vocabulary_review = false;
    let mut has_production = false;
    let mut has_mistake_classification = false;
    let mut has_scheduled_review = false;
    for activity in &contract.lesson.activities {
        match activity {
            Activity::TimedReading {
                id,
                passage_id,
                recommended_duration_seconds,
            } => {
                insert_unique("activity ID", &mut activity_ids, id.as_str())?;
                has_timed_reading = true;
                if passage_id != &contract.lesson.reading_passage.id {
                    return Err(ContractError::MissingReference {
                        source_kind: "timed-reading activity",
                        source_id: id.to_string(),
                        target_kind: "reading passage",
                        target_id: passage_id.to_string(),
                    });
                }
                if *recommended_duration_seconds == 0 {
                    return Err(ContractError::InvalidTimer {
                        activity_id: id.to_string(),
                    });
                }
            }
            Activity::ObjectiveQuestion {
                id,
                objective_id,
                question_id,
                prompt,
                options,
            } => {
                insert_unique("activity ID", &mut activity_ids, id.as_str())?;
                has_objective_question = true;
                require_text("activity.prompt", prompt)?;
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
                for option in options {
                    require_text("activity.option.text", &option.text)?;
                }
                let option_ids: HashSet<_> =
                    options.iter().map(|option| option.id.as_str()).collect();
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
            Activity::VocabularyReview {
                id,
                vocabulary_entry_ids: entry_ids,
            } => {
                insert_unique("activity ID", &mut activity_ids, id.as_str())?;
                has_vocabulary_review = true;
                require_nonempty("activity.vocabulary_entry_ids", entry_ids.len())?;
                ensure_unique(
                    "activity vocabulary entry ID",
                    entry_ids.iter().map(|entry_id| entry_id.as_str()),
                )?;
                for entry_id in entry_ids {
                    ensure_reference(
                        "vocabulary-review activity",
                        id.as_str(),
                        "vocabulary entry",
                        entry_id.as_str(),
                        &vocabulary_entry_ids,
                    )?;
                }
            }
            Activity::Production {
                id,
                production_prompt_id,
            } => {
                insert_unique("activity ID", &mut activity_ids, id.as_str())?;
                has_production = true;
                ensure_reference(
                    "production activity",
                    id.as_str(),
                    "production prompt",
                    production_prompt_id.as_str(),
                    &production_prompt_ids,
                )?;
            }
            Activity::MistakeClassification {
                id,
                mistake_tag_ids: tag_ids,
            } => {
                insert_unique("activity ID", &mut activity_ids, id.as_str())?;
                has_mistake_classification = true;
                require_nonempty("activity.mistake_tag_ids", tag_ids.len())?;
                ensure_unique(
                    "activity mistake tag ID",
                    tag_ids.iter().map(|tag_id| tag_id.as_str()),
                )?;
                for tag_id in tag_ids {
                    ensure_reference(
                        "mistake-classification activity",
                        id.as_str(),
                        "mistake tag",
                        tag_id.as_str(),
                        &mistake_tag_ids,
                    )?;
                }
            }
            Activity::ScheduledReview {
                id,
                review_policy_id,
            } => {
                insert_unique("activity ID", &mut activity_ids, id.as_str())?;
                has_scheduled_review = true;
                ensure_reference(
                    "scheduled-review activity",
                    id.as_str(),
                    "review policy",
                    review_policy_id.as_str(),
                    &review_policy_ids,
                )?;
            }
        }
    }
    for (family, found) in [
        ("timed_reading", has_timed_reading),
        ("objective_question", has_objective_question),
        ("vocabulary_review", has_vocabulary_review),
        ("production", has_production),
        ("mistake_classification", has_mistake_classification),
        ("scheduled_review", has_scheduled_review),
    ] {
        if !found {
            return Err(ContractError::MissingActivityFamily { family });
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

    let mut policy_count = BTreeMap::<&str, usize>::new();
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
        validate_asset_path(&asset.package_path)?;
        require_text("asset.media_type", &asset.media_type)?;
        if asset.byte_length == 0 {
            return Err(ContractError::InvalidAssetLength {
                asset_id: asset.id.to_string(),
            });
        }
        validate_sha256("asset.sha256", &asset.sha256)?;
    }
    Ok(())
}

fn validate_asset_path(path: &str) -> Result<(), ContractError> {
    let valid = path.starts_with("assets/")
        && !path.ends_with('/')
        && !path.contains('\\')
        && !path.contains(':')
        && path
            .split('/')
            .all(|segment| !segment.is_empty() && segment != "." && segment != "..");
    if valid {
        Ok(())
    } else {
        Err(ContractError::InvalidAssetPath {
            path: path.to_string(),
        })
    }
}

fn validate_digest(contract: &LessonContract) -> Result<(), ContractError> {
    validate_sha256("manifest.content_digest", &contract.manifest.content_digest)?;
    let computed = contract
        .computed_content_digest()
        .map_err(ContractError::DigestSerialization)?;
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

fn require_text(field: &'static str, value: &str) -> Result<(), ContractError> {
    if value.trim().is_empty() {
        Err(ContractError::BlankText { field })
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
