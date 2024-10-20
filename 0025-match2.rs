fn main() {

  let month: u8 = 6;

  let month_query = match month {
    1 => "January",
    2 => "February",
    3 => "March",
    4 => "April",
    5 => "May",
    6 => "June",
    7 => "July",
    8 => "August",
    9 => "September",
    10 => "October",
    11 => "November",
    12 => "December",
    _ => "Use a month number between 1 and 12!"
  };

  println!("Month query result: {month_query}") // Month query result: June

}

