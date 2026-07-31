# academa-validation

`academa-validation` owns pure, deterministic validation across the three reviewed
Daily Reading contract surfaces: `LessonManifest`, `PublicLesson`, and
`ProtectedAnswers`. It accepts an in-memory `LessonContract` and returns either
success or the first error in a fixed validation order.

## Responsibilities

- Enforce supported schema and package versions.
- Verify stable identity agreement and cross-file references.
- Require the minimum ordered Daily Reading activity families.
- Validate protected-answer policy cardinality and answer-release references.
- Validate semantic fields, package-relative asset paths, hashes, and the declared
  semantic digest.

## Non-goals and dependencies

The crate performs no filesystem, network, OCR, AI, ZIP, HTTP, database, learner-state,
frontend, or cloud work. It may depend on `academa-content`; no provider or hosted crate
may enter this boundary.

## Public contract

Call `academa_validation::validate(&contract)` with a decoded
`academa_content::LessonContract`. Loading files remains a caller concern until the
package compiler owns that boundary.

## Determinism, security, and versioning

Validation order is fixed, answer-bearing references are checked against learner-safe
question identifiers, asset paths must remain below `assets/`, and only schema/package
version 1 is accepted. Invalid inputs never trigger I/O or mutate the contract.

## Verification

Run `cargo test -p academa-validation` and
`cargo clippy -p academa-validation --all-targets --all-features -- -D warnings`.
Integration tests load the sanitized fixture, prove Serde round trips, and exercise
answer-leak, reference, version, path, and deterministic-error failures.

## Stability and related contracts

This is an additive Phase 0 contract crate governed by issues #6, #13, and #14. It
does not change the tagged `aet-v0.1-vocabulary-pipeline` behavior and is not yet a
general public API.
