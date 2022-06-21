
// 使成功打印
fn main() {
  let _f: bool = false;

  let mut t = true;
  println!("{}", t);
  t = _f;
  if !t {
      println!("Success!")
  }
} 
