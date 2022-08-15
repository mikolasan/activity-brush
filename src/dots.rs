
#[allow(dead_code)]
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
