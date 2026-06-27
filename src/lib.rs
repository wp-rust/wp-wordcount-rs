use regex::Regex;
use std::sync::OnceLock;

/// Possible ways of counting text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Strategy {
    Words,
    CharactersExcludingSpaces,
    CharactersIncludingSpaces,
}

impl Strategy {
    /// Returns the string representation of the strategy.
    pub fn as_str(&self) -> &'static str {
        match self {
            Strategy::Words => "words",
            Strategy::CharactersExcludingSpaces => "characters_excluding_spaces",
            Strategy::CharactersIncludingSpaces => "characters_including_spaces",
        }
    }
}

/// L10n settings for word counting.
#[derive(Debug, Clone, Default)]
pub struct L10n {
    /// Optional default strategy.
    pub strategy: Option<Strategy>,
    /// Array of shortcode names to be removed during counting.
    pub shortcodes: Vec<String>,
}

/// Base settings fields that can be configured by users.
#[derive(Debug, Clone, Default)]
pub struct UserSettings {
    pub html_regexp: Option<Regex>,
    pub html_comment_regexp: Option<Regex>,
    pub space_regexp: Option<Regex>,
    pub html_entity_regexp: Option<Regex>,
    pub connector_regexp: Option<Regex>,
    pub remove_regexp: Option<Regex>,
    pub words_regexp: Option<Regex>,
    pub characters_excluding_spaces_regexp: Option<Regex>,
    pub characters_including_spaces_regexp: Option<Regex>,
    pub l10n: Option<L10n>,
}

/// Complete settings object with all required properties.
#[derive(Debug, Clone)]
pub struct Settings {
    pub html_regexp: Regex,
    pub html_comment_regexp: Regex,
    pub space_regexp: Regex,
    pub html_entity_regexp: Regex,
    pub connector_regexp: Regex,
    pub remove_regexp: Regex,
    pub words_regexp: Regex,
    pub characters_excluding_spaces_regexp: Regex,
    pub characters_including_spaces_regexp: Regex,
    pub l10n: L10n,
    pub shortcodes: Vec<String>,
    pub shortcodes_regexp: Option<Regex>,
    pub strategy: Strategy,
}

// Regex cell getters using OnceLock
fn get_default_html_regexp() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    CELL.get_or_init(|| Regex::new(r"(?i)</?[a-z][^>]*?>").unwrap())
}

fn get_default_html_comment_regexp() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    CELL.get_or_init(|| Regex::new(r"(?s)<!--.*?-->").unwrap())
}

fn get_default_space_regexp() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    CELL.get_or_init(|| Regex::new(r"(?i)&nbsp;|&#160;").unwrap())
}

fn get_default_html_entity_regexp() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    CELL.get_or_init(|| Regex::new(r"&\S+?;").unwrap())
}

fn get_default_connector_regexp() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    CELL.get_or_init(|| Regex::new(r"--|\u{2014}").unwrap())
}

fn get_default_remove_regexp() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    CELL.get_or_init(|| {
        Regex::new(r"[\u{0021}-\u{002F}\u{003A}-\u{0040}\u{005B}-\u{0060}\u{007B}-\u{007E}\u{0080}-\u{00BF}\u{00D7}\u{00F7}\u{2000}-\u{2BFF}\u{2E00}-\u{2E7F}]").unwrap()
    })
}

fn get_default_words_regexp() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    CELL.get_or_init(|| Regex::new(r"\S\s+").unwrap())
}

fn get_default_characters_excluding_spaces_regexp() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    CELL.get_or_init(|| Regex::new(r"\S").unwrap())
}

fn get_default_characters_including_spaces_regexp() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    CELL.get_or_init(|| {
        Regex::new(r"[^\x{0C}\n\r\t\x{0B}\u{00AD}\u{2028}\u{2029}]").unwrap()
    })
}

impl Settings {
    /// Creates combined settings.
    pub fn new(strategy: Strategy, user_settings: Option<&UserSettings>) -> Self {
        let mut html_regexp = get_default_html_regexp().clone();
        let mut html_comment_regexp = get_default_html_comment_regexp().clone();
        let mut space_regexp = get_default_space_regexp().clone();
        let mut html_entity_regexp = get_default_html_entity_regexp().clone();
        let mut connector_regexp = get_default_connector_regexp().clone();
        let mut remove_regexp = get_default_remove_regexp().clone();
        let mut words_regexp = get_default_words_regexp().clone();
        let mut characters_excluding_spaces_regexp = get_default_characters_excluding_spaces_regexp().clone();
        let mut characters_including_spaces_regexp = get_default_characters_including_spaces_regexp().clone();
        let mut l10n = L10n::default();

        if let Some(us) = user_settings {
            if let Some(ref r) = us.html_regexp { html_regexp = r.clone(); }
            if let Some(ref r) = us.html_comment_regexp { html_comment_regexp = r.clone(); }
            if let Some(ref r) = us.space_regexp { space_regexp = r.clone(); }
            if let Some(ref r) = us.html_entity_regexp { html_entity_regexp = r.clone(); }
            if let Some(ref r) = us.connector_regexp { connector_regexp = r.clone(); }
            if let Some(ref r) = us.remove_regexp { remove_regexp = r.clone(); }
            if let Some(ref r) = us.words_regexp { words_regexp = r.clone(); }
            if let Some(ref r) = us.characters_excluding_spaces_regexp { characters_excluding_spaces_regexp = r.clone(); }
            if let Some(ref r) = us.characters_including_spaces_regexp { characters_including_spaces_regexp = r.clone(); }
            if let Some(ref l) = us.l10n { l10n = l.clone(); }
        }

        let shortcodes = l10n.shortcodes.clone();
        let shortcodes_regexp = if !shortcodes.is_empty() {
            let pattern = format!(r"\[\/?(?:{})[^\]]*?\]", shortcodes.join("|"));
            Regex::new(&pattern).ok()
        } else {
            None
        };

        // If the strategy matches CharactersExcludingSpaces or CharactersIncludingSpaces,
        // use it. Otherwise, default to Words.
        let actual_strategy = match strategy {
            Strategy::CharactersExcludingSpaces => Strategy::CharactersExcludingSpaces,
            Strategy::CharactersIncludingSpaces => Strategy::CharactersIncludingSpaces,
            _ => Strategy::Words,
        };

        Self {
            html_regexp,
            html_comment_regexp,
            space_regexp,
            html_entity_regexp,
            connector_regexp,
            remove_regexp,
            words_regexp,
            characters_excluding_spaces_regexp,
            characters_including_spaces_regexp,
            l10n,
            shortcodes,
            shortcodes_regexp,
            strategy: actual_strategy,
        }
    }
}

fn strip_tags(settings: &Settings, text: &str) -> String {
    settings.html_regexp.replace_all(text, "\n").into_owned()
}

fn strip_html_comments(settings: &Settings, text: &str) -> String {
    settings.html_comment_regexp.replace_all(text, "").into_owned()
}

fn strip_shortcodes(settings: &Settings, text: &str) -> String {
    if let Some(ref re) = settings.shortcodes_regexp {
        re.replace_all(text, "\n").into_owned()
    } else {
        text.to_string()
    }
}

fn strip_spaces(settings: &Settings, text: &str) -> String {
    settings.space_regexp.replace_all(text, " ").into_owned()
}

fn strip_html_entities(settings: &Settings, text: &str) -> String {
    settings.html_entity_regexp.replace_all(text, "").into_owned()
}

fn strip_connectors(settings: &Settings, text: &str) -> String {
    settings.connector_regexp.replace_all(text, " ").into_owned()
}

fn strip_removables(settings: &Settings, text: &str) -> String {
    settings.remove_regexp.replace_all(text, "").into_owned()
}

fn transpose_astrals_to_countable_char(_settings: &Settings, text: &str) -> String {
    text.chars()
        .map(|c| if c > '\u{FFFF}' { 'a' } else { c })
        .collect()
}

fn transpose_html_entities_to_countable_chars(settings: &Settings, text: &str) -> String {
    settings.html_entity_regexp.replace_all(text, "a").into_owned()
}

fn count_words(text: &str, regex: &Regex, settings: &Settings) -> usize {
    let text = strip_tags(settings, text);
    let text = strip_html_comments(settings, &text);
    let text = strip_shortcodes(settings, &text);
    let text = strip_spaces(settings, &text);
    let text = strip_html_entities(settings, &text);
    let text = strip_connectors(settings, &text);
    let text = strip_removables(settings, &text);
    let text = format!("{}\n", text);
    regex.find_iter(&text).count()
}

fn count_characters(text: &str, regex: &Regex, settings: &Settings) -> usize {
    let text = strip_tags(settings, text);
    let text = strip_html_comments(settings, &text);
    let text = strip_shortcodes(settings, &text);
    let text = transpose_astrals_to_countable_char(settings, &text);
    let text = strip_spaces(settings, &text);
    let text = transpose_html_entities_to_countable_chars(settings, &text);
    let text = format!("{}\n", text);
    regex.find_iter(&text).count()
}

/// Count some words or characters.
///
/// # Examples
///
/// ```rust
/// use wp_wordcount_rs::{count, Strategy};
///
/// let count_val = count("Words to count", Strategy::Words, None);
/// assert_eq!(count_val, 3);
/// ```
pub fn count(text: &str, strategy: Strategy, user_settings: Option<&UserSettings>) -> usize {
    let settings = Settings::new(strategy, user_settings);
    match settings.strategy {
        Strategy::Words => {
            count_words(text, &settings.words_regexp, &settings)
        }
        Strategy::CharactersIncludingSpaces => {
            count_characters(text, &settings.characters_including_spaces_regexp, &settings)
        }
        Strategy::CharactersExcludingSpaces => {
            count_characters(text, &settings.characters_excluding_spaces_regexp, &settings)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestDataItem {
        message: &'static str,
        string: &'static str,
        words: usize,
        characters_excluding_spaces: usize,
        characters_including_spaces: usize,
    }

    #[test]
    fn test_word_counter() {
        let mock_data = UserSettings {
            l10n: Some(L10n {
                strategy: None,
                shortcodes: vec!["shortcode".to_string()],
            }),
            ..Default::default()
        };

        let data_provider = vec![
            TestDataItem {
                message: "Basic test.",
                string: "one two three",
                words: 3,
                characters_excluding_spaces: 11,
                characters_including_spaces: 13,
            },
            TestDataItem {
                message: "HTML tags.",
                string: "one <em class=\"test\">two</em><br />three",
                words: 3,
                characters_excluding_spaces: 11,
                characters_including_spaces: 12,
            },
            TestDataItem {
                message: "Line breaks.",
                string: "one\ntwo\nthree",
                words: 3,
                characters_excluding_spaces: 11,
                characters_including_spaces: 11,
            },
            TestDataItem {
                message: "Encoded spaces.",
                string: "one&nbsp;two&#160;three",
                words: 3,
                characters_excluding_spaces: 11,
                characters_including_spaces: 13,
            },
            TestDataItem {
                message: "Punctuation.",
                string: "It’s two three \u{2026} 4?",
                words: 4,
                characters_excluding_spaces: 15,
                characters_including_spaces: 19,
            },
            TestDataItem {
                message: "Numbers as word",
                string: "Should be 4 words",
                words: 4,
                characters_excluding_spaces: 14,
                characters_including_spaces: 17,
            },
            TestDataItem {
                message: "Em dash.",
                string: "one\u{2014}two--three",
                words: 3,
                characters_excluding_spaces: 14,
                characters_including_spaces: 14,
            },
            TestDataItem {
                message: "Shortcodes.",
                string: "one [shortcode attribute=\"value\"]two[/shortcode]three",
                words: 3,
                characters_excluding_spaces: 11,
                characters_including_spaces: 12,
            },
            TestDataItem {
                message: "Astrals.",
                string: "\u{1F4A9}", // 💩 emoji
                words: 1,
                characters_excluding_spaces: 1,
                characters_including_spaces: 1,
            },
            TestDataItem {
                message: "HTML comment.",
                string: "one<!-- comment -->two three",
                words: 2,
                characters_excluding_spaces: 11,
                characters_including_spaces: 12,
            },
            TestDataItem {
                message: "HTML entity.",
                string: "&gt; test",
                words: 1,
                characters_excluding_spaces: 5,
                characters_including_spaces: 6,
            },
            TestDataItem {
                message: "Empty tags",
                string: "<p></p>",
                words: 0,
                characters_excluding_spaces: 0,
                characters_including_spaces: 0,
            },
            TestDataItem {
                message: "Empty string",
                string: "",
                words: 0,
                characters_excluding_spaces: 0,
                characters_including_spaces: 0,
            },
        ];

        for item in data_provider {
            // Words
            let words_count = count(item.string, Strategy::Words, Some(&mock_data));
            assert_eq!(
                words_count, item.words,
                "{} (words) failed: expected {}, got {}",
                item.message, item.words, words_count
            );

            // Characters excluding spaces
            let char_ex_count = count(
                item.string,
                Strategy::CharactersExcludingSpaces,
                Some(&mock_data),
            );
            assert_eq!(
                char_ex_count, item.characters_excluding_spaces,
                "{} (characters_excluding_spaces) failed: expected {}, got {}",
                item.message, item.characters_excluding_spaces, char_ex_count
            );

            // Characters including spaces
            let char_in_count = count(
                item.string,
                Strategy::CharactersIncludingSpaces,
                Some(&mock_data),
            );
            assert_eq!(
                char_in_count, item.characters_including_spaces,
                "{} (characters_including_spaces) failed: expected {}, got {}",
                item.message, item.characters_including_spaces, char_in_count
            );
        }
    }
}
