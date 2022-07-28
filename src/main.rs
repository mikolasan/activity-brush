use std::fs::File;
use chrono::{Datelike, NaiveDate, NaiveDateTime, Duration};
use cairo::{Format,
  FontExtents, 
  TextExtents, 
  ffi::{cairo_move_to, 
    cairo_show_text, 
    cairo_image_surface_create, 
    cairo_create, 
    cairo_font_extents,
    cairo_select_font_face, 
    cairo_set_font_size,
    cairo_text_extents,
    FONT_SLANT_NORMAL,
    FONT_WEIGHT_BOLD, cairo_surface_t,
  }, ImageSurface
  
};

// https://doc.rust-lang.org/book/ch03-05-control-flow.html
// https://doc.rust-lang.org/std/vec/struct.Vec.html
// create commits by specifing dates https://stackoverflow.com/questions/454734/how-can-one-change-the-timestamp-of-an-old-commit-in-git

fn text_to_dots(text: String) -> Vec<[u8; 7]> {
    // load font

    // use font to render a text -> picture
    let surface = text_to_surface(text);
    let result = unsafe {ImageSurface::from_raw_full(surface)};
    if result.is_ok() {
      let mut image = result.expect("image surface wow");
      let image_result = image.data();
      if image_result.is_ok() {
        let data = image_result.expect("data is there");
        println!("image surface length {}", data.len());
      } else {
        let e = image_result.expect_err("borrow error then");
        println!("no data in the surface: {}", e.to_string());
      }
    } else {
      println!("image surface is not created");
    }

    // on a matrix highlight cells (dots) that correspond to the outlines
    // in other words: picture to matrix

    
    let mut dots = Vec::new();
    let mut week: [u8; 7] = [0; 7];

    dots.push(week);
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
                week[i] = Some(date.and_hms(9, 10, 11));
            }
        }
        dates.push(week);
        column += 1;
    }
    dates
}

fn print_dots(dots: Vec<[u8; 7]>) {
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

fn print_dates(dates: Vec<[Option<NaiveDateTime>; 7]>) {
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

fn dates_to_commits() {
    
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
  save_surface_as_png(surface);
}

fn text_to_surface(text: String) -> *mut cairo_surface_t {
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

  let mut te: TextExtents = TextExtents::empty();
  let chars = text.as_bytes().as_ptr();
  unsafe {cairo_text_extents(context, chars as *const i8, &mut te)};
  
  let x = te.x_bearing;
  let y = fe.height - fe.descent;
  // println!("{}  {} {} {}", te.x_bearing, te.width, fe.descent, fe.height);
  
  unsafe {cairo_move_to(context, x, y)};

  unsafe {cairo_show_text(context, chars as *const i8)};

  surface
}

fn save_surface_as_png(surface: *mut cairo_surface_t) {
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

fn main() {
  let text: String = "Hello".to_string();
  let dots = text_to_dots(text);
  // let dots = vec![
  //     [0,1,1,1,1,1,0],
  //     [0,1,1,0,1,1,0],
  //     [1,1,1,1,1,1,1]
  // ];
  //print_dots(dots);

  // let start_date = NaiveDate::from_ymd(2022, 12, 29);
  // println!("start date: {}", start_date.format("%Y-%m-%d").to_string());
  // let dates = dots_to_dates(start_date, dots);
  // print_dates(dates);

  // text_to_png(text)
  // text_to_dots(text);
}