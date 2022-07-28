use std::fs::File;

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
    FONT_WEIGHT_BOLD,
  }, ImageSurface
  
};

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

fn main() {
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
    
    unsafe {cairo_text_extents(context, &text as *const i8, &mut te)};
    
    let x = (i as f64) * 40.0 + 25.0 - te.x_bearing - te.width / 2.0;
    let y = 60.0 - fe.descent + fe.height / 2.0;
    unsafe {cairo_move_to(context, x, y)};
  
    unsafe {cairo_show_text(context, &text as *const i8)};
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