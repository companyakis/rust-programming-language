fn main() {

  let vec_1: Vec<i16> = Vec::new();

  println!("Empty Vector 1: {:?}", vec_1); // Vector 1: []

  let years: Vec<u16> = vec![2022, 2023, 2024];

  let names: Vec<String> = vec!["Mustafa".to_string(), "Aygün".to_string()];

  for name in names {
    println!("Name: {}", &name)
  }

}

// Name: Mustafa
// Name: Aygün

