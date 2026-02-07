// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.

use crate::hebrew_chirho;
use anyhow::{Context as ContextChirho, Result as ResultChirho};
use quick_xml::events::Event as EventChirho;
use quick_xml::reader::Reader as ReaderChirho;
use rusqlite::Connection as ConnectionChirho;
use std::fs;
use std::path::Path as PathChirho;

const SOLID_ROCK_XML_DIR_CHIRHO: &str = "solid-rock-hb/xml/SR";

pub fn import_solid_rock_chirho(conn_chirho: &ConnectionChirho) -> ResultChirho<()> {
    let tx_chirho = conn_chirho.unchecked_transaction()?;

    // Clear existing data for re-import
    tx_chirho.execute_batch("DELETE FROM solid_rock_words_chirho;")?;

    let mut total_words_chirho: u64 = 0;

    // Process each XML file (01_SR.xml through 39_SR.xml)
    for book_num_chirho in 1..=39 {
        let filename_chirho = format!("{:02}_SR.xml", book_num_chirho);
        let filepath_chirho =
            PathChirho::new(SOLID_ROCK_XML_DIR_CHIRHO).join(&filename_chirho);

        if !filepath_chirho.exists() {
            eprintln!(
                "WARNING: Missing file {}, skipping book {}",
                filepath_chirho.display(),
                book_num_chirho
            );
            continue;
        }

        let words_chirho = parse_tei_file_chirho(&filepath_chirho, book_num_chirho)?;

        for word_chirho in &words_chirho {
            tx_chirho.execute(
                "INSERT INTO solid_rock_words_chirho (book_num_chirho, chapter_chirho, verse_chirho, word_position_chirho, word_original_chirho, word_consonants_chirho, word_vowels_chirho, tei_context_chirho)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                rusqlite::params![
                    word_chirho.book_num_chirho,
                    word_chirho.chapter_chirho,
                    word_chirho.verse_chirho,
                    word_chirho.word_position_chirho,
                    word_chirho.word_original_chirho,
                    word_chirho.word_consonants_chirho,
                    word_chirho.word_vowels_chirho,
                    word_chirho.tei_context_chirho,
                ],
            )?;
            total_words_chirho += 1;
        }

        println!(
            "  Book {:02}: {} words imported",
            book_num_chirho,
            words_chirho.len()
        );
    }

    tx_chirho.commit()?;
    println!("  Total: {} words imported from Solid Rock", total_words_chirho);

    Ok(())
}

struct ParsedWordChirho {
    book_num_chirho: i64,
    chapter_chirho: i64,
    verse_chirho: i64,
    word_position_chirho: i64,
    word_original_chirho: String,
    word_consonants_chirho: String,
    word_vowels_chirho: String,
    tei_context_chirho: Option<String>,
}

fn parse_tei_file_chirho(
    path_chirho: &PathChirho,
    book_num_chirho: i64,
) -> ResultChirho<Vec<ParsedWordChirho>> {
    let xml_text_chirho = fs::read_to_string(path_chirho)
        .with_context(|| format!("Failed to read {}", path_chirho.display()))?;

    let mut reader_chirho = ReaderChirho::from_str(&xml_text_chirho);

    let mut words_chirho: Vec<ParsedWordChirho> = Vec::new();
    let mut current_chapter_chirho: i64 = 0;
    let mut current_verse_chirho: i64 = 0;
    let mut word_position_in_verse_chirho: i64 = 0;
    let mut inside_w_chirho = false;
    let mut current_word_text_chirho = String::new();
    let mut buf_chirho = Vec::new();

    loop {
        match reader_chirho.read_event_into(&mut buf_chirho) {
            Ok(EventChirho::Start(ref e_chirho)) | Ok(EventChirho::Empty(ref e_chirho)) => {
                match e_chirho.name().as_ref() {
                    b"milestone" => {
                        // Parse milestone attributes for chapter/verse refs
                        let mut unit_chirho = String::new();
                        let mut n_chirho = String::new();

                        for attr_chirho in e_chirho.attributes().flatten() {
                            match attr_chirho.key.as_ref() {
                                b"unit" => {
                                    unit_chirho = String::from_utf8_lossy(&attr_chirho.value)
                                        .to_string();
                                }
                                b"n" => {
                                    n_chirho = String::from_utf8_lossy(&attr_chirho.value)
                                        .to_string();
                                }
                                _ => {}
                            }
                        }

                        if unit_chirho == "chapter" && !n_chirho.is_empty() {
                            // Format: B01K1 → chapter 1
                            if let Some(k_pos_chirho) = n_chirho.find('K') {
                                if let Ok(ch_chirho) =
                                    n_chirho[k_pos_chirho + 1..].parse::<i64>()
                                {
                                    current_chapter_chirho = ch_chirho;
                                }
                            }
                        } else if unit_chirho == "verse" && !n_chirho.is_empty() {
                            // Format: B01K1V1 → verse 1
                            if let Some(v_pos_chirho) = n_chirho.find('V') {
                                if let Ok(vs_chirho) =
                                    n_chirho[v_pos_chirho + 1..].parse::<i64>()
                                {
                                    current_verse_chirho = vs_chirho;
                                    word_position_in_verse_chirho = 0;
                                }
                            }
                        }
                    }
                    b"w" => {
                        inside_w_chirho = true;
                        current_word_text_chirho.clear();
                    }
                    _ => {}
                }
            }
            Ok(EventChirho::Text(ref e_chirho)) => {
                if inside_w_chirho {
                    current_word_text_chirho
                        .push_str(&e_chirho.unescape().unwrap_or_default());
                }
            }
            Ok(EventChirho::End(ref e_chirho)) => {
                if e_chirho.name().as_ref() == b"w" && inside_w_chirho {
                    inside_w_chirho = false;

                    let word_trimmed_chirho = current_word_text_chirho.trim().to_string();
                    if !word_trimmed_chirho.is_empty()
                        && current_chapter_chirho > 0
                        && current_verse_chirho > 0
                    {
                        let consonants_chirho =
                            hebrew_chirho::strip_to_consonants_chirho(&word_trimmed_chirho);
                        let vowels_chirho =
                            hebrew_chirho::voweled_form_chirho(&word_trimmed_chirho);

                        words_chirho.push(ParsedWordChirho {
                            book_num_chirho,
                            chapter_chirho: current_chapter_chirho,
                            verse_chirho: current_verse_chirho,
                            word_position_chirho: word_position_in_verse_chirho,
                            word_original_chirho: word_trimmed_chirho,
                            word_consonants_chirho: consonants_chirho,
                            word_vowels_chirho: vowels_chirho,
                            tei_context_chirho: None,
                        });
                        word_position_in_verse_chirho += 1;
                    }
                }
            }
            Ok(EventChirho::Eof) => break,
            Err(e_chirho) => {
                eprintln!(
                    "XML parse error at position {}: {:?}",
                    reader_chirho.error_position(),
                    e_chirho
                );
                break;
            }
            _ => {}
        }
        buf_chirho.clear();
    }

    Ok(words_chirho)
}
