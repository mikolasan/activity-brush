use chrono::{NaiveDate, NaiveDateTime, Duration};

// About iterators
// https://aloso.github.io/2021/03/09/creating-an-iterator

pub type Week<D> = [Option<D>; 7];
// type VecOfWeeks<D> = Vec<Week<D>>;

pub struct VecOfWeeksIter<'a, D> {
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

pub fn wrap_into_iter<'a, D>(data: &'a Vec<[Option<D>; 7]>) -> VecOfWeeksIter<'a, D> {
  VecOfWeeksIter {
    remaining_weeks: &data[..],
    week_day: 0
  }
}

#[allow(dead_code)]
pub fn dots_to_dates(start_date: NaiveDate, dots: Vec<[u8; 7]>) -> Vec<[Option<NaiveDateTime>; 7]> {
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

#[allow(dead_code)]
pub fn dots_to_dates_flat(start_date: NaiveDate, dots: Vec<[u8; 7]>) -> Vec<NaiveDateTime> {
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

#[allow(dead_code)]
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