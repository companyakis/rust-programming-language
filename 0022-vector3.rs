fn main() {

  // Reading Elements of Vectors

  let years: Vec<u16> = vec![1990, 2018, 2020, 2024, 2025];

  let first_year = &years[0];

  println!("First year is {}.", first_year);

  // last year?

  let last_year = &years[years.len() - 1];

  println!("Last year is {}.", last_year) // Last year is 2025. 
  
}


