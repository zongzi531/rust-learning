有时我们希望某一个项只对特定的包可见，那么就可以使用 pub(in Crate) 语法.

```rust
pub mod a {
  pub const I: i32 = 3;

  fn semisecret(x: i32) -> i32 {
      use self::b::c::J;
      x + J
  }

  pub fn bar(z: i32) -> i32 {
      semisecret(I) * z
  }
  pub fn foo(y: i32) -> i32 {
      semisecret(I) + y
  }

  mod b {
      pub(in crate::a) mod c {
          pub(in crate::a) const J: i32 = 4;
      }
  }
}
```
