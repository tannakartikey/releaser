mod tags;
use colored::*;
use git2::{Repository, Sort};
use std::str;

const REPO_URL: &str = "/home/kartikey/source/sidecar_api_clone";

fn main() {
  let repo = Repository::open(REPO_URL).expect("Could not open the repo");
  let tag = tags::find_latest(&repo);

  println!(
    "\nLast tag found: {name:} ({id:})\n",
    id = &(tag.id().to_string())[0..6],
    name = tag.name().unwrap().bold().yellow().on_blue()
  );

  let mut revwalk = repo.revwalk().unwrap();

  revwalk.set_sorting(Sort::NONE).unwrap();

  let head = repo.head().unwrap();
  let to = head.target().unwrap();
  let to = repo.find_commit(to).unwrap();
  let to = to.id().to_string();
  let from = tag.target().unwrap();
  let from = from.id().to_string();

  let range = format!("{}..{}", from, to);

  revwalk.push_range(&range).unwrap();

  println!(
    "from : {} -> to: {}\n",
    &from[0..6].bold(),
    &to[0..6].bold()
  );

  for comm in revwalk {
    let commit = repo.find_commit(comm.unwrap()).unwrap();
    if commit.parents().count() > 1 {
      continue;
    };
    let first_line = commit.message().unwrap().lines().nth(0).unwrap();
    let hash = commit.id().to_string();
    let hash = &hash[0..6];
    println!("{} {}", hash.yellow(), first_line,);
  }
}
