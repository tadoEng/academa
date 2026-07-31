# Daily Reading v1 contract fixture

This is an original, sanitized contract fixture. It contains no private article text,
teacher material, learner data, private source path, or OneDrive reference.

The `valid/` directory is the package-facing bridge between issue #14 and later issue
#15 tests. It defines immutable content references only; it does not instantiate
`Submission`, `ScoreOutcome`, `FeedbackRelease`, `MistakeRecord`, or `ReviewItem`.

Before submission, `lesson.json` is the only learner-facing contract. The ordered
activity list represents timed reading, a committed objective question, contextual
vocabulary review, controlled production, mistake classification, and scheduled
review. `protected-answers.json` remains server-protected and must not be returned or
cached on the learner surface before answer commitment. `manifest.json` carries only
local logical IDs and semantic identity; Axum later issues `publication_id`,
`lesson_version_id`, `package_digest`, and `published_at`.

The semantic `content_digest` covers the decoded learner-safe lesson and protected
answer policy in their declared order. Paths, timestamps, build information, ZIP
metadata, and server-issued IDs are absent from the digest input by construction.
Stable IDs and provenance references are lowercase path-independent values. Asset
declarations, when present, map one stable asset ID to one validated package-relative
path below `assets/`, byte length, media type, SHA-256 digest, and delivery class.

The `invalid/` examples each replace one file from `valid/`:

- `answer-leak/lesson.json` adds protected material to the learner-safe surface;
- `broken-reference/protected-answers.json` targets a nonexistent question;
- `unsupported-version/manifest.json` declares schema version 2.
