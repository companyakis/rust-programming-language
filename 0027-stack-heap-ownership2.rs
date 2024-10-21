fn main() {

  let name1 = "Aypara".to_string();

  let name2 = "Ayhan".to_string();

  print_name(&name1);

  print_name(&name2)


}

//  An example of using an immutable reference in a function

fn print_name(name: &String) {

  println!("My name is {name}")
}
