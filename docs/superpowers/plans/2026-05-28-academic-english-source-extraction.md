# Academic English Source Extraction Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Turn the expanded `D:\OneDrive\01. IELTS\00.Academic English` source folder into a clean, non-destructive staging library that can later populate AET article, topic, lecture, and writing data without committing private source PDFs.

**Architecture:** Keep OneDrive as the private raw/source store and `D:\Work\aet\data` as the deterministic AET fixture/export store. Normalize filenames and folder roles first, then extract only approved learning data into `article.toml` and `vocab.csv` when a source pack is ready.

**Tech Stack:** Windows PowerShell, AET Rust CLI, CSV/TOML, Typst CLI, private OneDrive storage.

---

## Current Source Review

Inventory checked on 2026-05-28:

- `Daily Reading`: 45 files, 98.20 MB, with article folders `00` through `09`.
- `Lectures`: 32 files, 224.52 MB, mostly lecture PDFs plus repeated reading/vocabulary packs.
- `Vocabulary`: 5 files, 32.29 MB, current topic-bank material.
- `Writing`: 6 files, 4.90 MB, writing lessons and review checklists.
- `Reading`: 1 markdown guide.
- Root `catch-up-plan-2026-05-27.md`: active two-day completion checklist for Daily Reading 06-09 plus writing review.

Main findings:

- `Daily Reading\Original PDF\00` and `01` are already the cleanest AET examples: `article.pdf` plus `teacher-vocab.pdf`.
- `Daily Reading\Original PDF\02` also has the expected shape and includes `audio.mp3`.
- `Daily Reading\Original PDF\03` has both `article.pdf` and `article 1.pdf`; one should be quarantined after visual/content confirmation.
- `Daily Reading\Original PDF\04` through `09` are the new priority cleanup set. They contain a mix of generic names, original long source names, markup PDFs, audio duplicates, and `highlights.md`.
- `Lectures\01. lecture-1\05. reading` repeats `dengue-bangladesh` and `vietnam-two-child-policy`, and adds `swedish-schools-unplugged`; treat lecture copies as private source references unless a pack is intentionally promoted into Daily Reading/AET.
- `Lectures\01. lecture-1\02. sentence-structure` is large reference material, not Phase 1 article vocabulary input.
- `Vocabulary\00. poverty-global-issues` is a future topic-bank pack, not a Phase 1 article fixture.
- `Writing` and `Reading` are useful study/reference material, but they belong to later AET phases unless we create manual article-like vocabulary packs from them.

## Target Storage Convention

For every daily article source folder:

```text
Daily Reading\Original PDF\<nn. slug>\
  article.pdf
  teacher-vocab.pdf
  audio.mp3                  # optional
  article.markup.pdf          # optional iPad annotation copy
  teacher-vocab.markup.pdf    # optional
  highlights.md               # typed notes, form score, useful language
```

For duplicates or uncertain originals:

```text
Daily Reading\_duplicates\<nn. slug>\
  <original filename>
  README.md                   # why this was quarantined
```

For future topic-bank sources:

```text
Vocabulary\<topic-slug>\
  topic-source.pdf
  topic-source.markup.pdf     # optional
  teacher-vocab.pdf
  highlights.md               # optional
```

For writing/proofreading sources:

```text
Writing\<lesson-slug>\
  lesson.pdf
  exercise.pdf                # optional
  review-checklist.md
  corrections.md              # optional, later Phase 4
```

For lecture bundles:

```text
Lectures\<nn. lecture-slug>\
  README.md
  class-notes.md
  <numbered module folders>
```

Lecture bundle files should stay private source material. Do not copy them into `D:\Work\aet\data` unless the user explicitly promotes a module into an AET pack.

## Extraction Priority

1. Daily Reading 06-09: active catch-up plan, already have `highlights.md`; these are the highest value AET population candidates.
2. Daily Reading 04-05: need normalization first, then vocabulary extraction.
3. Daily Reading 03: resolve `article.pdf` vs `article 1.pdf`.
4. Lecture reading `swedish-schools-unplugged`: useful new article candidate, but only after Daily Reading 04-09.
5. `Vocabulary\01.History`: store as future topic-bank material; do not force into Phase 1 article format.
6. `Lectures\01. lecture-1\02. sentence-structure`: index only; defer extraction until grammar/writing phases.

## Phase Boundary

Phase 1 AET accepts only:

- `article.toml`
- locked 19-column `vocab.csv`
- deterministic exports: `anki-basic.tsv`, `anki-cloze.tsv`, `anki-production.tsv`, `vocabulary.pdf`

Do not add source PDFs, full article text, teacher notes, lecture PDFs, raw private corrections, or OneDrive source names to Git.

`build-topic` may be used as a cross-article export convenience, but this cleanup must not become Phase 3 master vocabulary storage.

## Task 1: Freeze a Read-Only Inventory

**Files:**
- Create outside Git if desired: `D:\OneDrive\01. IELTS\00.Academic English\source-inventory-2026-05-28.md`
- Do not modify: `D:\Work\aet\data`

- [ ] **Step 1: Export a full relative file list**

Run:

```powershell
$root = 'D:\OneDrive\01. IELTS\00.Academic English'
Get-ChildItem -LiteralPath $root -Recurse -File -Force |
  Sort-Object FullName |
  Select-Object @{Name='MB';Expression={[math]::Round($_.Length/1MB,2)}},
                @{Name='RelativePath';Expression={[IO.Path]::GetRelativePath($root,$_.FullName)}} |
  Format-Table -AutoSize
```

Expected: about 90 files and about 360 MB total.

- [ ] **Step 2: Save the inventory privately**

If a saved inventory is useful, save it under the OneDrive root, not in Git:

```powershell
$root = 'D:\OneDrive\01. IELTS\00.Academic English'
$out = Join-Path $root 'source-inventory-2026-05-28.md'
Get-ChildItem -LiteralPath $root -Recurse -File -Force |
  Sort-Object FullName |
  ForEach-Object { "- $([IO.Path]::GetRelativePath($root,$_.FullName)) ($([math]::Round($_.Length/1MB,2)) MB)" } |
  Set-Content -LiteralPath $out -Encoding UTF8
```

Expected: `source-inventory-2026-05-28.md` appears in OneDrive and remains outside the AET repo.

## Task 2: Normalize Daily Reading Storage

**Files:**
- Modify privately: `D:\OneDrive\01. IELTS\00.Academic English\Daily Reading\Original PDF\04.Working Mother`
- Modify privately: `D:\OneDrive\01. IELTS\00.Academic English\Daily Reading\Original PDF\05.Chinese Fast-Food Insurgents`
- Modify privately: `D:\OneDrive\01. IELTS\00.Academic English\Daily Reading\Original PDF\06.Chinese companies winning global south`
- Modify privately: `D:\OneDrive\01. IELTS\00.Academic English\Daily Reading\Original PDF\07. No phone in US schools`
- Modify privately: `D:\OneDrive\01. IELTS\00.Academic English\Daily Reading\Original PDF\08.New Taliban Decree on Divorce Formalize Child Marriage`
- Modify privately: `D:\OneDrive\01. IELTS\00.Academic English\Daily Reading\Original PDF\09.Why you should (almost) always look on the bright side of life`

- [ ] **Step 1: Review each folder before moving files**

Run:

```powershell
$root = 'D:\OneDrive\01. IELTS\00.Academic English\Daily Reading\Original PDF'
Get-ChildItem -LiteralPath $root -Directory |
  Where-Object Name -match '^(04|05|06|07|08|09)' |
  ForEach-Object {
    "`n## $($_.Name)"
    Get-ChildItem -LiteralPath $_.FullName -File | Select-Object Name, Length, LastWriteTime
  }
```

Expected: every folder is listed with the files that need to be classified.

- [ ] **Step 2: Rename only obvious canonical copies**

Canonical filenames:

```text
article.pdf
teacher-vocab.pdf
audio.mp3
article.markup.pdf
teacher-vocab.markup.pdf
highlights.md
```

Rules:

- If a generic canonical file already exists, leave it in place.
- If a long original filename has the same size as the generic canonical file, move the long filename to `_duplicates`.
- If a long original filename is the only article PDF, rename it to `article.pdf`.
- If a long original filename is the only vocabulary PDF, rename it to `teacher-vocab.pdf`.
- If an audio file duplicates `audio.mp3`, move the original named audio file to `_duplicates`.
- Do not delete anything.

- [ ] **Step 3: Quarantine uncertain duplicates**

Example for a single folder after confirmation:

```powershell
$src = 'D:\OneDrive\01. IELTS\00.Academic English\Daily Reading\Original PDF\06.Chinese companies winning global south'
$dup = 'D:\OneDrive\01. IELTS\00.Academic English\Daily Reading\_duplicates\06.Chinese companies winning global south'
New-Item -ItemType Directory -Force -Path $dup | Out-Null
Move-Item -LiteralPath (Join-Path $src 'Chinesecompaniesarewinningtheglobalsouth--.pdf') -Destination $dup
Move-Item -LiteralPath (Join-Path $src '005 Leaders - Globalisation.mp3') -Destination $dup
```

Expected: the source folder keeps only canonical names plus `highlights.md`; originals are preserved in `_duplicates`.

## Task 3: Complete Human Learning Notes Before AET Population

**Files:**
- Modify privately: `Daily Reading\Original PDF\06...\highlights.md`
- Modify privately: `Daily Reading\Original PDF\07...\highlights.md`
- Modify privately: `Daily Reading\Original PDF\08...\highlights.md`
- Modify privately: `Daily Reading\Original PDF\09...\highlights.md`
- Modify privately: `Writing\00. argumentative-essay\review-checklist.md`
- Modify privately: `Writing\03.Capitalization\review-checklist.md`

- [ ] **Step 1: Finish catch-up checklist**

Follow `D:\OneDrive\01. IELTS\00.Academic English\catch-up-plan-2026-05-27.md`.

Expected:

- each `highlights.md` has form score and mistakes,
- each article has at least 3 useful language items,
- writing review checklists are filled.

- [ ] **Step 2: Keep notes typed**

Use this shape in each `highlights.md`:

```markdown
# Highlights

## Done
- [ ] Read article
- [ ] Checked teacher vocabulary
- [ ] Completed form

## Form Result
- Score:
- Mistakes:

## Useful Collocations
- 

## Strong Sentences
- 

## Questions
- 
```

Expected: later AET population can read the typed notes without extracting private PDF annotations.

## Task 4: Promote One Daily Article Into AET Data

**Files:**
- Create: `D:\Work\aet\data\articles\<article-slug>\article.toml`
- Create: `D:\Work\aet\data\articles\<article-slug>\vocab.csv`
- Read privately: OneDrive article folder

- [ ] **Step 1: Pick the next article**

Recommended first article after current fixtures:

```text
Daily Reading\Original PDF\06.Chinese companies winning global south
```

Reason: it has article, teacher vocabulary, audio, and `highlights.md`.

- [ ] **Step 2: Create `article.toml` from metadata only**

Use the existing fixture format:

```powershell
Get-Content D:\Work\aet\data\articles\dengue-bangladesh\article.toml
```

Expected fields:

```toml
id = "<slug>"
title = "<title>"
source_name = "<source>"
date = "<yyyy-mm-dd>"

[stats]
row_count = 0
has_ipa = false
```

Do not paste full article text into `article.toml`.

- [ ] **Step 3: Create locked 19-column `vocab.csv`**

Use exactly this header:

```csv
id,term,ipa,category,type,definition_en,meaning_vi,source_sentence,my_sentence,priority,collocation_pattern,ielts_topics,skill_use,difficulty,card_types,origin,review_status,anki_export,tags
```

Rules:

- Leave `ipa` empty if the teacher source has no IPA.
- Set `origin=teacher` only for teacher-provided entries.
- Set `origin=codex` for Codex-suggested rows.
- Use `review_status=reviewed` until the user explicitly approves rows.
- Set `anki_export=no` until the row is approved for export.
- Never fabricate `my_sentence`.

- [ ] **Step 4: Validate before building**

Run:

```powershell
cargo run -p aet-cli -- validate data/articles/<article-slug>
```

Expected: validation passes or shows actionable row errors.

- [ ] **Step 5: Build after validation passes**

Run:

```powershell
cargo run -p aet-cli -- build data/articles/<article-slug>
```

Expected:

- `dist/<article-slug>/anki-basic.tsv`
- `dist/<article-slug>/anki-cloze.tsv`
- `dist/<article-slug>/anki-production.tsv`
- `dist/<article-slug>/vocabulary.pdf`
- session timer output

## Task 5: Keep Non-Article Sources as Future Queues

**Files:**
- Modify privately: `Vocabulary\<topic-slug>`
- Modify privately: `Writing\<lesson-slug>`
- Modify privately: `Lectures\<lecture-slug>`

- [ ] **Step 1: Mark topic-bank candidates**

For `Vocabulary\00. poverty-global-issues` and `Vocabulary\01.History`, add or update private `highlights.md` only if notes are needed. Do not create AET master vocabulary storage in Phase 1.

- [ ] **Step 2: Mark writing candidates**

Keep `Writing\00. argumentative-essay` and `Writing\03.Capitalization` as writing review material. Do not convert them into `data/articles` unless the user wants vocabulary cards from a specific lesson.

- [ ] **Step 3: Index lecture bundle only**

Use `Lectures\01. lecture-1\README.md` and `class-notes.md` to describe what exists. Keep the large grammar PDFs as reference sources.

## Verification Gates

Before saying the cleanup/population work is done:

- [ ] OneDrive source folders still contain clean originals or quarantined duplicates.
- [ ] No private PDFs are copied into Git.
- [ ] `git status --short` shows no `dist/`, private source, or OneDrive files staged.
- [ ] For any new AET article folder, run:

```powershell
cargo run -p aet-cli -- validate data/articles/<article-slug>
cargo run -p aet-cli -- build data/articles/<article-slug>
```

- [ ] For repo-wide confidence after new fixtures:

```powershell
cargo check --workspace --target-dir .codex-target
cargo test --workspace --target-dir .codex-target
```

## Recommended Next Move

Do not start by extracting everything. First normalize Daily Reading 04-09 and finish the `highlights.md` notes for 06-09. Then promote exactly one article, preferably `06.Chinese companies winning global south`, into `D:\Work\aet\data\articles` as a pilot. Once that validates and builds cleanly, repeat the same path for 07-09.
