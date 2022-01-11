use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        d: [i32; n],
    }
 
    let unique: HashSet<i32> = d.into_iter().collect();
    println!("{}", unique.len());
}
