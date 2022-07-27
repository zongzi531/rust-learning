// 填空
fn main() {  
  let mut s = String::from("hello, world");

  let slice1: &str = &s[..]; // 使用两种方法
  assert_eq!(slice1, "hello, world");

  let slice2 = &s[..5];
  assert_eq!(slice2, "hello");

  let slice3: &mut String = &mut s; 
  slice3.push('!');
  assert_eq!(slice3, "hello, world!");

  println!("Success!")
}
