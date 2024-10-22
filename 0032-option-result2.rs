fn main() {

  let n1 = 256.0;

  let power1 = find_root_zero_point_25(n1);

  match power1 {
      
      Some(value) => println!("{n1} ^ 0.25 = {value}"), // 256 ^ 0.25 = 4

      None => println!("{n1} is not a real number!")
  }

  let n2 = -27.5;

  let power2 = find_root_zero_point_25(n2);

  match power2 {

    Some(value) => println!("{n2} ^ 0.25 = {value}"),

    None => println!("{n2} is not a real number!") // -27.5 is not a real number!
      
  }

}


fn find_root_zero_point_25(number: f64) -> Option<f64> {

  if number >= 0.0 {
    Some(number.powf(0.25))
  }
  else {
    None
  }
} 
