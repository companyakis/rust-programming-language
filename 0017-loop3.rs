// loop labels

fn main() {

  let mut counter: u8 = 0;

  'counter_up : loop {
    println!("Counter value: {counter}");

    let mut decreasing_value: u8 = 15;

    loop {
      if decreasing_value == 3 {
        break;
      }

      if counter == 7 {
        break 'counter_up;
      }
      decreasing_value -= 1;
    }

    counter += 1;

  }
  println!("Final counter value: {counter}")

}

// Counter value: 0
// Counter value: 1      
// Counter value: 2      
// Counter value: 3      
// Counter value: 4      
// Counter value: 5      
// Counter value: 6      
// Counter value: 7      
// Final counter value: 7
