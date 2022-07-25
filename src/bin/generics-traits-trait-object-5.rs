
// 使用至少两种方法让代码工作
// 不要添加/删除任何代码行
// trait MyTrait {
//   fn f(&self) -> Self;
// }

// impl MyTrait for u32 {
//   fn f(&self) -> u32 { 42 }
// }

// impl MyTrait for String {
//   fn f(&self) -> String { self.clone() }
// }

// fn my_function(x: impl MyTrait) -> impl MyTrait  {
//   x.f()
// }

// fn main() {
//   my_function(13_u32);
//   my_function(String::from("abc"));

//   println!("Success!")
// }

// ===

// 使用至少两种方法让代码工作
// 不要添加/删除任何代码行
trait MyTrait {
  fn f(&self) -> Box<dyn MyTrait>;
}

impl MyTrait for u32 {
  fn f(&self) -> Box<dyn MyTrait> { Box::new(42) }
}

impl MyTrait for String {
  fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }
}

fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait>  {
  x.f()
}

fn main() {
  my_function(Box::new(13_u32));
  my_function(Box::new(String::from("abc")));

  println!("Success!")
}
