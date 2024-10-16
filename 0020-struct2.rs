fn main() {

  // mutable!

  let mut dep_fintech = Department {
    name: "FinTech Department".to_string(),
    head: String::from("Mustafa Büyükdereli"),
    number_of_employees: 12,
    yearly_usd_budget: 2_500_000
  };

  dep_fintech.yearly_usd_budget = 2_650_000;

  println!("FinTech Department yearly budget ${}.", dep_fintech.yearly_usd_budget);

}

#[derive(Debug)]
struct Department {
  name: String,
  head: String,
  number_of_employees: u8,
  yearly_usd_budget: u32
} 

// FinTech Department yearly budget $2650000.
