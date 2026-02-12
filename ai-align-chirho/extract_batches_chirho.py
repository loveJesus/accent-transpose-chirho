#!/usr/bin/env python3
# For God so loved the world that he gave his only begotten Son,
# that whoever believes in him should not perish but have eternal life.

"""
Extract mismatch words with full verse context into batch files for AI alignment.
Each batch contains ~10 mismatch words with their SR and MapM verse contexts.
"""

import sqlite3
import os
import shutil

DB_PATH_CHIRHO = os.path.join(os.path.dirname(__file__), '..', 'accent_transpose_chirho.db')
OUT_DIR_CHIRHO = os.path.join(os.path.dirname(__file__), 'batches_chirho')
BATCH_SIZE_CHIRHO = 10

def main_chirho():
    if os.path.exists(OUT_DIR_CHIRHO):
        shutil.rmtree(OUT_DIR_CHIRHO)
    os.makedirs(OUT_DIR_CHIRHO)

    conn_chirho = sqlite3.connect(DB_PATH_CHIRHO)

    # 1. Get all mismatch words
    print("Fetching mismatch words...")
    mismatches_chirho = conn_chirho.execute("""
        SELECT
            tw.id_chirho,
            sw.book_num_chirho, sw.chapter_chirho, sw.verse_chirho, sw.word_position_chirho,
            sw.word_original_chirho,
            COALESCE(mw.word_full_chirho, '') as mapm_word
        FROM transposed_words_chirho tw
        JOIN solid_rock_words_chirho sw ON sw.id_chirho = tw.solid_rock_word_id_chirho
        LEFT JOIN mapm_words_chirho mw ON mw.id_chirho = tw.mapm_word_id_chirho
        WHERE tw.status_chirho = 'mismatch'
        ORDER BY sw.book_num_chirho, sw.chapter_chirho, sw.verse_chirho, sw.word_position_chirho
    """).fetchall()
    print(f"  Found {len(mismatches_chirho)} mismatch words")

    # 2. Get unique verses that have mismatches
    verse_keys_chirho = set()
    for row in mismatches_chirho:
        verse_keys_chirho.add((row[1], row[2], row[3]))

    # 3. Pre-load all SR words for mismatch verses
    print("Loading SR verse context...")
    sr_verses_chirho = {}
    for book, ch, vs in verse_keys_chirho:
        words = conn_chirho.execute("""
            SELECT word_position_chirho, word_original_chirho
            FROM solid_rock_words_chirho
            WHERE book_num_chirho=? AND chapter_chirho=? AND verse_chirho=?
            ORDER BY word_position_chirho
        """, (book, ch, vs)).fetchall()
        sr_verses_chirho[(book, ch, vs)] = words

    # 4. Pre-load all MapM words for mismatch verses
    print("Loading MapM verse context...")
    mapm_verses_chirho = {}
    for book, ch, vs in verse_keys_chirho:
        words = conn_chirho.execute("""
            SELECT mw.word_position_chirho, mw.word_full_chirho
            FROM mapm_words_chirho mw
            JOIN mapm_verses_chirho mv ON mv.id_chirho = mw.verse_id_chirho
            WHERE mv.book_num_chirho=? AND mv.chapter_chirho=? AND mv.verse_chirho=?
            ORDER BY mw.word_position_chirho
        """, (book, ch, vs)).fetchall()
        mapm_verses_chirho[(book, ch, vs)] = words

    conn_chirho.close()

    # 5. Build batch files
    print("Generating batch files...")
    batch_num_chirho = 0
    lines_chirho = []

    for i, (tw_id, book, ch, vs, pos, sr_word, mapm_word) in enumerate(mismatches_chirho):
        # Build SR verse with target word bracketed
        sr_words = sr_verses_chirho.get((book, ch, vs), [])
        sr_verse_str = ' '.join(
            f'[{w}]' if p == pos else w
            for p, w in sr_words
        )

        # Build MapM verse with same-position word bracketed
        mapm_words = mapm_verses_chirho.get((book, ch, vs), [])
        mapm_verse_str = ' '.join(
            f'[{w}]' if p == pos else w
            for p, w in mapm_words
        )

        lines_chirho.append(f"WORD_ID: {tw_id}")
        lines_chirho.append(f"REF: book={book} ch={ch} vs={vs} pos={pos}")
        lines_chirho.append(f"SR_WORD: {sr_word}")
        lines_chirho.append(f"MAPM_WORD_AT_POS: {mapm_word}")
        lines_chirho.append(f"SR_VERSE: {sr_verse_str}")
        lines_chirho.append(f"MAPM_VERSE: {mapm_verse_str}")
        lines_chirho.append("")

        if (i + 1) % BATCH_SIZE_CHIRHO == 0 or i == len(mismatches_chirho) - 1:
            batch_file = os.path.join(OUT_DIR_CHIRHO, f"batch_{batch_num_chirho:05d}_chirho.txt")
            with open(batch_file, 'w', encoding='utf-8') as f:
                f.write('\n'.join(lines_chirho))
            lines_chirho = []
            batch_num_chirho += 1

            if batch_num_chirho % 200 == 0:
                print(f"  Generated {batch_num_chirho} batches...")

    print(f"Done! Generated {batch_num_chirho} batch files in {OUT_DIR_CHIRHO}")
    print(f"Each batch has up to {BATCH_SIZE_CHIRHO} words with full verse context.")

if __name__ == '__main__':
    main_chirho()
