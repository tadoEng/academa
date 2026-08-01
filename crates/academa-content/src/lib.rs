#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod digest;
mod ids;
mod model;

pub use digest::compute_content_digest;
pub use ids::*;
pub use model::*;

/// A complete reviewed lesson contract loaded from its three package-facing JSON files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonContract {
    /// Local package metadata and declared semantic digest.
    pub manifest: LessonManifest,
    /// Learner-safe content that may be served before answer commitment.
    pub lesson: PublicLesson,
    /// Server-only answer, scoring, feedback, and review policy material.
    pub protected_answers: ProtectedAnswers,
}

impl LessonContract {
    /// Calculates the semantic digest over public content and protected answer policy.
    ///
    /// # Errors
    ///
    /// Returns [`serde_json::Error`] if canonical serialization fails.
    pub fn computed_content_digest(&self) -> Result<String, serde_json::Error> {
        compute_content_digest(&self.lesson, &self.protected_answers)
    }
}
