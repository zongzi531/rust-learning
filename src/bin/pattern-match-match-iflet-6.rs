
fn main() {
  let o = Some(7);

  // 移除整个 `match` 语句块，使用 `if let` 替代
  if let Some(i) = o {
    println!("This is a really long string and `{:?}`", i);
  }
}
