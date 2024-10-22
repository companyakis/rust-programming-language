fn main() {

  let a = 12.25;
  let b = -13.0;
  let c = 0.0;

  let division_result1 = is_divisible(a, b);

  match division_result1 {

    Ok(value) => println!("{a} / {b} = {value}"), // 12.25 / -13 = -0.9423076923076923
    
    Err(e) => println!("{e}")
      
  }

  let division_result2 = is_divisible(b, c);

  match division_result2 {

    Ok(value) => println!("{b} / {c} = {value}"),

    Err(e) => println!("{e}") // Zero division error!
      
  }

}


fn is_divisible(x: f64, y: f64) -> Result<f64, String> {

  if y == 0.0 {
    Err(String::from("Zero division error!"))
  }
  else {
    Ok(x / y)
  }
}



