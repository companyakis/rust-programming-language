fn main() {

  let sales_department = department_info("Sales".to_string(), "Aygün Kaplan".to_string(), 6, 540_000);

  println!("Sales department yearly budget: ${}", sales_department.yearly_usd_budget) // Sales department yearly budget: $540000

}

#[derive(Debug)]
struct Department {
  name: String,
  head: String,
  number_of_employees: u8,
  yearly_usd_budget: u32
} 

fn department_info(name: String, head: String, number_of_employees: u8, yearly_usd_budget: u32) -> Department {
  Department {
    name: name,
    head: head,
    number_of_employees: number_of_employees,
    yearly_usd_budget: yearly_usd_budget
  }
}
