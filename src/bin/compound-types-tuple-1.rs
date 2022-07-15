
fn main() {
  let _t0: (u8,i16) = (0, -1);
  // 元组的成员还可以是一个元组
  let _t1: (u8, (i16, u32)) = (0, (-1, 1));
  // 填空让代码工作
  let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
}
