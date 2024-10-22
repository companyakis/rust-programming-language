fn main() {

//   The Option enum is defined in Rust's standard library as follows:

//   pub enum Option<T> {
//     Some(T),
//     None,
// }

  let n1 = 256.0;

  let power1 = find_root_zero_point_25(n1);

  // 256 ** (0.25)

  println!("Power 1: {:?}", power1); // Power 1: Some(4.0)

}

// what an ugly function:)

fn find_root_zero_point_25(number: f64) -> Option<f64> {

  if number >= 0.0 {
    Some(number.powf(0.25))
  }
  else {
    None
  }
} 
