fn main() {

  // empty but mutable

  let mut vec_1: Vec<i16> = Vec::new();

  vec_1.push(-200);
  vec_1.push(0);
  vec_1.push(654);
  vec_1.push(150);
  vec_1.push(-2000);

  println!("Vector: {:?}", vec_1); // Vector: [-200, 0, 654, 150, -2000]

}


