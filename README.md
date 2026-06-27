# wp-wordcount-rs

A fast, highly-optimized Rust conversion of the official WordPress block editor word count utility [`@wordpress/wordcount`](https://github.com/WordPress/gutenberg/tree/trunk/packages/wordcount).

This crate perfectly replicates the word, character (including spaces), and character (excluding spaces) counting rules and configuration options of the Gutenberg package. It is designed for Rust applications that need to compute matching metrics for post content, blocks, and text strings.

## Features

- **Exact Compatibility**: Perfectly replicates the behavior of the WordPress editor, including Edge cases.
- **HTML Tag & Comment Stripping**: Removes HTML tags and comment blocks when computing counts.
- **Shortcode Removal**: Dynamically compiles and strips configurable WordPress shortcodes (e.g., `[gallery]`).
- **Punctuation & Formatting Removal**: Removes standard and extended punctuation symbols when counting words.
- **Astral Character Preservation**: Treats surrogate pairs / astral characters (like emoji 💩) as single characters.
- **HTML Entity Support**: Translates/strips HTML entities (e.g. `&nbsp;`, `&#160;`, `&gt;`) to compute correct character/word boundaries.
- **High Performance**: Pre-compiles regular expressions once using `std::sync::OnceLock`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
wp-wordcount-rs = { git = "https://github.com/bhubbard/wp-wordcount-rs" }
```

## Quick Start

```rust
use wp_wordcount_rs::{count, Strategy, UserSettings, L10n};

fn main() {
    // 1. Basic word count
    let word_count = count("Hello world, this is Rust!", Strategy::Words, None);
    assert_eq!(word_count, 5);

    // 2. Character count including spaces (excluding formatting characters)
    let char_count = count("Hello world!", Strategy::CharactersIncludingSpaces, None);
    assert_eq!(char_count, 12);

    // 3. Count with Custom Settings (e.g., removing registered shortcodes)
    let user_settings = UserSettings {
        l10n: Some(L10n {
            strategy: None,
            shortcodes: vec!["custom_shortcode".to_string()],
        }),
        ..Default::default()
    };

    let post_content = "Word [custom_shortcode] another word";
    let custom_count = count(post_content, Strategy::Words, Some(&user_settings));
    assert_eq!(custom_count, 3);
}
```

## Counting Strategies

The crate supports three counting strategies, matching the WordPress API:
- `Strategy::Words` - Counts words, stripping tags, HTML comments, shortcodes, and removables (punctuation/symbols).
- `Strategy::CharactersExcludingSpaces` - Counts character code points, excluding spaces.
- `Strategy::CharactersIncludingSpaces` - Counts character code points, including spaces but excluding formatting and separator characters (like `\f`, `\n`, `\r`, `\t`, `\v`, `\u{00AD}`, etc.).

## License

Consistent with the Gutenberg repository, this project is licensed under the [GNU General Public License v2.0 or later (GPL-2.0-or-later)](https://www.gnu.org/licenses/old-licenses/gpl-2.0.html).
