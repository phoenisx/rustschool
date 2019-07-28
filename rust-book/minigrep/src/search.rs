pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  content
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  content
    .lines()
    .filter(|line| line.to_lowercase().contains(&(query.to_lowercase())))
    .collect()
}

#[cfg(test)]
mod tests {

  #[test]
  fn test_search_case_sensitive() {
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

  #[test]
  fn test_search_case_insensitive() {
    let search_str = "ThE";
    // parah string is a list of lines, which acts like pre-formatted text, thus don't use
    // unnecessary spaces or tabs, which might give unexpected results.
    let parah = "\
Aim for THE Goal,
and things will come to
Their place by themselves...
    ";

    assert_eq!(
      vec!["Aim for THE Goal,", "Their place by themselves..."], // should match line three...
      super::search_case_insensitive(search_str, parah)
    );
  }
}
