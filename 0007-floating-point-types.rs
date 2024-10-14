fn main() {

    // The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision.

    let pi_one: f32 = 22.0 / 7.0;

    println!("f32 pi: {}", pi_one);

    let pi_two: f64 = 22.0 / 7.0;

    println!("f64 pi: {}", pi_two);
    
}

// f32 pi: 3.142857
// f64 pi: 3.142857142857143
