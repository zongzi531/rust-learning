
// 填空
fn main() {
  let s = String::from("I like dogs");
  // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
  let s1 = s.replace("dogs", "cats");

  assert_eq!(s1, "I like cats")
}
