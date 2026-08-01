# academa-package

`academa-package` is the deterministic package boundary for reviewed Academa lessons.
It compiles the three typed Daily Reading JSON surfaces plus declared assets into
stable ZIP bytes, and validates those bytes again before a hosted adapter may accept
them.

## Responsibilities

- Decode and validate reviewed `academa-content` lesson contracts.
- Canonicalize typed JSON and inventory every payload checksum.
- Enforce declared-asset closure and exact asset bytes.
- Write deterministic, stored-only ZIP archives.
- Revalidate untrusted ZIP bytes under explicit memory and file-count limits.
- Compute the exact transport digest without creating local publication identity.

## Non-goals

This crate contains no CLI adapter, HTTP framework, learner state, persistence, cloud
provider, frontend contract, OCR, AI, network access, or ZIP extraction. Dependency
direction is strictly `academa-content -> academa-validation -> academa-package`.

## Determinism and security

Version 1 packages contain canonical UTF-8 JSON with LF endings and one final newline.
The checksum inventory covers every payload except itself. ZIP entries use stored
compression, fixed metadata, canonical relative paths, deterministic ordering, and no
directories or comments. Validation rejects unsafe or duplicate raw paths, unsupported
compression, undeclared entries, checksum mismatches, and limit violations.

## Public API

Use `compile_directory` at the reviewed local filesystem edge and `validate_package`
at the byte-only receiving edge. `compute_package_digest` returns the lowercase,
`sha256:`-prefixed digest of exact ZIP bytes. `PackageLimits::default()` is the Phase 0
direct-upload profile; callers may supply a stricter nonzero profile.

```rust
use std::path::Path;

use academa_package::{PackageLimits, compile_directory, validate_package};

# fn example() -> Result<(), academa_package::PackageError> {
let compiled = compile_directory(Path::new("reviewed-lesson"))?;
let validated = validate_package(&compiled.bytes, PackageLimits::default())?;
assert_eq!(validated.package_digest, compiled.package_digest);
# Ok(())
# }
```

The validator applies errors in a fixed security order: nonzero limit profile,
archive-byte limit, duplicate central-directory names and ZIP parsing, file-count and
entry metadata limits, raw paths and entry kinds, bounded body reads, required files,
typed contract validation, checksum inventory, declared-asset closure, and canonical
JSON encoding. The ZIP remains transport and audit material; later runtime storage may
use decoded content but must not treat the archive as mutable learner state.

See `docs/lesson-engine/PACKAGE-FORMAT.md` and
`docs/lesson-engine/DIGESTS.md` for the implemented version-1 wire rules.

## Verification

```powershell
cargo test -p academa-package --locked --target-dir .codex-target
cargo clippy -p academa-package --all-targets --all-features --locked -- -D warnings
$env:RUSTDOCFLAGS='-D missing-docs'; cargo doc -p academa-package --no-deps --locked --target-dir .codex-target
```
