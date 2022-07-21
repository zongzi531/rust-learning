
fn main() {
  let boolean = true;

  // 使用 match 表达式填空，并满足以下条件
  //
  // boolean = true => binary = 1
  // boolean = false => binary = 0
  let binary = match boolean {
      true => 1,
      false => 0,
  };

  assert_eq!(binary, 1);
}
