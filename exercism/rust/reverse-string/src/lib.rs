mod reverse_string;

pub fn reverse(input: &str) -> String {
  reverse_string::grapheme_reverse(input)
}
