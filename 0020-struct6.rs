fn main() {

  let rectangle = Rectangle {width: 15.0, height: 12.0};

  println!("Perimeter: {}", rectangle.perimeter()); // Perimeter: 54

  println!("Area: {}", rectangle.area()) // Area: 180

}

#[derive(Debug)]
struct Rectangle {
  width: f64,
  
  height: f64
}

impl Rectangle {

  fn perimeter(&self) -> f64 {

    2.0 * (self.width + self.height) 
  }

  fn area(&self) -> f64 {

    self.width * self.height
  }
}






