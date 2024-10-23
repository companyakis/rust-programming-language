fn main() {

  let sample_circle = Circle {radius: 5.0};

  let sample_rectangular = Rectangular {width: 5.25, height: 4.15};

  let sample_square = Square {side: 6.0};

  sample_circle.area();

  sample_circle.perimeter();

  sample_rectangular.area();

  sample_rectangular.perimeter();

  sample_square.area();

  sample_square.perimeter();

  // Circle area: 78.5
  // Circle perimeter: 31.400002
  // Rectangular area: 21.7875  
  // Rectangular perimeter: 18.8
  // Square area: 36
  // Square perimeter: 24     

}

trait Geometry {

  fn area(&self);

  fn perimeter(&self);
}

struct Circle {
  radius: f32
}

impl Geometry for Circle {

  fn area(&self) {
      println!("Circle area: {}", self.radius * self.radius * 3.14)
  }

  fn perimeter(&self) {
      println!("Circle perimeter: {}", 2.0 * 3.14 * self.radius)
  }
    
}

struct Rectangular {
  width: f32,
  height: f32
}

impl Geometry for Rectangular {
    
    fn area(&self) {
        println!("Rectangular area: {}", self.height * self.width)
    }

    fn perimeter(&self) {
        println!("Rectangular perimeter: {}", 2.0 * (self.height + self.width))
    }
}

struct Square {
  side: f32
}

impl Geometry for Square {

  fn area(&self) {
      println!("Square area: {}", self.side * self.side)
  }

  fn perimeter(&self) {
      println!("Square perimeter: {}", 4.0 * self.side)
  }
    
}
