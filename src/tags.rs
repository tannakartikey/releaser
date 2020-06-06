use crate::version;

pub fn find_latest(repo: &git2::Repository) -> Result<git2::Tag, git2::Error> {
  let tags = repo.tag_names(Some("*"))?;
  let mut tags: Vec<&str> = tags.iter().map(|tag| tag.unwrap()).collect();
  alphanumeric_sort::sort_str_slice(&mut tags);

  let tag: &str = tags.last().unwrap();
  let tag = repo.revparse_single(tag)?.peel_to_tag()?;
  Ok(tag)
}

pub fn new_tag<'a>(patch_version: &version::Version, tag: &'a git2::Tag) -> String {
  let tag = parse_tag(tag);

  let new_version: Vec<String> = tag
    .iter()
    .enumerate()
    .map(|(index, e)| increase_version(e, index, &patch_version.value))
    .collect();
  let new_version = new_version.join(".");

  return new_version;
}

fn increase_version(mut value: &str, index: usize, version: &version::VersionValue) -> String {
  if *version == version::VersionValue::MAJOR && index == 0 {
    let first_letter = value.chars().next().unwrap();
    if first_letter.is_alphabetic() {
      value = &value[1..];
      return first_letter.to_string() + &((value.parse::<i32>().unwrap()) + 1).to_string();
    } else {
      return ((value.parse::<i32>().unwrap()) + 1).to_string();
    }
  } else if *version == version::VersionValue::MINOR && index == 1 {
    return ((value.parse::<i32>().unwrap()) + 1).to_string();
  } else if *version == version::VersionValue::PATCH && index == 2 {
    return ((value.parse::<i32>().unwrap()) + 1).to_string();
  } else {
    return value.to_string();
  }
}

fn parse_tag<'a>(tag: &'a git2::Tag) -> Vec<&'a str> {
  return tag.name().unwrap().split_terminator('.').collect();
}
