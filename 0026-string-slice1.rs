fn main() {

  let proverb: String = String::from("Out of sight, out of mind");

  let slice1 = &proverb[0..3];

  let slice2 = &proverb[11..=17]; // careful! => ..=

  println!("Slice 1: {slice1} - slice 2: {slice2}") // Slice 1: Out - slice 2: t, out 

}

