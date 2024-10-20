fn main() {

  let year: u16 = 2024; // stored on the stack

  let my_name = String::from("Mustafa"); // stored on the heap 

  // move

  let a_name = my_name;

  // error!!!

  // println!("My name is {my_name}"); // value borrowed here after move

  println!("Whose name is {a_name}?") // Whose name is Mustafa?

}

