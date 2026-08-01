# Deterministic Lesson Package Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a real `academa-package` crate that deterministically compiles the
approved Daily Reading fixture into bounded ZIP bytes and revalidates the same package
without adding hosted or frontend behavior.

**Architecture:** `academa-package` depends one-way on `academa-content` and
`academa-validation`. Its local edge reads a reviewed directory; its receiving edge
accepts only bytes plus explicit limits. Both edges share checksum, archive-path,
asset-closure, and transport-digest logic.

**Tech Stack:** Rust 2024, Serde/serde_json, SHA-256, thiserror, `zip` 8.6 with default
features disabled and stored entries only.

---

### Task 1: Establish the package crate and ownership contract

**Files:**
- Modify: `Cargo.toml`
- Modify: `Cargo.lock`
- Create: `crates/academa-package/Cargo.toml`
- Create: `crates/academa-package/README.md`
- Create: `crates/academa-package/src/lib.rs`

- [x] Add `crates/academa-package` to the workspace with only these dependencies:

```toml
[dependencies]
academa-content = { path = "../academa-content" }
academa-validation = { path = "../academa-validation" }
serde.workspace = true
serde_json.workspace = true
sha2.workspace = true
thiserror.workspace = true
zip.workspace = true
```

Add `zip = { version = "8.6.0", default-features = false }` to workspace dependencies.

- [x] Create a crate root with `#![doc = include_str!("../README.md")]` and
  `#![warn(missing_docs)]`. The README states responsibilities, non-goals, dependency
  direction, determinism/security rules, public API, and verification commands.

- [x] Run `cargo check -p academa-package --locked --target-dir .codex-target`.
  Expected: PASS without a placeholder crate or unused dependency warning.

### Task 2: Define package metadata, limits, digest, and errors

**Files:**
- Create: `crates/academa-package/src/checksums.rs`
- Create: `crates/academa-package/src/digest.rs`
- Create: `crates/academa-package/src/error.rs`
- Create: `crates/academa-package/src/limits.rs`
- Modify: `crates/academa-package/src/lib.rs`
- Test: `crates/academa-package/tests/package_contract.rs`

- [x] Write failing tests for this exact public surface:

```rust
pub struct FileChecksum {
    pub path: String,
    pub byte_length: u64,
    pub sha256: String,
}

pub struct ChecksumInventory {
    pub schema_version: u32,
    pub files: Vec<FileChecksum>,
}

pub struct PackageLimits {
    pub max_archive_bytes: u64,
    pub max_files: usize,
    pub max_file_bytes: u64,
    pub max_total_uncompressed_bytes: u64,
}

pub fn compute_package_digest(package_bytes: &[u8]) -> String;
```

Cover the exact checksum JSON shape, lowercase prefixed digest, default limits, and
zero-limit rejection.

- [x] Run `cargo test -p academa-package --test package_contract --locked --target-dir
  .codex-target`. Expected: RED because the contract does not exist.

- [x] Implement `Serialize`, `Deserialize`, equality, and `deny_unknown_fields` for
  checksum types. Implement default limits as 64 MiB / 256 / 32 MiB / 64 MiB and return
  `PackageError::InvalidLimits { field }` when any field is zero. Format SHA-256 without
  another hexadecimal dependency.

- [x] Run focused tests and Rustdoc missing-docs. Expected: PASS.

### Task 3: Implement deterministic directory compilation

**Files:**
- Create: `crates/academa-package/src/canonical_json.rs`
- Create: `crates/academa-package/src/compiler.rs`
- Create: `crates/academa-package/src/path.rs`
- Modify: `crates/academa-package/src/lib.rs`
- Test: `crates/academa-package/tests/compile_directory.rs`

- [x] Write failing tests that copy `contracts/fixtures/daily-reading-v1/valid` to two
  fresh roots and require:

```rust
let first = compile_directory(&first_root)?;
let second = compile_directory(&second_root)?;
assert_eq!(first.bytes, second.bytes);
assert_eq!(first.content_digest, second.content_digest);
assert_eq!(first.package_digest, second.package_digest);
```

Rewrite input JSON with different whitespace/object order and require the same compiled
bytes. Add synthetic declared-asset cases for missing, extra, byte-length, SHA-256, and
reparse-point/symlink rejection without modifying the shared valid fixture.

- [x] Run the focused test. Expected: RED because `compile_directory` is absent.

- [x] Implement:

```rust
pub struct CompiledPackage {
    pub bytes: Vec<u8>,
    pub content_digest: String,
    pub package_digest: String,
}

pub fn compile_directory(root: &Path) -> Result<CompiledPackage, PackageError>;
```

Read exactly the three fixed JSON files, decode `LessonContract`, call
`academa_validation::validate`, verify declared assets, and reject undeclared entries
beneath `assets/`. Canonical JSON is pretty UTF-8 with LF endings and one final LF.
Build a path-sorted inventory excluding `checksums.json`.

- [x] Write deterministic ZIP bytes with `ZipWriter`, `CompressionMethod::Stored`,
  `SimpleFileOptions::DEFAULT`, fixed Unix system/permissions, no directories or
  comments, lexicographic payload order, and `checksums.json` last. Finish explicitly
  and propagate every error.

- [x] Run `cargo test -p academa-package --test compile_directory --locked --target-dir
  .codex-target`. Expected: PASS.

### Task 4: Implement fail-closed package revalidation

**Files:**
- Create: `crates/academa-package/src/archive.rs`
- Modify: `crates/academa-package/src/lib.rs`
- Test: `crates/academa-package/tests/validate_package.rs`

- [x] Write happy-path and mutation tests for:

```rust
pub struct ValidatedPackage {
    pub contract: LessonContract,
    pub package_digest: String,
}

pub fn validate_package(
    package_bytes: &[u8],
    limits: PackageLimits,
) -> Result<ValidatedPackage, PackageError>;
```

Compile then validate must recover the original typed contract. Mutating one payload
byte while retaining the old inventory must produce a checksum error.

- [x] Build adversarial in-memory archives and assert precise variants for traversal,
  absolute/drive/backslash/dot paths, duplicates, directories, missing fixed files,
  undeclared extras, deflated entries, invalid inventory version/order/duplicates/
  self-entry/closure, wrong lengths/digests, and each of the four limits.

- [x] Run the focused test. Expected: RED because byte revalidation is absent.

- [x] Implement bounded validation in fixed order: archive bytes, limit profile, ZIP
  parse, entry count/metadata/size, raw path and uniqueness, stored-only compression,
  bounded body reads, typed decode, `academa-validation`, inventory closure, asset
  closure, and exact transport digest. Never extract to disk.

- [x] Run `cargo test -p academa-package --locked --target-dir .codex-target`.
  Expected: all package unit, integration, and doctests PASS.

### Task 5: Document the implemented package boundary and run all gates

**Files:**
- Create: `docs/lesson-engine/PACKAGE-FORMAT.md`
- Create: `docs/lesson-engine/DIGESTS.md`
- Modify: `crates/academa-package/README.md`
- Modify: `docs/superpowers/plans/2026-08-01-deterministic-lesson-package.md`

- [x] Document only implemented behavior: file layout, canonical JSON, checksum
  self-exclusion, fixed ZIP metadata, candidate versus server-authoritative digest,
  limits, raw paths, stored-only compression, error order, and the rule that ZIP is
  transport/audit material rather than runtime storage.

- [x] Mark completed plan boxes. Scan design, plan, README, and package docs for
  placeholders, contradictions, stale manifest fields, locally issued server IDs, or
  claims of Axum/cloud behavior; fix every hit.

- [x] Run:

```powershell
cargo fmt --all --check
cargo clippy -p academa-package --all-targets --all-features --locked -- -D warnings
$env:RUSTDOCFLAGS='-D missing-docs'; cargo doc -p academa-package --no-deps --locked --target-dir .codex-target
cargo test --workspace --locked --target-dir .codex-target
```

Expected: PASS without warnings.

- [x] Commit the intentional implementation, then run the clean-tree-dependent baseline:

```powershell
pwsh -NoProfile -ExecutionPolicy Bypass -File scripts/verify-phase1-baseline.ps1
```

Expected: 36 deterministic artifacts and 3 normalized PDFs match.

- [x] Prove scope with `git diff --check origin/master...HEAD`, `git diff --name-only
  origin/master...HEAD`, and `cargo metadata --no-deps --format-version 1`. No private
  source, AET exporter, hosted, frontend, learner-state, or infrastructure file may be
  present.

## Review gate

Luna stops after one clean implementation commit and reports the exact commit, changed
files, test counts, command results, and uncertainty. The lead reviewer checks the full
diff, reruns focused adversarial tests plus workspace/baseline gates, and alone decides
whether to push and open a draft PR.
