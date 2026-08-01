# Academa Lesson Package Format

## Scope

Package version 1 is the deterministic transport boundary for one reviewed Daily
Reading lesson. `academa-package` compiles a local reviewed directory and revalidates
the resulting bytes without extracting them. The package is transport and audit
material, not runtime learner-state storage.

## File layout

```text
lesson-package.zip
├── manifest.json
├── lesson.json
├── protected-answers.json
├── assets/...                 # only paths declared by lesson.json
└── checksums.json
```

The first three JSON files and all assets are written in lexicographic path order.
`checksums.json` is always last. No directory entries are emitted.

## Canonical JSON

All four JSON surfaces are serialized from typed Rust values as UTF-8, with stable
struct field order, two-space pretty indentation, LF newlines, and exactly one final
LF. Revalidation decodes with unknown-field rejection where the contract types require
it, validates the typed lesson, verifies checksums, and requires the stored JSON bytes
to equal the canonical serialization.

## Checksum inventory

`checksums.json` has schema version 1 and a `files` array sorted strictly by path. Each
entry contains `path`, exact uncompressed `byte_length`, and a lowercase
`sha256:`-prefixed digest. It covers every archive payload except `checksums.json`.
Self-exclusion avoids recursive hashing while archive closure still requires exactly
one inventory file.

## ZIP metadata

The compiler uses only stored compression. Every file has DOS timestamp
`1980-01-01 00:00:00`, explicit Unix metadata, permissions `0o644`, and no file or
archive comment. Identical reviewed typed content and asset bytes therefore produce
identical ZIP bytes.

## Paths and assets

Raw entry names must be UTF-8 relative paths using `/`. Empty paths, leading or
trailing `/`, empty segments, `.`, `..`, backslashes, colons, control characters,
drive prefixes, duplicates, directories, symbolic links, encryption, and non-stored
compression are rejected. A sanitized path is never used to authorize a raw name.

Compilation validates every filesystem path prefix with link-aware metadata before
reading a file. Symlinks and Windows reparse points are rejected rather than followed.
Every declared asset must exist with its exact length and SHA-256 digest, and every
file below `assets/` must be declared. Revalidation repeats inventory and declared-
asset closure checks in memory.

## Bounded revalidation

The default Phase 0 direct-upload profile permits at most 64 MiB of archive bytes, 256
files, 32 MiB for one uncompressed file, and 64 MiB total uncompressed bytes. A custom
profile may be stricter, but every field must be nonzero. Metadata limits are enforced
before entry bodies are read, and bodies are never written to disk.

The deterministic error order is: limit profile, archive bytes, bounded central-
directory preflight for ZIP structure, declared file count, and raw-name duplicates;
ZIP parsing; raw entry metadata and aggregate sizes; bounded body reads; required
files; typed contract validation; inventory closure and payload checksums; declared-
asset closure; then canonical JSON bytes. ZIP64-marked candidates that the bounded
preflight cannot resolve fail closed before ZIP parsing.

## Exclusions

The format does not define HTTP routes, server IDs, authentication, learner state,
database or object-storage layout, frontend DTOs, extraction workflows, streaming
compression, OCR, AI, or network behavior.
