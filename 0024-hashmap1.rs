use std::collections::HashMap;

fn main() {

  let mut salaries: HashMap<String, u16> = HashMap::new();

  salaries.insert("Bilge".to_string(), 65_000);

  salaries.insert(String::from("Kutluk"), 52000);

  salaries.insert(String::from("Sevda"), 50000);

  println!("Salaries: {:?}", salaries); // Salaries: {"Bilge": 65000, "Kutluk": 52000, "Sevda": 50000}

}
