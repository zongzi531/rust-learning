
// 移除某个部分让代码工作
fn main() {
  let x: i32 = 5;
  let mut y: i32 = 5;
  println!("{}", y);

  y = x;
  
  let z: i32 = 10; // 这里 z 的类型是? 
  println!("{},{},{}", x, y, z);
}
