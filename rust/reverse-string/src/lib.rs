extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(word: &str) -> String {
    UnicodeSegmentation::graphemes(word, true).rev().collect::<String>()
}
