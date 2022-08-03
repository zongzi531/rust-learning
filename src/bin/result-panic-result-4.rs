use std::num::ParseIntError;

// 使用两种方式填空: map, and then
fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
  //  n_str.parse::<i32>().map(|i| i + 2)
  n_str.parse::<i32>().and_then(|i| Ok(i + 2))
}

fn main() {
    assert_eq!(add_two("4").unwrap(), 6);

    println!("Success!")
}
