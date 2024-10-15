fn main() {

    let result = add_multiply(-12, -35_000, 587);

    println!("a + b + c = {}", result.0);

    println!("a * b * c = {}", result.1)

}

fn add_multiply(a: i128, b: i128, c: i128) -> (i128, i128) {

    let addition = a + b + c;

    let multiplication = a * b * c;

    (addition, multiplication)

}

// a + b + c = -34425
// a * b * c = 246540000
