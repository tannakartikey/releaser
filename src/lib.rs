use colored::*;
use git2::Repository;
mod changelog;
pub mod config;
mod tags;
mod version;
use std::error::Error;

const REPO_URL: &str = "/home/kartikey/source/sidecar_api_clone";

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
  let version = version::Version::new(config.version);

  let repo = Repository::open(REPO_URL)?;
  let tag = tags::find_latest(&repo)?;
  let changelog = changelog::prepare(&repo, &tag)?;
  let release_tag = tags::new_tag(&version, &tag);

  println!(
    "\nLast tag found: {name:} ({id:})\n",
    id = &(tag.id().to_string())[0..6],
    name = tag.name().unwrap().bold().yellow().on_blue()
  );

  println!(
    "{title:}\n\n{changelog:}",
    title = "Changelog:".bold().yellow().on_blue(),
    changelog = changelog
  );

  println!("Upgrade type: {:?}", version.value);

  println!(
    "\nNew version will be: {}\n",
    release_tag.bold().yellow().on_blue()
  );

  repo
    .tag(
      &release_tag,
      &repo
        .revparse_single(&repo.head().unwrap().target().unwrap().to_string())
        .unwrap(),
      &repo.signature().unwrap(),
      &changelog,
      false,
    )
    .unwrap();

  Ok(())
}
