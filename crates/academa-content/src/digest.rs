use sha2::{Digest, Sha256};

use crate::{ProtectedAnswers, PublicLesson};

#[derive(serde::Serialize)]
struct SemanticContent<'a> {
    schema_version: u32,
    lesson: &'a PublicLesson,
    protected_answers: &'a ProtectedAnswers,
}

/// Computes the lowercase SHA-256 semantic identity for reviewed lesson content.
///
/// The envelope deliberately contains no filesystem path, timestamp, build metadata,
/// ZIP metadata, publication identity, or database identity.
pub fn compute_content_digest(
    lesson: &PublicLesson,
    protected_answers: &ProtectedAnswers,
) -> Result<String, serde_json::Error> {
    let semantic_content = SemanticContent {
        schema_version: lesson.schema_version,
        lesson,
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
