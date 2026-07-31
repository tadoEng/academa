# Daily Reading Contract Fixture Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add one sanitized, executable Daily Reading package-facing contract that proves stable logical identities, public/protected separation, semantic digest stability, and explicit rejection of unsafe or incompatible inputs without changing the protected AET pipeline.

**Architecture:** Add one focused `academa-content` library beside the protected `aet-*` crates. The library owns only reviewed package-facing Rust models, JSON loading, semantic SHA-256 calculation, and cross-reference validation; learner state, ZIP assembly, HTTP, generated TypeScript, and infrastructure remain outside this slice.

**Tech Stack:** Rust 1.94, Serde/serde_json, thiserror, SHA-256, Cargo integration tests, JSON fixtures.

---

## File structure

- Modify `Cargo.toml` to register `crates/academa-content` and the SHA-256 workspace dependency.
- Create `crates/academa-content/Cargo.toml` for the package-facing contract library.
- Create `crates/academa-content/README.md` for ownership, determinism, security, and non-goals.
- Create `crates/academa-content/src/lib.rs` for documented public exports and fixture loading.
- Create `crates/academa-content/src/ids.rs` for validated, typed logical IDs.
- Create `crates/academa-content/src/model.rs` for manifest, public lesson, and protected-answer DTOs.
- Create `crates/academa-content/src/digest.rs` for canonical semantic serialization and SHA-256.
- Create `crates/academa-content/src/validation.rs` for schema-version, identity, uniqueness, reference, and digest validation.
- Create `crates/academa-content/tests/daily_reading_fixture.rs` for executable positive and negative proofs.
- Create `contracts/fixtures/daily-reading-v1/README.md` for the sanitized fixture contract.
- Create `contracts/fixtures/daily-reading-v1/valid/{manifest.json,lesson.json,protected-answers.json}`.
- Create `contracts/fixtures/daily-reading-v1/invalid/answer-leak/lesson.json`.
- Create `contracts/fixtures/daily-reading-v1/invalid/broken-reference/protected-answers.json`.
- Create `contracts/fixtures/daily-reading-v1/invalid/unsupported-version/manifest.json`.

## Task 1: Establish the additive crate and RED fixture test

- [x] Add `academa-content` to the workspace without modifying any `aet-*` crate.
- [x] Add a test that loads the three valid fixture files through `LessonContract::load` and expects successful validation.
- [x] Run `cargo test -p academa-content --test daily_reading_fixture` and confirm it fails because the crate contract does not yet exist.
- [x] Keep the observed RED checkpoint uncommitted so branch history remains buildable.

## Task 2: Define the minimum package-facing model

- [x] Implement typed stable IDs for course, module, lesson, objective, vocabulary entry, activity, question, option, answer policy, mistake tag, review policy, prompt, paragraph, and asset references.
- [x] Reject empty IDs, uppercase characters, path separators, whitespace, and characters outside lowercase ASCII letters, digits, and hyphens.
- [x] Define a versioned manifest containing only `schema_version`, `package_version`, stable course/module/lesson IDs, and `content_digest`.
- [x] Define the learner-safe lesson model for one reading passage, selected vocabulary, one objective question, one production prompt, mistake-tag/review-policy references, declared assets, and sanitized provenance.
- [x] Define protected answer policies containing accepted responses, scoring, explanation, evidence references, mistake/review mappings, and release policy.
- [x] Apply `#[serde(deny_unknown_fields)]` and snake-case wire naming so answer-bearing fields cannot be silently accepted in public lesson objects.
- [x] Run the focused fixture test and confirm parsing succeeds before semantic validation is added.

## Task 3: Implement semantic validation and digest identity

- [x] Validate supported schema/package version `1`, matching course/module/lesson IDs, non-empty required collections, and unique logical IDs.
- [x] Validate every protected question, option, mistake tag, review policy, evidence paragraph, and learner-safe reference against the public lesson.
- [x] Require exactly one protected policy for every scored question and forbid protected policies for unknown questions.
- [x] Serialize only the reviewed public lesson plus protected answers into a fixed semantic envelope and calculate lowercase `sha256:<64-hex>`.
- [x] Verify the manifest digest against the calculated semantic digest while excluding paths, timestamps, build metadata, ZIP metadata, and server-issued IDs by construction.
- [x] Run the focused test, capture the computed digest from the intentional mismatch, update the reviewed fixture manifest, and rerun to GREEN.

## Task 4: Prove security and compatibility failures

- [x] Add an answer-leaking public lesson fixture containing a protected answer field and assert Serde rejects the unknown field.
- [x] Add a protected-answer fixture referencing a nonexistent question and assert semantic validation reports the exact broken reference.
- [x] Add a manifest with schema version `2` and assert validation reports `UnsupportedSchemaVersion { found: 2 }`.
- [x] Serialize the valid public lesson and recursively assert that answer-policy, accepted-answer, scoring, explanation, evidence, and release-policy keys are absent.
- [x] Load identical reviewed content through two different filesystem paths and assert the semantic digest is unchanged.
- [x] Assert two separately loaded valid contracts serialize and digest identically.

## Task 5: Document and verify the slice

- [x] Document that OCR, extraction, and AI-assisted drafting end before this crate receives reviewed canonical input.
- [x] Document that the fixture is original/sanitized, contains no private OneDrive paths or teacher content, and bridges #14 to later #15 tests without defining learner-state records.
- [x] Run `cargo fmt --check --all`.
- [x] Run `cargo test -p academa-content`.
- [x] Run `cargo test --workspace --target-dir .codex-target`.
- [x] Run `powershell -ExecutionPolicy Bypass -File scripts/verify-phase1-baseline.ps1` with disposable index state ignored so the strict clean-worktree gate can execute.
- [x] Review `git diff --check`, the complete changed-file set, dependency graph, and privacy-sensitive strings before preparing the commit.

## Explicit exclusions

- No `academa-learning` or `academa-package` crate.
- No ZIP creation, archive checksums, source adapters, OCR, or AI calls.
- No learner submissions, outcomes, feedback releases, mistakes, reviews, or persistence.
- No Axum, database, object storage, TypeScript generation, SvelteKit, or cloud-provider code.
- No private PDF, full article text, teacher notes, private corrections, or OneDrive paths.

## Independent review corrections — 2026-08-01

- [x] Remove filesystem loading from `academa-content`; fixture tests own temporary file I/O.
- [x] Move real cross-file validation into the additive `academa-validation` crate.
- [x] Replace unrestricted provenance text with a validated path-independent reference.
- [x] Make multi-question error ordering deterministic.
- [x] Represent the full ordered Daily Reading activity flow, including timer guidance.
- [x] Add a package-relative asset path and archive-traversal rejection.
- [x] Require nonblank semantic content and nonempty declared assets.
- [x] Add Serde round-trip, JSON-order, provenance-path, asset-path, and deterministic-error proofs.
- [x] Apply the #13 crate README, crate-level Rustdoc, and missing-documentation lint contract.
