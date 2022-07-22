
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  // 填空，让 p 匹配第二个分支
  let p = Point { x: 2, y: 10 };

  match p {
      Point { x, y: 0 } => println!("On the x axis at {}", x),
      // 第二个分支
      Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
      Point { x, y } => println!("On neither axis: ({}, {})", x, y),
  }
}
