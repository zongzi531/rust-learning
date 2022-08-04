use crate::front_of_house;

pub fn fix_incorrect_order() {
  cook_order();
  // 使用三种方式填空
  //1. 使用关键字 `super`
  //2. 使用绝对路径
  // super::front_of_house::serving::serve_order();
  // crate::front_of_house::serving::serve_order();
  front_of_house::serving::serve_order();
}

pub fn cook_order() {}
