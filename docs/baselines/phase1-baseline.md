# AET Phase 1 Baseline Verification Report

## Result

Stage 0 of issue #4 reproduces the protected AET Phase 1 outputs from a clean generated-output state.

- Pre-change commit: `2067ce540ac0921e4e5b9342fbb6b3c839caba85`
- Pre-change tree: `127b5d3e35019ff576013e830a79196527fc6a4e`
- Baseline manifest: `baseline/phase1/manifest.json`
- Verification entrypoint: `scripts/verify-phase1-baseline.ps1`
- Tag to create after merge: `aet-v0.1-vocabulary-pipeline`

The tag is intentionally not created by this branch. It must point to the merged baseline-freeze commit.

The snapshots were generated directly from the production behavior at the pre-change tree above. The later freeze commit adds the manifest, snapshots, verification script, report, and regression assertions; it does not change any production exporter or fixture input.

## Recorded toolchain

| Tool | Captured version |
| --- | --- |
| Rustup | `rustup 1.29.0 (28d1352db 2026-03-05)` |
| Rust compiler | `rustc 1.94.1 (e408947bf 2026-03-25)` |
| Cargo | `cargo 1.94.1 (29ea6fb6a 2026-03-24)` |
| Typst | `typst 0.14.2 (b33de9de)` |
| Git | `git version 2.52.0.windows.1` |
| PowerShell | `7.6.4` |
| OS | `Microsoft Windows NT 10.0.26200.0` |

The verifier requires the exact Rust, Cargo, and Typst versions because those tools produce the contractual artifacts. Git, PowerShell, and OS are recorded as capture provenance rather than byte-level requirements.

## Protected inputs and outputs

The four canonical fixture inputs remain in their existing tracked locations. Their byte lengths and SHA-256 digests are locked in the manifest:

- `data/articles/vietnam-two-child-policy/article.toml`
- `data/articles/vietnam-two-child-policy/vocab.csv`
- `data/articles/dengue-bangladesh/article.toml`
- `data/articles/dengue-bangladesh/vocab.csv`

The verifier builds:

1. `vietnam-two-child-policy` — IPA absent;
2. `dengue-bangladesh` — IPA present;
3. topic export `health-medicine`.

Thirty-six generated text artifacts are byte-for-byte deterministic. They include the three Anki TSV files, import guide, seven template/style files, and generated `vocab-pack.typ` for each of the three builds. Their canonical copies live under `tests/baseline/phase1/generated/`, with SHA-256 values in `baseline/phase1/snapshot-checksums.sha256`.

The Rust integration test copies the canonical `data/articles` tree into a unique temporary workspace. It invokes the compiled AET binary there, generates a fresh temporary `dist/`, and compares those files with the committed snapshots. The repository `dist/` tree and the snapshots are never used as the command output directory.

The three generated PDFs are also protected, but their raw hashes are not stable. Typst 0.14.2 changes exactly these build metadata fields:

- `Info.ModDate`;
- `Info.CreationDate`;
- `XMP.xmp:ModifyDate`;
- `XMP.xmp:CreateDate`;
- `XMP.xmpMM:InstanceID`;
- `Trailer.ID[1]`.

Two independent builds changed only those fields. After replacing each occurrence with a fixed same-length value, all three PDFs produced stable SHA-256 digests. The verifier requires every field to appear exactly once before normalization, then checks the normalized digest and original byte length. The stable document identifier, object ordering, fonts, text, pages, links, and every other PDF byte remain contractual.

## Fail-closed behavior

The verification script exits non-zero when any protected condition fails:

- a required tool is missing, returns an error, or has the wrong contractual version;
- the repository has tracked or untracked non-ignored changes before or after verification;
- a manifest path is absolute, escapes through `..`, or is declared more than once;
- a canonical input or snapshot is missing, has the wrong length, or has the wrong checksum;
- the snapshot tree and checksum authority disagree;
- a snapshot contains a Windows, UNC, macOS, Linux home, or temporary absolute path;
- any Rust check, test, validation, or build command fails;
- generated output contains a missing or extra file, including an undeclared PDF;
- a deterministic artifact differs from its snapshot;
- a PDF normalization field is missing, duplicated, or has changed shape;
- a normalized PDF digest or original byte length differs.

The manifest, checksum file, and snapshot tree jointly declare every protected input and output exactly once. The generated file set must equal the union of the 36 deterministic declarations and three PDF declarations.

## Verification evidence

The baseline verifier completed successfully on 2026-07-30:

- `cargo check --workspace --target-dir .codex-target` passed;
- `cargo test --workspace --target-dir .codex-target` passed with 35 tests;
- both fixtures validated with zero errors;
- both fixtures built all existing Anki and Typst outputs;
- `health-medicine` built from 99 rows across one contributing article;
- 36 deterministic generated files matched their snapshots;
- 3 normalized PDFs matched their expected digests.

The fixture warnings about empty `my_sentence` values are existing Phase 1 learner-work warnings and are not baseline failures.

Run the complete gate from the repository root:

```powershell
.\scripts\verify-phase1-baseline.ps1
```

The script removes only the ignored repository-local `dist/` directory before the final build, then reproduces and verifies every protected artifact.

## Scope boundary

This freeze adds regression evidence only. It does not change current AET production behavior and does not introduce:

- generalized Course, Lesson, activity, attempt, mistake, or review models;
- issue #6 or #8 contract types;
- crate renaming or repository restructuring;
- Axum, Neon, R2, authentication, Cloudflare, or SvelteKit;
- private OneDrive course material;
- changes to the Anki or Typst export formats.

After this baseline PR merges and the tag is created, the joint #6/#8 contract fixture may begin without losing the known Phase 1 behavior.
