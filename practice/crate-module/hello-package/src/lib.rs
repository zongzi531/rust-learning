mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() -> String {
  // 使用绝对路径调用
  // crate::front_of_house::hosting::add_to_waitlist();

  // 使用相对路径调用
  front_of_house::hosting::add_to_waitlist();
  back_of_house::cook_order();
  String::from("yummy yummy!")
}
