
//  修改 `assert_eq!` 让代码工作
fn main() {
  let x: u32 = 5;
  assert_eq!("u32".to_string(), type_of(&x));
}

// 以下函数可以获取传入参数的类型，并返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
  format!("{}", std::any::type_name::<T>())
}
