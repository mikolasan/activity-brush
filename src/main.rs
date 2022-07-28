use std::fs::File;

use cairo::{Format,
  FontExtents, 
  TextExtents, 
  ffi::{cairo_move_to, 
    cairo_show_text, 
    cairo_image_surface_create, 
    cairo_create, 
    cairo_font_extents, 
    cairo_text_extents,
    STATUS_SUCCESS,
    cairo_status_t,
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
  let width = 120;
  let height = 120;
  
  unsafe{
    let surface = cairo_image_surface_create(
      i32::from(Format::ARgb32), width, height);
    let context = cairo_create(surface);

    // https://gtk-rs.org/gtk-rs-core/stable/latest/docs/cairo/struct.FontExtents.html
    let mut fe: FontExtents = FontExtents::empty();
    cairo_font_extents(context, &mut fe);

    let alphabet = "AbCdEfGhIjKlMnOpQrStUvWxYz";
    
    let mut te: TextExtents = TextExtents::empty();
    for (i, letter) in alphabet.chars().enumerate() {
      let text: [i8; 2] = [letter as i8, '\0' as i8];
      
      cairo_text_extents(context, &text as *const i8, &mut te);
      
      let x = (i as f64) + 0.5 - te.x_bearing - te.width / 2.0;
      let y = 0.5 - fe.descent + fe.height / 2.0;
      cairo_move_to(context, x, y);
    
      cairo_show_text(context, &text as *const i8);
    }

    let result = ImageSurface::from_raw_full(surface);
    if result.is_ok() {
      let image = result.expect("image surface wow");
      let mut file_result = File::create("text.png");
      if file_result.is_ok() {
        let file = file_result.expect("file is open");
        image.write_to_png(file);
      }
    }

  }

}