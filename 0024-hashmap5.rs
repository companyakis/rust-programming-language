use std::collections::HashMap;

fn main() {

  // remove a key-value pair from a hash map using the remove method

  let mut salaries: HashMap<String, u16> = HashMap::new();

  salaries.insert("Bilge".to_string(), 65_000);
  salaries.insert(String::from("Kutluk"), 52000);
  salaries.insert(String::from("Sevda"), 50000);

  // remove

  salaries.remove(&String::from("Sevda"));

  for (name, salary) in &salaries {

    println!("Name: {name} and salary: {salary}")
  }

}

// Name: Kutluk and salary: 52000
// Name: Bilge and salary: 65000


