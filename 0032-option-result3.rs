fn main() {

// Result is a generic enum that can hold two types T and E. Result has two variants:

// Ok(T): Represents a successful operation with a value of type T
// Err(E): Represents an error with a value of type E
      
// The Result enum is defined in Rust's standard library as follows:

// pub enum Result<T, E> {
//     Ok(T),
//     Err(E),
// } 

  let a = 12.25;
  let b = -13.0;
  let c = 0.0;

  let division_result1 = is_divisible(a, b);

  println!("Division result 1: {:?}", division_result1); // Division result 1: Ok(-0.9423076923076923)

  let division_result2 = is_divisible(b, c);

  println!("Division result 2: {:?}", division_result2); // Division result 2: Err("Zero division error!")

}

fn is_divisible(x: f64, y: f64) -> Result<f64, String> {

  if y == 0.0 {
    Err(String::from("Zero division error!"))
  }
  else {
    Ok(x / y)
  }
}



