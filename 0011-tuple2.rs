fn main() {

    let mut student: (&str, u8, u16) = ("Aygün", 124, 2014);

    student.1 = 114;

    println!("Student info: {:?}", student); // Student info: ("Aygün", 114, 2014)

}
