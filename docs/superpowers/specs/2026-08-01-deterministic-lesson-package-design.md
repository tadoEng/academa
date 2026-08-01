# Deterministic Lesson Package Design

**Status:** Approved for implementation under issues #6, #7, #13, and #14.

## Goal

Create the first real `academa-package` crate. It compiles one reviewed canonical
Daily Reading directory into deterministic `lesson-package.zip` bytes and applies the
same fail-closed validation when those bytes later enter the hosted publication
boundary.

The crate ends at the package boundary. It contains no Axum, learner state,
persistence, cloud provider, frontend, OCR, AI, or network behavior.

## Considered approaches

### 1. Typed directory compiler plus in-memory archive validator — selected

The local side reads the three reviewed JSON files and declared assets, decodes them
into `academa-content`, calls `academa-validation`, canonicalizes JSON, builds the ZIP,
and returns bytes. The receiving side validates ZIP bytes using the same crate and
returns the decoded contract plus the exact transport digest.

This keeps filesystem ownership on the local compiler edge while exposing a byte-only
revalidation API suitable for a later Axum adapter.

### 2. Streaming archive abstraction

A generic async/streaming reader would reduce peak memory for large packages, but it
would add lifetime, cancellation, and partial-read complexity before the first bounded
Daily Reading transaction needs it.

### 3. CLI-first compiler

Putting packaging directly in `aet-cli` would be quick, but it would couple the hosted
revalidator to CLI filesystem conventions and violate the shared pure-Rust boundary.

## Crate boundary

```text
academa-content
        ↓
academa-validation
        ↓
academa-package
```

`academa-package` owns package file names and version-1 archive rules, deterministic
canonical JSON bytes, checksum inventory serialization, declared-asset closure and byte
verification, deterministic ZIP writing, bounded ZIP parsing and security validation,
and exact transport digest calculation.

It does not own server-issued `publication_id` or `lesson_version_id`. The local
manifest remains the approved six-field `LessonManifest`; `package_digest` is computed
from exact ZIP bytes but is not embedded in the package or promoted to an authoritative
publication identity locally.

## Public API

```rust
pub struct CompiledPackage {
    pub bytes: Vec<u8>,
    pub content_digest: String,
    pub package_digest: String,
}

pub struct ValidatedPackage {
    pub contract: LessonContract,
    pub package_digest: String,
}

pub struct PackageLimits {
    pub max_archive_bytes: u64,
    pub max_files: usize,
    pub max_file_bytes: u64,
    pub max_total_uncompressed_bytes: u64,
}

pub fn compile_directory(root: &Path) -> Result<CompiledPackage, PackageError>;
pub fn validate_package(
    package_bytes: &[u8],
    limits: PackageLimits,
) -> Result<ValidatedPackage, PackageError>;
pub fn compute_package_digest(package_bytes: &[u8]) -> String;
```

`PackageLimits::default()` is the Phase 0 direct-upload profile: 64 MiB archive bytes,
256 files, 32 MiB for one file, and 64 MiB total uncompressed bytes. Callers may choose
stricter limits. A caller-supplied profile with any zero field is rejected.

## Version-1 archive format

```text
lesson-package.zip
├── manifest.json
├── lesson.json
├── protected-answers.json
├── assets/...                 # only declared files; lexicographic path order
└── checksums.json
```

No directory entries are emitted. JSON is serialized from typed values with stable
struct field order, pretty indentation, UTF-8, LF newlines, and exactly one final
newline.

`checksums.json` has this versioned shape:

```json
{
  "schema_version": 1,
  "files": [
    {
      "path": "lesson.json",
      "byte_length": 123,
      "sha256": "sha256:..."
    }
  ]
}
```

The inventory is sorted by path and covers every payload entry except
`checksums.json`. Excluding the inventory from itself prevents a recursive checksum.
Archive closure still requires exactly one `checksums.json` entry.

The compiler uses ZIP `Stored` compression, fixed DOS timestamp
`1980-01-01 00:00:00`, fixed Unix file permissions `0o644`, no directory entries,
no comments, and fixed entry order. This makes exact package bytes reproducible while
keeping `content_digest` independent of transport metadata.

## Compile flow

```text
read the three fixed JSON files
→ reject oversized inputs before allocation growth
→ decode with deny_unknown_fields
→ validate the typed LessonContract
→ read each declared asset by its validated package path
→ verify asset length and SHA-256
→ reject missing and undeclared files below assets/
→ canonicalize typed JSON
→ create path-sorted checksum inventory
→ write deterministic ZIP
→ compute exact package digest
```

The compiler reads only the reviewed root and declared assets. Symlinks/reparse points
are rejected rather than followed. It performs no network, OCR, or AI operation.

## Revalidation flow

```text
enforce archive-byte limit
→ parse ZIP
→ enforce file-count and size limits before reading bodies
→ reject unsafe, duplicate, directory, encrypted, or unsupported-compression entries
→ require the four fixed files and allow only declared assets
→ decode and validate LessonContract
→ verify checksum inventory and asset declarations
→ compute exact package digest
→ return typed contract
```

Raw ZIP entry names must be UTF-8, use `/`, be relative, contain no empty, `.`, `..`,
drive-prefix, or colon segment, and match their canonical spelling. Validation never
uses a sanitized path as permission to accept an unsafe raw path.

Only `Stored` entries are accepted in package version 1. This makes the decompression
boundary explicit and removes compression-bomb ambiguity from the first transaction.

## Error contract

`PackageError` is a documented `thiserror` enum with stable semantic variants for I/O
and JSON context, invalid contracts, unsafe or duplicate paths, missing/extra files,
unsupported entries, checksum inventory failures, asset closure mismatches, all four
limits, invalid limit profiles, and ZIP format failures. Production code returns
`Result`; it contains no `unwrap`, `expect`, or panic path.

## Verification

The package PR must prove:

- two clean compilations of the same reviewed directory are byte-identical;
- formatting and JSON object-order differences do not alter compiled bytes;
- compile then revalidate returns the original typed contract;
- exact ZIP mutation changes `package_digest` and fails checksum validation;
- asset rename changes transport bytes while preserving semantic digest;
- missing, extra, duplicate, wrong-length, and wrong-hash assets fail closed;
- traversal, absolute, backslash, duplicate, directory, and deflated entries fail;
- all four limit classes fail at their exact boundary;
- protected answers remain physically separate from `lesson.json`;
- strict fmt, Clippy, Rustdoc, workspace tests, and the Phase 1 baseline pass.

## Explicit exclusions

- No AET source adapter in this PR.
- No private OneDrive material or generated private lesson package.
- No CLI command or source-extraction workflow.
- No learner records, Axum routes, database, R2, authentication, or authorization.
- No frontend DTO or TypeScript generation.
- No ZIP extraction to disk.
- No generalized compression or streaming support.
