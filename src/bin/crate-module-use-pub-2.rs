
// 使用两种方式填空
// 不要添加新的代码行
// use std::collections::{HashMap,HashSet,BTreeMap};
use std::collections::*;

fn main() {
    let _c1:HashMap<&str, i32> = HashMap::new();
    let mut c2 = BTreeMap::new();
    c2.insert(1, "a");
    let _c3: HashSet<i32> = HashSet::new();
}
