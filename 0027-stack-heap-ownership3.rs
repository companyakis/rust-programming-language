fn main() {

  // Mutable References

  // To create a mutable reference in Rust, we use the &mut syntax

  let mut info = String::from("This is the first line.");

  update_info(&mut info);

  // updated text

  println!("{info}") // This is the first line. This line's added!..

}

fn update_info(txt: &mut String) {

  txt.push_str(" This line's added!..")
}
