use proconio::input;

fn main() {
    input! {
        k: i32,
        a: i32,
        b: i32,
    }

    if a % k == 0{
        println!("OK");
    } else if a + k - a % k <= b {
        println!("OK");
    } else {
        println!("NG");
    }
}
