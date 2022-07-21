
// 就地修复错误
fn main() {
  let age = Some(30);
  if let Some(age) = age { // 创建一个新的变量，该变量与之前的 `age` 变量同名
     assert_eq!(age, 30);
  } // 新的 `age` 变量在这里超出作用域
  
  match age {
      // `match` 也能实现变量遮蔽
      Some(age) =>  println!("age 是一个新的变量，它的值是 {}",age),
      _ => ()
  }
}
