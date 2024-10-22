extern crate rand;
use rand::Rng;

fn main() {

  let mut rng = rand::thread_rng();  // rng => random number generator

  let random_integer1: u8 = rng.gen_range(0..50); // the result is NOT constant

  //println!("Random integer 1: {random_integer1}");

  let random_integer2: u16 = rng.gen_range(500..=1000); // 1000 is inclusive

  //println!("Random integer 2: {random_integer2}");

  let random_negative_integer: i16 = rng.gen_range(-800..-300);

  println!("Random negative integer: {random_negative_integer}"); // Now first result => Random negative integer: -439

}
