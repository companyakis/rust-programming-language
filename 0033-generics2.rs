fn main() {

  let words = vec!["bilge", "yener", "aygün"];

  println!("{}", find_the_largest_one(&words));

  let char_list = vec!['k', 'm', 'x', 'a'];

  println!("{}", find_the_largest_one(&char_list));

}

fn find_the_largest_one<T: std::cmp::PartialOrd>(elements: &[T]) -> &T {

  let mut largest_element = &elements[0];

  for element in elements {
    if element > largest_element {
      largest_element = element;
    }
  }

  largest_element
}



