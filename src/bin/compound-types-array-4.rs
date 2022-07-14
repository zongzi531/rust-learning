trait I64Char {}

struct I64(i64);
struct Char(char);

impl I64Char for I64 {}
impl I64Char for Char {}

fn main() {
  // 修复错误
  let _arr: [Box<dyn core::fmt::Debug>; 3] = [Box::new(1), Box::new(2), Box::new('3')];
  let _arr1: [Box<dyn I64Char>; 3] = [Box::new(I64(1)), Box::new(I64(2)), Box::new(Char('3'))];
}
