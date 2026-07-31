//! Package-facing content contracts for reviewed Academa lessons.
//!
//! This crate begins at the deterministic boundary: callers provide human-reviewed
//! canonical JSON and receive typed content, semantic validation, and a content digest.
//! It does not perform extraction, OCR, AI-assisted drafting, ZIP compilation,
//! publication, or learner-state persistence.

mod digest;
mod ids;
mod model;
mod validation;

use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;

pub use digest::compute_content_digest;
pub use ids::*;
pub use model::*;
pub use validation::ContractError;

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
    /// Loads `manifest.json`, `lesson.json`, and `protected-answers.json` from `root`.
    ///
    /// # Errors
    ///
    /// Returns [`ContractError`] when a file cannot be read or decoded. Call
    /// [`Self::validate`] separately to enforce cross-file invariants.
    pub fn load(root: &Path) -> Result<Self, ContractError> {
        Ok(Self {
            manifest: read_json(&root.join("manifest.json"))?,
            lesson: read_json(&root.join("lesson.json"))?,
            protected_answers: read_json(&root.join("protected-answers.json"))?,
        })
    }

    /// Validates schema versions, identities, cross-file references, and the content digest.
    ///
    /// # Errors
    ///
    /// Returns the first deterministic [`ContractError`] found.
    pub fn validate(&self) -> Result<(), ContractError> {
        validation::validate(self)
    }

    /// Calculates the semantic digest over public content and protected answer policy.
    ///
    /// # Errors
    ///
    /// Returns [`ContractError::DigestSerialization`] if canonical serialization fails.
    pub fn computed_content_digest(&self) -> Result<String, ContractError> {
        compute_content_digest(&self.lesson, &self.protected_answers)
            .map_err(ContractError::DigestSerialization)
    }
}

fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, ContractError> {
    let bytes = std::fs::read(path).map_err(|source| ContractError::ReadFile {
        path: PathBuf::from(path),
        source,
    })?;
    serde_json::from_slice(&bytes).map_err(|source| ContractError::DecodeJson {
        path: PathBuf::from(path),
        source,
    })
}
