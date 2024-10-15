fn main() {

  let mut x: u8 = 0;

  loop {

    println!("X value : {x}");

    x += 1;

    if x == 10 {
      break
    }
  }
}

// X value : 0
// X value : 1
// X value : 2
// X value : 3
// X value : 4
// X value : 5
// X value : 6
// X value : 7
// X value : 8
// X value : 9
