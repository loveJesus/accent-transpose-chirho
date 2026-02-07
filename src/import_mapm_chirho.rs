// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.

use crate::hebrew_chirho;
use anyhow::{Context as ContextChirho, Result as ResultChirho};
use rusqlite::Connection as ConnectionChirho;
use serde::Deserialize as DeserializeChirho;
use std::fs;

const MAPM_JSON_PATH_CHIRHO: &str = "MapM/MapM.json";

#[derive(DeserializeChirho)]
struct MapmRootChirho {
    books: Vec<MapmBookChirho>,
}

#[derive(DeserializeChirho)]
struct MapmBookChirho {
    name: String,
    chapters: Vec<MapmChapterChirho>,
}

#[derive(DeserializeChirho)]
struct MapmChapterChirho {
    chapter: i64,
    #[allow(dead_code)]
    name: String,
    verses: Vec<MapmVerseChirho>,
}

#[derive(DeserializeChirho)]
struct MapmVerseChirho {
    verse: i64,
    #[allow(dead_code)]
    chapter: i64,
    #[allow(dead_code)]
    name: String,
    text: String,
}

/// Map English book name from MapM to SRHB book number (1-39).
fn book_name_to_num_chirho(name_chirho: &str) -> Option<i64> {
    match name_chirho {
        "Genesis" => Some(1),
        "Exodus" => Some(2),
        "Leviticus" => Some(3),
        "Numbers" => Some(4),
        "Deuteronomy" => Some(5),
        "Joshua" => Some(6),
        "Judges" => Some(7),
        "1 Samuel" | "I Samuel" => Some(8),
        "2 Samuel" | "II Samuel" => Some(9),
        "1 Kings" | "I Kings" => Some(10),
        "2 Kings" | "II Kings" => Some(11),
        "Isaiah" => Some(12),
        "Jeremiah" => Some(13),
        "Ezekiel" => Some(14),
        "Hosea" => Some(15),
        "Joel" => Some(16),
        "Amos" => Some(17),
        "Obadiah" => Some(18),
        "Jonah" => Some(19),
        "Micah" => Some(20),
        "Nahum" => Some(21),
        "Habakkuk" => Some(22),
        "Zephaniah" => Some(23),
        "Haggai" => Some(24),
        "Zechariah" => Some(25),
        "Malachi" => Some(26),
        "Psalms" => Some(27),
        "Proverbs" => Some(28),
        "Job" => Some(29),
        "Song of Solomon" => Some(30),
        "Ruth" => Some(31),
        "Lamentations" => Some(32),
        "Ecclesiastes" => Some(33),
        "Esther" => Some(34),
        "Daniel" => Some(35),
        "Ezra" => Some(36),
        "Nehemiah" => Some(37),
        "1 Chronicles" | "I Chronicles" => Some(38),
        "2 Chronicles" | "II Chronicles" => Some(39),
        _ => None,
    }
}

/// Returns true if the token contains no Hebrew consonants — i.e., it's purely
/// punctuation, whitespace, or non-breaking spaces (paseq, sof pasuq, etc.).
fn is_punctuation_only_chirho(token_chirho: &str) -> bool {
    !token_chirho
        .chars()
        .any(|c_chirho| hebrew_chirho::is_consonant_chirho(c_chirho))
}

/// Split a maqaf-joined token into sub-words.
/// Maqaf (U+05BE, ־) connects Hebrew words. We split on it so each sub-word
/// gets its own position. The maqaf stays attached to the end of the preceding word
/// (same as how Solid Rock TEI XML does NOT include maqaf in <w> tags, but
/// it sometimes appears). We strip the trailing maqaf for cleaner matching.
fn split_on_maqaf_chirho(token_chirho: &str) -> Vec<String> {
    const MAQAF_CHIRHO: char = '\u{05BE}';

    if !token_chirho.contains(MAQAF_CHIRHO) {
        return vec![token_chirho.to_string()];
    }

    let parts_chirho: Vec<&str> = token_chirho.split(MAQAF_CHIRHO).collect();
    parts_chirho
        .iter()
        .filter(|p_chirho| !p_chirho.is_empty())
        .map(|p_chirho| p_chirho.to_string())
        .collect()
}

pub fn import_mapm_chirho(conn_chirho: &ConnectionChirho) -> ResultChirho<()> {
    let json_text_chirho =
        fs::read_to_string(MAPM_JSON_PATH_CHIRHO).context("Failed to read MapM JSON")?;
    let root_chirho: MapmRootChirho =
        serde_json::from_str(&json_text_chirho).context("Failed to parse MapM JSON")?;

    let tx_chirho = conn_chirho.unchecked_transaction()?;

    // Clear existing data for re-import
    tx_chirho.execute_batch(
        "DELETE FROM mapm_words_chirho; DELETE FROM mapm_verses_chirho;",
    )?;

    let mut verse_count_chirho: u64 = 0;
    let mut word_count_chirho: u64 = 0;

    for book_chirho in &root_chirho.books {
        let book_num_chirho = match book_name_to_num_chirho(&book_chirho.name) {
            Some(n_chirho) => n_chirho,
            None => {
                eprintln!("WARNING: Unknown book name '{}', skipping", book_chirho.name);
                continue;
            }
        };

        for chapter_chirho in &book_chirho.chapters {
            for verse_chirho in &chapter_chirho.verses {
                // Insert verse
                tx_chirho.execute(
                    "INSERT INTO mapm_verses_chirho (book_name_chirho, book_num_chirho, chapter_chirho, verse_chirho, text_chirho)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    rusqlite::params![
                        book_chirho.name,
                        book_num_chirho,
                        chapter_chirho.chapter,
                        verse_chirho.verse,
                        verse_chirho.text,
                    ],
                )?;
                let verse_id_chirho = tx_chirho.last_insert_rowid();
                verse_count_chirho += 1;

                // Split verse text into words — first by whitespace, then
                // sub-split maqaf-joined words (U+05BE) so each atomic word
                // gets its own position, matching Solid Rock's <w> boundaries.
                let whitespace_tokens_chirho: Vec<&str> = verse_chirho
                    .text
                    .split_whitespace()
                    .collect();

                let mut pos_chirho: i64 = 0;
                for token_chirho in &whitespace_tokens_chirho {
                    // Split on maqaf (U+05BE). Each sub-word becomes its own entry.
                    let sub_words_chirho = split_on_maqaf_chirho(token_chirho);

                    for sub_word_chirho in &sub_words_chirho {
                        // Skip tokens that are purely punctuation (e.g., standalone
                        // paseq ׀ U+05C0, sof pasuq ׃ U+05C3, or NBSP+paseq).
                        if is_punctuation_only_chirho(sub_word_chirho) {
                            continue;
                        }
                        let consonants_chirho =
                            hebrew_chirho::strip_to_consonants_chirho(sub_word_chirho);
                        let vowels_chirho =
                            hebrew_chirho::voweled_form_chirho(sub_word_chirho);
                        let cantillation_chirho =
                            hebrew_chirho::extract_cantillation_chirho(sub_word_chirho);

                        tx_chirho.execute(
                            "INSERT INTO mapm_words_chirho (verse_id_chirho, word_position_chirho, word_full_chirho, word_consonants_chirho, word_vowels_chirho, word_cantillation_chirho)
                             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                            rusqlite::params![
                                verse_id_chirho,
                                pos_chirho,
                                sub_word_chirho,
                                consonants_chirho,
                                vowels_chirho,
                                cantillation_chirho,
                            ],
                        )?;
                        word_count_chirho += 1;
                        pos_chirho += 1;
                    }
                }
            }
        }
    }

    tx_chirho.commit()?;

    println!(
        "  Imported {} verses, {} words from MapM",
        verse_count_chirho, word_count_chirho
    );

    Ok(())
}
