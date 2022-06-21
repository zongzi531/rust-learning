
fn main() {
  println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
  match tp {
      1 => {
          // TODO
      }
      _ => {
          // TODO
      }
  };
  
  // 这里与其返回一个 None，不如使用发散函数替代
  never_return_fn()
}

// 使用三种方法实现以下发散函数
fn never_return_fn() -> ! {
  panic!()
  // unimplemented!()
}
