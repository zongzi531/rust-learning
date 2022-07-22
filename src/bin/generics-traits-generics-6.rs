struct Point<T, U> {
  x: T,
  y: U,
}

impl<T, U> Point<T, U> {
  // 实现 mixup，不要修改其它代码！
  fn mixup<T2, U2>(self, p2: Point<T2, U2>) -> Point<T, U2> {
    Point { x: self.x, y: p2.y }
  }
}

fn main() {
  let p1 = Point { x: 5, y: 10 };
  let p2 = Point { x: "Hello", y: '中'};

  let p3 = p1.mixup(p2);

  assert_eq!(p3.x, 5);
  assert_eq!(p3.y, '中');
}
