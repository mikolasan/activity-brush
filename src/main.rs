use std::fs::{self, File};
use std::io::{self, Result, Write, ErrorKind};
use std::env;
use std::path::Path;
use chrono::{NaiveDate, NaiveDateTime, Weekday};

mod dates;
use dates::{wrap_into_iter, dots_to_dates};
mod dots;
mod git;
use git::{git_init, git_add, git_commit};
mod raster;
use raster::text_to_dots;


fn directory_exists(path: &Path) -> Result<bool> {
  let result = fs::metadata(path).map(|metadata| metadata.is_dir());
  if let Err(error) = result {
    match error.kind() {
      ErrorKind::NotFound => {
        println!("no metadata, means no directory");
        return Ok(false);
      }
      _ => {
        println!("this kind: {}", error.kind());
        return Err(error);
      }
    }
  }
  result
}

fn ask_for_confirmation(prompt: &String) -> Result<bool> {
  let mut input = String::new();
  println!("{prompt}");
  let confirmation = io::stdin()
    .read_line(&mut input)
    .map(|_| input.trim_end())?;

  let answer = match confirmation {
    "Y" => true,
    "N" => false,
    _ => {
      println!("You must print 'Y' or 'N' only!\nI'll ask again...");
      return ask_for_confirmation(prompt);
    }
  };
  Ok(answer)
}

fn dates_to_commits<'a>(date_iterator: impl Iterator<Item = &'a NaiveDateTime>) -> Result<()> {
  // make temp dir
  let git_repo_dir = "temp_git";
  let repo_root = Path::new(git_repo_dir);
  if directory_exists(repo_root)? {
    let prompt = format!("Do you want do delete '{}' and all its content? (Y/N)", repo_root.display());
    if ask_for_confirmation(&prompt)? {
      match fs::remove_dir_all(repo_root) {
        Ok(_) => println!("Removed!"),
        Err(e) => println!("cannot remove: {e}"),
      }
    } else {
      println!("okay, not deleting this directory")
    }
  }
  
  fs::create_dir(repo_root)?;
  let return_path = env::current_dir()?;
  env::set_current_dir(&repo_root)?;
  println!("Changed working directory to {}", repo_root.display());
  
  git_init()?;
  
  let work_file = "work.txt";
  let file_path = Path::new(work_file);

  let mut file = File::create(file_path)?;
  
  // initial commit
  git_add(file_path)?;
  
  let mut consecutive_counter: usize = 0;
  for date_time in date_iterator {
    let date = date_time.format("%Y-%m-%dT%H:%M:%S").to_string();

    file.write_all(date.as_bytes())?;
    git_commit(consecutive_counter, &date)?;
    
    consecutive_counter += 1;
  }

  //  git remote add origin git@github.com:boooobs/test.git
  //  git push -u --force origin master

  env::set_current_dir(&return_path)?;
  
  Ok(())
}

fn main() {
  let text: String = "hello".to_string();
  let dots = text_to_dots(text);
  // print_dots(&dots);

  let start_date = NaiveDate::from_weekday_of_month(2021, 08, Weekday::Sun, 2);
  println!("start date: {}", start_date.format("%Y-%m-%d").to_string());
  let dates = dots_to_dates(start_date, dots);
  let it = wrap_into_iter(&dates);
  
  // let dates = dots_to_dates_flat(start_date, dots);
  // let it = dates.iter();

  match dates_to_commits(it) {
    Ok(_) => {},
    Err(e) => println!("Error happened in 'dates_to_commits': {e}"),
  }
}
