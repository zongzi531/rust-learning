
// 修改下面的代码以打印如下内容: 
// 25
// 25
// 25
// 循环中不会发生任何内存分配
fn main() {
  let mut s = String::with_capacity(25);

  println!("{}", s.capacity());

  for _ in 0..2 {
      s.push_str("hello");
      println!("{}", s.capacity());
  }

  println!("Success!")
}
