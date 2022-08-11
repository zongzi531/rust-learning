/* Annotate struct with lifetime:
1. `r` and `s` must has different lifetimes
2. lifetime of `s` is bigger than that of 'r'
*/
struct DoubleRef<'a, 'b: 'a,  T> {
  r: &'a T,
  s: &'b T
}
fn main() {
  println!("Success!")
}
