fn main() {

  let cat = Cat {name: String::from("Sylvester")};

  cat.speak();

  let bird = Bird {name: String::from("Tweety")};

  bird.speak();

// Sylvester says 'Miyavvv!'
// Tweety says 'Cikkkk!'   

}

// Defining Traits 

trait Speak {

  fn speak(&self);
}
struct Cat {
  name: String
}

impl Speak for Cat {
  fn speak(&self) {
      println!("{} says 'Miyavvv!'", self.name)
  }
}

struct Bird {
  name: String
}

impl Speak for Bird {
  fn speak(&self) {
      println!("{} says 'Cikkkk!'", self.name)
  }
}




