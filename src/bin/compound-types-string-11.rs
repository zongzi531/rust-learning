
fn main() {
  let s1 = String::from("hi,中国");
  let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
  assert_eq!(h, "h");

  let h1 = &s1[3..6];// 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
  assert_eq!(h1, "中");
}
