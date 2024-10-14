fn main() {

    // We can shadow a variable by using the same variable’s name and repeating the use of the let keyword

    let year = 2022;

    println!("Year is {year}");

    let year = year + 2;

    {
        let year = year * 3;

        println!("Year is {year}");
    }

    println!("Year is {year}");
    
}

// Year is 2022
// Year is 6072
// Year is 2024
