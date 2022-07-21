
enum Foo {
  Bar,
  Baz,
  Qux(u32)
}

fn main() {
  let a = Foo::Qux(10);

  // 移除以下代码，使用 `match` 代替
  match a {
      Foo::Bar => println!("match foo::bar"),
      Foo::Baz => println!("match foo::baz"),
      _ => println!("match others"),
  }
}
