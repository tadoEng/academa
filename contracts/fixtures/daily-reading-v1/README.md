# Daily Reading v1 contract fixture

This is an original, sanitized contract fixture. It contains no private article text,
teacher material, learner data, private source path, or OneDrive reference.

The `valid/` directory is the package-facing bridge between issue #14 and later issue
#15 tests. It defines immutable content references only; it does not instantiate
`Submission`, `ScoreOutcome`, `FeedbackRelease`, `MistakeRecord`, or `ReviewItem`.

The semantic `content_digest` covers the decoded learner-safe lesson and protected
answer policy in their declared order. Paths, timestamps, build information, ZIP
metadata, and server-issued IDs are absent from the digest input by construction.

The `invalid/` examples each replace one file from `valid/`:

- `answer-leak/lesson.json` adds protected material to the learner-safe surface;
- `broken-reference/protected-answers.json` targets a nonexistent question;
- `unsupported-version/manifest.json` declares schema version 2.
