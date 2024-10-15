fn main() {

    let result = plus_1001(1080);

    println!("Result 1 = {result}");

    // without a variable!

    println!("Result 2 = {}", plus_1001(-98701));


}

fn plus_1001(a: i64) -> i64 {

    a + 1001
}

// Result 1 = 2081  
// Result 2 = -97700
