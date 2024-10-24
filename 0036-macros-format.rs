fn main() {

  let e: String = "Ceteris paribus".to_string();

  let result1: String = format!("Expression: {} and its length: {}", e, e.len());

  println!("{result1}"); // Expression: Ceteris paribus and its length: 15

  let proverb = String::from("Out of sight, out of minde");

  println!("{}", format!("Expression: {} and its length: {}", proverb, proverb.len())) // Expression: Out of sight, out of minde and its length: 26

}
