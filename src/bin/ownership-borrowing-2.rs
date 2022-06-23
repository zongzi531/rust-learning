
fn main() {
  let x = 5;
  let y = &x;

  // 只能修改以下行
  assert_eq!(5, *y);
}
