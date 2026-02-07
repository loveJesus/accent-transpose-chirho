// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.

use crate::hebrew_chirho;
use anyhow::Result as ResultChirho;
use rusqlite::Connection as ConnectionChirho;
use unicode_normalization::UnicodeNormalization as UnicodeNormalizationChirho;

/// For comparison, strip cantillation, punctuation, and meteg (U+05BD),
/// normalize kamatz katan (U+05C7) to kamatz (U+05B8),
/// then apply NFC normalization to handle combining mark order differences.
fn comparison_form_chirho(text_chirho: &str) -> String {
    let stripped_chirho: String = text_chirho
        .chars()
        .filter(|c_chirho| {
            !hebrew_chirho::is_cantillation_chirho(*c_chirho)
                && !hebrew_chirho::is_punctuation_chirho(*c_chirho)
                && *c_chirho != '\u{05BD}' // meteg — appears in MapM but not SR
        })
        .map(|c_chirho| {
            if c_chirho == '\u{05C7}' {
                '\u{05B8}' // normalize kamatz katan → kamatz
            } else {
                c_chirho
            }
        })
        .collect();
    stripped_chirho.nfc().collect()
}

/// Apply cantillation marks from a MapM word onto a Solid Rock word.
///
/// Strategy: For each character position in the MapM full word, if there's a cantillation
/// mark, we record which consonant it follows. Then we insert those marks at the same
/// consonant positions in the SR word.
fn apply_cantillation_chirho(
    sr_word_chirho: &str,
    mapm_full_chirho: &str,
) -> String {
    // Build a map: for each character index in MapM, track cantillation marks
    // that follow each "base" character (consonant or vowel).
    // We reconstruct the SR word by interleaving its characters with the
    // cantillation marks from MapM at matching positions.

    let sr_chars_chirho: Vec<char> = sr_word_chirho.chars().collect();
    let mapm_chars_chirho: Vec<char> = mapm_full_chirho.chars().collect();

    // Extract the "skeleton" of MapM (non-cantillation chars) and where cantillation
    // marks attach.
    // For each non-cantillation char in MapM, collect any cantillation marks that follow it.
    let mut mapm_skeleton_chirho: Vec<(char, Vec<char>)> = Vec::new();
    // Track cantillation before any base character (rare but possible)
    let mut leading_cantillation_chirho: Vec<char> = Vec::new();

    for &c_chirho in &mapm_chars_chirho {
        if hebrew_chirho::is_cantillation_chirho(c_chirho) {
            if let Some(last_chirho) = mapm_skeleton_chirho.last_mut() {
                last_chirho.1.push(c_chirho);
            } else {
                leading_cantillation_chirho.push(c_chirho);
            }
        } else {
            mapm_skeleton_chirho.push((c_chirho, Vec::new()));
        }
    }

    // Now walk through SR chars and MapM skeleton in parallel.
    // For each SR char, if the corresponding MapM skeleton char matches (ignoring
    // punctuation differences), apply the cantillation marks.
    let mut result_chirho = String::new();

    // Add any leading cantillation
    for c_chirho in &leading_cantillation_chirho {
        result_chirho.push(*c_chirho);
    }

    let mut mapm_idx_chirho: usize = 0;

    for &sr_char_chirho in &sr_chars_chirho {
        result_chirho.push(sr_char_chirho);

        // Try to match this SR char with the current MapM skeleton position
        if mapm_idx_chirho < mapm_skeleton_chirho.len() {
            let (mapm_char_chirho, ref cantillation_chirho) =
                mapm_skeleton_chirho[mapm_idx_chirho];

            // Check if the base characters match (the SR char should correspond
            // to the MapM skeleton char)
            if sr_char_chirho == mapm_char_chirho {
                // Apply cantillation marks from MapM
                for &mark_chirho in cantillation_chirho {
                    result_chirho.push(mark_chirho);
                }
                mapm_idx_chirho += 1;
            } else if hebrew_chirho::is_cantillation_chirho(sr_char_chirho) {
                // SR char is cantillation (shouldn't happen normally), skip MapM advance
            } else {
                // Mismatch — still advance MapM to stay aligned
                mapm_idx_chirho += 1;
                for &mark_chirho in cantillation_chirho {
                    result_chirho.push(mark_chirho);
                }
            }
        }
    }

    result_chirho
}

pub fn transpose_chirho(conn_chirho: &ConnectionChirho) -> ResultChirho<()> {
    let tx_chirho = conn_chirho.unchecked_transaction()?;

    // Clear existing transposition results
    tx_chirho.execute_batch("DELETE FROM transposed_words_chirho;")?;

    // Get all distinct verse references from Solid Rock
    let verse_refs_chirho: Vec<(i64, i64, i64)> = {
        let mut stmt_chirho = tx_chirho.prepare(
            "SELECT DISTINCT book_num_chirho, chapter_chirho, verse_chirho
             FROM solid_rock_words_chirho
             ORDER BY book_num_chirho, chapter_chirho, verse_chirho",
        )?;
        stmt_chirho
            .query_map([], |row_chirho| {
                Ok((
                    row_chirho.get(0)?,
                    row_chirho.get(1)?,
                    row_chirho.get(2)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?
    };

    let mut stats_unmodified_chirho: u64 = 0;
    #[allow(unused)]
    let mut stats_cantillated_chirho: u64 = 0;
    let mut stats_not_present_chirho: u64 = 0;
    let mut stats_mismatch_chirho: u64 = 0;

    for (book_chirho, chapter_chirho, verse_chirho) in &verse_refs_chirho {
        // Get SR words for this verse
        let mut sr_stmt_chirho = tx_chirho.prepare_cached(
            "SELECT id_chirho, word_position_chirho, word_original_chirho, word_vowels_chirho
             FROM solid_rock_words_chirho
             WHERE book_num_chirho=?1 AND chapter_chirho=?2 AND verse_chirho=?3
             ORDER BY word_position_chirho",
        )?;

        let sr_words_chirho: Vec<(i64, i64, String, String)> = sr_stmt_chirho
            .query_map(
                rusqlite::params![book_chirho, chapter_chirho, verse_chirho],
                |row_chirho| {
                    Ok((
                        row_chirho.get(0)?,
                        row_chirho.get(1)?,
                        row_chirho.get(2)?,
                        row_chirho.get(3)?,
                    ))
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;

        // Get MapM words for the same verse
        let mut mapm_stmt_chirho = tx_chirho.prepare_cached(
            "SELECT mw.id_chirho, mw.word_position_chirho, mw.word_full_chirho, mw.word_vowels_chirho
             FROM mapm_words_chirho mw
             JOIN mapm_verses_chirho mv ON mv.id_chirho = mw.verse_id_chirho
             WHERE mv.book_num_chirho=?1 AND mv.chapter_chirho=?2 AND mv.verse_chirho=?3
             ORDER BY mw.word_position_chirho",
        )?;

        let mapm_words_chirho: Vec<(i64, i64, String, String)> = mapm_stmt_chirho
            .query_map(
                rusqlite::params![book_chirho, chapter_chirho, verse_chirho],
                |row_chirho| {
                    Ok((
                        row_chirho.get(0)?,
                        row_chirho.get(1)?,
                        row_chirho.get(2)?,
                        row_chirho.get(3)?,
                    ))
                },
            )?
            .collect::<Result<Vec<_>, _>>()?;

        if mapm_words_chirho.is_empty() {
            // No MapM data for this verse
            for (sr_id_chirho, _, sr_word_chirho, _) in &sr_words_chirho {
                tx_chirho.execute(
                    "INSERT INTO transposed_words_chirho (solid_rock_word_id_chirho, mapm_word_id_chirho, word_result_chirho, status_chirho, confidence_chirho, notes_chirho)
                     VALUES (?1, NULL, ?2, 'not_present_in_mapm', 0.0, 'No MapM verse found')",
                    rusqlite::params![sr_id_chirho, sr_word_chirho],
                )?;
                stats_not_present_chirho += 1;
            }
            continue;
        }

        // Position-based matching
        for (sr_id_chirho, sr_pos_chirho, sr_word_chirho, _sr_vowels_chirho) in &sr_words_chirho {
            let sr_comparison_chirho = comparison_form_chirho(sr_word_chirho);

            // Try exact position match first
            let matched_chirho = mapm_words_chirho
                .iter()
                .find(|(_, mapm_pos_chirho, _, _)| mapm_pos_chirho == sr_pos_chirho);

            if let Some((mapm_id_chirho, _, mapm_full_chirho, mapm_vowels_chirho)) = matched_chirho
            {
                let mapm_comparison_chirho = comparison_form_chirho(mapm_vowels_chirho);

                if sr_comparison_chirho == mapm_comparison_chirho {
                    // Perfect match — apply cantillation
                    let result_word_chirho =
                        apply_cantillation_chirho(sr_word_chirho, mapm_full_chirho);
                    tx_chirho.execute(
                        "INSERT INTO transposed_words_chirho (solid_rock_word_id_chirho, mapm_word_id_chirho, word_result_chirho, status_chirho, confidence_chirho, notes_chirho)
                         VALUES (?1, ?2, ?3, 'unmodified', 1.0, NULL)",
                        rusqlite::params![sr_id_chirho, mapm_id_chirho, result_word_chirho],
                    )?;
                    stats_unmodified_chirho += 1;
                } else {
                    // Position matches but text differs — still apply cantillation
                    // but mark as cantillated (needs review)
                    let result_word_chirho =
                        apply_cantillation_chirho(sr_word_chirho, mapm_full_chirho);
                    let note_chirho = format!(
                        "Text mismatch: SR='{}' MapM='{}'",
                        sr_comparison_chirho, mapm_comparison_chirho
                    );
                    tx_chirho.execute(
                        "INSERT INTO transposed_words_chirho (solid_rock_word_id_chirho, mapm_word_id_chirho, word_result_chirho, status_chirho, confidence_chirho, notes_chirho)
                         VALUES (?1, ?2, ?3, 'mismatch', 0.5, ?4)",
                        rusqlite::params![
                            sr_id_chirho,
                            mapm_id_chirho,
                            result_word_chirho,
                            note_chirho,
                        ],
                    )?;
                    stats_mismatch_chirho += 1;
                }
            } else {
                // No positional match (SR has more words than MapM)
                tx_chirho.execute(
                    "INSERT INTO transposed_words_chirho (solid_rock_word_id_chirho, mapm_word_id_chirho, word_result_chirho, status_chirho, confidence_chirho, notes_chirho)
                     VALUES (?1, NULL, ?2, 'not_present_in_mapm', 0.0, 'No MapM word at this position')",
                    rusqlite::params![sr_id_chirho, sr_word_chirho],
                )?;
                stats_not_present_chirho += 1;
            }
        }
    }

    tx_chirho.commit()?;

    println!("Transposition complete:");
    println!("  unmodified (perfect match):  {}", stats_unmodified_chirho);
    println!("  mismatch (text differs):     {}", stats_mismatch_chirho);
    println!("  not_present_in_mapm:         {}", stats_not_present_chirho);
    println!(
        "  total:                       {}",
        stats_unmodified_chirho + stats_mismatch_chirho + stats_not_present_chirho
    );

    Ok(())
}
