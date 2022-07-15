
fn main() {
  // 填空，需要稍微计算下
  let (x, y) = sum_multiply((2, 3));

  assert_eq!(x, 5);
  assert_eq!(y, 6);
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
  (nums.0 + nums.1, nums.0 * nums.1)
}
