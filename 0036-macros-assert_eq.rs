fn main() {

  let x: u16 = 1010;

  let result = sum(100, 987);

  assert_eq!(x, result, "They should be equal, but NOT!")

  // assertion `left == right` failed: They should be equal, but NOT!

}

fn sum(a: u16, b: u16) -> u16 {

  a + b
}







