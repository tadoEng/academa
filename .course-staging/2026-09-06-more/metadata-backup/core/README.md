# Core — the learning system

Created 2026-08-10, architecture frozen the same day. Updated 2026-09-06.
Goal: **IELTS 7.5**.

**Core is self-contained, and organised by topic.** Every section has a
`materials\` folder split into topic subfolders; each subfolder holds *every
distinct* teacher document on that topic — lesson, teaching note and exercises
together — deduplicated by hash so you never get the same file twice.

**382 files, 1.4 GB.**

| Section | Topics | Files |
|---|---:|---:|
| `01. sentence-structure` | 7 levels | 79 |
| `02. error-catalogue` | 10 error groups | 61 |
| `03. writing` | 8 topics | 33 |
| `04. vocabulary` | 3 layers | 8 |
| `07. listening` | 21 packets | 117 |

The redundancy is deliberate — but only where it is **real**. A structure cannot
be cross-checked from one source, so where the teacher explained clauses three
times across three lectures, all three are here. What is *not* here is the same
lecture five times.

Deduplication is by size cluster plus name, not by hash: five scans of one lecture
differ by a few kilobytes, so a hash check keeps all five. Within a topic folder,
files within 2% of each other sharing a normalised name are one document — one
clean copy and one latest annotated copy are kept. That removed 303 MB on
2026-08-10 without losing a single distinct document.

Canonical originals stay in `Lectures`, `Sentence`, `Writing`, `Vocabulary` and
`Daily Reading`. Nothing was moved or deleted; these are verified copies.

## Assessment tools

- **`03. writing\writing-assessment.html`** — the four band criteria turned into
  diagnostic questions, with the band 6→7 line for each and every failure mapped
  to an error ID. Includes the per-essay scoring sheet.
- **`08. reading\reading-strategy.html`** — method per question type
  (TFNG, headings, matching, completion, MCQ), the time budget, and a decision
  tree that separates the five reasons an answer can be wrong.

## Open these two first

**`learning-order.html`** — what to study, in what order. The two tracks
(sentence structure is sequential; errors and vocabulary are driven by your error
log), each level's stage-4 test, the weekly rhythm, and what to do right now.

**`protocol.html`** — how we work together on it. Roles, the correction loop,
Modes A–E.

The folders are the library. **These two pages are the distillation.**

## How we actually work

**`protocol.html`** — open it in a browser. Academic English Learning Protocol
**v1.1**: fixed roles for you, the test files, ChatGPT and Claude; the universal
loop; Modes A-E for reading, sentence structure, writing, listening and
vocabulary; the disagreement and source-dispute paths; the August-to-December
phases. Frozen until practice exposes a problem.

Five rules from it govern everything else here:

1. **Reasoning before answer.** Seeing the key first rewrites your memory of why
   you chose what you chose, and the diagnosis becomes worthless.
2. **Confidence at answer time.** Right-but-unsure is fragile knowledge the score
   hides completely; wrong-but-certain is a broken rule you are applying with
   conviction. Neither is visible without it.
3. **Observed error ≠ root cause.** The ID names the symptom, the cause names
   what to teach. Causes aggregate across skills; IDs do not.
4. **The original question is not the retry** — and one retry is not enough.
   Same session, then 2-3 days, then 1-2 weeks. Status ends at `STABLE`, never
   `RESOLVED`.
5. **Recording takes 2-5 minutes.** If study time turns into documentation time,
   the system is failing. Claude maintains; you learn.

## The loop this exists to close

```
TEACHER MATERIAL → learn → Core notes → practice → mistake
   → classify (Sxx/Wxx/Rxx/Lxx/SPxx/Vxx) → understand why
   → targeted revision → retry → mastery
```

Your library has always had the top half. This is the bottom half.

## Layout

| Folder | Holds |
|---|---|
| `01. sentence-structure` | The roadmap, Levels 1-7, with mastery stages |
| `02. error-catalogue` | Frozen error taxonomy + the five-pass self-check |
| `03. writing` | Paragraph, essay, and the four Task 2 prompts |
| `04. vocabulary` | Four layers: collocations, academic, word families, and the full topic reservoir |
| `05. my-writing` | Your own drafts — empty by design |
| `06. progress` | `error-log.md`, the single canonical record |
| `07. listening` | Interim practice: 23 Daily Reading recordings |
| `08. reading` | Strategy + the three benchmark tests with your attempts and teacher corrections |

Read in that order.

## Naming convention — frozen 2026-08-10

Four grammars. **A prefix encodes exactly one semantic dimension**, and the shape
of the prefix tells you which dimension.

| Grammar | Means | Example |
|---|---|---|
| `NN. name` | Ordered curriculum — start at the low number | `01. sentence-structure`, `03. writing\materials\02. question-types` |
| `SLx-name` | Sentence mastery **level** — x is the rung, not the order | `SL4-clauses` |
| `Sxx` `Wxx` `Rxx` `Lxx` `SPxx` `Vxx` | Stable **error taxonomy** — never renumber | `S04-agreement`, `S10-S11-punctuation` |
| `A-` `B-` `C-` | Vocabulary **priority layer**, A first | `A-collocations` |

`00` is reserved for overview, index, or mixed material — never a specific item.
Hence `00. Core` at the top and `S00-mixed-common-errors` for the cross-category
drill.

**`S00` is a folder marker, not an error ID.** It never appears in the error log;
there is no such error as "S00". A mixed drill that reveals a real mistake gets
logged under whatever that mistake actually is.

Two things are deliberately *not* normalised:

- **Listening packet folders** keep their source names exactly
  (`43.Too Many Master's Courses`), including the library's own inconsistent
  spacing. Provenance beats tidiness — you can always trace a folder back to its
  Daily Reading packet.
- **Learner and markup files** keep their names: `* tu.pdf` is your worked copy,
  `*.markup.pdf` is annotated. Never renamed, never overwritten.

No further prefixes. The folder hierarchy already says which section you are in,
so `WR-`, `VOC-`, `DR-` style tags would be noise.

## Core v1 — frozen 2026-08-10

| Type | Files |
|---|---:|
| PDF | 319 |
| MP3 | 23 |
| Markdown | 35 |
| HTML | 4 |
| CSV | 1 |
| **Total** | **382** |

342 of those are library assets, every one tracked in `core-manifest.csv` with its
origin and SHA-256. The other 40 are Core-native: these notes, the four pages, and
the manifest. 112 internal references, 0 broken.

**No more restructuring.** From here the system runs; it does not get rebuilt.

## Provenance

`core-manifest.csv` — every asset with its original name and path, byte size,
SHA-256, relation (`EXACT`, `EXACT_RENAMED`, `ANNOTATED`, `LEARNER_COPY`) and
whether it belongs to the 94-document curriculum spine. Machine-readable, so
coverage claims are reproducible rather than eyeballed.

Two counts, and they mean different things:

- **94** — the curriculum spine you selected by hand: sentence → error →
  paragraph/essay → IELTS. That is what `study-order.md` walks you through.
- **367** — everything in Core: the spine, plus alternative explanations,
  exercises, teacher notes, the vocabulary reservoir, listening packets and test
  evidence. **You are not expected to read 367 files.**

## Admission rule — content curation frozen 2026-08-10

Whole-library sweeps are finished. A new document enters Core only if:

1. it is part of **current teaching**, or
2. a **real logged error** exposed a missing resource, or
3. it is **unique teacher-authored or annotated** content.

Otherwise it stays in the library. This makes Core maintenance incremental
curriculum upkeep rather than collection-building.

## Two rules that keep this from rotting

**One fact, one home.** Error detail lives only in `06. progress\error-log.md`.
Raw scores and time live only in `IELTS Test\06. scores\score-log.md`. Practice
queues and material indexes live only in the `IELTS Test` trackers. Nothing is
recorded twice.

**Skeleton first, depth on demand.** The catalogue lists every error in 2-5 lines
so you can *name* a mistake from day one. A page only grows when you actually
make that error repeatedly. Documentation follows learning; it does not replace it.

## The 20%

**Foundation (5)** — subject/verb/complement and complement vs modifier · the ten
basic patterns · phrase vs clause · adverb/relative/noun clauses · coordination
and subordination.

**Named errors (7)** — `S01`-`S03` boundaries · `S04` agreement · `S05` modifier
attachment · `S06` parallelism · `S07` pronoun reference · `S08` tense ·
`S10`/`S11` punctuation.

**Writing (5)** — paragraph shape · **`W07`: identify the question type before
writing a word** · opinion and discussion skeletons · academic word choice · a
fixed proofreading routine.

**Vocabulary (3)** — collocations before single words · core academic vocabulary ·
active recall in the production direction.

## IELTS coverage status

Verified against the library on 2026-08-10. Marked here so gaps cannot silently
disappear from the plan:

| Component | Status | Materials |
|---|---|---:|
| Reading | **ACTIVE** | 81 |
| Writing Task 2 | **ACTIVE** | 15 |
| Grammar / sentence structure | **ACTIVE** | substantial |
| Vocabulary | **ACTIVE** | 30+ topic sets |
| Listening | **INTERIM** | 23 recordings in `07. listening` |
| Writing Task 1 | **ACTIVE** | 2 teacher-supplied books |
| Speaking | **UNSOURCED** | 2 stubs |

- **Listening** — teacher material expected shortly. Until then the Daily Reading
  audio in `07. listening\materials` is the practice source. It builds the ear,
  not exam technique: no questions, no four-section structure.
- **Task 1** — the 2026-09-06 Lecture 23 folder supplied the Collins book and
  the teacher's Task & Report course. Both are in
  `03. writing\materials\13. task1-report`, the canonical Writing folder, and
  the IELTS review tree.
- **Speaking** — still nothing, and still the largest untouched component. Worth
  one question to the teacher.

## Caveat

This 20% comes from the syllabus, not from your errors — the log is empty. Two
scores exist: Reading Test 2 **33/40**, Test 3 **30/40**. Once mistakes are
logged by ID, re-rank against what is actually failing.
