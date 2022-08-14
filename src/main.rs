use std::fs::{self, File};
use std::io::{self, Result, Write, Error, ErrorKind};
use std::env;
use std::path::Path;
use std::process::Command;
use regex::Regex;
use chrono::{Datelike, NaiveDate, NaiveDateTime, Duration, Weekday};
use cairo::{Format,
  FontExtents, 
  TextExtents, 
  Context,
  Surface,
  ffi::{cairo_move_to, 
    cairo_show_text, 
    cairo_image_surface_create, 
    cairo_create, 
    cairo_font_extents,
    cairo_select_font_face, 
    cairo_set_font_size,
    cairo_text_extents,
    cairo_surface_get_reference_count,
    FONT_SLANT_NORMAL,
    FONT_WEIGHT_BOLD, cairo_surface_t,
  },
  ImageSurface, 
  ImageSurfaceData,  
};

// https://doc.rust-lang.org/book/ch03-05-control-flow.html
// https://doc.rust-lang.org/std/vec/struct.Vec.html
// create commits by specifing dates https://stackoverflow.com/questions/454734/how-can-one-change-the-timestamp-of-an-old-commit-in-git

fn text_to_dots(text: String) -> Vec<[u8; 7]> {
  // use font to render a text -> picture
  let mut surface = text_to_surface(text);
  let width: usize = surface.width() as usize;
  let height: usize = surface.height() as usize;
  save_surface_as_png(&surface);
  // println!("reference counter z {}", unsafe {cairo_surface_get_reference_count(surface.to_raw_none())});
  let data = surface.data().unwrap_or_else(|error| {
    panic!("no data in the surface: {}", error.to_string());
  });
  // println!("image data length {}", data.len());
  // for y in 0..height {
  //     for x in 0..width {
  //       // print!("{}", x * 4 + y * width * 4);
  //       let r = data[x * 4 + y * width * 4];
  //       let g = data[x * 4 + y * width * 4 + 1];
  //       let b = data[x * 4 + y * width * 4 + 2];
  //       let a = data[x * 4 + y * width * 4 + 3];
  //       print!("{}", if r > 0 || g > 0 || b > 0 || a > 0 {"X"} else {"_"});
  //   }
  //   println!("");
  // }

  // on a matrix highlight cells (dots) that correspond to the outlines
  // in other words: picture to matrix
  let mut dots = Vec::new();

  let threshold = 30.0;
  let box_size = height / 7;
  println!("box size {box_size}");
  for i_x in 0..(width / box_size) {
    let mut week: [u8; 7] = [0; 7];
    for i_y in 0..(height / box_size) {
      let mut total_box_color: u64 = 0;
      for y in i_y * box_size..(i_y + 1) * box_size{
        for x in i_x * box_size..(i_x + 1) * box_size {
          // print!("{}", x * 4 + y * width * 4);
          let r = data[x * 4 + y * width * 4];
          let g = data[x * 4 + y * width * 4 + 1];
          let b = data[x * 4 + y * width * 4 + 2];
          let a = data[x * 4 + y * width * 4 + 3];
          total_box_color += (r + g + b + a) as u64
        }
      }
      // print!(" {} ", if total_box_color as f64 / (4 * box_size * box_size) as f64 > 1.0 {"X"} else {"_"});
      // print!("{}", if total_box_color as f64 / (4 * box_size * box_size) as f64 > threshold {"X"} else {"_"});
      let busy_day = if total_box_color as f64 / (4 * box_size * box_size) as f64 > threshold {1} else {0};
      week[i_y] = busy_day;
    }
    // println!("");
    dots.push(week);
  }

  dots
}

fn dots_to_dates(start_date: NaiveDate, dots: Vec<[u8; 7]>) -> Vec<[Option<NaiveDateTime>; 7]> {
  let mut dates = Vec::new();
  
  let mut column: usize = 0;
  while column < dots.len() {
    let mut week: [Option<NaiveDateTime>; 7] = [None; 7];
    for week_day in 0..7 {
      let i: usize = week_day;
      if dots[column][i] > 0 {
        let days: i64 = (i + 7 * column).try_into().unwrap();
        let duration = Duration::days(days);
        let date: NaiveDate = start_date + duration;
        let min: u32 = 10;
        let sec: u32 = 11;
        week[i] = Some(date.and_hms(9, min, sec));
      }
    }
    dates.push(week);
    column += 1;
  }
  dates
}

fn dots_to_dates_flat(start_date: NaiveDate, dots: Vec<[u8; 7]>) -> Vec<NaiveDateTime> {
  let mut dates = Vec::new();
  
  let mut column: usize = 0;
  while column < dots.len() {
    for week_day in 0..7 {
      let i: usize = week_day;
      if dots[column][i] > 0 {
        let days: i64 = (i + 7 * column).try_into().unwrap();
        let duration = Duration::days(days);
        let date: NaiveDate = start_date + duration;
        for n in 0..dots[column][i] as u32 {
          let min: u32 = n / 60;
          let sec: u32 = n % 60;
          dates.push(date.and_hms(9, min, sec));
        }
      }
    }
    column += 1;
  }
  dates
}

fn print_dots(dots: &Vec<[u8; 7]>) {
  for week_day in 0..7 {
    let mut column = 0;
    while column < dots.len() {
      let i = dots[column][week_day];
      print!("{i}");
      column +=1;
    }
    println!("");
  }
}

fn print_dates(dates: &Vec<[Option<NaiveDateTime>; 7]>) {
  for week_day in 0..7 {
    let mut column = 0;
    while column < dates.len() {
      let i = dates[column][week_day];
      if i.is_some() {
        print!("{} ", i.expect("date is ok").format("%Y-%m-%d").to_string());
      } else {
        print!("xxxx-xx-xx ");
      }
      column +=1;
    }
    println!("");
  }
}

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

fn git_init() -> Result<()> {
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

fn git_add(path: &Path) -> Result<()> {
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

fn git_commit(i: usize, date: &String) -> Result<()> {
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

// Convert tutorial from C to Rust
// https://cairographics.org/tutorial/#L1understandingtext

// add default functions to bad/shallow libraries
// https://gist.github.com/ChrisWellsWood/84421854794037e760808d5d97d21421

trait Empty<T> {
  fn empty() -> T;
}

impl Empty<FontExtents> for FontExtents {
  fn empty() -> Self {
    FontExtents {
      ascent: 0.0,
      descent: 0.0,
      height: 0.0,
      max_x_advance: 0.0,
      max_y_advance: 0.0,
    }
  }
}

impl Empty<TextExtents> for TextExtents {
  fn empty() -> Self {
    TextExtents {
      x_bearing: 0.0,
      y_bearing: 0.0,
      width: 0.0,
      height: 0.0,
      x_advance: 0.0,
      y_advance: 0.0 
    }
  }
}

fn text_to_png(text: String) {
  let surface = text_to_surface(text);
  save_surface_as_png(&surface);
}

fn text_to_surface(text: String) -> ImageSurface {
  // output size
  let width = 450;
  let height = 60;
  
  let surface_ptr = unsafe {cairo_image_surface_create(
    i32::from(Format::ARgb32), width, height)};
  // println!("reference counter a {}", unsafe {cairo_surface_get_reference_count(surface_ptr)});
  let surface: ImageSurface = unsafe {ImageSurface::from_raw_full(surface_ptr).unwrap()};
  // println!("reference counter b {}", unsafe {cairo_surface_get_reference_count(surface_ptr)});
  let context: Context = Context::new(&surface).unwrap();
  let context_ptr = context.to_raw_none();
  // println!("reference counter c {}", unsafe {cairo_surface_get_reference_count(surface_ptr)});

  let font_family = b"Source Code Pro\0".as_ptr();
  unsafe {cairo_select_font_face(context_ptr, font_family as *const i8,
    FONT_SLANT_NORMAL,
    FONT_WEIGHT_BOLD)};

  unsafe {cairo_set_font_size(context_ptr, 70.0)};

  // https://gtk-rs.org/gtk-rs-core/stable/latest/docs/cairo/struct.FontExtents.html
  let mut fe: FontExtents = FontExtents::empty();
  unsafe {cairo_font_extents(context_ptr, &mut fe)};

  let mut te: TextExtents = TextExtents::empty();
  let chars = text.as_bytes().as_ptr();
  unsafe {cairo_text_extents(context_ptr, chars as *const i8, &mut te)};
  
  let x = te.x_bearing;
  let y = fe.height - fe.descent - 15.0;
  println!("{}  {} {} {}", te.x_bearing, te.width, fe.descent, fe.height);
  
  unsafe {cairo_move_to(context_ptr, x, y)};

  unsafe {cairo_show_text(context_ptr, chars as *const i8)};

  surface
}

fn save_surface_as_png(surface: &ImageSurface) {
  let mut file = File::create("text.png").unwrap();
  surface.write_to_png(&mut file).unwrap();
}

fn test_cairo() {
  // output size
  let width = 1024;
  let height = 768;
  
  let surface = unsafe {cairo_image_surface_create(
    i32::from(Format::ARgb32), width, height)};
  let context = unsafe {cairo_create(surface)};

  let font_family = b"Source Code Pro\0".as_ptr();
  unsafe {cairo_select_font_face(context, font_family as *const i8,
    FONT_SLANT_NORMAL,
    FONT_WEIGHT_BOLD)};

  unsafe {cairo_set_font_size(context, 60.0)};

  // https://gtk-rs.org/gtk-rs-core/stable/latest/docs/cairo/struct.FontExtents.html
  let mut fe: FontExtents = FontExtents::empty();
  unsafe {cairo_font_extents(context, &mut fe)};

  let alphabet = "AbCdEfGhIjKlMnOpQrStUvWxYz";
  
  let mut te: TextExtents = TextExtents::empty();
  for (i, letter) in alphabet.chars().enumerate() {
    let text: [i8; 2] = [letter as i8, '\0' as i8];
    let text_ptr = text.as_ptr();
    unsafe {cairo_text_extents(context, text_ptr, &mut te)};
    
    let x = (i as f64) * 40.0 + 25.0 - te.x_bearing - te.width / 2.0;
    let y = 60.0 - fe.descent + fe.height / 2.0;
    unsafe {cairo_move_to(context, x, y)};
  
    unsafe {cairo_show_text(context, text_ptr)};
  }

  let result = unsafe {ImageSurface::from_raw_full(surface)};
  if result.is_ok() {
    let image = result.expect("image surface wow");
    let file_result = File::create("text.png");
    if file_result.is_ok() {
      let mut file = file_result.expect("file is open");
      let write_result = image.write_to_png(&mut file);
      if write_result.is_err() {
        write_result.err();
      }
    }
  }

}

pub type Week<D> = [Option<D>; 7];
pub type VecOfWeeks<D> = Vec<Week<D>>;

struct VecOfWeeksIter<'a, D> {
  remaining_weeks: &'a [Week<D>],
  week_day: usize,
}

impl<D> Default for VecOfWeeksIter<'_, D> {
  fn default() -> Self {
    VecOfWeeksIter {
      remaining_weeks: &[],
      week_day: 0,
    }
  }
}

impl<'a, D> Iterator for VecOfWeeksIter<'a, D> {
  type Item = &'a D;

  fn next(&mut self) -> Option<Self::Item> {
    let mut value = None;
    while !value.is_some() && self.remaining_weeks.len() > 0 {
      value = self.remaining_weeks[0][self.week_day].as_ref();
    
      self.week_day += 1;
      if self.week_day >= 7 {
        self.week_day = 0;
        self.remaining_weeks = &self.remaining_weeks[1..];
      }
    }
    value
  }
}

fn wrap_into_iter<'a, D>(data: &'a Vec<[Option<D>; 7]>) -> VecOfWeeksIter<'a, D> {
  VecOfWeeksIter {
    remaining_weeks: &data[..],
    week_day: 0
  }
}

fn main() {
  let text: String = "CAPPUCCINO!".to_string();
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

#[test]
fn test_borrowing_iterator() {
  // iterator is not tied to NaiveDateTime, so we will use just numbers in the test ;)
  let dates = vec![
    [None; 7],
    [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)],
    [Some(8), None, Some(9), None, Some(10), None, Some(11)],
    [None; 7],
    [Some(12), Some(13), None, None, None, None, None],
  ];
  let numbers: Vec<i32> = wrap_into_iter(&dates).copied().collect();
  assert_eq!(numbers, vec![1,2,3,4,5,6,7,8,9,10,11,12,13]);
}