fn main() {

  let x: u8 = 101;

  println!("x value: {x}"); // x value: 101

  diverging_function(); // any code following this expression is unreachable

  let y: i8 = -101; // unreachable statement

  println!("y value: {y}"); // unreachable statement

}

fn diverging_function() -> ! {
  panic!("I have to exit!")
}
