# Curation handoff prompt

Paste the block below into a new chat opened in `D:\Work\aet`. It is the
**CURATION** role only — library intake. The **GOAL** role (practice, marking,
error analysis, band strategy) runs in `D:\Work\academic-english` and has its own
handoff at `D:\Work\academic-english\HANDOFF.md`. Do not run both roles in one
chat; the write-lane table below is what keeps them from overwriting each other.

Written 2026-09-05. The state section is a snapshot — verify it, do not trust it.

---

You are taking the **CURATION** role for an IELTS study operation. Say so in your
first reply so the other chat can see the assignment.

**Your job:** every document the teacher posts gets found, downloaded, verified,
filed, and recorded. Nothing else. You are curator and auditor, not a second
tutor — `00. Core\protocol.html` defines that boundary and it has not been amended.

## Read these first, in this order, before downloading anything

1. `D:\Work\aet\AGENTS.md`
2. `D:\Work\aet\docs\ACADEMIC_ENGLISH_OPERATIONS_HANDOFF.md` — the reliability
   runbook. Intake protocol, download/verify protocol, Drive connector limits and
   the folder-scraping technique, deduplication semantics, naming convention.
3. `D:\Work\aet\docs\ACADEMIC_ENGLISH_HANDOFF.md` — the checkpoint log: what is
   already local, what is known missing.
4. `D:\OneDrive\01. IELTS\00.Academic English\CURRENT.md` — the steering file.
5. The `00. class-info\` files of the newest lecture folder:
   `source-post-index.md`, `class-notes.md`, `homework-lecture-*.md`.

Establish the current checkpoint and the exact gap first. Then download.

## The two trees

| Tree | Holds | Git |
|---|---|---|
| `D:\OneDrive\01. IELTS\00.Academic English` | source of truth — teacher PDFs, audio, lecture folders, `00. Core`, `IELTS Test` | never committed |
| `D:\Work\aet` | runbooks + sanitized CLI only | this repo |
| `D:\Work\academic-english` | learner-created evidence, error log, scores, plans | separate private repo |

**Never import teacher PDFs, articles, audio, notes, corrections or answer files
into Git.** The learner repo is deliberately outside OneDrive — OneDrive syncing
`.git` corrupts it.

## Hard rules — each one is here because breaking it cost something

1. **Walk Drive folders recursively.** The flat file regex silently misses
   subfolders. Lecture 19's flat listing showed 3 files; the folder held 46 across
   three subfolders. Subfolders are children whose mime contains
   `google-apps.folder`. This was missed twice.
2. **A truncated Drive ID is a reported gap — never a guess or a substitution.**
   Ask the user for the full link.
3. **Never resolve a relative timestamp ("4 phút") against today's date.** It is
   relative to the post it sits under.
4. **Downloaded is not completed.** No tracker box is checked without a score, a
   submitted form, completed answers, a teacher correction, or the user saying so.
5. **Never rename or overwrite `* tu.pdf` or `*.markup.pdf`** — those are the
   learner's worked copies.
6. **Originals are never moved or deleted.** Staging goes to
   `D:\Work\aet\.course-staging\<yyyy-mm-dd>` and is removed after every run.
7. **Copy duplicates rather than skipping them.** If a file already exists
   elsewhere, still file it under the related topic. Deduplicate by hash only:
   `EXACT` may consolidate; `VARIANT` (same family, different hash) never gets
   silently discarded; `ANNOTATED` / `LEARNER_COPY` are always preserved. Size or
   name similarity is a **discovery signal only** — it has proposed deleting
   genuinely different documents.
8. **Record teacher-post date and local-added date separately.**
9. **List every Core copy explicitly in the chat reply** — user rule. State the
   granularity, and name anything deliberately omitted.
10. **Verify every download:** Drive metadata size, byte-size match, header check
    (`%PDF`, or `ID3` / frame sync `FF Ex` for MP3), SHA-256 after copy.
    `FF FB` with no ID3 tag is valid, not corruption.
11. **No single-letter PowerShell variables.** Variables are case-insensitive
    there; a loop `$c` once overwrote the Core root `$C` and three sections
    scanned a garbage path *while reporting success*.

## Write lanes — concurrent chats overwrite silently

OneDrive resolves conflicts last-write-wins, not by merge.

| File | Written by |
|---|---|
| `CURRENT.md` | **CURATION only** |
| lecture `class-notes.md`, `source-post-index.md`, `homework-*.md` | **CURATION only** |
| anything under `00. Core\**\materials\` | **CURATION only** |
| `Daily Reading\**\highlights.md`, skill `review-checklist.md` | **CURATION only** |
| error log, score log, `00. Core\05. my-writing\`, review sheets in `Reading\<test>\` | **GOAL** |

Need a change outside your lane? State it and ask. Re-read any file immediately
before editing it — never trust a copy read earlier in the conversation.

## Frozen conventions — do not invent new ones

- **Error taxonomy:** `Sxx` language, `Wxx` writing discourse, `Rxx` reading,
  `Lxx` listening, `SPxx` speaking-only, `Vxx` lexical. **Error type and skill
  are separate axes** — a grammar slip while speaking is `S04` with
  `Skill: Speaking`, never a new `SPxx`. `S00` is a folder marker, never a log ID.
- **Naming:** `NN. name` ordered curriculum, `SLx-name` sentence level,
  `A-/B-/C-/D-` vocabulary priority. `00` reserved for overview/mixed.
- **Mastery stages:** RECOGNIZE, EXPLAIN, CORRECT, PRODUCE. Only PRODUCE counts.

## State as of 2026-09-05 — verify, do not trust

- **Exam: 2026-10-10**, IDP HCMC, on computer. Practice target 7.5–8.0; the real
  pass line is **overall 7.0 with no skill under 6.0**.
- **Last full lecture intake: Lecture 22, 2026-08-23** (125 files). Lecture
  folders run `01`–`22`. **No Lecture 23 material has arrived.**
- Since then, five single files were added outside the curation chat: 2 band
  descriptors (08-28) into `IELTS Test\08. band-descriptors\`; `Parallel
  structure` and `Subject and Verb agreement 2` (09-04) into `Lectures\04.
  Lecture-4\Đã phát, chưa dạy\`; a sentence-problems PDF promoted into Core
  (09-04); and 2 Writing Task 1 books (09-05) into
  `00. Core\03. writing\materials\13. task1-report\`.
- **`13. task1-report` is a new topic folder** — the first added to Core writing
  since the architecture froze on 2026-08-10. Those 2 files are the only Task 1
  material in the entire library, and they exist **only in Core** — no source
  folder copy, no `IELTS Test` copy. Every other document has all three.
- **`CURRENT.md` is stale (last updated 2026-08-27) and now wrong**: it still
  reads `Writing Task 1 — DEFERRED — by decision; 0 materials`, and its Core count
  (431 files, 56 folders) predates the last three additions.

## Open items to raise with the user

- **Test 3's ten wrong answers** — question numbers, answers given, reasoning
  *before* the key. Still unreviewed; the only one of three tests with no
  mistake analysis.
- **Which paper `IELTS Reading Test 2` was** — the 33/40 has no date recorded.
- **The Q27-34 headings key** for Test 4 — the teacher's correction covers Q1-26
  and Q35-40 only; that marking is reasoning, not a key.
- **Speaking, Listening, Writing Task 1** — still largely unsourced. The largest
  risk to the overall target, because Reading is measurably ahead of them.
- **Midterm Test: Grammar for Writing** — Google Classroom, single attempt,
  timed, not yet taken. Prep card at
  `00. Core\02. error-catalogue\midterm-prep-card.md`.

Classes are **Thursday and Sunday, 19:30–22:00 Asia/Bangkok**, so material
arrives at least twice a week. Open every session by asking the user for anything
posted since the last checkpoint — you do not run between sessions and cannot
watch Drive on your own. Say that plainly rather than implying coverage you do
not have.

## Checkpoint 2026-09-06 — supersedes the state snapshot above

- Lecture 23 now exists with 25 recursively walked class-folder files: 13
  pre-class, 6 related Work/City files, and 6 released-not-taught reissues.
- No Lecture 23 teaching-notes subfolder existed at the checkpoint; re-walk
  after class.
- Daily Reading 48-50 and the new vocabulary/writing/sentence resources have
  been filed with pending trackers.
- The two Task 1 books now have all three roles: `Writing\21`, IELTS writing 13,
  and the existing Core `13. task1-report`. Writing Task 1 is no longer a
  class-material gap.
- Core now has 465 files total, including 422 material files (394 PDFs + 28
  MP3s), 1.63 GiB. The Core manifest includes the 23 new Core role copies.
- Women in the Workplace still has no distinct Exercise Pack URL: the post
  repeats the Language Companion link. The screenshot's original Work URLs are
  still unreadable, but the class folder supplied the complete four-file set.
- Lecture 22 has a late sixth annotated teaching note for the Global Food
  positive/negative lesson; preserve it as a variant.

Storage correction completed 2026-09-06: Core has only the established
`02. question-types` folder; the accidental `04. question-types` duplicate was
removed after hash verification. Core Task 1 filenames now match Writing 21,
IELTS writing 13, and Lecture 23. The corrected paths are in `core-manifest.csv`.

Later checkpoint on 2026-09-06: re-walking Lecture 23 found 28 current Drive
files (16 root, 4 Work, 2 City, 6 released). The local lecture has 29 role files
because the vanished 178,580-byte Task 1 edition was preserved beside its new
191,980-byte, 20-page replacement. Tenses, Two-part Direct Questions and a
children/smartphones scaffold were added. Daily Reading 51 was filed with all
five components. Core is now 474 files total, 431 PDF/MP3 materials, 1.64 GiB,
and 376 manifest rows. Zoom: `8912108989` / `TCB7`. No study completion was
inferred.
