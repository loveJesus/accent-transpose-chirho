// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.

use anyhow::Result as ResultChirho;
use rusqlite::Connection as ConnectionChirho;
use serde::Serialize as SerializeChirho;
use std::collections::BTreeMap as BTreeMapChirho;
use std::fs;
use std::path::Path as PathChirho;

const JSON_OUTPUT_DIR_CHIRHO: &str = "export-chirho/json-chirho";
const SITE_DATA_DIR_CHIRHO: &str = "site-chirho/data-chirho";

/// Book number → English name (matching MapM convention).
fn book_num_to_name_chirho(num_chirho: i64) -> &'static str {
    match num_chirho {
        1 => "Genesis",
        2 => "Exodus",
        3 => "Leviticus",
        4 => "Numbers",
        5 => "Deuteronomy",
        6 => "Joshua",
        7 => "Judges",
        8 => "1 Samuel",
        9 => "2 Samuel",
        10 => "1 Kings",
        11 => "2 Kings",
        12 => "Isaiah",
        13 => "Jeremiah",
        14 => "Ezekiel",
        15 => "Hosea",
        16 => "Joel",
        17 => "Amos",
        18 => "Obadiah",
        19 => "Jonah",
        20 => "Micah",
        21 => "Nahum",
        22 => "Habakkuk",
        23 => "Zephaniah",
        24 => "Haggai",
        25 => "Zechariah",
        26 => "Malachi",
        27 => "Psalms",
        28 => "Proverbs",
        29 => "Job",
        30 => "Song of Solomon",
        31 => "Ruth",
        32 => "Lamentations",
        33 => "Ecclesiastes",
        34 => "Esther",
        35 => "Daniel",
        36 => "Ezra",
        37 => "Nehemiah",
        38 => "1 Chronicles",
        39 => "2 Chronicles",
        _ => "Unknown",
    }
}

#[derive(SerializeChirho)]
struct WordChirho {
    position_chirho: i64,
    sr_original_chirho: String,
    result_chirho: String,
    status_chirho: String,
    confidence_chirho: f64,
    notes_chirho: Option<String>,
}

#[derive(SerializeChirho)]
struct VerseChirho {
    verse_chirho: i64,
    words_chirho: Vec<WordChirho>,
}

#[derive(SerializeChirho)]
struct StatsChirho {
    total: u64,
    unmodified: u64,
    mismatch: u64,
    not_present: u64,
}

#[derive(SerializeChirho)]
struct ChapterChirho {
    book_num_chirho: i64,
    book_name_chirho: String,
    chapter_chirho: i64,
    stats_chirho: StatsChirho,
    verses_chirho: Vec<VerseChirho>,
}

#[derive(SerializeChirho)]
struct ManifestBookChirho {
    book_num_chirho: i64,
    book_name_chirho: String,
    chapters_chirho: Vec<ManifestChapterChirho>,
    stats_chirho: StatsChirho,
}

#[derive(SerializeChirho)]
struct ManifestChapterChirho {
    chapter_chirho: i64,
    file_chirho: String,
    word_count_chirho: u64,
    stats_chirho: StatsChirho,
}

#[derive(SerializeChirho)]
struct ManifestChirho {
    total_stats_chirho: StatsChirho,
    books_chirho: Vec<ManifestBookChirho>,
}

pub fn export_chirho(conn_chirho: &ConnectionChirho) -> ResultChirho<()> {
    // Create output directories
    fs::create_dir_all(JSON_OUTPUT_DIR_CHIRHO)?;
    fs::create_dir_all(SITE_DATA_DIR_CHIRHO)?;

    // Query all transposed words joined with SR words, ordered for chapter grouping
    let mut stmt_chirho = conn_chirho.prepare(
        "SELECT
            sr.book_num_chirho,
            sr.chapter_chirho,
            sr.verse_chirho,
            sr.word_position_chirho,
            sr.word_original_chirho,
            tw.word_result_chirho,
            tw.status_chirho,
            tw.confidence_chirho,
            tw.notes_chirho
         FROM transposed_words_chirho tw
         JOIN solid_rock_words_chirho sr ON sr.id_chirho = tw.solid_rock_word_id_chirho
         ORDER BY sr.book_num_chirho, sr.chapter_chirho, sr.verse_chirho, sr.word_position_chirho",
    )?;

    // Collect rows into a nested structure: book → chapter → verse → words
    // Use BTreeMap for sorted keys
    let mut books_chirho: BTreeMapChirho<i64, BTreeMapChirho<i64, BTreeMapChirho<i64, Vec<WordChirho>>>> =
        BTreeMapChirho::new();

    let rows_chirho = stmt_chirho.query_map([], |row_chirho| {
        Ok((
            row_chirho.get::<_, i64>(0)?,    // book_num
            row_chirho.get::<_, i64>(1)?,    // chapter
            row_chirho.get::<_, i64>(2)?,    // verse
            row_chirho.get::<_, i64>(3)?,    // word_position
            row_chirho.get::<_, String>(4)?, // sr_original
            row_chirho.get::<_, String>(5)?, // result
            row_chirho.get::<_, String>(6)?, // status
            row_chirho.get::<_, f64>(7)?,    // confidence
            row_chirho.get::<_, Option<String>>(8)?, // notes
        ))
    })?;

    let mut total_words_chirho: u64 = 0;

    for row_chirho in rows_chirho {
        let (
            book_num_chirho,
            chapter_chirho,
            verse_chirho,
            position_chirho,
            sr_original_chirho,
            result_chirho,
            status_chirho,
            confidence_chirho,
            notes_chirho,
        ) = row_chirho?;

        let word_chirho = WordChirho {
            position_chirho,
            sr_original_chirho,
            result_chirho,
            status_chirho,
            confidence_chirho,
            notes_chirho,
        };

        books_chirho
            .entry(book_num_chirho)
            .or_default()
            .entry(chapter_chirho)
            .or_default()
            .entry(verse_chirho)
            .or_default()
            .push(word_chirho);

        total_words_chirho += 1;
    }

    println!("  Exporting {} words across {} books...", total_words_chirho, books_chirho.len());

    let mut manifest_books_chirho: Vec<ManifestBookChirho> = Vec::new();
    let mut global_unmodified_chirho: u64 = 0;
    let mut global_mismatch_chirho: u64 = 0;
    let mut global_not_present_chirho: u64 = 0;

    for (&book_num_chirho, chapters_chirho) in &books_chirho {
        let book_name_chirho = book_num_to_name_chirho(book_num_chirho).to_string();
        let mut manifest_chapters_chirho: Vec<ManifestChapterChirho> = Vec::new();
        let mut book_unmodified_chirho: u64 = 0;
        let mut book_mismatch_chirho: u64 = 0;
        let mut book_not_present_chirho: u64 = 0;

        for (&chapter_num_chirho, verses_chirho) in chapters_chirho {
            let mut chapter_verses_chirho: Vec<VerseChirho> = Vec::new();
            let mut ch_unmodified_chirho: u64 = 0;
            let mut ch_mismatch_chirho: u64 = 0;
            let mut ch_not_present_chirho: u64 = 0;
            let mut ch_total_chirho: u64 = 0;

            for (&verse_num_chirho, words_chirho) in verses_chirho {
                let mut verse_words_chirho: Vec<WordChirho> = Vec::new();

                for w_chirho in words_chirho {
                    match w_chirho.status_chirho.as_str() {
                        "unmodified" | "cantillated" => ch_unmodified_chirho += 1,
                        "mismatch" => ch_mismatch_chirho += 1,
                        "not_present_in_mapm" => ch_not_present_chirho += 1,
                        _ => {}
                    }
                    ch_total_chirho += 1;

                    verse_words_chirho.push(WordChirho {
                        position_chirho: w_chirho.position_chirho,
                        sr_original_chirho: w_chirho.sr_original_chirho.clone(),
                        result_chirho: w_chirho.result_chirho.clone(),
                        status_chirho: w_chirho.status_chirho.clone(),
                        confidence_chirho: w_chirho.confidence_chirho,
                        notes_chirho: w_chirho.notes_chirho.clone(),
                    });
                }

                chapter_verses_chirho.push(VerseChirho {
                    verse_chirho: verse_num_chirho,
                    words_chirho: verse_words_chirho,
                });
            }

            let chapter_stats_chirho = StatsChirho {
                total: ch_total_chirho,
                unmodified: ch_unmodified_chirho,
                mismatch: ch_mismatch_chirho,
                not_present: ch_not_present_chirho,
            };

            let chapter_data_chirho = ChapterChirho {
                book_num_chirho,
                book_name_chirho: book_name_chirho.clone(),
                chapter_chirho: chapter_num_chirho,
                stats_chirho: StatsChirho {
                    total: ch_total_chirho,
                    unmodified: ch_unmodified_chirho,
                    mismatch: ch_mismatch_chirho,
                    not_present: ch_not_present_chirho,
                },
                verses_chirho: chapter_verses_chirho,
            };

            // Write chapter JSON
            let filename_chirho = format!("book{:02}_ch{:03}.json", book_num_chirho, chapter_num_chirho);
            let json_path_chirho = PathChirho::new(JSON_OUTPUT_DIR_CHIRHO).join(&filename_chirho);
            let json_str_chirho = serde_json::to_string(&chapter_data_chirho)?;
            fs::write(&json_path_chirho, &json_str_chirho)?;

            // Also copy to site data dir
            let site_path_chirho = PathChirho::new(SITE_DATA_DIR_CHIRHO).join(&filename_chirho);
            fs::write(&site_path_chirho, &json_str_chirho)?;

            manifest_chapters_chirho.push(ManifestChapterChirho {
                chapter_chirho: chapter_num_chirho,
                file_chirho: filename_chirho,
                word_count_chirho: ch_total_chirho,
                stats_chirho: chapter_stats_chirho,
            });

            book_unmodified_chirho += ch_unmodified_chirho;
            book_mismatch_chirho += ch_mismatch_chirho;
            book_not_present_chirho += ch_not_present_chirho;
        }

        let book_total_chirho = book_unmodified_chirho + book_mismatch_chirho + book_not_present_chirho;
        manifest_books_chirho.push(ManifestBookChirho {
            book_num_chirho,
            book_name_chirho: book_name_chirho.clone(),
            chapters_chirho: manifest_chapters_chirho,
            stats_chirho: StatsChirho {
                total: book_total_chirho,
                unmodified: book_unmodified_chirho,
                mismatch: book_mismatch_chirho,
                not_present: book_not_present_chirho,
            },
        });

        global_unmodified_chirho += book_unmodified_chirho;
        global_mismatch_chirho += book_mismatch_chirho;
        global_not_present_chirho += book_not_present_chirho;

        println!(
            "    {} — {} chapters, {} words",
            book_name_chirho,
            chapters_chirho.len(),
            book_total_chirho,
        );
    }

    let global_total_chirho = global_unmodified_chirho + global_mismatch_chirho + global_not_present_chirho;

    let manifest_chirho = ManifestChirho {
        total_stats_chirho: StatsChirho {
            total: global_total_chirho,
            unmodified: global_unmodified_chirho,
            mismatch: global_mismatch_chirho,
            not_present: global_not_present_chirho,
        },
        books_chirho: manifest_books_chirho,
    };

    // Write manifest to both dirs
    let manifest_json_chirho = serde_json::to_string_pretty(&manifest_chirho)?;
    fs::write(
        PathChirho::new(JSON_OUTPUT_DIR_CHIRHO).join("manifest_chirho.json"),
        &manifest_json_chirho,
    )?;
    fs::write(
        PathChirho::new(SITE_DATA_DIR_CHIRHO).join("manifest_chirho.json"),
        &manifest_json_chirho,
    )?;

    println!("\nExport complete:");
    println!("  Total words:   {}", global_total_chirho);
    println!("  Unmodified:    {} ({:.1}%)", global_unmodified_chirho, global_unmodified_chirho as f64 / global_total_chirho as f64 * 100.0);
    println!("  Mismatch:      {} ({:.1}%)", global_mismatch_chirho, global_mismatch_chirho as f64 / global_total_chirho as f64 * 100.0);
    println!("  Not present:   {} ({:.1}%)", global_not_present_chirho, global_not_present_chirho as f64 / global_total_chirho as f64 * 100.0);
    println!("  JSON files:    {}/", JSON_OUTPUT_DIR_CHIRHO);
    println!("  Site data:     {}/", SITE_DATA_DIR_CHIRHO);

    Ok(())
}
