use std::collections::HashMap;

fn main() {

  // update!

  let mut salaries: HashMap<String, u16> = HashMap::new();

  salaries.insert("Bilge".to_string(), 65_000);
  salaries.insert(String::from("Kutluk"), 52000);
  salaries.insert(String::from("Sevda"), 50000);

  // update salary

  salaries.insert(String::from("Sevda"), 43000);

  let salary_sevda = salaries.get(&String::from("Sevda")).copied().unwrap_or(0);

  println!("Sevda's new salary: {salary_sevda}") // Sevda's new salary: 43000

}



