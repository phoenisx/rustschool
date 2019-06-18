extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(original: &str) -> String {
  return UnicodeSegmentation::graphemes(original, true)
    .rev()
    .collect::<String>();
}
