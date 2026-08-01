# Daily Reading Contract Fixture Implementation Plan

> **Final-state implementation record:** This plan describes the two-crate architecture
> implemented by PR #17 after independent and exact-head review corrections. It
> supersedes the original one-crate draft; no filesystem loading or cross-file
> validation belongs to `academa-content`.

**Goal:** Add one sanitized, executable Daily Reading package-facing contract that
proves stable logical identities, learner-safe/protected separation, semantic digest
stability, deterministic validation, and explicit rejection of unsafe or inconsistent
inputs without changing the protected AET pipeline.

**Architecture:** `academa-content` is a pure in-memory type and semantic-digest crate.
`academa-validation` depends one-way on `academa-content` and owns deterministic
cross-file validation. Fixture tests perform local file loading; ZIP assembly, hosted
ingestion, learner state, frontend contracts, and infrastructure remain outside this
slice.

**Tech stack:** Rust 1.94, Serde/serde_json, thiserror, SHA-256, Cargo integration tests,
and original sanitized JSON fixtures.

---

## Final file structure

- `Cargo.toml` and `Cargo.lock` register both additive crates and dependencies.
- `crates/academa-content/`
  - `README.md` documents the pure contract and digest boundary.
  - `src/ids.rs` defines validated path-independent logical IDs.
  - `src/model.rs` defines manifest, learner-safe lesson, activity, asset, provenance,
    and protected-answer types.
  - `src/digest.rs` defines the explicit semantic digest projection.
  - `src/lib.rs` exposes the in-memory contract without filesystem access.
- `crates/academa-validation/`
  - `README.md` documents deterministic cross-file invariants.
  - `src/lib.rs` validates versions, identity, semantic content, ordered activity flow,
    protected mappings, assets, and digest agreement.
  - `tests/daily_reading_fixture.rs` loads fixtures and proves positive and negative
    behavior through the public APIs.
- `contracts/fixtures/daily-reading-v1/`
  - `README.md` documents public/protected, semantic/transport, and activity/policy
    boundaries.
  - `valid/{manifest.json,lesson.json,protected-answers.json}` is the executable fixture.
  - `invalid/` contains answer-leak, broken-reference, and unsupported-version inputs.

## Task 1: Establish pure content contracts

- [x] Add `academa-content` without modifying any `aet-*` crate.
- [x] Define stable typed IDs that reject whitespace, case drift, path separators, and
  machine-local provenance values.
- [x] Define separate `PublicLesson` and `ProtectedAnswers` types with
  `deny_unknown_fields`.
- [x] Represent the ordered Daily Reading flow: timed reading, committed objective
  question, vocabulary review, controlled production, mistake classification, and
  scheduled review.
- [x] Define asset declarations with stable ID, canonical package-relative path, media
  type, byte length, SHA-256 hash, and delivery classification.
- [x] Keep filesystem, ZIP, hosted, learner-state, and frontend behavior outside the
  crate.

## Task 2: Define semantic digest identity

- [x] Hash an explicit canonical projection rather than serializing `PublicLesson`
  wholesale.
- [x] Include learner-facing semantic content, protected answer policy, stable asset
  identity, media type, byte length, semantic asset hash, delivery classification, and
  reviewed provenance/rights decisions.
- [x] Exclude `package_path`, transport filenames, filesystem paths, storage keys,
  timestamps, ZIP metadata, publication audit data, and server-issued IDs.
- [x] Prove JSON formatting and field order do not change the digest.
- [x] Prove asset path-only renames do not change the digest while asset hash changes do.

## Task 3: Enforce deterministic cross-file validation

- [x] Add the real `academa-validation` crate with the only allowed dependency edge:
  `academa-validation -> academa-content`.
- [x] Validate schema/package versions, stable identities, nonblank semantic content,
  uniqueness, and cross-file references in a fixed order.
- [x] Require exactly one protected answer policy per scored question.
- [x] Require every protected mistake-tag and review-policy mapping to appear in the
  ordered classification and scheduled-review activities.
- [x] Allow additional valid activity references for manual or non-scored paths.
- [x] Reject unsafe asset paths and duplicate canonical package paths while leaving
  real archive-byte closure to the later package compiler.

## Task 4: Prove security and compatibility failures

- [x] Reject protected answer fields on the learner-safe surface.
- [x] Reject broken question references and unsupported versions.
- [x] Reject absolute/path-bearing provenance values.
- [x] Reject unsafe and duplicate asset package paths.
- [x] Reject answer-policy mistake/review mappings absent from the ordered activities.
- [x] Prove deterministic multi-question failure ordering and Serde round trips.
- [x] Prove protected feedback changes semantic identity.

## Task 5: Documentation and verification

- [x] Include technical crate READMEs as crate-level Rustdoc.
- [x] Enforce missing-documentation warnings on both public crates.
- [x] Run formatting, focused tests, strict Clippy, and Rustdoc missing-docs gates.
- [x] Run the complete workspace test suite.
- [x] Run the strict PowerShell 7 Phase 1 baseline verifier from a clean commit.
- [x] Confirm the exact changed-file set contains no private source material or unrelated
  frontend, hosted, cloud, or learner-state changes.

## Explicit exclusions

- No `academa-learning` or `academa-package` crate.
- No ZIP creation, archive checksums, AET source adapter, OCR, or AI calls.
- No submissions, outcomes, feedback releases, mistakes, reviews, or persistence.
- No Axum, database, object storage, TypeScript generation, SvelteKit, or provider SDK.
- No private PDFs, full articles, teacher notes, private corrections, or OneDrive paths.
