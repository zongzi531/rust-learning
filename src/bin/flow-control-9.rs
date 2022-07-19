
// 填空，不要修改其它代码
fn main() {
  let mut count = 0u32;

  println!("Let's count until infinity!");

  // 无限循环
  loop {
      count += 1;

      if count == 3 {
          println!("three");

          // 跳过当此循环的剩余代码
          continue;
      }

      println!("{}", count);

      if count == 5 {
          println!("OK, that's enough");

          break;
      }
  }

  assert_eq!(count, 5);
}
