# academa-content

`academa-content` is the pure package-facing type system for reviewed Academa lessons.
It starts after human approval and defines stable logical IDs, learner-safe content,
separately protected answer policy, path-independent provenance, and semantic content
identity.

## Responsibilities and public contract

- `LessonManifest` carries locally authored schema/package versions, logical
  course/module/lesson IDs, and `content_digest`.
- `PublicLesson` carries the ordered Daily Reading flow and contains no accepted
  answers, scoring, explanations, or evidence locations.
- `ProtectedAnswers` carries server-only scoring, feedback, mistake, review, and
  answer-release policy.
- `LessonContract` groups those three already-decoded surfaces in memory.
- `compute_content_digest` hashes the canonical typed public and protected semantic
  content while excluding paths, timestamps, ZIP metadata, and server-issued IDs.

## Non-goals and dependency policy

This crate performs no filesystem, ZIP, HTTP, database, network, OCR, AI, learner-state,
frontend, or cloud-provider work. It depends only on Serde, SHA-256, and library-grade
error support. Cross-file validation belongs to `academa-validation`; loading and
archive assembly will belong to the package boundary.

## Minimal usage

```rust
use academa_content::LessonContract;

fn semantic_identity(contract: &LessonContract) -> String {
    contract
        .computed_content_digest()
        .expect("reviewed contract types should serialize")
}
```

## Determinism, security, and versioning

Logical IDs and provenance references are lowercase path-independent identifiers.
`PublicLesson` and `ProtectedAnswers` are different Rust and JSON types. Serde rejects
unknown fields so protected keys cannot be silently accepted on the learner-safe
surface. Schema compatibility is explicit and validated outside this crate.

## Verification

Run `cargo test -p academa-content` and
`cargo clippy -p academa-content --all-targets --all-features -- -D warnings`.
The `academa-validation` integration tests execute the full sanitized contract fixture.

## Stability and related contracts

This is an additive Phase 0 contract governed by issues #6, #13, and #14. It preserves
the tagged `aet-v0.1-vocabulary-pipeline` and is not yet a general public API. Package,
HTTP, persistence, and frontend contracts are separate versioned surfaces.
