fn main() {

  let numbers: Vec<i8> = vec![-8, 3, 0, -11, 22, -98, 101, 1, 4];

  let slice1 =&numbers[0..4];

  println!("Vector slice 1: {:?}", slice1);

  let slice2 =&numbers[0..=4]; // index 4 is inclusive!

  println!("Vector slice 2: {:?}", slice2);

}

// Vector slice 1: [-8, 3, 0, -11]
// Vector slice 2: [-8, 3, 0, -11, 22]


