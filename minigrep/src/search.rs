pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  let mut search_array = Vec::new();
  for line in content.lines() {
    if line.contains(query) {
      search_array.push(line);
    }
  }
  search_array
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_search() {
    let search_str = "selv";
    // parah string is a list of lines, which acts like pre-formatted text, thus don't use
    // unnecessary spaces or tabs, which might give unexpected results.
    let parah = "\
Aim for your goal,
and things will come to
their place by themselves...
    ";

    assert_eq!(
      vec!["their place by themselves..."], // should match line three...
      super::search(search_str, parah)
    );
  }
}
