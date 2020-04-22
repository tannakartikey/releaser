pub fn find_latest(repo: &git2::Repository) -> git2::Tag<'_> {
  let tags = repo.tag_names(Some("*")).expect("No tags found!");
  let mut tags: Vec<&str> = tags.iter().map(|tag| tag.unwrap()).collect();
  alphanumeric_sort::sort_str_slice(&mut tags);

  let tag: &str = tags.last().unwrap();
  let tag = repo.revparse_single(tag).unwrap();
  let tag: &git2::Tag = tag.as_tag().unwrap();
  return tag.clone();
}
