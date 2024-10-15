fn main() {

    let student: (&str, u8, u16) = ("Aygün", 124, 2014);

    let (name, id, birth_year) = student;

    println!("Name: {name}");

    println!("{}", student.2)

}

// Name: Aygün
// 2014
