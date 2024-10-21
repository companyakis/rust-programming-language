// The Clone trait is a standard trait in Rust that provides the clone function, which creates a deep copy of a value. 
// When you call clone on a value, it returns a new value that is an independent instance of the original value but has the same data.

fn main() {

  let my_name = "Mustafa".to_string();

  let clone_my_name1 = my_name.clone();

  let clone_my_name2 = my_name.clone();

  println!("{my_name} - {clone_my_name1} - {clone_my_name2}"); // Mustafa - Mustafa - Mustafa

  // call and print the function

  println!("{}", update_name(&my_name)) // Mustafa Büyükdereli
  
}


fn update_name(txt: &String) -> String {

  let mut cloned_txt = txt.clone();

  cloned_txt.push_str(" Büyükdereli");

  cloned_txt
}
