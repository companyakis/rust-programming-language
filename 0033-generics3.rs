fn main() {

  let name: SomeData<String> = SomeData {x: String::from("Mustafa"), y: String::from("Büyükdereli")};

  let rectangle_width_height: SomeData<f32> = SomeData {x: 3.45, y: 12.5};

  let two_integers: SomeData<u16> = SomeData { x: 2024, y: 101 };

  name.print_data();

  rectangle_width_height.print_data();

  two_integers.print_data();

// Data 1: Mustafa and Data 2: Büyükdereli
// Data 1: 3.45 and Data 2: 12.5
// Data 1: 2024 and Data 2: 101 


}
struct SomeData<T> {
  x: T,
  y: T
}

impl<T : std::fmt::Display> SomeData<T> {

  fn print_data(&self) {
    println!("Data 1: {} and Data 2: {}", self.x, self.y)
  }
    
}

