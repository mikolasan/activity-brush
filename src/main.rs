use cairo::{Format, FontExtents, TextExtents, ffi::{cairo_move_to, cairo_show_text, cairo_image_surface_create, cairo_create, cairo_font_extents, cairo_text_extents}, };

fn main() {

  let width = 120;
  let height = 120;
  
  unsafe{
    let surface = cairo_image_surface_create(i32::from(Format::ARgb32), width, height);
    let cr = cairo_create(surface);

// cairo_font_extents_t fe;
// cairo_text_extents_t te;
// char alphabet[] = "AbCdEfGhIjKlMnOpQrStUvWxYz";
// char letter[2];

// cairo_font_extents (cr, &fe);
// for (i=0; i < strlen(alphabet); i++) {
//     *letter = '\0';
//     strncat (letter, alphabet + i, 1);
//     cairo_text_extents (cr, letter, &te);
//     cairo_move_to (cr, i + 0.5 - te.x_bearing - te.width / 2,
//             0.5 - fe.descent + fe.height / 2);
//     cairo_show_text (cr, letter);
// }

    let mut fe: FontExtents;
    let te: TextExtents;
    let alphabet = "AbCdEfGhIjKlMnOpQrStUvWxYz";
    
    cairo_font_extents(cr, &mut fe);

    for (i, letter) in alphabet.chars().enumerate() {
      let text: [i8; 2] = [letter as i8, '\0' as i8];
      
      cairo_text_extents(cr, &text as *const i8, &mut te);
      
      let x = (i as f64) + 0.5 - te.x_bearing - te.width / 2.0;
      let y = 0.5 - fe.descent + fe.height / 2.0;
      cairo_move_to(cr, x, y);
    
      cairo_show_text(cr, &text as *const i8);
    }
  }

}