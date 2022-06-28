fn main() {
  // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
  // 填空以输出 "I'm writing Rust"
  let byte_escape = "I'm writing Ru\x73\x74!";
  println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

  // 也可以使用 Unicode 形式的转义字符
  let unicode_codepoint = "\u{211D}";
  let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

  println!("Unicode character {} (U+211D) is called {}",
              unicode_codepoint, character_name );

  // 还能使用 \ 来连接多行字符串
  let long_string = "String literals
                      can span multiple lines.
                      The linebreak and indentation here \
                       can be escaped too!";
  println!("{}", long_string);
}
