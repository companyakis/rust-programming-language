fn main() {

  // with parameter
  say_hello(Some("Mustafa"));

  // without parameter
  say_hello(None);

}

  // Rust doesn't support default parameters in functions
  // But we can use the Option type


fn say_hello(name: Option<&str>) {

  match name {

    Some(i) => println!("Hello, {i}"),
    None => println!("Hello, unknown person!"),   
  }
}
