fn main() {

    let greeting_return = greeting();

    println!("{greeting_return}"); // Hello!


}

fn greeting() -> String {

    "Hello!".to_string()
}
