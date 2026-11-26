# Academic English Operations Handoff

This is the reliability runbook for agents maintaining the private course tree:

`D:\OneDrive\01. IELTS\00.Academic English`

The private OneDrive tree is the source of truth. This repository contains only
the operational instructions and sanitized AET code. Never import teacher PDFs,
articles, audio, notes, corrections, or private answer files into Git.

## Resume contract

At the start of every run, read these files in order:

1. `D:\Work\aet\AGENTS.md`
2. `D:\Work\aet\docs\ACADEMIC_ENGLISH_HANDOFF.md`
3. `D:\Work\aet\docs\ACADEMIC_ENGLISH_OPERATIONS_HANDOFF.md`
4. `D:\OneDrive\01. IELTS\00.Academic English\CURRENT.md`
5. `CURRENT.md`'s named lecture class-info files: `source-post-index.md`,
   `class-notes.md`, and `homework-lecture-*.md` when present.

Do not start by downloading. First establish the current local checkpoint and
the exact gap.

## Standing intake rule — set by the user 2026-08-10

**Every teacher input from now on gets processed, not just the ones the user asks
about.** Classes are Thursday and Sunday, 19:30-22:00 Asia/Bangkok, so material
arrives at least twice a week.

At the **start of every session**, before anything else:

1. Read the private `CURRENT.md` for the last checkpoint.
2. Check the Drive connector for anything new:
   `search_files` with `sharedWithMe = true and modifiedTime > '<last checkpoint>'`,
   and `list_recent_files`. This catches material the user has opened in a browser.
3. Compare the newest local file date against the class calendar. If a Thursday or
   Sunday has passed with no new local material, **say so** — do not wait to be asked.
4. If the user pastes posts or links, run the full intake protocol below.

**Division of labour, set 2026-08-10.** Claude monitors and manages the **local
Core** — intake, filing, trackers, counts, provenance. The tutor (ChatGPT) handles
the **pedagogical meaning** of whatever the user brings to a tutoring session:
what the teacher taught or changed, which Core concept it belongs to, whether it
changes a rule or strategy in use, whether it exposes a new error category, and
whether the current learning priority should shift. Claude does the local filing
afterwards. Neither role covers the other.

**Be honest about the limit.** Claude does not run continuously and cannot watch
Drive between sessions. Link-only files are invisible to the connector until the
user opens them once. So "monitoring" means: every session begins with this check,
and no supplied material is ever left unprocessed. If the user wants true
unattended polling, that needs a scheduled cloud routine — offer it, do not fake it.

After any intake, always report: what arrived, what was verified, what is still
pending, and what remains an unresolved gap.

### Always list Core copies explicitly — user rule, 2026-08-13

**Anything copied into `00. Core` gets itemised in the reply**, so the user can
review it at a glance without opening folders. One row per file:

| Core path | Source | Why it earned entry |
|---|---|---|

State the **granularity plainly**: whether a whole folder went in, or selected
files were distributed into existing topic folders. "Copied the Academic writing
folder" and "copied its four files into `10. paragraph` and
`11. cause-effect-essay`" are different actions and the user wants to know which.

Also state what was **deliberately not copied** and why — already local,
duplicate, or out of scope. A silent omission looks identical to an oversight.

## Intake protocol

For each teacher post or supplied Drive link:

1. Record the teacher-post date separately from the local-added date. If the
   post is relative (`Yesterday`, `13 giờ`, etc.), preserve that uncertainty
   instead of inventing an exact timestamp.

   **Never resolve a relative timestamp against the current date.** It is
   relative to when the *user captured the post*, and the user confirmed
   (2026-08-13) that capture can be several days before the post reaches Claude.
   Anchor on Drive `createdTime`/`modifiedTime` — those are absolute — and mark
   anything else **inferred**. Worked example: Daily Reading 45 was posted
   `4 phút` after its files were uploaded at 2026-08-11 09:17, so the post is
   ~09:21 that day; resolving `4 phút` against "today" would have moved it two
   days and been wrong. Counter-example in the same batch: the sentence-structure
   preview test has `createdTime` **2021-08-17** — an old document re-shared, so a
   file timestamp is evidence for a post date only when the two are close.
2. Resolve every Drive URL/ID. A truncated ID is an unresolved gap, not an
   invitation to guess or substitute an older similarly named file.
3. List the Drive folder before downloading. For each item capture title, ID,
   MIME type, size, created time, modified time, and parent folder.
4. Build a small manifest with these columns:
   `teacher_post_date`, `drive_id`, `title`, `mime_type`, `drive_size`,
   `drive_created`, `drive_modified`, `destination`, `local_match`, `action`,
   `header_check`, and `notes`.
5. Compare expected Drive size with the local tree. A same-size match is a
   useful duplicate signal, but it is not proof of identity when two Drive IDs
   differ and the distinction matters; hash-check that file before reusing it.

## Safe download and copy protocol

- Use the authenticated Google Drive connector's raw-file path, or an
  authenticated-compatible raw download only after metadata/listing.
- Stage downloads outside the repository's tracked content, for example under
  `D:\Work\aet\.course-staging\<yyyy-mm-dd>`. Never leave private binaries in
  the Git worktree after the run.
- Verify exact byte size before copying.
- Verify every PDF begins with `%PDF-`.
- Verify every MP3 begins with `ID3` **or a valid MPEG audio frame** — a frame
  sync starts `FF Fx`. Two library MP3s (`Vocabulary\15. crimes-collocations`,
  `22. family-collocations`) begin `FF FB 90 04` with no ID3 tag and are perfectly
  valid. A checker that tests only for `ID3` will report false corruption; that
  happened on 2026-08-10.
- Copy files into OneDrive; never move or delete the Drive/local original.
- If a target exists with a different size, stop and investigate. Do not
  overwrite user-marked or annotated files.
- Keep duplicate contexts when they matter: active class material,
  released-not-taught material, teaching-note material, and canonical skill
  copies may legitimately have separate paths.

## Classification rules

Use these destination roles:

- `Lectures\<N>. lecture-N\01. pre-class-materials`: active preparation.
- `Lectures\<N>. lecture-N\02. teaching-notes`: post-class teacher notes.
- `Lectures\<N>. lecture-N\04. released-not-taught`: distributed material not
  confirmed taught.
- `Lectures\<N>. lecture-N\05. related-posts`: class-access copies of packets
  whose canonical home is `Daily Reading`, `Vocabulary`, or `Writing`.

**Duplicate policy, set by the user on 2026-08-10: copy everything new, even when
it is byte-identical to a file already local. Do not skip it and do not stop at
indexing it — place the copy in the related folder.** A teacher who re-uploads
the same PDF to Google Classroom has given it a new role (homework rather than
reading material), and the user wants that role visible as a file. Record the
byte-identity in the tracker instead of collapsing the copies.
- `Daily Reading\Original PDF\<N>.<topic>`: article/audio/vocabulary/quiz
  packets with `highlights.md`.
- `Vocabulary`, `Writing`, `Sentence`, and `Reading`: canonical skill sources
  with a visible `review-checklist.md` where appropriate.
- `IELTS Test\01. reading\materials`, `03. writing\materials`, and
  `05. vocabulary-grammar\materials`: review copies, not replacements for
  the source course files.

## Tracker and completion rules

After a successful copy, update the visible trackers that apply:

- top-level `CURRENT.md`;
- lecture `source-post-index.md`;
- lecture `class-notes.md`;
- `homework-lecture-*.md` when homework is explicitly supplied;
- Daily Reading `highlights.md`;
- skill `review-checklist.md`;
- IELTS reading/writing/support trackers and `materials-index.md` when a
  canonical IELTS copy is created.

Downloaded or copied means **available**, not completed. Do not check an item
off without a score, submitted result, completed answer file, explicit user
confirmation, or teacher feedback/correction. Do not invent homework when a
teacher only says that work will be corrected in class.

## Final audit

Before reporting:

1. Re-list each relevant Drive source folder and compare its item count with the
   local packet count.
2. Recheck all newly copied PDF/MP3 headers and expected sizes.
3. Confirm unresolved links are written as explicit gaps in the source index.
4. Confirm teacher-post and local-added dates remain separate.
5. Remove only the temporary staging files created by this run.
6. Run `git status --short` from `D:\Work\aet`; do not stage or commit private
   course material, and preserve unrelated user changes.

## Current checkpoint — 2026-08-06

The private `CURRENT.md` is the authoritative detailed checkpoint. At this
checkpoint:

- Lecture 15 has 2 identified preparation PDFs and 7 teaching-note PDFs. The
  Job Description exercise link is still unresolved because the supplied Drive
  ID is truncated.
- Lecture 16 has 7 active preparation PDFs, 27 released-not-taught PDFs, and 4
  teaching-note PDFs. Attendance and completion remain unrecorded.
- Daily Reading 41 is Female Breadwinners with article, audio, teacher
  vocabulary, vocabulary exercises, IELTS reading practice, and an unchecked
  `highlights.md`.
- New UK/US English, Job Description, and Proofreading 9 materials have
  canonical skill copies and remain pending.
- Collocations for IELTS: Family has a verified PDF and MP3 in
  `Vocabulary\22. family-collocations`, with an IELTS support copy under
  `IELTS Test\05. vocabulary-grammar\materials\30. Family Collocations`.
  Teacher-post date is 2026-08-03 08:59 (Asia/Bangkok); study remains pending.
- Lecture 17 now has 3 standalone pre-class PDFs, 6 teaching-note PDFs, class
  trackers, and class-access related copies. The exact class-materials folder
  link beginning `1XGhOwebfGQq10VYpjOXY5yO4NZS` is truncated and unresolved.
- Daily Reading 42 (Gen Z Can't Afford a First Date) has 4 PDFs and no supplied
  audio. Daily Reading 43 (Too Many Master's Courses) has 4 PDFs and 1 MP3.
- New canonical/support packets: Health - Medicine Vocabulary 1 update,
  International Trade, and Academic Word Choice. All remain pending.
- This run reconciled 19 exact Drive files (18 PDFs and 1 MP3) from metadata and
  one six-item teaching-notes folder listing. The final target audit checked 30
  canonical/lecture copies with SHA-256 equality, expected sizes, and no header
  failures.

## Checkpoint update — 2026-08-09

- IELTS Reading Test 3 is done and teacher-corrected in the 2026-08-09 class
  (user-confirmed). Score and mistakes remain unrecorded, so the tracker shows
  the work as done with the score fields explicitly empty.
- The user annotates their own working copies in place. When a copy is larger
  than its canonical source, ask before treating it as drift: an annotated or
  teacher-corrected copy is evidence, not corruption. Never overwrite it with the
  clean source, and never rename a `* tu.pdf` working copy.
- No teacher post or Drive link has been supplied for the 2026-08-09 class, so no
  lecture folder was created for it.

## Checkpoint update — 2026-08-10

- Lecture 18 exists for the 2026-08-09 class. Daily Reading 44 and the Health &
  Medicine Vocabulary Reference are local and pending. Lecture 17 homework is
  recorded from Google Classroom.

### Known Drive connector limits

- `get_file_metadata` resolves any **exact** file or folder ID, including
  link-shared items that are not in the user's Drive index. Use it first.
- `search_files` only covers indexed files. It will **not** find a link-shared
  file by title, and a `parentId = '<folder>'` listing returns empty for
  link-shared folders — confirmed against `Lecture 18`, `Teaching notes 18`, and
  the already-synced Lecture 17 teaching-notes folder. An empty listing is a tool
  limit, never evidence that a folder is empty.
- Consequence: a truncated ID cannot be recovered by search. Ask the user for the
  full link.

### Listing a link-shared folder (works — use this before asking the user)

The connector cannot enumerate a folder, but the public folder page can:

```powershell
$html = (Invoke-WebRequest -Uri "https://drive.google.com/drive/folders/<FOLDER_ID>" -UseBasicParsing).Content
[regex]::Matches($html,'\\x22([\w-]{28,44})\\x22(?:[^\]]{0,600}?)\\x22([^\\]{2,120}?\.(?:pdf|mp3))\\x22')
```

Quotes are escaped as `\x22` in the embedded payload; group 1 is the file ID and
group 2 the filename. This recovered all 3 `Lecture 18` files and all 5
`Teaching notes 18` files on 2026-08-10, including one the first, looser pass had
missed.

Two mistakes cost a re-run on 2026-08-10; do not repeat them:

- **Escaped characters break naive patterns.** The payload writes `application\/pdf`
  and `\\u0026` for `&`, so a group like `[^\\]+?` silently matches nothing and the
  folder reports **zero files**. Terminate string groups on the literal `\x22`
  instead: `((?:(?!\\x22).)*?)`.
- **Recurse — every time, no exceptions.** Lecture 1 holds only subfolders, so a
  files-only scan called it empty. Walk children where the mime type contains
  `google-apps.folder`. The full 16-lecture walk was 118 folders, not 16.

  **This was then repeated on Lecture 19 (2026-08-13).** The file-pattern match
  reported 3 files; the folder actually held **46 across three subfolders**
  (`Academic writing`, `Đã phát, chưa dạy`, `Teaching notes`). The recursive
  walker already existed and was not used. **Never list a Drive folder with the
  file regex alone** — always walk it, even when the flat listing looks complete
  and matches what the post described.

The full entry shape, which also yields byte size without any API call:

```
\x22<ID>\x22,\x5b\x22<PARENT>\x22\x5d,\x22<TITLE>\x22,\x22<MIME>\x22,0,null,0,0,0,<createdMs>,<modifiedMs>,null,null,<SIZE>,
```

Byte size is the cheap way to diff a whole Drive tree against the local library:
index every local file by length, then flag Drive entries whose size appears
nowhere locally. A size match is strong evidence of presence but not proof —
verify by hash anything you act on.

**Scraped output is a lead, never the record.** Feed every recovered ID back
through `get_file_metadata` and confirm the returned `parentId` matches the
folder you listed, plus title and byte size, before downloading. A cross-check
that proves the method is honest: a scraped ID that also appears in a
user-supplied post link must match exactly, as `Test 3.pdf`
(`1fVJsLnssTvlPtyT9kEyHnVrbxqu6KbQs`) did.

This does not rescue a truncated folder ID — the Lecture 17 gap still needs the
full link from the user.
- Download that avoids the metadata path: `https://drive.google.com/uc?export=download&id=<ID>`
  works for link-shared files under ~25 MB. Always compare the result against the
  byte size from `get_file_metadata` and check the header; an HTML interstitial
  fails both checks.

## Learning Protocol v1.1 — frozen 2026-08-10

`D:\OneDrive\01. IELTS\00.Academic English\00. Core\protocol.html` is the working
agreement between the user, the test files, ChatGPT (tutor) and Claude (curator
and auditor). Claude's role is defined there: record, curate, audit weekly
patterns, intake new teacher material — **not** to act as a second tutor
explaining every question.

Log schema in `00. Core\06. progress\error-log.md` must stay in sync with it.
Fields that carry the diagnostic weight:

- **`Root cause` is separate from `Error ID`.** The ID is the symptom; the cause
  is what to teach. Causes aggregate across skills — a reading miss and a
  listening miss both from *syntactic parsing* point at one ladder level.
- **`Confidence` is recorded at answer time or not at all.** Reconstructed values
  are contaminated. Right-at-low-confidence is fragile knowledge the score hides.
- **Status ends at `STABLE`, never `RESOLVED`.** Retries at same session, 2-3
  days, then 1-2 weeks, each with a due date so the weekly review surfaces them.

Two standing constraints on this repo's agent: logging an error is a 2-5 minute
job, so do not expand the schema; and the source key is canonical unless there is
concrete evidence of an inconsistency, in which case flag `SOURCE-DISPUTE`,
verify, preserve the original, and record the resolution rather than silently
overriding it.

## Deduplication semantics — locked 2026-08-10

Four relations. Only the first ever justifies deleting anything.

| Relation | Test | Action |
|---|---|---|
| **EXACT** | Same SHA-256 | Keep one copy in Core |
| **VARIANT** | Same lesson or document family, **different SHA-256** | May consolidate conceptually; **never silently discard** |
| **ANNOTATED / LEARNER_COPY** | Teacher marks or the user's worked copy | **Always preserve**, regardless of similarity |
| **Size or name similarity** | Heuristic only | A **discovery signal**, never evidence of equivalence |

Language matters in the record. Write *"X → represented by Y; same document
family, not byte-identical"* — never *"X is identical to Y"* — unless the hashes
actually match. `@agreement GfW3.pdf` (17,963,368) and `agreement-gfw3.pdf`
(18,038,865) are the same lesson and **not** byte-identical.

Why this is not pedantry: `Clauses 9.1.pdf` (527,364) and `Clause 10.pdf`
(523,324) sit 0.77% apart and are **different editions**. A size-only rule would
have deleted one and reported success. The user's own document list is what
caught it.

`00. Core\core-manifest.csv` records the relation for every asset:
`CorePath, OriginalName, OriginalPath, Bytes, SHA256, Relation, CurriculumSpine`.
Regenerate it after any Core change so claims like "94/94 represented" stay
reproducible instead of resting on filename heuristics.

## Core naming convention — frozen 2026-08-10

Four grammars, one semantic dimension each. Full statement lives in
`00. Core\README.md`; do not invent a fifth.

| Grammar | Means |
|---|---|
| `NN. name` | Ordered curriculum or section |
| `SLx-name` | Sentence mastery level (SL = Sentence Level) |
| `Sxx` `Wxx` `Rxx` `Lxx` `SPxx` `Vxx` | Stable error taxonomy — never renumber |
| `A-` `B-` `C-` | Vocabulary priority layer |

`00` is reserved for overview/index/mixed material. `S00-mixed-common-errors` is
a **folder marker, not an error ID** — it must never appear in the error log.

The ladder prefix was `L1`-`L7` until 2026-08-10 and was renamed to `SL1`-`SL7`
because `L` already meant Listening in the error taxonomy; `L4` versus `L04` was
distinguishable only by zero-padding, which is too fragile once IDs appear in
logs. Error IDs are frozen, so the folder names moved instead.

Never normalise Daily Reading packet folder names inside Core — they carry
provenance back to the source library, including its inconsistent spacing.
Never rename `* tu.pdf` or `*.markup.pdf`.

## Handoff report format

Use this compact structure in the final response:

1. **Reconciled:** source folders/posts checked and date cutoff.
2. **Synced:** counts by lecture/packet and canonical copies.
3. **Verified:** size/header/count results.
4. **Open gaps:** unresolved IDs, inaccessible files, or relative dates.
5. **Status:** what remains pending and whether any Git changes were made.

## Checkpoint update — 2026-08-23

Lecture 21 exists for the 2026-08-20 Thursday class: 59 files across class-info,
pre-class, teaching-notes, released-not-taught, and related-posts. Daily Reading
46 and 47 are local. Six new skill folders. Core is 429 files.

The live item is the **Midterm Test: Grammar for Writing** — Google Classroom,
`extendedforms.io`, single attempt, timed.

### The Drive folder page changed its render — the old regex silently returns zero

**This broke the walker and will break it again if not remembered.** The documented
`\x22`-escaped JS payload pattern matched **nothing** on the Lecture 21 folder. The
page now renders folder contents as **real HTML table rows**:

```html
<tr data-selectable data-id="<FILE_ID>" ...>
  ... data-tooltip="<TITLE> PDF" ...
```

Working extraction, both patterns worth keeping since either render may appear:

```powershell
# IDs — one row per item, each id repeats ~3x per row
[regex]::Matches($html,'<tr data-selectable data-id="([\w-]{20,50})"')
# Titles
[regex]::Matches($html,'data-id="([\w-]{20,50})"[^>]*data-tooltip="((?:(?!").)*?)"')
```

**Zero matches means the pattern is wrong, not that the folder is empty.** Same
failure class as the 2026-08-10 escaped-character bug. When a listing returns zero,
verify against a known-child ID before believing it.

**This render gives no byte size.** Size came from `get_file_metadata` per ID
instead — which the runbook already requires before acting on any scraped ID.

### Recursion caught a gap the flat listing would have buried

The Lecture 21 folder root held 5 files and **3 subfolders**; the full tree is 47
files. One subfolder was `Teaching notes`, one was `Đã phát, chưa dạy`, and the
third — **titled with a trailing newline, so it produced no tooltip** — was the
Daily Reading 46 packet that had been an open gap for nine days.

Two lessons compound here:
- **A row with no tooltip is not a non-entity.** Resolve every scraped `data-id`
  through `get_file_metadata`, including ones the title regex skipped.
- The recursion rule has now paid off three times (Lecture 1, Lecture 19,
  Lecture 21). It is not optional.

### Diffing without scraped sizes

Build a local index once and match on **exact** byte size:

```bash
find . -type f \( -iname "*.pdf" -o -iname "*.mp3" \) -printf "%s\t%p\n"
```

1,319 entries covered the whole library. This is what showed all 29
`Đã phát, chưa dạy` files were already held and only 26 files needed downloading.

**Third near-collision recorded, and the closest yet:**
`capitalization, spelling, abbreviations, and numbers` = **303,866** bytes;
`Misplaced Modifiers G7.pdf` = **303,924** bytes. **58 bytes**, unrelated
documents. With `Clause 10` vs `Clauses 9.1` (0.77%) and `Misplaced Modifiers G7`
vs `Modifiers 9.1` (1.2%), the rule is settled: **exact size or hash, never a
tolerance.**

### Relative timestamps — the rule held again

The captured feed gave only `5 ngày`, `4 ngày`, `3 ngày`, `2 ngày`, `1 ngày`.
None was resolved against the current date. Every absolute time in the Lecture 21
source index comes from Drive `createdTime`.

This mattered: **four of the thirteen posts point at documents created in 2022 and
2023** — old references re-shared. Their file timestamps are recorded as **not
evidence** of post date, and those post dates are marked inferred. Anchoring
blindly on `createdTime` would have dated posts three years early, the mirror
image of the Reading 45 mistake.

### Recorded as written

The Zoom post for Lecture 21 links the **Lecture 20** class folder
(`19i9I3bUsBmZlPFlLwVpbxBHTwjvVHx3X`), not Lecture 21's
(`1KVSRVF17hjzSkoyLdL-QE9TaZeWU6n7t`). Teacher reused the previous link. Both were
walked. Not corrected in his text.

The weekday/date off-by-one did **not** recur: "sáng thứ 5 (20/8)" is correct.

## Checkpoint update — 2026-08-23 (second intake)

Lecture 22 exists for the Sunday 2026-08-23 class, filed **before** the class:
6 files, 4 already held, 2 new. Both supplied links full — second consecutive
intake with no truncated IDs. Core is 431 files.

Zoom credentials were **not** in the posts; the teacher said they arrive the
morning of the class. Recorded as pending, not guessed. The meeting ID has been
stable at `8912108989` across Lectures 20-21 while the passcode changed each time
(`TCB8`, `TCB3`) — so the ID may be predicted, the passcode never.

### VARIANT handling, worked example worth keeping

`Tenses & Time Shifts.pdf` (7,779,728) versus the held
`tenses-and-time-shifts.pdf` (**136,420**). Same name, same lesson family,
**57× size difference**, different SHA-256.

A name-based dedupe would have skipped the download entirely and reported the file
as already present. A size-tolerance rule would not have — but only because the
gap is huge; the failure mode is the same one that `Clause 10` vs `Clauses 9.1`
(0.77% apart) exposes from the other direction. **Neither name nor size is a
substitute for the exact-size/hash test.**

Both files kept, the new one dated in its filename. Recorded as *same document
family*, explicitly **not** as *identical*.

### A cross-check that validates the scraping method

`Positive & Negative Development Essay.pdf` appeared in the Lecture 22 folder
under Drive ID `1xzz3Jn7rNwO-Wqe1M5X1fx8OWnGnxFgi` — **the same ID** resolved and
downloaded hours earlier from a standalone post, byte-identical at 416,789.

When a scraped ID matches one already independently verified, the walk is
returning real data rather than plausible-looking noise. Worth doing deliberately
when a folder listing is the only evidence available.

### Taxonomy boundary held

`Nouns & Determiners` covers count/noncount nouns, articles and quantifiers — the
teacher calls them the commonest writing errors. The frozen taxonomy has **no ID**
for them: not `S04` agreement, not `V02` wrong word choice.

Per the division of labour set 2026-08-10, deciding whether teacher material
exposes a **new error category** is the tutor's call, not Claude's. The material
was filed structurally under `SL3-phrases` and the gap was recorded as an open
question in three places. **No ID was invented.** This is the correct handling of
a frozen taxonomy meeting genuinely new content.

## Session close — 2026-08-23

State at handoff. All counts verified against the tree, not carried from memory.

| | |
|---|---:|
| Core | **438 files** |
| Library PDFs/MP3s | 1,451 |
| Lecture folders | 22 |
| Daily Reading packets | 48 |
| Error log entries | **4 — the first ever** |

Two classes intaken in one session: **Lecture 21** (Thu 2026-08-20, 59 files) and
**Lecture 22** (Sun 2026-08-23, 14 files, filed before *and* after the class).

**No truncated link remains outstanding anywhere in the library.** Both
long-standing gaps closed this session — Daily Reading 46 (recovered as a
subfolder of the Lecture 21 folder) and the Lecture 15 job-description exercise
(full link supplied, 21 days open, the oldest gap on record).

### What changed in how this work is done

The session moved past pure curation for the first time. Test 4 was **scanned,
marked, and logged** — 36/40, band 8.0 — and the error log went from empty to
carrying a real finding.

**The finding: 3 of 4 errors are `R06`, wrong form.** `gift`/gifts,
`perfectioness`/perfectionists, `dissatified`/dissatisfied. Every one located
correctly and lost in transcription, against headings 8/8, TFNG 11/11, summary
completion 8/8. Comprehension is at target; marks are going in the copying.

The user has asked to widen Claude's role toward marking, pattern audit, retry
scheduling and drill-building. **`protocol.html` has not been amended** — it still
defines Claude as curator/auditor and not a second tutor. Amending it is the
user's decision and was left open deliberately. Do not act as if it is settled.

### Lessons that cost something this session

- **The Drive folder page render changed.** The documented `\x22`-payload regex
  returns zero matches; folders now come back as HTML `<tr data-selectable
  data-id=...>` rows. Both patterns are recorded above. Zero matches means the
  pattern is wrong, not that the folder is empty.
- **A row with no tooltip is not a non-entity.** The Daily Reading 46 subfolder
  had a trailing newline in its title and produced no tooltip. It was found only
  because every scraped `data-id` was resolved through `get_file_metadata`. That
  one folder was a nine-day-old gap.
- **File mtime is a save time and proves nothing about when work was done.** A
  `quiz tu.pdf` save at 20:26:35 sat seven seconds after the teacher's in-class
  correction export. It looked damning; the real explanation was a two-tool
  workflow — iPad for Passages 1-2, Bluebeam Revu for Passage 3, saved on close.
  **Ask before inferring.** The question was right to raise and wrong to assume.
- **Exact size or hash, never a tolerance.** Third near-collision recorded:
  `capitalization, spelling, abbreviations, and numbers` (303,866) vs
  `Misplaced Modifiers G7.pdf` (303,924) — **58 bytes**, unrelated documents.
- **A stale gap is as bad as an unrecorded one.** `CURRENT.md` still described
  the Lecture 17 folder as an open gap thirteen days after it closed. Gaps get
  recorded in three places and closed in one. **Reconcile open gaps across
  `CURRENT.md`, the lecture source indexes and both handoff docs as part of the
  final audit.**

### Two taxonomy gaps raised and deliberately NOT closed

Both are the tutor's call under the 2026-08-10 division of labour. **No ID was
invented in either case.**

1. **No error ID for article / countability errors** — the topic of Lecture 22's
   homework (`Nouns & Determiners`). Not `S04`, not `V02`.
2. **No error ID for matching-information questions.** `R03` covers matching
   *headings*; Test 4 Q15 was matching information by keyword. Logged as `R03`
   **provisional** and flagged.

### Open items at handoff

- **Midterm Test: Grammar for Writing** — Google Classroom, `extendedforms.io`,
  single attempt, timed, **not yet taken**. Prep card ready at
  `00. Core\02. error-catalogue\midterm-prep-card.md`.
- **Q27-34 heading key** — the teacher's correction covers Q1-26 and Q35-40 only.
  The heading marking is Claude's reasoning, not a key. Ask him.
- **Speaking / Listening / Writing Task 1** — still unsourced, still unasked.
  Now the largest risk to the 7.5 overall target, because Reading is measurably
  ahead of it and the other three are unmeasured.
- **Basic editing practice** — homework, answers to be posted as a comment.
- **Confidence has never once been recorded at answer time.** All four error-log
  entries read "not recorded". Next test: one sitting, 60 minutes, clock running,
  H/M/L per question.

## Two-session split — set by the user 2026-08-23

Work is now divided across **two concurrent Claude chats**. Both read the same
resume contract; they do **not** have the same write permissions.

| Role | Owns | Scope |
|---|---|---|
| **CURATION** | The library | Intake, Drive walks, downloads, verification, filing, folder structure, trackers, provenance, counts |
| **GOAL — IELTS 7.5** | The learning | Practice, marking, error analysis, drills, band strategy, study sequencing |

**Lanes attach to the role, never to a particular chat window.** The roles swapped
once already on 2026-08-23, when the chat opened for GOAL became the upload
thread instead. Nothing needed rewriting, because the table below is keyed by
role. A session takes a role when the user says so, and states which role it
holds at the top of its first reply so both sides can see the assignment.

### Write rule — read this before editing anything

**Concurrent chats editing the same file will silently overwrite each other.**
OneDrive resolves conflicts by last-write-wins, not by merge.

| File | Written by |
|---|---|
| `CURRENT.md` | **CURATION only** |
| lecture `class-notes.md`, `source-post-index.md`, `homework-*.md` | **CURATION only** |
| `Daily Reading\**\highlights.md`, skill `review-checklist.md` | **CURATION only** |
| Anything under `00. Core\**\materials\` | **CURATION only** |
| `00. Core\06. progress\error-log.md` | **GOAL** |
| `IELTS Test\06. scores\score-log.md` | **GOAL** |
| `00. Core\05. my-writing\` | **GOAL** |
| review/answer sheets inside a `Reading\<test>\` folder | **GOAL** |
| this runbook and `ACADEMIC_ENGLISH_HANDOFF.md` | **CURATION**, except the role/lane section itself, which either may correct — say so in the reply so the other does not duplicate it |

If a session needs a change outside its lane, it **states the change and asks**
rather than writing. Re-read a file immediately before editing it; do not trust a
copy read earlier in the conversation.

**The steering file stays single-writer.** `CURRENT.md` is the one file every
session opens first, so a lost write there is the most expensive possible
conflict. The goal chat reports what changed; the curation chat records it.

### Note on scope creep

This split does **not** by itself widen Claude's role. `protocol.html` still
defines Claude as curator and auditor, not a second tutor, and it has not been
amended. The goal chat inherits that constraint until the user changes it
deliberately.

## Checkpoint update — 2026-08-23, third intake

The session-opening Drive check found material the user had not supplied. The
standing intake rule is what surfaced it; nothing here came from a paste.

### Teacher material can disappear from Drive — the copy-everything policy is load-bearing

`Tenses & Time Shifts.pdf` (`1QEvlWdoFUY2qm9LN1obg0cmZcLwVXOrI`, 7,779,728 bytes)
was walked, downloaded and filed during the pre-class intake that morning. Hours
later `get_file_metadata` returns **"Requested entity was not found."** and the ID
is gone from the folder listing. The local copy is now the only one.

This is the first recorded case of a source file being deleted upstream, and it
retroactively settles an argument. That file was flagged the same day as a
**VARIANT** of the 136,420-byte `tenses-and-time-shifts.pdf` — same name, same
lesson family, 57x the size, different SHA-256. A name-based dedupe would have
skipped the download and reported it already present. The document would now be
lost, and the tracker would say it was held.

**Treat every Drive listing as a snapshot with an unknown expiry.** Download on
sight. `Requested entity was not found` on an ID that resolved earlier the same
day is a **deletion**, not a connector fault — record it in the source index
rather than retrying.

### A class folder keeps changing after the class

The Lecture 22 folder root was 6 files at the morning walk and 6 files plus a
`Teaching notes` subfolder at 23:42 — but not the *same* six.
`@PosNeg_Lesson_Global_Food.pdf` was added at 16:06, three and a half hours before
the class and after the morning walk, and `Tenses & Time Shifts.pdf` was removed.

Net count was unchanged. **A matching file count is not evidence a folder is
unchanged** — diff by ID, never by count.

Re-walking a lecture folder is cheap and belongs in every session-opening check
for the current and previous lecture, not just when a new post arrives.

### Filing something with no annotated counterpart

`posneg-lesson-global-food.pdf` sat in the class-folder root exactly like the five
pre-class files, but the teaching-notes folder holds no annotated version of it.
Filed to `01. pre-class-materials` for consistency with its siblings, and recorded
explicitly as **distributed, not confirmed taught**. Two of the other five
(`Punctuation 8`, `Tenses & Time Shifts`) are in the same position.

`04. released-not-taught` stays for material distributed *outside* a class folder.
Do not reclassify a class-folder file into it merely for lacking annotations.

### A tracker that disagrees with the file on disk

`22. lecture-22\01. pre-class-materials\positive-negative-development-essay.pdf`
is 381,349 bytes. The source index and class notes both record 416,789 for that
slot, which is what the four other copies hold.

Text and embedded fonts are identical; only the PDF serialisation differs. It is a
**VARIANT**, so nothing was deleted or overwritten. The origin is unexplained and
its 22:57:32 mtime proves nothing about it — the same `quiz tu.pdf` trap from
earlier the same day. **Recorded in three places as an open question and left
alone.** Reconciling a tracker against disk is the auditor's job; guessing at the
provenance of a file you did not write is not.

## Checkpoint update — 2026-09-06, later Lecture 23 intake

The Lecture 23 Drive root replaced a 178,580-byte Task 1 file with a distinct
191,980-byte revision and added Tenses plus two Two-part Question documents.
The local tree keeps both Task 1 editions. The current Drive snapshot is 28
files; the local lecture has 29 role files because of that preserved vanished
variant. Daily Reading 51 arrived with all five components. Core now has 376
manifest rows. Compare IDs and hashes, preserve vanished variants, and treat
`CURRENT.md` as the live numerical checkpoint.
