use serde::{Deserialize, Serialize};

use crate::{
    ActivityId, AnswerPolicyId, AssetId, CourseId, LessonId, MistakeTagId, ModuleId, ObjectiveId,
    OptionId, ParagraphId, ProductionPromptId, QuestionId, ReviewPolicyId, VocabularyEntryId,
};

/// The only package metadata authored locally before publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LessonManifest {
    pub schema_version: u32,
    pub package_version: u32,
    pub course_id: CourseId,
    pub module_id: ModuleId,
    pub lesson_id: LessonId,
    pub content_digest: String,
}

/// Learner-safe lesson content that contains no answer-bearing material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicLesson {
    pub schema_version: u32,
    pub course_id: CourseId,
    pub module_id: ModuleId,
    pub lesson_id: LessonId,
    pub title: String,
    pub objectives: Vec<Objective>,
    pub reading_passage: ReadingPassage,
    pub vocabulary: Vec<VocabularyEntry>,
    pub activities: Vec<Activity>,
    pub production_prompts: Vec<ProductionPrompt>,
    pub mistake_tags: Vec<MistakeTagReference>,
    pub review_policies: Vec<ReviewPolicyReference>,
    pub assets: Vec<AssetDeclaration>,
    pub provenance: Provenance,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Objective {
    pub id: ObjectiveId,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadingPassage {
    pub title: String,
    pub paragraphs: Vec<PassageParagraph>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PassageParagraph {
    pub id: ParagraphId,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VocabularyEntry {
    pub id: VocabularyEntryId,
    pub term: String,
    pub ipa: Option<String>,
    pub definition: String,
    pub source_sentence: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Activity {
    ObjectiveQuestion {
        id: ActivityId,
        objective_id: ObjectiveId,
        question_id: QuestionId,
        prompt: String,
        options: Vec<AnswerOption>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerOption {
    pub id: OptionId,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductionPrompt {
    pub id: ProductionPromptId,
    pub objective_id: ObjectiveId,
    pub prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MistakeTagReference {
    pub id: MistakeTagId,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewPolicyReference {
    pub id: ReviewPolicyId,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssetDeclaration {
    pub id: AssetId,
    pub media_type: String,
    pub byte_length: u64,
    pub sha256: String,
    pub delivery_classification: AssetDeliveryClassification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetDeliveryClassification {
    LearnerSafe,
    Protected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Provenance {
    pub source_kind: SourceKind,
    pub source_reference: String,
    pub rights_status: RightsStatus,
    pub human_reviewed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    SanitizedFixture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RightsStatus {
    OriginalFixture,
    ApprovedPrivateStudy,
}

/// Server-side material that must never enter a pre-submission learner response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProtectedAnswers {
    pub schema_version: u32,
    pub lesson_id: LessonId,
    pub answer_policies: Vec<AnswerPolicy>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnswerPolicy {
    pub id: AnswerPolicyId,
    pub question_id: QuestionId,
    pub accepted_option_ids: Vec<OptionId>,
    pub scoring: ScoringPolicy,
    pub feedback: FeedbackTemplate,
    pub mistake_tag_id: MistakeTagId,
    pub review_policy_id: ReviewPolicyId,
    pub release_policy: AnswerReleasePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScoringPolicy {
    pub max_score: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FeedbackTemplate {
    pub explanation: String,
    pub evidence_references: Vec<EvidenceReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceReference {
    pub paragraph_id: ParagraphId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnswerReleasePolicy {
    AfterCommit,
}
