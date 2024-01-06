use rust_kata_exercises::{find_short_better_solution};
use std::cmp;

fn main() {
    println!("shortest is {}", find_short_better_solution("hello my only little friend"));

    // Because at the time of writing this I didn't understand the min_by_key syntax
    println!("understand min by {}", cmp::min_by_key(-2, -5, |x: &i32| x.abs()))
}