extern crate rand;
use rand::Rng;

fn main() {

  let mut rng = rand::thread_rng();  // rng => random number generator

  let random_floating_number: f32 = rng.gen_range(12.5..37.0);

  println!("Random floating number: {random_floating_number}") //Random floating number: 36.036858

}
