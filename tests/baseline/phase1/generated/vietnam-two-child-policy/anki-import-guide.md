# AET Anki Import Guide

Article: `vietnam-two-child-policy`

The TSV files contain note data only. The learning experience is controlled by the Anki note type templates in `anki-templates/`.

## Recommended Import Setup

Create these custom note types in Anki before importing:

1. `AET Basic Type Answer`
2. `AET Production Type Answer`
3. `AET Cloze Type Answer`

`AET Basic Type Answer` and `AET Production Type Answer` need these fields, in this order:

```text
Front
Back
Extra
Tags
```

When importing `anki-basic.tsv` or `anki-production.tsv`:

- Type: `AET Basic Type Answer` or `AET Production Type Answer`
- Field separator: Tab
- Field 1 -> `Front`
- Field 2 -> `Back`
- Field 3 -> `Extra`
- Field 4 -> `Tags`
- Tags column: field 4

Import `anki-cloze.tsv` with `AET Cloze Type Answer` if you want a typed-answer box, or Anki's built-in `Cloze` note type if you only want normal reveal cards:

`AET Cloze Type Answer` needs these fields, in this order:

```text
Text
Back Extra
Extra
Tags
```

- Field 1 -> `Text`
- Field 2 -> `Back Extra`
- Field 3 -> `Extra`
- Field 4 -> `Tags`

## AET Basic Type Answer

Use:

- Front: `anki-templates/basic-front.html`
- Back: `anki-templates/basic-back.html`
- Styling: `anki-templates/card-styling.css`

## AET Production Type Answer

Use:

- Front: `anki-templates/production-front.html`
- Back: `anki-templates/production-back.html`
- Styling: `anki-templates/card-styling.css`

## AET Cloze Type Answer

Clone Anki's built-in `Cloze` note type, then use:

- Front: `anki-templates/cloze-front.html`
- Back: `anki-templates/cloze-back.html`
- Styling: `anki-templates/card-styling.css`

## Learning Behavior

- Basic and production cards use an open response box. Your attempt is saved locally and shown on the back for self-comparison.
- Cloze cards use Anki's native typed-answer comparison with `{{type:cloze:Text}}`.
- Hints are hidden in disclosure panels so they do not steal the recall attempt.
- Final grading stays manual through Anki's Again/Hard/Good/Easy buttons.

## Multiple Choice

Anki does not have native multiple-choice grading for TSV imports. You can make a visual multiple-choice card with custom HTML/JavaScript, but it will not behave like Anki's typed-answer comparison. For AET v0.1, typed answer is the reliable option.
