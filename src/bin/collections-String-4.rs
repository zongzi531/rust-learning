
// 填空并修复错误
fn main() {
  let s = String::from("hello, 世界");
  let slice1 = &s[0..1]; //提示: `h` 在 UTF-8 编码中只占用 1 个字节
  assert_eq!(slice1, "h");

  let slice2 = &s[7..10];// 提示: `中` 在 UTF-8 编码中占用 3 个字节
  assert_eq!(slice2, "世");
  
  // 迭代 s 中的所有字符
  for (i, c) in s.chars().enumerate() {
      if i == 7 {
          assert_eq!(c, '世')
      }
  }

  println!("Success!")
}
