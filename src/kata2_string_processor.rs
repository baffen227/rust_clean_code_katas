// ============================================
// Kata 2: String Processor
// Focus: Extracting Small Functions, Avoiding Repetition, Clear Interface
// ============================================
use std::fmt;

pub struct TextStatistics {
    pub word_count: usize,
    pub line_count: usize,
    pub character_count: usize,
    pub character_count_no_spaces: usize,
}

impl TextStatistics {
    /// Create statistics from text
    pub fn from_text(text: &str) -> Self {
        Self {
            word_count: count_words(text),
            line_count: count_lines(text),
            character_count: count_characters(text),
            character_count_no_spaces: count_characters_excluding_spaces(text),
        }
    }
}

/// Count words
fn count_words(text: &str) -> usize {
    text.split_whitespace()
        .filter(|word| !word.is_empty())
        .count()
}

/// Count lines
fn count_lines(text: &str) -> usize {
    if text.is_empty() {
        0
    } else {
        text.lines().count()
    }
}

/// Count total characters
fn count_characters(text: &str) -> usize {
    text.chars().count()
}

/// Count characters excluding spaces
fn count_characters_excluding_spaces(text: &str) -> usize {
    text.chars()
        .filter(|ch| !ch.is_whitespace())
        .count()
}

impl fmt::Display for TextStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Words: {}, Lines: {}, Characters: {} ({})",
            self.word_count,
            self.line_count,
            self.character_count,
            self.character_count_no_spaces
        )
    }
}

#[cfg(test)]
mod text_statistics_tests {
    use super::*;

    #[test]
    fn test_empty_text() {
        let stats = TextStatistics::from_text("");
        assert_eq!(stats.word_count, 0);
        assert_eq!(stats.line_count, 0);
        assert_eq!(stats.character_count, 0);
    }

    #[test]
    fn test_single_line() {
        let stats = TextStatistics::from_text("Hello world");
        assert_eq!(stats.word_count, 2);
        assert_eq!(stats.line_count, 1);
        assert_eq!(stats.character_count, 11);
        assert_eq!(stats.character_count_no_spaces, 10);
    }
}
