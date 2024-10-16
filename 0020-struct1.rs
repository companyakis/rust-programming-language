fn main() {

  let dep_fintech = Department {
    name: "FinTech Department".to_string(),
    head: String::from("Mustafa Büyükdereli"),
    number_of_employees: 12,
    yearly_usd_budget: 2_500_000
  };

  println!("FinTech Department info: {:?}", dep_fintech);

  println!("FinTech Department head is {}.", dep_fintech.head);

}

#[derive(Debug)]
struct Department {
  name: String,
  head: String,
  number_of_employees: u8,
  yearly_usd_budget: u32
} 

// FinTech Department info: Department { name: "FinTech Department", head: "Mustafa Büyükdereli", number_of_employees: 12, yearly_usd_budget: 2500000 }
// FinTech Department head is Mustafa Büyükdereli.
