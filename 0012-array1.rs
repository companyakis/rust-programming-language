fn main() {

    // Unlike a tuple, every element of an array must have the same type

    // Arrays in Rust have a fixed length

    let years = [2023, 2024, 2025];

    println!("Years => {:?}", years);

    let names: [&str; 4] = ["Aygün", "Aybilge", "Hakan", "Yiğit"];

    let name_1 = &names[0];

    println!("Name one: {name_1}")

}
