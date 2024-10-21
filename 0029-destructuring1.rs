fn main() {

  let (a, b);

  [a, ..,b] = [1990, 2018, 2020, 2024, 2025];

  print!("a: {a} - b: {b}") // a: 1990 - b: 2025

  // let (x, y, z) = ("First", "Second", "Third");

  //println!("x: {x}"); // x: First

  // only take x and y

  let (x, y, _) = ("First", "Second", "Third");

  println!("x: {x}");

  println!("y: {y}"); // y: Second

}
