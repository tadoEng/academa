# AET Phase 1 Baseline Freeze Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Freeze the exact pre-migration AET Phase 1 behavior at commit `2067ce540ac0921e4e5b9342fbb6b3c839caba85` so a clean checkout can reproduce and verify both article fixtures and the current `health-medicine` topic export.

**Architecture:** Keep the existing Rust CLI, data models, Anki exporter, and Typst exporter unchanged. Add only a versioned baseline contract: canonical input digests, byte-for-byte snapshots for deterministic generated files, semantic PDF assertions for Typst's timestamp-bearing PDFs, and one PowerShell verification entrypoint.

**Tech Stack:** Rust 1.94.1, Cargo 1.94.1, Typst 0.14.2, PowerShell 7, SHA-256.

---

## Task 1: Record the immutable baseline identity

- [x] Record the pre-change commit and tree IDs.
- [x] Record the pinned Rust toolchain plus observed Cargo, Typst, Git, PowerShell, and OS versions.
- [x] Record the post-merge tag name without creating the tag in this branch.

## Task 2: Capture the Phase 1 contract

- [x] Hash the four canonical fixture input files.
- [x] Build both article fixtures and `health-medicine`.
- [x] Snapshot every deterministic text/template output.
- [x] Document that Typst writes `CreationDate` and `ModDate`, making raw PDF hashes nondeterministic.
- [x] Protect each PDF through its deterministic Typst source and normalized digest while retaining every non-build-metadata byte.

## Task 3: Add regression verification

- [x] Extend the existing round-trip test to compare deterministic outputs with committed snapshots.
- [x] Add `scripts/verify-phase1-baseline.ps1` to run the full check/test/build sequence and validate input, output, and normalized PDF contracts.
- [x] Keep `dist/` generated and ignored.

## Task 4: Prove clean reproduction

- [x] Run the verification entrypoint from the implementation worktree.
- [x] Re-run it from a clean detached worktree at the final commit.
- [x] Confirm no private OneDrive material, Phase 2 types, hosted infrastructure, or current export behavior changed.
- [x] Prepare tag `aet-v0.1-vocabulary-pipeline` for creation only after merge.
