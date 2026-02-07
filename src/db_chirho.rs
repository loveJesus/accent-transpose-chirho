// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.

use anyhow::Result as ResultChirho;
use rusqlite::Connection as ConnectionChirho;

const DB_PATH_CHIRHO: &str = "accent_transpose_chirho.db";

pub fn open_db_chirho() -> ResultChirho<ConnectionChirho> {
    let conn_chirho = ConnectionChirho::open(DB_PATH_CHIRHO)?;
    conn_chirho.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
    Ok(conn_chirho)
}

pub fn create_tables_chirho(conn_chirho: &ConnectionChirho) -> ResultChirho<()> {
    conn_chirho.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS mapm_verses_chirho (
            id_chirho INTEGER PRIMARY KEY,
            book_name_chirho TEXT NOT NULL,
            book_num_chirho INTEGER NOT NULL,
            chapter_chirho INTEGER NOT NULL,
            verse_chirho INTEGER NOT NULL,
            text_chirho TEXT NOT NULL,
            UNIQUE(book_num_chirho, chapter_chirho, verse_chirho)
        );

        CREATE TABLE IF NOT EXISTS mapm_words_chirho (
            id_chirho INTEGER PRIMARY KEY,
            verse_id_chirho INTEGER NOT NULL REFERENCES mapm_verses_chirho(id_chirho),
            word_position_chirho INTEGER NOT NULL,
            word_full_chirho TEXT NOT NULL,
            word_consonants_chirho TEXT NOT NULL,
            word_vowels_chirho TEXT NOT NULL,
            word_cantillation_chirho TEXT NOT NULL,
            UNIQUE(verse_id_chirho, word_position_chirho)
        );

        CREATE TABLE IF NOT EXISTS solid_rock_words_chirho (
            id_chirho INTEGER PRIMARY KEY,
            book_num_chirho INTEGER NOT NULL,
            chapter_chirho INTEGER NOT NULL,
            verse_chirho INTEGER NOT NULL,
            word_position_chirho INTEGER NOT NULL,
            word_original_chirho TEXT NOT NULL,
            word_consonants_chirho TEXT NOT NULL,
            word_vowels_chirho TEXT NOT NULL,
            tei_context_chirho TEXT,
            UNIQUE(book_num_chirho, chapter_chirho, verse_chirho, word_position_chirho)
        );

        CREATE TABLE IF NOT EXISTS transposed_words_chirho (
            id_chirho INTEGER PRIMARY KEY,
            solid_rock_word_id_chirho INTEGER NOT NULL REFERENCES solid_rock_words_chirho(id_chirho),
            mapm_word_id_chirho INTEGER REFERENCES mapm_words_chirho(id_chirho),
            word_result_chirho TEXT NOT NULL,
            status_chirho TEXT NOT NULL CHECK(status_chirho IN ('unmodified', 'cantillated', 'not_present_in_mapm', 'mismatch')),
            confidence_chirho REAL,
            notes_chirho TEXT,
            UNIQUE(solid_rock_word_id_chirho)
        );

        CREATE INDEX IF NOT EXISTS idx_mapm_words_verse_chirho
            ON mapm_words_chirho(verse_id_chirho);

        CREATE INDEX IF NOT EXISTS idx_mapm_verses_ref_chirho
            ON mapm_verses_chirho(book_num_chirho, chapter_chirho, verse_chirho);

        CREATE INDEX IF NOT EXISTS idx_solid_rock_words_ref_chirho
            ON solid_rock_words_chirho(book_num_chirho, chapter_chirho, verse_chirho);
        ",
    )?;
    Ok(())
}
