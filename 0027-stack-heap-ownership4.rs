fn main() {
  // We can only have one mutable reference to a variable at a time 
  // This is to prevent data races and other types of undefined behavior
  // We can have as many immutable references to a variable as you want

  let mut my_name = "Mustafa".to_string();

  // immutable references

  let ref_name1 = &my_name;

  let ref_name2 = &my_name;

  let ref_name3 = &my_name;

  println!("{ref_name1}, {ref_name2}, {ref_name3}..."); // Mustafa, Mustafa, Mustafa...

  // mutable references

  let ref_name4 = &mut my_name;

  //let ref_name5 = &mut my_name; // cannot borrow `my_name` as mutable more than once at a time

  ref_name4.push_str(" Büyükdereli"); 

  println!("{ref_name4}"); // Mustafa Büyükdereli


}

