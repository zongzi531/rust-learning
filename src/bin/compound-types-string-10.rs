/* 填空并修复所有错误 */
fn main() {
  let raw_str = "Escapes don't work here: \x3F \u{211D}";
  assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

  // 如果你希望在字符串中使用双引号，可以使用以下形式
  let quotes = r#"And then I said: "There is no escape!""#;
  println!("{}", quotes);

  // 如果希望在字符串中使用 # 号，可以如下使用：
  let  delimiter = r###"A string with "# in it. And even "##!"###;
  println!("{}", delimiter);

  // 填空
  let long_delimiter = "Hello, \"##\"";
  assert_eq!(long_delimiter, "Hello, \"##\"")
}
