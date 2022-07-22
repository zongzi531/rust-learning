#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
// #![feature]` may not be used on the stable release channel
// $rustup install nightly
// $cargo +nightly run --bin generics-traits-const-generics-3

fn check_size<T>(val: T)
where
    Assert<{ core::mem::size_of::<T>() < 768 }>: IsTrue,
{
    //...
}

// 修复 main 函数中的错误
fn main() {
    check_size([0u8; 767]); 
    check_size([0i32; 191]);
    check_size(["hello你好"; 47]); // size of &str ?
    check_size([(); 31].map(|_| "hello你好".to_string()));  // size of String?
    check_size(['中'; 191]); // size of char ?
}



pub enum Assert<const CHECK: bool> {}

pub trait IsTrue {}

impl IsTrue for Assert<true> {}
