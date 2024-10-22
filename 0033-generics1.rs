extern crate rand;
use rand::Rng;

fn main() {

  let mut rng = rand::thread_rng(); 

  let random_numbers = vec![rng.gen_range(0..50), rng.gen_range(0..50), rng.gen_range(0..50), rng.gen_range(0..50), rng.gen_range(0..50), rng.gen_range(0..50)];

  println!("Largest number is: {}", find_the_largest_one(&random_numbers))

}

fn find_the_largest_one<T: std::cmp::PartialOrd>(elements: &[T]) -> &T {

  let mut largest_element = &elements[0];

  for element in elements {
    if element > largest_element {
      largest_element = element;
    }
  }

  largest_element
}



