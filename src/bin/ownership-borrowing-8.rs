
fn main() {
  // 通过修改下面一行代码来修复错误
  let mut s = String::from("hello, ");

  borrow_object(&mut s)
}

fn borrow_object(s: &mut String) {}
