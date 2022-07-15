
// 修复所有错误
fn main() {
  let mut s = String::from("hello world");

  // 这里, &s 是 `&String` 类型，但是 `first_word` 函数需要的是 `&str` 类型。
  // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
  let word = first_word(&s);

  // s.clear(); // error!

  println!("the first word is: {}", word);

  s.clear();
}
fn first_word(s: &str) -> &str {
  &s[..1]
}
