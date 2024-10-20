use std::collections::HashMap;

fn main() {

  let mut salaries: HashMap<String, u16> = HashMap::new();

  salaries.insert("Bilge".to_string(), 65_000);

  salaries.insert(String::from("Kutluk"), 52000);

  salaries.insert(String::from("Sevda"), 50000);

  // get method

  let employee_aygun = "Aygün".to_string();

  let salary_of_aygun = salaries.get(&employee_aygun).copied().unwrap_or(0);

  println!("Does the hashmap the salary info: {salary_of_aygun}"); // Does the hashmap the salary info: 0

  // without a variable!

  println!("Sevda's salary: {}", salaries.get(&String::from("Sevda")).copied().unwrap_or(0)) // Sevda's salary: 50000

}


