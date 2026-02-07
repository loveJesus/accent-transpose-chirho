# For God so loved the world that he gave his only begotten Son, that whoever believes in him should not perish but have eternal life.

# accent-transpose-chirho

Cantillation accents from the **Mikra according to the Masora** (MapM) mapped onto the **Solid Rock Hebrew Bible**.

## What this does

The **Solid Rock Hebrew Bible (SRHB)** is a TEI XML critical edition with 2,500+ textual adjustments. It has consonants and vowel points but is missing cantillation marks.

**Mikra according to the Masora (MapM)** is a Hebrew Bible text that includes full cantillation marks along with consonants and vowel points.

This tool matches each word in Solid Rock to its MapM counterpart by position and vowel pattern, then transfers the cantillation marks from MapM onto the Solid Rock text.

### Results

| Status | Words | Percentage |
|--------|------:|----------:|
| Matched (cantillation applied) | 283,868 | 92.8% |
| Mismatch (text differs between codices) | 21,334 | 7.0% |
| Not found in MapM | 636 | 0.2% |
| **Total** | **305,838** | |

## Live viewer

**[solid-rock-accents-chirho.bible.systems](https://solid-rock-accents-chirho.bible.systems/)**

Browse the entire Hebrew Bible with color-coded cantillation status:
- **Green** — matched, cantillation successfully transposed
- **Orange** — text differs between the two editions
- **Red** — word not found in MapM

## PDF

**[bible_chirho.pdf](https://media-solid-rock-accents-chirho.bible.systems/bible_chirho.pdf)** — Full Hebrew Bible with color-coded cantillation marks, rendered in KeterYG font.

## Usage

Requires Rust (edition 2024) and a populated SQLite database.

```bash
# Import source texts
cargo run -- import-mapm-chirho
cargo run -- import-solid-rock-chirho

# Run cantillation transposition
cargo run -- transpose-chirho

# Export JSON (per-chapter files + manifest)
cargo run -- export-chirho

# Export PDF via Typst
cargo run -- export-pdf-chirho
```

### JSON output

`export-chirho` produces 930 per-chapter JSON files in `export-chirho/json-chirho/` plus a `manifest_chirho.json` with book/chapter stats. Each word includes the original Solid Rock text, the cantillated result, match status, and confidence score.

### PDF output

`export-pdf-chirho` generates a Typst source file (`export-chirho/bible_chirho.typ`) and compiles it to PDF using the `typst` CLI. Install Typst with `cargo install typst-cli`.

## Data sources

- **Solid Rock Hebrew Bible**: [github.com/loveJesus/solid-rock-hb](https://github.com/loveJesus/solid-rock-hb) (Git submodule at `solid-rock-hb/`)
- **MapM**: JSON export of Mikra according to the Masorah (`MapM/MapM.json`)

## Architecture

- **Rust** with rusqlite, serde_json, quick-xml, unicode-normalization
- **SQLite** database with 4 tables: `mapm_verses_chirho`, `mapm_words_chirho`, `solid_rock_words_chirho`, `transposed_words_chirho`
- **Unicode-aware** comparison: NFC normalization, kamatz katan → kamatz mapping, meteg stripping, maqaf word splitting
- **Static site**: pure HTML/CSS/JS with KeterYG Hebrew font, deployed to Cloudflare Pages
- **PDF**: Typst typesetting with RTL Hebrew support and color-coded status markers
