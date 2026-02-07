// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.

/// Hebrew Unicode utilities for cantillation mark transposition.
///
/// Unicode ranges:
///   Cantillation (te'amim): U+0591..=U+05AF
///   Vowel points (nikkud):  U+05B0..=U+05BD, U+05BF, U+05C1..=U+05C2, U+05C4..=U+05C5, U+05C7
///   Consonants:             U+05D0..=U+05EA
///   Punctuation:            U+05BE (maqaf), U+05C0 (paseq), U+05C3 (sof pasuq), U+05C6 (nun hafukha)

/// Returns true if the character is a Hebrew cantillation mark (te'amim).
pub fn is_cantillation_chirho(c_chirho: char) -> bool {
    ('\u{0591}'..='\u{05AF}').contains(&c_chirho)
}

/// Returns true if the character is a Hebrew vowel point (nikkud).
pub fn is_nikkud_chirho(c_chirho: char) -> bool {
    matches!(c_chirho,
        '\u{05B0}'..='\u{05BD}' |
        '\u{05BF}' |
        '\u{05C1}'..='\u{05C2}' |
        '\u{05C4}'..='\u{05C5}' |
        '\u{05C7}'
    )
}

/// Returns true if the character is a Hebrew consonant.
pub fn is_consonant_chirho(c_chirho: char) -> bool {
    ('\u{05D0}'..='\u{05EA}').contains(&c_chirho)
}

/// Returns true if the character is Hebrew punctuation (maqaf, paseq, sof pasuq, nun hafukha).
pub fn is_punctuation_chirho(c_chirho: char) -> bool {
    matches!(c_chirho, '\u{05BE}' | '\u{05C0}' | '\u{05C3}' | '\u{05C6}')
}

/// Strip cantillation marks from a Hebrew string, keeping consonants + vowels + punctuation.
pub fn strip_cantillation_chirho(text_chirho: &str) -> String {
    text_chirho.chars().filter(|c_chirho| !is_cantillation_chirho(*c_chirho)).collect()
}

/// Strip all diacritics (cantillation + nikkud), keeping only consonants + punctuation.
pub fn strip_to_consonants_chirho(text_chirho: &str) -> String {
    text_chirho
        .chars()
        .filter(|c_chirho| !is_cantillation_chirho(*c_chirho) && !is_nikkud_chirho(*c_chirho))
        .collect()
}

/// Extract only the cantillation marks from a Hebrew string.
pub fn extract_cantillation_chirho(text_chirho: &str) -> String {
    text_chirho.chars().filter(|c_chirho| is_cantillation_chirho(*c_chirho)).collect()
}

/// Get consonants + vowels (no cantillation) — this is the comparison form.
pub fn voweled_form_chirho(text_chirho: &str) -> String {
    strip_cantillation_chirho(text_chirho)
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_strip_cantillation_chirho() {
        // בְּרֵאשִׁ֖ית with tipcha (U+0596) → בְּרֵאשִׁית without
        let with_chirho = "בְּרֵאשִׁ\u{0596}ית";
        let without_chirho = "בְּרֵאשִׁית";
        assert_eq!(strip_cantillation_chirho(with_chirho), without_chirho);
    }

    #[test]
    fn test_strip_to_consonants_chirho() {
        let word_chirho = "בְּרֵאשִׁית";
        let consonants_chirho = strip_to_consonants_chirho(word_chirho);
        assert_eq!(consonants_chirho, "בראשית");
    }

    #[test]
    fn test_extract_cantillation_chirho() {
        // Word with tipcha U+0596 and etnachta U+0591
        let word_chirho = "בְּרֵאשִׁ\u{0596}ית";
        let marks_chirho = extract_cantillation_chirho(word_chirho);
        assert_eq!(marks_chirho, "\u{0596}");
    }

    #[test]
    fn test_is_classification_chirho() {
        assert!(is_consonant_chirho('ב'));
        assert!(is_nikkud_chirho('\u{05B0}')); // sheva
        assert!(is_cantillation_chirho('\u{0591}')); // etnachta
        assert!(is_punctuation_chirho('\u{05C3}')); // sof pasuq
    }
}
