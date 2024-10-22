extern crate rand;
use rand::Rng;
use rand::distributions::Uniform;

fn main() {

  let mut rng = rand::thread_rng();

  let random = rng.sample(Uniform::new(10, 25));

  println!("A random number: {random}")

  // try three times

  // A random number: 24

  // A random number: 22

  // A random number: 19

}




