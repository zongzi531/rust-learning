
fn main() {
  let t = (String::from("hello"), String::from("world"));

  // 填空，不要修改其它代码
  let (ref s1, ref s2) = t;

  println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}
