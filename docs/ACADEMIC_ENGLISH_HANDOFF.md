# Academic English Agent Handoff

Last reconciled: 2026-08-10, Asia/Bangkok

## Start Here

For the step-by-step reliability protocol and the latest known checkpoint,
also read:

- `D:\Work\aet\docs\ACADEMIC_ENGLISH_OPERATIONS_HANDOFF.md`

This work has two separate layers:

1. `D:\OneDrive\01. IELTS\00.Academic English`
   - Private course source of truth.
   - Contains teacher PDFs, articles, audio, post-class notes, exercises, and personal completion trackers.
   - Do not commit any of this private material to Git.

2. `D:\Work\aet`
   - Deterministic Rust CLI and sanitized fixture/export workspace.
   - Read `AGENTS.md`, `DECISIONS.md`, and `WORKER_BRIEF.md` before changing code or importing data.
   - Do not bulk-import the OneDrive course tree.

## First Ten Minutes

1. Read:
   - `D:\Work\aet\AGENTS.md`
   - This handoff
   - `D:\OneDrive\01. IELTS\00.Academic English\CURRENT.md`
   - The class-info files for the lecture named in `CURRENT.md`
2. Scan the OneDrive tree read-only before creating, moving, or renaming anything.
3. Compare the newest local file date with the current date and the teacher posts supplied by the user.
4. Use `highlights.md`, `review-checklist.md`, or `class-notes.md` as visible proof. Do not infer that downloaded material is completed.
5. Verify every downloaded PDF or MP3 by file header before reporting success.

## Current Course State

- Live class cadence: Thursday and Sunday evenings, normally `19:30-22:00`.
- Lecture 5 was attended and completed on Thursday, 2026-06-11.
- Lecture 5 materials and post-class notes were synchronized on 2026-06-12.
- Lecture 6 was scheduled for Sunday, 2026-06-14, `19:30-22:00`.
- Lecture 6 materials were announced and synchronized on 2026-06-13 from:
  `https://drive.google.com/drive/folders/1DgXn1_XTHMxN4B3rVYzjpwkGQfTiccYO`
- Lecture 6 post-class teaching notes were synchronized on 2026-06-18 from:
  `https://drive.google.com/drive/folders/1hseY4ftGG9Fm9HN1MmsVBFMYtObSPDts`
- The local Lecture 6 packet contains the original eight prep PDFs plus eight verified teaching-note PDFs under:
  `D:\OneDrive\01. IELTS\00.Academic English\Lectures\06. lecture-6`
- Homework - Lecture 6 is tracked in:
  `D:\OneDrive\01. IELTS\00.Academic English\Lectures\06. lecture-6\00. class-info\homework-lecture-6.md`
- Lecture 7 was scheduled for Thursday, 2026-06-18, `19:30-22:00`.
- Lecture 7 materials were synchronized on 2026-06-18 from:
  `https://drive.google.com/drive/folders/1-MZS3zwcnHx7zy1VXuR-UxToT10KNM6C`
- Lecture 7 post-class teaching notes were synchronized on 2026-06-19 from:
  `https://drive.google.com/drive/folders/1Ry_50mVzepNlWZ7NSD-TJNwijZbje_31`
- Lecture 7 now includes six verified teaching-note PDFs under:
  `D:\OneDrive\01. IELTS\00.Academic English\Lectures\07. lecture-7\03. teaching-notes`
- Homework - Lecture 7 is tracked in:
  `D:\OneDrive\01. IELTS\00.Academic English\Lectures\07. lecture-7\00. class-info\homework-lecture-7.md`
- Lecture 8 was scheduled for Sunday, 2026-06-21, `19:30-22:00`.
- Lecture 8 materials were announced on 2026-06-20 14:23 from:
  `https://drive.google.com/drive/folders/18eojS6knftjijHVG_Y1JaeovdIgjqeVb`
- Lecture 8 Zoom details were posted on 2026-06-21 13:10:
  Meeting ID `8912108989`, passcode `NXT5`, secretary Trang `0363278899`.
- Lecture 8 post-class teaching notes were posted on 2026-06-21 22:15 from:
  `https://drive.google.com/drive/folders/1BfRQJbH99lMSN849lYRmP7vFOjA1Rifm`
- The local Lecture 8 packet contains verified pre-class materials and teaching-note PDFs under:
  `D:\OneDrive\01. IELTS\00.Academic English\Lectures\08. lecture-8`
- Homework - Lecture 8 is tracked in:
  `D:\OneDrive\01. IELTS\00.Academic English\Lectures\08. lecture-8\00. class-info\homework-lecture-8.md`
  It currently includes sentence problems, sentence variety, and environment part 1. Completion is not recorded.
- Lecture 9 was scheduled for Thursday, 2026-06-25, `19:30-22:00`.
- Lecture 9 materials were announced on 2026-06-24 16:37 from:
  `https://drive.google.com/drive/folders/1NsG4o0hhul4IO4apJUqDIF8kBGjlFHYk`
- Lecture 9 teaching notes were synchronized on 2026-06-25 from:
  `https://drive.google.com/drive/folders/1KfrQq2AF8i7eRgxsAkRXDLnZ4k-idIMl`
- The local Lecture 9 packet contains verified pre-class materials and teaching-note PDFs under:
  `D:\OneDrive\01. IELTS\00.Academic English\Lectures\09. lecture-9`
- Lecture 10 was scheduled for Sunday, 2026-06-28, `19:30-22:00`.
- Lecture 10 materials were announced on 2026-06-27 19:12 from:
  `https://drive.google.com/drive/folders/1_foWGOaHNitL5q0PUOl3YHbWG8BnyeDM`
- Lecture 10 teaching notes were posted on 2026-06-28 22:11 from:
  `https://drive.google.com/drive/folders/14W4dqhBqdIDFJ0Bbsg3kYxNzFtRtiody`
- The local Lecture 10 packet contains verified pre-class materials, teaching-note PDFs, homework PDFs, and class-info trackers under:
  `D:\OneDrive\01. IELTS\00.Academic English\Lectures\10. lecture-10`
- Homework - Lecture 10 is tracked in:
  `D:\OneDrive\01. IELTS\00.Academic English\Lectures\10. lecture-10\00. class-info\homework-lecture-10.md`
- Lecture 11 was scheduled for Thursday, 2026-07-02, `19:30-22:00`.
- Lecture 11 materials were announced on 2026-07-01 15:19 from:
  `https://drive.google.com/drive/folders/1O2hcGPGZdwMMUHWjIwhsJdAGtFreY5pm`
- Lecture 11 teaching notes were posted from:
  `https://drive.google.com/drive/folders/18wO8omX3MZLi4OYrSv1Ty-r9eOTZQMBE`
- The local Lecture 11 packet contains two verified prep PDFs and nine verified teaching-note PDFs under:
  `D:\OneDrive\01. IELTS\00.Academic English\Lectures\11. lecture-11`
- Lecture 12 was scheduled for Sunday, 2026-07-05, `19:30-22:00`.
- Lecture 12 materials were announced on 2026-07-04 17:44 from:
  `https://drive.google.com/drive/folders/1l26ghT6BW-iN7CAHYJcsCccDYVdNTQpc`
- Lecture 12 teaching notes were posted on 2026-07-05 22:09 from:
  `https://drive.google.com/drive/folders/16RmxNq8rcEs7C0_5VVJ9e7YjKx34p9Y4`
- The local Lecture 12 packet contains three verified prep PDFs, nine verified teaching-note PDFs, and a three-form homework tracker under:
  `D:\OneDrive\01. IELTS\00.Academic English\Lectures\12. lecture-12`
- Lecture 5 post-class notes Drive folder:
  `https://drive.google.com/drive/folders/1Q0LRaT5POlSwzo3rUtHJUGwfjN0-XnaW`
- The local Lecture 5 post-class set contains 11 verified PDFs under:
  `D:\OneDrive\01. IELTS\00.Academic English\Lectures\05. lecture-5\01. teaching-notes`
- Lecture 4 teaching notes contain seven verified PDFs. They were already local and were not duplicated.

## Latest Added Packets

### Daily Reading

- `18.Causes of Cancer`
  - Teacher posted: 2026-06-08 14:50
  - Has article, audio, teacher vocabulary, vocabulary exercises, reading quiz, and `highlights.md`.
- `19.Cities used to sprawl now they are growing taller`
  - Teacher posted: 2026-06-10 06:54
  - Has article, teacher vocabulary, exercises, quiz, and `highlights.md`.
  - No audio was provided.
- `20.Business Steve Jobs Innovator and CEO`
  - Teacher posted: approximately 2026-06-11 23:11, inferred from the Facebook relative timestamp captured on June 12.
  - Has article/exercises, teacher vocabulary, and `highlights.md`.
  - First half of a two-reading class assignment and still pending.
- `21.Will the Next Steve Jobs Be From China`
  - Same approximately 2026-06-11 23:11 teacher post.
  - Has article, teacher vocabulary, original IELTS reading with answers, reading quiz, vocabulary exercises, and `highlights.md`.
  - Linked to Reading 20; the teacher instructed students to skim first and guess vocabulary from context before consulting the vocabulary analysis.
- `22.Why eldest siblings are brainier`
  - Teacher posted: 2026-06-13; exact time not captured.
  - Has article, teacher vocabulary with 46 entries, vocabulary exercises with answers, IELTS reading quiz with answers, and `highlights.md`.
  - No audio was provided.
- `23.Chinas gaokao is changing`
  - Teacher posted: 2026-06-14 08:12.
  - Has article, teacher vocabulary, vocabulary exercises, IELTS reading with answers, and `highlights.md`.
  - No audio was provided.
- `24.A deadly new strain of mpox is raising alarm`
  - Teacher posted: 2026-06-14 15:10.
  - Has article, audio, teacher vocabulary, vocabulary exercises, and `highlights.md`.
  - A post-class annotated copy also appears in Lecture 6 teaching notes.
- `25.No to GMO`
  - Teacher posted: captured on 2026-06-18 as a relative Facebook timestamp; exact post time not recovered.
  - Has article, audio, teacher vocabulary, vocabulary exercises, vocabulary exercise key, and `highlights.md`.
- `26.El Nino 2026`
  - Teacher posted: captured on 2026-06-18 as a relative Facebook timestamp; exact post time not recovered.
  - Has article, bilingual vocabulary with IPA, vocabulary exercises, IELTS reading practice, and `highlights.md`.
  - No audio was provided.
- `27.Hungry and angry`
  - Teacher posted: 2026-06-21 10:23.
  - Has article, audio, teacher vocabulary, vocabulary exercises with key, IELTS reading practice, and `highlights.md`.
- `28.After 44 Years on Death Row`
  - Teacher posted: 2026-06-21 17:36.
  - Has article, teacher vocabulary, vocabulary exercise, IELTS reading practice, and `highlights.md`.
  - No audio was provided.
- `29.Starving Children Dont Cry`
  - Teacher posted: captured on 2026-06-22 as a relative Facebook timestamp; exact post time not recovered.
  - Has article, teacher vocabulary, vocabulary exercise, IELTS reading practice, and `highlights.md`.
  - No audio was provided.

Daily Reading packets currently run from `00` through `29`.

New reading workflow note:

- `D:\OneDrive\01. IELTS\00.Academic English\Daily Reading\READING_WORKFLOW.md`
  - Use for the newer daily-reading format where each packet can contain article, audio, vocabulary analysis, vocabulary exercise, and IELTS reading practice.

- Reading 14: These Chinese Millennials Are Chilling
  - Completion remains user-confirmed.
  - The teacher's vocabulary exercise answer key was added and verified on 2026-06-13.

### Other Study Areas

- `Vocabulary\03. health-medicine`
  - Health and Medicine, part 1
  - Illness vocabulary in context
  - Review tracker present.
- `Vocabulary\04. negative-prefixes`
  - Review tracker present.
- `Writing\07. sentence-problems`
  - Practices 12-15: fragments, choppy sentences, run-ons, and comma splices.
  - Completion is not recorded.
- `Lectures\05. lecture-5\02. agreement`
  - Agreement reference and exercises.
  - Review tracker present.
- `Lectures\05. lecture-5\03. sentence-variety`
  - Lesson and review tracker present.
- `Lectures\06. lecture-6\00. class-info\homework-lecture-6.md`
  - Three Google Classroom form links recorded; completion not recorded.
- `Lectures\07. lecture-7`
  - Writing mechanics: pronoun reference and case, parallel structure, comma, semicolon, colon.
  - Vocabulary: Law and Crimes part 2; Crime / Legal System / Prison.
  - Law & Crimes review PDFs added on 2026-06-19.
  - Post-class teaching notes, source index, class notes, and homework tracker present.
- `Lectures\08. lecture-8`
  - Pre-class materials, Zoom/class info, source index, teaching notes, and class notes are present.
  - The teaching notes include Law & Crimes review, sentence problems, sentence variety, adverb clause, transition signals, and the Hakamada death-row reading materials.
- `Lectures\09. lecture-9`
  - Pre-class materials, source index, class notes, and teaching notes are present.
  - Focus areas: capitalization, environment vocabulary part 1, IELTS Writing Task 2 opinion essay, academic essay, and proofreading.
- `Lectures\10. lecture-10`
  - Pre-class politics and argumentative-essay materials, source index, class notes, teaching notes, and homework tracker are present.
  - Focus areas: politics vocabulary/readings, argumentative essay, comma, semicolon, colon, Agreement GfW3, and sentence combining.
- `Vocabulary\05. environment`
  - Environment part 1 word bank and exercises.
  - Review tracker present.
- `Vocabulary\06. word-formation`
  - Word Formation Vocabulary Pack added from the 2026-06-27 teacher post.
  - Review tracker present.
- `Vocabulary\07. euphemism`
  - Euphemism lesson added from the 2026-06-28 teacher post.
  - Review tracker present.
- `Vocabulary\08. politics`
  - Politics C1-C2 vocabulary and exercise added from the 2026-06-28 teacher post.
  - Review tracker present.
- `Vocabulary\09. difficult-verb-pairs`
  - Difficult Verb Pairs lesson added from the 2026-06-29 teacher post.
  - Review tracker present.
- `Writing\08. ielts-writing-task-2`
  - Opinion essay document added on 2026-06-25.
  - Review tracker present.
- `Writing\09. argumentative-essay`
  - Argumentative Essay 1 document added from the 2026-06-27 teacher post.
  - Review tracker present.
- `Writing\10. cliches`
  - Cliches complete lesson added from the 2026-06-29 teacher post.
  - Review tracker present.
- `Vocabulary\10. foreign-words-and-phrases`
  - Foreign Words and Phrases lesson added from the 2026-07-01 teacher post.
  - Review tracker present.
- `Writing\11. academic-writing-checklists`
  - 35 Mistakes to Avoid in Academic Writing added from the 2026-07-04 teacher post.
  - Review tracker present.
- `Writing\03.Capitalization`
  - Older capitalization lesson remains.
  - New capitalization PDF from 2026-06-24 added as `capitalization-2026-06-24.pdf`.
  - Review tracker updated.
- `IELTS Test`
  - Separate IELTS-test tracking folder added on 2026-06-26.
  - Contains skill trackers for reading, listening, writing, speaking, vocabulary/grammar, and scores.
  - Primary review materials are copied into the real skill folders:
    - `01. reading\materials`
    - `03. writing\materials`
    - `05. vocabulary-grammar\materials`
  - IELTS Reading materials are grouped by practice folder, for example `21. ...\article.pdf` and `21. ...\quiz.pdf`.
  - IELTS Writing and support materials are also grouped by topic/document folder.
  - Tracking convention is recorded in `IELTS Test\00. dashboard\data-tracking-map.md`.
  - `99. staging-copy-original-sync` keeps the first-pass staging copy as an archive; daily review should use the active skill folders.
  - Originals remain in their source course folders; do not move or delete them.
  - On 2026-07-01, new copies were added for Argumentative Essay 1, Cliches, Word Formation, Euphemism, Politics C1-C2, and Difficult Verb Pairs.
- `CURRENT.md`
  - Top-level steering file added on 2026-06-26.
  - Open this first to see the current priorities, next three actions, active folders, and done signals.

## Known Completion State

- Reading 06: complete, 14/15.
- Reading 07: not started in its tracker.
- Reading 08: complete, 13/15.
- Reading 09: test complete, 13/15; language/questions review remains.
- Reading 10: queued.
- Reading 11: queued.
- Readings 12-17: files exist but do not yet have normalized tracking.
- Readings 18-29: normalized and tracked, but study remains unchecked unless the user later provides completion evidence.
- Lecture 1 Google Forms are complete.
- Lecture 1 PDF exercises still shown as pending:
  - `Clauses 9.1.pdf`
  - `Phrases 9.1.pdf`
  - `using sentence structure.pdf`
- Homework - Lecture 7:
  - Law Vocabulary part 1 complete on 2026-06-22 with score `33/37`.
  - Remaining Lecture 7 homework items are still pending unless the user later provides completion evidence.
- Homework - Lecture 8:
  - All items are pending unless the user later provides completion evidence:
    - Sentence problems exercise
    - Sentence variety exercise
    - Environment vocabulary part 1 word bank
    - Environment vocabulary part 1 exercises
- Homework - Lecture 10:
  - All items are pending unless the user later provides completion evidence:
    - Agreement Exercise 3 Google Form
    - Law Vocabulary part 2 Google Form
    - Comma exercise
    - Semicolon and colon exercise
    - Agreement GfW3
    - Sentence combining using coordination and subordination
- Homework - Lecture 12:
  - All items are pending unless the user later provides completion evidence:
    - Capitalization Exercise 2 Google Form
    - Punctuation 1 Exercise 2 Google Form
    - Modifier Usage Exercise 2 Google Form
- Homework - Lecture 11:
  - No Google Classroom homework post was supplied. Do not infer a homework list.

Do not mark any unchecked item complete without user-provided evidence such as a score, submitted result, or explicit completion statement.

## Folder and Naming Rules

Daily Reading working packets should prefer:

- `article.pdf`
- `audio.mp3`
- `teacher-vocab.pdf`
- `highlights.md`

Additional exercises may use clear stable names such as:

- `reading-quiz.pdf`
- `vocabulary-exercises.pdf`

Lecture folders should keep:

- `00. class-info\class-notes.md`
- `00. class-info\source-post-index.md`
- teacher post-class PDFs in `01. teaching-notes`

Keep teacher-post date and local-added date separate. If Facebook only provides a relative time and the exact timestamp cannot be recovered, say so explicitly.

## Important Safety Rules

- OneDrive is private raw material. Never commit teacher PDFs, article text, corrections, or audio.
- Avoid deleting duplicate lecture files without explicit user approval. Some duplicates preserve the distinction between taught and distributed-but-not-taught material.
- Do not rename or overwrite user-marked article PDFs. If markup exists, prefer `*.markup.pdf` beside the clean source.
- Do not fabricate quiz scores, post dates, IPA, homework status, or what was actually taught.
- Use Google Drive metadata/listing before downloading.
- After downloading, validate `%PDF` for PDFs and `ID3`/MP3 frame signatures for audio.

## Recommended Next Actions

1. Review Lecture 12 and complete one form:
   - Commas, usage glossary, and correcting problems with modifiers
   - Capitalization and US-government teaching notes
   - Capitalization Exercise 2, Punctuation 1 Exercise 2, or Modifier Usage Exercise 2
2. Review Lecture 11 IELTS Reading Test 1:
   - Complete at least the first two readings without a dictionary.
   - Record score or mistakes when the work is finished.
3. Keep `CURRENT.md` updated after each study session:
   - Current priority
   - Next 3 actions
   - Done signals and score/mistake updates
4. Use `IELTS Test` for IELTS-specific practice:
   - Reading practice copies under `IELTS Test\01. reading\materials`
   - Task 2 writing copies under `IELTS Test\03. writing\materials`
   - Support materials under `IELTS Test\05. vocabulary-grammar\materials`
5. Complete Homework - Lecture 10 and record scores/evidence:
   - Agreement Exercise 3
   - Law Vocabulary part 2
   - Comma exercise
   - Semicolon and colon exercise
   - Agreement GfW3
   - Sentence combining using coordination and subordination
6. Complete Homework - Lecture 8 and record scores/evidence:
   - Sentence problems exercise
   - Sentence variety exercise
   - Environment vocabulary part 1 word bank and exercises
7. Complete remaining Homework - Lecture 7 and record scores/evidence:
   - Sentence problems practices
   - Crime / prison / legal system vocabulary in context
   - Law & Crimes review, parts 1 and 2
8. Study the newest daily readings using `Daily Reading\READING_WORKFLOW.md`:
   - Reading 29: Starving Children Dont Cry
   - Reading 28: After 44 Years on Death Row
   - Reading 27: Hungry and angry
9. Review Lecture 8 post-class teaching notes and mark covered/confusing points in class notes.
10. Review Lecture 7 post-class teaching notes:
   - Sentence structure
   - Sentence problems
   - Complement, phrase, clause, sentence exercise 2
   - Introduction to essay
   - Law and Crimes part 2
11. Complete Homework - Lecture 6 forms and record scores:
   - Complement, Phrase, Clause, Sentence 2
   - Glossary of Usage Exercise 1
   - Pronoun Usage Exercise 2
12. Ask the user for any Lecture 5, Lecture 6, Lecture 7, Lecture 8, Lecture 9, or Lecture 10 homework scores not yet recorded.
13. Normalize or audit Readings 12-17 only after current Lecture 8/9/10 work is safe.
14. Only after the learning trackers are current, consider staging one narrow, user-reviewed vocabulary packet for AET. Do not bulk-import.

## Useful Read-Only Scan

Run from PowerShell:

```powershell
$root = 'D:\OneDrive\01. IELTS\00.Academic English'
Get-ChildItem -LiteralPath $root -Recurse -File |
  Sort-Object LastWriteTime -Descending |
  Select-Object -First 50 LastWriteTime, Length,
    @{Name='RelativePath'; Expression={
      [IO.Path]::GetRelativePath($root, $_.FullName)
    }}
```

The user values visible, auditable completion over chat-only confirmation. Update the relevant tracker and then summarize what changed.

## Latest private-tree sync — 2026-08-06

- Lecture 17 has three known standalone pre-class PDFs and six teaching-note PDFs copied from the exact Drive folder listing. The captured class-materials folder ID beginning `1XGhOwebfGQq10VYpjOXY5yO4NZS` is truncated and remains unresolved.
- Daily Reading 42 (`Gen Z Can't Afford a First Date`) and 43 (`Too Many Master's Courses`) are locally available with their exact article/question/vocabulary/exercise files; Reading 43 also has the supplied MP3.
- Health - Medicine Vocabulary 1, International Trade, and Academic Word Choice have canonical skill copies and IELTS review copies.
- Every new item remains pending. No teacher PDF, article, audio, correction, or score is tracked in Git.
- Detailed source IDs, sizes, and destinations are in `D:\OneDrive\01. IELTS\00.Academic English\Lectures\17. lecture-17\00. class-info\source-post-index.md` and the operations handoff.

## Completion evidence — 2026-08-09

- IELTS Reading Test 3 was completed by the user and corrected by the teacher in
  the Sunday class on 2026-08-09. This is user-confirmed and is the first
  completion evidence recorded since the 2026-08-06 sync.
- The teacher-corrected copy is `Reading\03.IELTS Reading Test 3\quiz tu.pdf`
  (2,912,953 bytes). An earlier working copy is
  `IELTS Test\01. reading\materials\IELTS Reading Test 3\quiz.pdf`
  (2,666,840 bytes, 2026-08-08). Both are user-modified; the clean 1,527,755-byte
  teacher source is intact in the Lecture 17 pre-class folder.
- Size divergence between a working copy and its canonical source is expected
  here and is not a sync fault. Do not "repair" a larger annotated copy back to
  the clean source.
- Score and wrong-answer types are still unrecorded. The reading tracker and
  score log carry the event with the score fields explicitly empty.
- No lecture folder, teacher post, or teaching-note link has been supplied for
  the 2026-08-09 class.

## Latest private-tree sync — 2026-08-10

- Ten exact Drive files (9 PDFs, 1 MP3) reconciled from metadata, downloaded,
  verified by byte size and `%PDF`/`ID3` header, and copied to 21 targets with
  SHA-256 equality. Nothing was overwritten and no original was moved.
- Daily Reading 44 `Why ever fewer young Chinese study abroad` added with
  article, IELTS questions, vocabulary table, exercise pack, and audio.
- Health & Medicine Vocabulary Reference (C1-C2) and its no-key exercises added
  from the 2026-08-07 10:19 post. Distinct from the 8795851-byte
  `Health - Medicine Vocabulary 1.pdf` synced on 2026-08-06.
- Lecture 17 Google Classroom homework recorded for the first time: Assessment
  Set A Google Form, the health & medicine collocations pair, and IELTS Reading
  Test 3.
- Lecture 18 created for the 2026-08-09 class and then completed the same day:
  both Drive folders were listed via the public folder page, every recovered ID
  was re-verified through Drive metadata including its parent, and 8 further
  exact files were copied — 3 class-materials PDFs into `01. pre-class-materials`
  and 5 teaching-note PDFs into `02. teaching-notes`.
- Lecture 18 focus: IELTS Task 2 question types (opinion vs discussion,
  advantages/disadvantages vs outweigh), discussion and opinion essay structure,
  and the Test 3 correction. New skill folder
  `Writing\16. ielts-task2-question-types` with an IELTS writing copy.
- The teacher's own marked-up Test 3 PDFs (in-class timestamps 19:36 and 20:40,
  1731068 and 2483936 bytes) are in the Lecture 18 teaching notes, beside the
  untouched 1527755-byte clean source.
- Totals for the day: 18 exact Drive files, 33 verified copy targets, 0 size,
  header, or hash failures.
- The Lecture 17 class-materials folder was resolved the same day from the full
  link `1XGhOwebfGQq10VYpjOXY5yO4NZS4nZzb`. Four files: three byte-identical to
  material already local, plus one genuinely new 96332-byte sentence-structure
  handout. **The long-standing Lecture 17 gap is closed.**

## IELTS back-catalogue reconciliation — 2026-08-10

The user's stated goal is **IELTS 7.5**, and `IELTS Test` is the command centre
for it. Every IELTS-relevant document in the course tree was audited and given a
review copy:

- `01. reading\materials`: 26 to 48 folders. 7 older Daily Reading packets have
  article + teacher questions; 15 are article-only and are marked as such so
  they are never scored as practice tests.
- `05. vocabulary-grammar\materials`: 35 to 42 folders — poverty/global issues,
  history, rain and storms, negative prefixes, adverb/relative/noun clauses,
  coordination and subordination, parallel structure, sentence problems.
- `03. writing\materials`: 11 to 12 folders — academic essay added.
- 43 files copied, every one SHA-256 verified, 0 header failures. No source file
  was moved, renamed, or deleted, and no `*.markup.pdf` was touched.

Deliberate exclusions, both recorded in the reading tracker: Daily Reading 04
(article byte-identical to Reading 16, packet holds user markup) and
`Reading\01. reading test` (empty folder).

One open question remains: two differently sized documents both named
`IELTS Reading Test 2` (3773202 and 5295430 bytes) — which is current.

## `00. Core` knowledge layer — 2026-08-10

A new top-level folder in the private tree holds the user's study system. On the
user's explicit instruction (2026-08-10) it is **self-contained and organised by
topic**: `<section>\materials\<topic>\` holds every *distinct* teacher document on
that topic — lesson, teaching note and exercises together. **301 files, 1.4 GB,
49 topic folders**, all SHA-256 and header verified; canonical originals were not
moved or deleted.

Two rules were set by the user and overrode earlier drafts:

1. **Copy, don't just reference.** An earlier version linked to PDFs by path; the
   user wants the material visible on opening the folder.
2. **Multiple sources per topic, not one best source.** The user's reasoning:
   "we can cross check between document, we just can learn from 1 source" — a
   structure cannot be verified against a single explanation. So the first
   version's one-document-per-concept selection was replaced with
   all-distinct-documents-per-topic.

3. **Hash dedupe is not enough — corrected the same day.** The first build kept
   every file with a distinct SHA-256, which left the *same lecture* five times:
   scans and annotated re-saves differ by a few kilobytes and all pass a hash
   check. The user caught it. Correct rule: within a topic folder, files whose
   sizes are within **2%** *and* whose normalised names match are one document —
   keep one clean copy plus the latest annotated copy, drop the rest.
   Removed 303 MB, no distinct document lost. Core went 301 files / 1.4 GB to
   **280 files / 1.1 GB**.

   Three guards, all learned from real failures in the dry runs:

   - **Size alone is unsafe.** `Misplaced Modifiers G7.pdf` (303,924) and
     `Modifiers 9.1.pdf` (307,584) are different documents 1.2% apart.
   - **Do not strip trailing digits when normalising names**, or
     `Agreement Exercise 1` and `Exercise 2` merge into one.
   - **PowerShell variables are case-insensitive.** A loop variable `$c`
     silently overwrote the `$C` root path, so three whole sections scanned a
     garbage path while the script reported success. Never use single-letter
     variables in these scripts.

   Always dry-run and read the deletion list before deleting. Every removal is
   recoverable from the canonical originals, which are never touched.

Listening: 21 packet folders, each with the recording plus its article,
IELTS questions, teacher vocabulary, exercises and `highlights.md`. Structure: `README.md` (the 20%
map), `01. sentence-structure\ladder.md` (rungs R1-R10), `02. error-catalogue`
(15 named errors E01-E15, reading question-type codes, and a five-pass
self-assessment), `03. writing\essay-playbook.md`, `04. vocabulary\method.md`,
and an intentionally empty `05. my-writing\`.

**Taxonomy frozen 2026-08-10, after review.** The first draft used a flat
`E01-E15`; it was replaced before any error was logged with domain prefixes:
`Sxx` language/sentence, `Wxx` writing discourse, `Rxx` reading, `Lxx` listening,
`SPxx` speaking-performance only, `Vxx` lexical. **Do not renumber.**

The load-bearing rule: **error type and skill are separate axes.** A subject-verb
slip while speaking is `S04` with `Skill: Speaking` — never a duplicate `SP04`.
`SPxx` holds only what cannot exist outside speaking (hesitation, under-developed
answers, self-correction, discourse organisation, intelligibility). This is what
lets the log show that one weakness crosses skills.

One fact, one home: error detail lives only in
`00. Core\06. progress\error-log.md`; raw scores and time only in
`IELTS Test\06. scores\score-log.md`; queues and material indexes only in the
IELTS trackers. The reading and writing trackers' own log sections were replaced
with pointers to avoid two homes for one fact.

Two findings drove the design and should not be lost: the library holds 1,093
files of teacher input and almost no user output (two worked reading tests, no
essay drafts, no speaking recordings), and the AET vocabulary-to-Anki pipeline in
this repo has been idle since 2026-05-19 with only two articles while dozens of
vocabulary packets accumulated.

### IELTS coverage — a curriculum gap, not an organisation gap

Verified 2026-08-10 against the tree:

| Component | Status | Materials |
|---|---|---:|
| Reading | ACTIVE | 81 |
| Writing Task 2 | ACTIVE | 15 |
| Grammar / sentence structure | ACTIVE | substantial |
| Writing Task 1 | **UNSOURCED** | 0 |
| Listening | **UNSOURCED** | 2 tracker stubs |
| Speaking | **UNSOURCED** | 2 tracker stubs |

Every "chart/graph" filename in the tree is `Chart of Transitional Signals`, a
cohesion handout — there is genuinely no Task 1 material. Since 7.5 is an
**overall** band, three of four scored components currently have no path to it.
The next action is a question to the teacher, not a folder change. Do not let
these rows drop out of the plan.

## First recorded scores — 2026-08-10

- IELTS Reading Test 2: **33/40** (test date not supplied).
- IELTS Reading Test 3: **30/40**, corrected in class 2026-08-09.

Both user-confirmed. Indicative Academic Reading bands are 7.5 and 7.0 against a
7.5 target, so 33+/40 is the working threshold. `test-2-tu.pdf` is therefore
confirmed as a completed test, which retires the earlier open question about it.

Neither test has mistake analysis yet, and that is the gap that matters now: a
raw score does not show which question types are losing marks. The Wrong Answer
Log in the reading tracker is still empty.

## Full Lectures 1-16 Drive audit — 2026-08-10

The user supplied all 16 lecture folder links. Every folder was walked
recursively: **118 folders, 790 files**. 785 were already present locally. Five
were missing and are now filed and verified:

| File | Bytes | Filed to |
|---|---:|---|
| `Clauses & Sentence G8.0.pdf` | 197602 | Lecture 2 phrase-and-clause materials |
| `From_Crime_to_Verdict_Law_Reading.pdf` | 86262 | Lecture 6 law vocabulary; new `04. released-not-taught` in Lectures 7 and 8 |
| `Complement- Phrase- Clause- Sentence Exercise 2.pdf` | 351096 | Lecture 7 writing mechanics |
| `Hakamada_Vocabulary_Exercise_Pack.pdf` | 126264 | Lecture 8 pre-class + IELTS `L08. Hakamada Death Row`. A **second version**; the 123559-byte copy already local was kept |
| `full_Forms_of_Money__Lesson.pdf` | 152962 | Lectures 14 and 15, `Vocabulary\18. forms-of-money`, IELTS support 25. Distinct from the 140924-byte `lesson.pdf` already there |

11 copy targets, 0 size, header, or hash failures.

Lecture 1's Drive folder contains only subfolders (Poverty & global issue
vocabulary, Pre-Learning Assessment, Proofreading, Reading, Sentence Structure,
Teaching notes) — this is why a files-only scan reported it as empty, and why
the walk must recurse.

## Latest private-tree sync — 2026-08-23

**Lecture 21 (Thursday 2026-08-20) — the first intake with zero unresolved links.**

All 15 supplied Drive IDs were full. 26 files downloaded, all verified by exact
byte size and `%PDF-` header (26/26). 103 copy targets in total — 71 from the
staged downloads, 32 sourced from the local library — every one SHA-256 verified.
**0 conflicts, 0 overwrites, 0 header failures.**

### What Lecture 21 was

A **correction class**, not a new-content class. All nine teaching-note files are
timestamped 2026-08-20 22:13 — minutes after class — and every one is an annotated
copy of a grammar-for-writing exercise that had been pending for months:

| Annotated | Bytes | Clean copy held | Growth |
|---|---:|---:|---:|
| Correcting Common Errors Exercise 1 | 201,534 | 93,306 | +116% |
| Using Modifiers Correctly Exercise 1 | 188,874 | 92,188 | +105% |
| Glossary Of Usage Exercise 1 | 150,236 | 91,807 | +64% |
| Capitalization Exercise 2 | 158,537 | 101,111 | +57% |
| Agreement Exercise 3 | 118,803 | 77,645 | +53% |
| Punctuation 1 Exercise 2 | 139,102 | 95,036 | +46% |
| Agreement Exercise 2 | 129,929 | 90,952 | +43% |
| comma 2 | 13,589,876 | 13,578,047 | +0.1% |
| Capitalization | 241,397 | — | new document |

**Evidence boundary, stated explicitly:** these prove the teacher corrected the
exercises in class. They do **not** prove the user attempted or submitted them.
The Lecture 5/6/10/12/13 homework trackers stay open pending user confirmation.

### New homework — a live deadline

**Midterm Test: Grammar for Writing**, Google Classroom, hosted on
`extendedforms.io`, **single attempt, timed inside the form**. No due date was
stated. The nine annotated files are the revision set. Also: **basic editing
practice**, answers to be posted as a post comment, corrected in class.

### Daily Reading 46 — a closed gap

The four DR46 links from the 2026-08-14 09:07 post were truncated and had been an
open gap since the Lecture 20 checkpoint. The teacher re-published the packet as a
**subfolder of the Lecture 21 class folder**, found by the recursive walk.

Recorded precisely: the new IDs do **not** match the truncated prefixes. These are
the same documents as **new Drive objects**. The original four objects remain
unrecoverable; the content is no longer missing.

### Daily Reading 47

`Does AI stop children from learning` — article, IELTS questions, vocabulary
reference, exercise pack. No audio. Teacher's framing: AI-assisted homework scores
18% higher, exam performance 20% lower.

### Other new material

`Sentence\07. sentence-variety`, `Writing\17. basic-editing-practice`,
`Writing\18. positive-negative-development-essay`,
`Writing\19. two-part-question-essay`, `Vocabulary\25. money-and-banking`, plus
two capitalization documents and the special sentence structure challenge. Each
has a review checklist. All pending.

**Positive/negative development is a sixth Task 2 type.** With Lecture 19's
two-part question, Core's playbook text saying "four Task 2 types" is now stale.

**Money & Banking is the first material in months that touches Speaking or
Listening** — the teacher framed it as Speaking Part 1 / Listening Section 1
vocabulary. It is a vocabulary pack, not a course. The question to the teacher
still needs asking.

### Core

**411 to 429 files, 1.5 to 1.6 GB.** 18 additions across `S00`, `S04`, `S09`,
`S10-S11`, `V01`, `SL6`, `SL7`, `02. question-types`, `08. proofreading`, and a
new `D-topic-vocabulary\money-and-banking`.

## Latest private-tree sync — 2026-09-06

Lecture 23 is now staged at
`D:\OneDrive\01. IELTS\00.Academic English\Lectures\23. lecture-23`.
The class folder `16jxZVo0GE6UuLEl3M040IzvjP6vblf-D` was walked recursively:
13 root files, 4 Work files, 2 City files, and 6 released-not-taught reissues.
All **25 files** are represented locally. No teaching-notes subfolder existed at
the checkpoint, so re-walk after class before calling the lecture complete.

The wider intake added Daily Reading 48-50; Urban/Rural, Peoples Characters,
Body Verbs, Football, Work, and City vocabulary; a newer Cliches lesson;
Sentence Structure and Word Order; Essay Organization; and the two IELTS
Writing Task 1 books. **99 role copies** were created with source/destination
SHA-256 verification and no overwrites.

Two source discrepancies remain explicit:

- Women in the Workplace Step 4 repeats the exact Step 3 Language Companion URL;
  no separate exercise file is claimed.
- The screenshot's four Work URLs were too small to transcribe safely. Guessed
  IDs were rejected; the class-folder Work subfolder supplied the verified
  two-PDF/two-audio set instead.

Lecture 22's Teaching notes folder gained a late sixth file:
`@PosNeg_Lesson_Global_Food.pdf` (1,833,569 bytes, Drive ID
`1XW-LbEn4jamiD7O789NWY9UlHy_cpl2f`). It is a distinct annotated variant and is
preserved separately in Lecture 22, Writing 18, and Core question types.

`CURRENT.md`, class provenance, packet/skill trackers, IELTS trackers, Core
README, and Core manifest were updated. Everything newly filed remains pending;
download or copy is never completion evidence. Core now has **465 files total**,
including 422 material files (394 PDFs + 28 MP3s), 1.63 GiB.

Storage correction on the same 2026-09-06 checkpoint: the late Global Food
annotation was consolidated from the accidentally created Core
`04. question-types` into the established `02. question-types`, and the empty
duplicate folder was removed. The two Core Task 1 copies were renamed to match
Writing 21, IELTS writing 13, and Lecture 23. Hashes and PDF headers were
unchanged; source copies outside Core were untouched. The Core manifest records
the corrected path and both Task 1 files.

Later on 2026-09-06, the Lecture 23 folder was re-walked after four new root
files appeared and the earlier 178,580-byte Task 1 object disappeared. The
current Drive snapshot is 28 files (16 root, 4 Work, 2 City, 6 released), while
the local lecture deliberately holds 29 role files because both the old 16-page
and new 20-page Task 1 editions are preserved. The other additions are an
18-page Tenses book (two byte-identical Drive objects), a 6-page Two-part Direct
Questions overview, and a 9-page children/smartphones scaffold. Zoom is
`8912108989` / `TCB7`.

Daily Reading 51, “In Many Countries the Biggest Barrier to Prosperity Is in
the Home”, is complete as supplied: article, IELTS questions, vocabulary
companion, exercises, and audio. It is filed in Daily Reading, IELTS Reading,
and Core listening. The later batch created 28 verified role copies without
overwrites. Core now has **474 files total**, including 431 PDF/MP3 materials
(402 PDFs + 29 MP3s), 1.64 GiB, and **376 manifest rows**. All study states remain
pending.
