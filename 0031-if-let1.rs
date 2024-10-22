fn main() {

  // number of employees

  let fintech_num = Departments::FinTech(8);
  
  if let Departments::FinTech(n) = fintech_num {

    println!("FinTech Dep has {n} employees...")
  }

  else {

    println!("Another department info...")
  }

}

enum Departments {

  FinTech(u8),
  Sales(u8),
  Finance(u8)
}
