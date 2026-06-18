# Academic English Agent Handoff

Last reconciled: 2026-06-18, Asia/Bangkok

## Start Here

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
   - `D:\OneDrive\01. IELTS\00.Academic English\Lectures\07. lecture-7\00. class-info\class-notes.md`
   - `D:\OneDrive\01. IELTS\00.Academic English\Lectures\07. lecture-7\00. class-info\source-post-index.md`
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
- Lecture 7 is scheduled for Thursday, 2026-06-18, `19:30-22:00`.
- Lecture 7 materials were synchronized on 2026-06-18 from:
  `https://drive.google.com/drive/folders/1-MZS3zwcnHx7zy1VXuR-UxToT10KNM6C`
- Lecture 7 Zoom ID and passcode were not included in the supplied posts.
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

Daily Reading packets currently run from `00` through `26`.

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
  - Class source index and class notes present.

## Known Completion State

- Reading 06: complete, 14/15.
- Reading 07: not started in its tracker.
- Reading 08: complete, 13/15.
- Reading 09: test complete, 13/15; language/questions review remains.
- Reading 10: queued.
- Reading 11: queued.
- Readings 12-17: files exist but do not yet have normalized tracking.
- Readings 18-26: normalized and tracked, but study remains unchecked unless the user later provides completion evidence.
- Lecture 1 Google Forms are complete.
- Lecture 1 PDF exercises still shown as pending:
  - `Clauses 9.1.pdf`
  - `Phrases 9.1.pdf`
  - `using sentence structure.pdf`

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

1. Before Lecture 7 on Thursday, 2026-06-18:
   - Check Facebook/Classroom for Zoom ID and passcode.
   - Review `Lectures\07. lecture-7\00. class-info\class-notes.md`.
   - Prioritize pronoun reference and case, parallel structure, punctuation, and Law and Crimes vocabulary.
2. Complete Homework - Lecture 6 forms and record scores:
   - Complement, Phrase, Clause, Sentence 2
   - Glossary of Usage Exercise 1
   - Pronoun Usage Exercise 2
3. Stay current on the newest readings:
   - Reading 26: El Nino 2026
   - Reading 25: No to GMO
   - Reading 24: mpox
   - Reading 23: gaokao
4. Ask the user for any Lecture 5 or Lecture 6 homework scores not yet recorded.
5. Normalize or audit Readings 12-17 only after tonight's Lecture 7 work is safe.
6. Only after the learning trackers are current, consider staging one narrow, user-reviewed vocabulary packet for AET. Do not bulk-import.

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
