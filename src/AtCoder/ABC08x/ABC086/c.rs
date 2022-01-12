use proconio::input;

fn main() {
    input! {
        n: usize,
        txy: [(i32, i32, i32); n],
    }
 
    for i in 0..n {
        let time;
        let distance;
        if i == 0 {
            time = txy[i].0;
            distance = txy[i].1 + txy[i].2;
        } else {
            time = txy[i].0 - txy[i-1].0;
            distance = (txy[i].1 - txy[i-1].1).abs() + (txy[i].2 - txy[i-1].2).abs();
        }
        if distance > time || (time - distance) % 2 == 1 {
            println!("No");
            return;
        }
    }
 
    println!("Yes");
}
