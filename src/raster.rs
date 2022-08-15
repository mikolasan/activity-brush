use cairo::{Format,
  FontExtents, 
  TextExtents, 
  Context,
  ffi::{cairo_move_to, 
    cairo_show_text, 
    cairo_image_surface_create, 
    cairo_create, 
    cairo_font_extents,
    cairo_select_font_face, 
    cairo_set_font_size,
    cairo_text_extents,
    FONT_SLANT_NORMAL,
    FONT_WEIGHT_BOLD,
  },
  ImageSurface, 
};
use std::fs::File;

pub fn text_to_dots(text: String) -> Vec<[u8; 7]> {
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

#[allow(dead_code)]
fn text_to_png(text: String) {
  let surface = text_to_surface(text);
  save_surface_as_png(&surface);
}

#[allow(dead_code)]
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
