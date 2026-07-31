use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Explains why a package-facing logical identifier was rejected.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "invalid {kind} `{value}`: expected lowercase ASCII letters or digits separated by single hyphens"
)]
pub struct IdError {
    kind: &'static str,
    value: String,
}

fn validate_id(kind: &'static str, value: String) -> Result<String, IdError> {
    let valid = !value.is_empty()
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--")
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-');

    if valid {
        Ok(value)
    } else {
        Err(IdError { kind, value })
    }
}

macro_rules! logical_id {
    ($(#[$meta:meta])* $name:ident, $kind:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            /// Creates an identifier after enforcing the canonical kebab-case format.
            pub fn new(value: impl Into<String>) -> Result<Self, IdError> {
                Self::try_from(value.into())
            }

            /// Returns the stable wire representation.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = IdError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                validate_id($kind, value).map(Self)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

logical_id!(
    /// Stable course identity chosen before publication.
    CourseId,
    "course ID"
);
logical_id!(
    /// Stable module identity chosen before publication.
    ModuleId,
    "module ID"
);
logical_id!(
    /// Stable lesson identity chosen before publication.
    LessonId,
    "lesson ID"
);
logical_id!(
    /// Stable learning-objective identity.
    ObjectiveId,
    "objective ID"
);
logical_id!(
    /// Stable reading-passage paragraph identity.
    ParagraphId,
    "paragraph ID"
);
logical_id!(
    /// Stable vocabulary-entry identity.
    VocabularyEntryId,
    "vocabulary entry ID"
);
logical_id!(
    /// Stable activity identity.
    ActivityId,
    "activity ID"
);
logical_id!(
    /// Stable question identity.
    QuestionId,
    "question ID"
);
logical_id!(
    /// Stable answer-option identity.
    OptionId,
    "option ID"
);
logical_id!(
    /// Stable protected-answer policy identity.
    AnswerPolicyId,
    "answer policy ID"
);
logical_id!(
    /// Stable mistake-taxonomy reference.
    MistakeTagId,
    "mistake tag ID"
);
logical_id!(
    /// Stable review-policy reference.
    ReviewPolicyId,
    "review policy ID"
);
logical_id!(
    /// Stable learner-production prompt identity.
    ProductionPromptId,
    "production prompt ID"
);
logical_id!(
    /// Stable declared asset identity.
    AssetId,
    "asset ID"
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logical_id_accepts_lowercase_kebab_case() {
        let id = LessonId::new("lesson-daily-reading-1").expect("ID should be valid");

        assert_eq!(id.as_str(), "lesson-daily-reading-1");
    }

    #[test]
    fn logical_id_rejects_path_dependent_value() {
        let error = LessonId::new("private/lesson").expect_err("path separator must be rejected");

        assert_eq!(error.kind, "lesson ID");
    }
}
