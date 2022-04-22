use std::ops::SubAssign;

pub mod sub_a;
pub mod sub_b;
pub fn run() {
    println!("HEre is vars modules!!");
    sub_a::func_a();
    sub_b::func_b();
}
