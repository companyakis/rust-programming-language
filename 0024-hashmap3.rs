use std::collections::HashMap;

fn main() {

  let mut salaries: HashMap<String, u16> = HashMap::new();

  salaries.insert("Bilge".to_string(), 65_000);

  salaries.insert(String::from("Kutluk"), 52000);

  salaries.insert(String::from("Sevda"), 50000);

  // key and value 

  for (name, salary) in &salaries {

    println!("Employee: {name} and salary : {salary}")
  }

// Employee: Kutluk and salary : 52000
// Employee: Bilge and salary : 65000
// Employee: Sevda and salary : 50000

}


