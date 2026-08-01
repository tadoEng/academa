use serde::{Deserialize, Serialize};

use crate::{
    ActivityId, AnswerPolicyId, AssetId, CourseId, LessonId, MistakeTagId, ModuleId, ObjectiveId,
    OptionId, ParagraphId, ProductionPromptId, ProvenanceReference, QuestionId, ReadingPassageId,
    ReviewPolicyId, VocabularyEntryId,
};

/// The only package metadata authored locally before publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LessonManifest {
    /// Canonical content schema version.
    pub schema_version: u32,
    /// Transport package format version.
    pub package_version: u32,
    /// Stable course identity.
    pub course_id: CourseId,
    /// Stable module identity.
    pub module_id: ModuleId,
    /// Stable logical lesson identity.
    pub lesson_id: LessonId,
    /// Semantic SHA-256 identity of reviewed content.
    pub content_digest: String,
}

/// Learner-safe lesson content that contains no answer-bearing material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicLesson {
    /// Canonical content schema version.
    pub schema_version: u32,
    /// Stable course identity.
    pub course_id: CourseId,
    /// Stable module identity.
    pub module_id: ModuleId,
    /// Stable logical lesson identity.
    pub lesson_id: LessonId,
    /// Learner-facing lesson title.
    pub title: String,
    /// Measurable learning objectives.
    pub objectives: Vec<Objective>,
    /// Learner-facing passage.
    pub reading_passage: ReadingPassage,
    /// Selected contextual vocabulary.
    pub vocabulary: Vec<VocabularyEntry>,
    /// Ordered learner activity flow.
    pub activities: Vec<Activity>,
    /// Reviewed learner-production prompts.
    pub production_prompts: Vec<ProductionPrompt>,
    /// Immutable mistake-taxonomy references.
    pub mistake_tags: Vec<MistakeTagReference>,
    /// Immutable review-policy references.
    pub review_policies: Vec<ReviewPolicyReference>,
    /// Declared package assets.
    pub assets: Vec<AssetDeclaration>,
    /// Reviewed source and rights record.
    pub provenance: Provenance,
}

/// One measurable learning objective.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Objective {
    /// Stable objective identity.
    pub id: ObjectiveId,
    /// Learner-facing outcome description.
    pub description: String,
}

/// One identified reading passage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadingPassage {
    /// Stable passage identity.
    pub id: ReadingPassageId,
    /// Learner-facing passage title.
    pub title: String,
    /// Ordered passage paragraphs.
    pub paragraphs: Vec<PassageParagraph>,
}

/// One identified paragraph within a passage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PassageParagraph {
    /// Stable paragraph identity.
    pub id: ParagraphId,
    /// Learner-facing paragraph text.
    pub text: String,
}

/// One reviewed vocabulary item in passage context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VocabularyEntry {
    /// Stable vocabulary-entry identity.
    pub id: VocabularyEntryId,
    /// Headword or phrase.
    pub term: String,
    /// Optional reviewed IPA; never fabricated.
    pub ipa: Option<String>,
    /// English definition for this context.
    pub definition: String,
    /// Reviewed source-context sentence.
    pub source_sentence: String,
}

/// One ordered Daily Reading activity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Activity {
    /// Timed passage-reading step.
    TimedReading {
        /// Stable activity identity.
        id: ActivityId,
        /// Passage presented by this step.
        passage_id: ReadingPassageId,
        /// Recommended learner timer duration.
        recommended_duration_seconds: u32,
    },
    /// Committed scored objective question.
    ObjectiveQuestion {
        /// Stable activity identity.
        id: ActivityId,
        /// Objective assessed by the question.
        objective_id: ObjectiveId,
        /// Stable question identity.
        question_id: QuestionId,
        /// Learner-facing question prompt.
        prompt: String,
        /// Learner-facing answer options without solution data.
        options: Vec<AnswerOption>,
    },
    /// Contextual vocabulary-review step.
    VocabularyReview {
        /// Stable activity identity.
        id: ActivityId,
        /// Ordered vocabulary entries to review.
        vocabulary_entry_ids: Vec<VocabularyEntryId>,
    },
    /// Controlled learner-production step.
    Production {
        /// Stable activity identity.
        id: ActivityId,
        /// Prompt presented by this step.
        production_prompt_id: ProductionPromptId,
    },
    /// Learner-visible mistake-classification step.
    MistakeClassification {
        /// Stable activity identity.
        id: ActivityId,
        /// Allowed immutable mistake-tag references.
        mistake_tag_ids: Vec<MistakeTagId>,
    },
    /// Scheduled retry or review step.
    ScheduledReview {
        /// Stable activity identity.
        id: ActivityId,
        /// Policy governing the later review.
        review_policy_id: ReviewPolicyId,
    },
}

/// One learner-visible option for an objective question.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerOption {
    /// Stable option identity.
    pub id: OptionId,
    /// Learner-facing option text.
    pub text: String,
}

/// One controlled learner-production prompt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductionPrompt {
    /// Stable prompt identity.
    pub id: ProductionPromptId,
    /// Objective developed by the prompt.
    pub objective_id: ObjectiveId,
    /// Learner-facing prompt text.
    pub prompt: String,
}

/// Package-facing reference into the mistake taxonomy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MistakeTagReference {
    /// Stable mistake-tag identity.
    pub id: MistakeTagId,
    /// Learner-safe display label.
    pub label: String,
}

/// Package-facing deterministic review-policy reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewPolicyReference {
    /// Stable review-policy identity.
    pub id: ReviewPolicyId,
    /// Learner-safe policy label.
    pub label: String,
}

/// One file declared for inclusion below the package `assets/` directory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssetDeclaration {
    /// Stable asset identity.
    pub id: AssetId,
    /// Validated package-relative path below `assets/`.
    pub package_path: String,
    /// Declared IANA media type.
    pub media_type: String,
    /// Exact source byte length.
    pub byte_length: u64,
    /// Exact lowercase SHA-256 digest.
    pub sha256: String,
    /// Learner-delivery security classification.
    pub delivery_classification: AssetDeliveryClassification,
}

/// Security classification controlling later asset delivery.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetDeliveryClassification {
    /// Asset may be delivered with authorized learner content.
    LearnerSafe,
    /// Asset requires a protected post-authorization path.
    Protected,
}

/// Reviewed, path-independent provenance and rights metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    /// Class of reviewed source.
    pub source_kind: SourceKind,
    /// Stable source identity without filesystem information.
    pub source_reference: ProvenanceReference,
    /// Approved publication-rights status.
    pub rights_status: RightsStatus,
    /// Whether a human approved the canonical content.
    pub human_reviewed: bool,
}

/// Supported reviewed-source class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    /// Original sanitized fixture used for contract proof.
    SanitizedFixture,
}

/// Supported rights decision at the deterministic boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RightsStatus {
    /// Original content created for a public-safe fixture.
    OriginalFixture,
    /// Private study material explicitly approved for restricted use.
    ApprovedPrivateStudy,
}

/// Server-side material that must never enter a pre-submission learner response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtectedAnswers {
    /// Canonical content schema version.
    pub schema_version: u32,
    /// Stable logical lesson identity.
    pub lesson_id: LessonId,
    /// Protected scoring and release policies.
    pub answer_policies: Vec<AnswerPolicy>,
}

/// Protected policy for scoring and releasing feedback for one question.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerPolicy {
    /// Stable answer-policy identity.
    pub id: AnswerPolicyId,
    /// Question governed by this policy.
    pub question_id: QuestionId,
    /// Accepted learner-safe option identities.
    pub accepted_option_ids: Vec<OptionId>,
    /// Deterministic scoring policy.
    pub scoring: ScoringPolicy,
    /// Protected feedback released by policy.
    pub feedback: FeedbackTemplate,
    /// Mistake tag recorded for an incorrect response.
    pub mistake_tag_id: MistakeTagId,
    /// Review policy referenced after an incorrect response.
    pub review_policy_id: ReviewPolicyId,
    /// Earliest permitted answer-release state.
    pub release_policy: AnswerReleasePolicy,
}

/// Deterministic score ceiling for one answer policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScoringPolicy {
    /// Maximum awarded whole-number score.
    pub max_score: u32,
}

/// Protected explanation and supporting evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeedbackTemplate {
    /// Explanation released only under the answer policy.
    pub explanation: String,
    /// Evidence locations supporting the explanation.
    pub evidence_references: Vec<EvidenceReference>,
}

/// Stable evidence pointer into the learner-facing passage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceReference {
    /// Referenced passage paragraph.
    pub paragraph_id: ParagraphId,
}

/// Policy controlling when protected answers may be released.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerReleasePolicy {
    /// Release only after the learner commits an answer.
    AfterCommit,
}
