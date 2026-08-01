# Academa Lesson Digests

## Semantic content digest

`LessonManifest.content_digest` is the reviewed semantic identity calculated by
`academa-content`. It covers learner-safe content, protected answer policy, reviewed
provenance and rights decisions, and each asset's identity, media type, length, hash,
and delivery classification. It deliberately excludes `package_path`, filesystem
paths, ZIP metadata, build metadata, storage keys, and publication identity.

Renaming an asset package path while preserving its reviewed bytes and declaration
semantics therefore preserves `content_digest`.

## Package transport digest

`compute_package_digest` hashes the exact ZIP byte stream and returns lowercase
`sha256:<64 hex digits>`. `CompiledPackage.package_digest` and
`ValidatedPackage.package_digest` use this calculation. Any entry name, ordering,
payload, checksum inventory, or ZIP metadata change produces a different candidate
transport digest.

The transport digest is not embedded inside the package because doing so would create
a recursive hash. It is also not a locally authoritative `publication_id` or
`lesson_version_id`. A later server boundary may record the candidate digest and issue
authoritative publication identity after revalidation, but that behavior is outside
this crate.

## Asset and inventory digests

Each asset declaration contains the digest of the exact asset bytes. Compilation
checks it before packaging. The checksum inventory independently covers the exact
canonical JSON and asset payload bytes stored in the ZIP. Revalidation first closes
and verifies the inventory, then repeats asset declaration length and digest checks.
This separates transport integrity from semantic asset approval without weakening
either boundary.
