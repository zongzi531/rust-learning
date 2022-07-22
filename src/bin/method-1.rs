struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  // 完成 area 方法，返回矩形 Rectangle 的面积
  fn area(&self) -> u32 {
    &self.width * &self.height
  }
}

fn main() {
  let rect1 = Rectangle { width: 30, height: 50 };

  assert_eq!(rect1.area(), 1500);
}
