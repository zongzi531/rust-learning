
// 填空让 `println` 输出，同时添加一些代码不要让最后一行的 `panic` 执行到
fn main() {
  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  if let n = six {
      println!("{:?}", n);
      return
  } 
      
  panic!("不要让这行代码运行！");
} 

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
      None => None,
      Some(i) => Some(i + 1),
  }
}
