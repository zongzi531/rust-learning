
// 填空
fn main() {
  let mut count = 0;
  'outer: loop {
      'inner1: loop {
          if count >= 20 {
              // 这只会跳出 inner1 循环
              break 'inner1; // 这里使用 `break` 也是一样的
          }
          count += 2;
      }

      count += 5;

      'inner2: loop {
          if count >= 30 {
              break 'outer;
          }

          continue 'outer;
      }
  }

  assert!(count == 30);
}
