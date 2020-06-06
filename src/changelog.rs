use colored::*;

pub fn prepare(repo: &git2::Repository, tag: &git2::Tag) -> Result<String, git2::Error> {
  let mut revwalk = repo.revwalk().unwrap();

  revwalk.set_sorting(git2::Sort::NONE).unwrap();

  let head = repo.head()?;
  let to = head.target().unwrap();
  let to = repo.find_commit(to)?.id().to_string();
  let from = tag.target()?.id().to_string();

  let range = format!("{}..{}", from, to);

  revwalk.push_range(&range)?;

  println!(
    "from : {} -> to: {}\n",
    &from[0..6].bold(),
    &to[0..6].bold()
  );

  let mut changelog = String::new();
  for comm in revwalk {
    let commit = repo.find_commit(comm?)?;
    if commit.parents().count() > 1 {
      continue;
    };
    let first_line = commit.message().unwrap().lines().nth(0).unwrap();
    let hash = commit.id().to_string();
    changelog.push_str(&format!("{} {}\n", hash, first_line)[..]);
  }
  Ok(changelog)
}
