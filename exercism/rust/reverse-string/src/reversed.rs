use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(original: &str) -> String {
  original.graphemes(true).rev().collect::<String>()
}
