# academa-content

`academa-content` owns the reviewed, package-facing lesson contract for the local
Lesson Engine. It loads learner-safe lesson JSON and separately protected answer
policy JSON, validates their stable logical references, and calculates semantic
content identity.

The crate starts after human approval. OCR, extraction heuristics, and AI-assisted
drafting are outside its boundary. Given the same reviewed JSON and declared assets,
its validation and digest output must be deterministic and must make no network call.

It does not own ZIP compilation, publication IDs, lesson-version IDs, learner
submissions, scoring outcomes, review records, Axum routes, frontend types, database
persistence, or cloud-provider integration.
