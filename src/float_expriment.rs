// const N: usize = 5;
// struct ArrayPari<T, W> {
//     left: [T; W],
//     right: [T; W],
// }
#[allow(unused_imports)]
use std::f32::{MAX, MIN};

fn larger(a: f64, b: f32) -> f32 {
    if a > b as f64 {
        return a as f32;
    }
    return b;
}

fn main() {
    // println!("large {}", larger(MIN as f64, MIN));
}
