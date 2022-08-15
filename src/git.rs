use std::env;
use std::io::{self, Result, Write, Error, ErrorKind};
use std::path::Path;
use std::process::Command;

// create commits by specifing dates https://stackoverflow.com/questions/454734/how-can-one-change-the-timestamp-of-an-old-commit-in-git

pub fn git_init() -> Result<()> {
  let output = Command::new("git")
    .arg("init")
    .output()?;
  if !output.status.success() {
    // println!("{}", output.stderr);
    io::stderr().write_all(&output.stderr)?;
    return Err(Error::new(ErrorKind::Other, "git init failed"));
  }
  println!("git initialized");
  Ok(())
}

pub fn git_add(path: &Path) -> Result<()> {
  let output = Command::new("git")
    .arg("add")
    .arg(path.display().to_string())
    .output()?;
  if !output.status.success() {
    // println!("{}", output.stderr);
    io::stderr().write_all(&output.stderr)?;
    return Err(Error::new(ErrorKind::Other, "git add failed"));
  }
  println!("file added to tracked files");
  Ok(())
}

pub fn git_commit(i: usize, date: &String) -> Result<()> {
  // GIT_COMMITTER_DATE="2017-10-08T09:51:07" git commit --all --message="commit 1" --date="2017-10-08T09:51:07"
  env::set_var("GIT_COMMITTER_DATE", format!("{date}"));
  let output = Command::new("git")
    .arg("commit")
    .arg("--all")
    .arg(format!("--message=\"commit {i}\""))
    .arg(format!("--date=\"{date}\""))
    .arg("--author=\"Nikolay Neupokoev <ne.nikolay@yandex.com>\"")
    .output()?;
  if !output.status.success() {
    // println!("{}", output.stderr);
    io::stderr().write_all(&output.stderr)?;
    return Err(Error::new(ErrorKind::Other, "git commit failed"));
  }
  println!("commited {i} {date}");
  Ok(())
}
