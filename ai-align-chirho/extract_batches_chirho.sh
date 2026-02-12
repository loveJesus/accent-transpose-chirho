#!/bin/bash
# For God so loved the world that he gave his only begotten Son,
# that whoever believes in him should not perish but have eternal life.

# Extract mismatch words with full verse context into batch files for AI alignment.
# Each batch file contains ~10 mismatch words with their SR and MapM verse contexts.

set -euo pipefail

DB_CHIRHO="$(dirname "$0")/../accent_transpose_chirho.db"
OUT_DIR_CHIRHO="$(dirname "$0")/batches_chirho"
BATCH_SIZE_CHIRHO=10

rm -rf "$OUT_DIR_CHIRHO"
mkdir -p "$OUT_DIR_CHIRHO"

echo "Extracting mismatch words from database..."

# Get all mismatch words with their verse location and both texts
sqlite3 -separator '|' "$DB_CHIRHO" "
SELECT
  tw.id_chirho,
  sw.book_num_chirho,
  sw.chapter_chirho,
  sw.verse_chirho,
  sw.word_position_chirho,
  sw.word_original_chirho,
  mw.word_full_chirho
FROM transposed_words_chirho tw
JOIN solid_rock_words_chirho sw ON sw.id_chirho = tw.solid_rock_word_id_chirho
LEFT JOIN mapm_words_chirho mw ON mw.id_chirho = tw.mapm_word_id_chirho
WHERE tw.status_chirho='mismatch'
ORDER BY sw.book_num_chirho, sw.chapter_chirho, sw.verse_chirho, sw.word_position_chirho
" > "$OUT_DIR_CHIRHO/all_mismatches_chirho.tsv"

TOTAL_CHIRHO=$(wc -l < "$OUT_DIR_CHIRHO/all_mismatches_chirho.tsv" | tr -d ' ')
echo "Found $TOTAL_CHIRHO mismatch words"

# Now generate batch files with full verse context
BATCH_NUM_CHIRHO=0
WORD_COUNT_CHIRHO=0
CURRENT_BATCH_CHIRHO=""

while IFS='|' read -r tw_id book ch vs pos sr_word mapm_word; do
  # Start new batch if needed
  if [ "$WORD_COUNT_CHIRHO" -eq 0 ]; then
    BATCH_FILE_CHIRHO="$OUT_DIR_CHIRHO/batch_$(printf '%05d' $BATCH_NUM_CHIRHO)_chirho.txt"
    CURRENT_BATCH_CHIRHO="$BATCH_FILE_CHIRHO"
    echo "=== BATCH $BATCH_NUM_CHIRHO ===" > "$BATCH_FILE_CHIRHO"
  fi

  # Get full SR verse
  SR_VERSE_CHIRHO=$(sqlite3 -separator ' ' "$DB_CHIRHO" "
    SELECT CASE WHEN word_position_chirho = $pos
      THEN '[' || word_original_chirho || ']'
      ELSE word_original_chirho
    END
    FROM solid_rock_words_chirho
    WHERE book_num_chirho=$book AND chapter_chirho=$ch AND verse_chirho=$vs
    ORDER BY word_position_chirho
  ")

  # Get full MapM verse
  MAPM_VERSE_CHIRHO=$(sqlite3 -separator ' ' "$DB_CHIRHO" "
    SELECT CASE WHEN word_position_chirho = $pos
      THEN '[' || word_full_chirho || ']'
      ELSE word_full_chirho
    END
    FROM mapm_words_chirho mw
    JOIN mapm_verses_chirho mv ON mv.id_chirho = mw.verse_id_chirho
    WHERE mv.book_num_chirho=$book AND mv.chapter_chirho=$ch AND mv.verse_chirho=$vs
    ORDER BY mw.word_position_chirho
  ")

  {
    echo ""
    echo "WORD_ID: $tw_id"
    echo "REF: book=$book ch=$ch vs=$vs pos=$pos"
    echo "SR_WORD: $sr_word"
    echo "MAPM_WORD_AT_POS: $mapm_word"
    echo "SR_VERSE: $SR_VERSE_CHIRHO"
    echo "MAPM_VERSE: $MAPM_VERSE_CHIRHO"
  } >> "$CURRENT_BATCH_CHIRHO"

  WORD_COUNT_CHIRHO=$((WORD_COUNT_CHIRHO + 1))

  if [ "$WORD_COUNT_CHIRHO" -ge "$BATCH_SIZE_CHIRHO" ]; then
    WORD_COUNT_CHIRHO=0
    BATCH_NUM_CHIRHO=$((BATCH_NUM_CHIRHO + 1))
    if [ $((BATCH_NUM_CHIRHO % 100)) -eq 0 ]; then
      echo "  Generated $BATCH_NUM_CHIRHO batches..."
    fi
  fi

done < "$OUT_DIR_CHIRHO/all_mismatches_chirho.tsv"

TOTAL_BATCHES_CHIRHO=$((BATCH_NUM_CHIRHO + 1))
echo "Generated $TOTAL_BATCHES_CHIRHO batch files in $OUT_DIR_CHIRHO"
echo "Each batch has up to $BATCH_SIZE_CHIRHO words with full verse context."
