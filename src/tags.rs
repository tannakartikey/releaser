use crate::version;

pub fn find_latest(repo: &git2::Repository) -> Result<git2::Tag, git2::Error> {
  let tags = repo.tag_names(Some("*"))?;
  let mut tags: Vec<&str> = tags.iter().map(|tag| tag.unwrap()).collect();
  alphanumeric_sort::sort_str_slice(&mut tags);

  let tag: &str = tags.last().unwrap();
  let tag = repo.revparse_single(tag)?.peel_to_tag()?;
  Ok(tag)
}

pub fn new_tag(version: &version::Version, tag: &git2::Tag) -> () {
  let tag = parse_tag(tag);
  println!("{:?}", tag);
  match version.value {
    version::VersionValue::MAJOR => println!("{}", "Major release!"),
    version::VersionValue::MINOR => println!("{}", "Minor release!"),
    version::VersionValue::PATCH => println!("{}", "Patch release!"),
  };
}

fn parse_tag<'a>(tag: &'a git2::Tag) -> Vec<&'a str> {
  return tag.name().unwrap().split_terminator('.').collect();
}
