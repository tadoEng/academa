use sha2::{Digest, Sha256};

use crate::{
    Activity, AssetDeclaration, AssetDeliveryClassification, AssetId, CourseId, LessonId,
    MistakeTagReference, ModuleId, Objective, ProductionPrompt, ProtectedAnswers, Provenance,
    PublicLesson, ReadingPassage, ReviewPolicyReference, VocabularyEntry,
};

#[derive(serde::Serialize)]
struct SemanticContent<'a> {
    schema_version: u32,
    lesson: SemanticLesson<'a>,
    protected_answers: &'a ProtectedAnswers,
}

#[derive(serde::Serialize)]
struct SemanticLesson<'a> {
    schema_version: u32,
    course_id: &'a CourseId,
    module_id: &'a ModuleId,
    lesson_id: &'a LessonId,
    title: &'a str,
    objectives: &'a [Objective],
    reading_passage: &'a ReadingPassage,
    vocabulary: &'a [VocabularyEntry],
    activities: &'a [Activity],
    production_prompts: &'a [ProductionPrompt],
    mistake_tags: &'a [MistakeTagReference],
    review_policies: &'a [ReviewPolicyReference],
    assets: Vec<SemanticAsset<'a>>,
    provenance: &'a Provenance,
}

#[derive(serde::Serialize)]
struct SemanticAsset<'a> {
    id: &'a AssetId,
    media_type: &'a str,
    byte_length: u64,
    sha256: &'a str,
    delivery_classification: AssetDeliveryClassification,
}

impl<'a> From<&'a AssetDeclaration> for SemanticAsset<'a> {
    fn from(asset: &'a AssetDeclaration) -> Self {
        Self {
            id: &asset.id,
            media_type: &asset.media_type,
            byte_length: asset.byte_length,
            sha256: &asset.sha256,
            delivery_classification: asset.delivery_classification,
        }
    }
}

impl<'a> From<&'a PublicLesson> for SemanticLesson<'a> {
    fn from(lesson: &'a PublicLesson) -> Self {
        Self {
            schema_version: lesson.schema_version,
            course_id: &lesson.course_id,
            module_id: &lesson.module_id,
            lesson_id: &lesson.lesson_id,
            title: &lesson.title,
            objectives: &lesson.objectives,
            reading_passage: &lesson.reading_passage,
            vocabulary: &lesson.vocabulary,
            activities: &lesson.activities,
            production_prompts: &lesson.production_prompts,
            mistake_tags: &lesson.mistake_tags,
            review_policies: &lesson.review_policies,
            assets: lesson.assets.iter().map(SemanticAsset::from).collect(),
            provenance: &lesson.provenance,
        }
    }
}

/// Computes the lowercase SHA-256 semantic identity for reviewed lesson content.
///
/// The envelope includes reviewed provenance and rights decisions, but deliberately
/// excludes asset package paths, filesystem paths, storage keys, transport filenames,
/// timestamps, build metadata, ZIP metadata, publication identity, and database identity.
pub fn compute_content_digest(
    lesson: &PublicLesson,
    protected_answers: &ProtectedAnswers,
) -> Result<String, serde_json::Error> {
    let semantic_content = SemanticContent {
        schema_version: lesson.schema_version,
        lesson: SemanticLesson::from(lesson),
        protected_answers,
    };
    let bytes = serde_json::to_vec(&semantic_content)?;
    let digest = Sha256::digest(bytes);
    const HEX: &[u8; 16] = b"0123456789abcdef";

    let mut encoded = String::with_capacity(71);
    encoded.push_str("sha256:");
    for byte in digest {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    Ok(encoded)
}
