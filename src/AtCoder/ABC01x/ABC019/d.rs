use std::io::{stdout, Write};

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let n = read();

    let mut index = 0;
    let mut max_wight = 0;
    for i in 1..=n {
        println!("? 1 {}", i);
        stdout().flush().unwrap();
        let weight = read();
        if weight > max_wight {
            max_wight = weight;
            index = i;
        }
    }

    max_wight = 0;
    for i in 1..=n {
        println!("? {} {}", index, i);
        stdout().flush().unwrap();
        let weight = read();
        if weight > max_wight {
            max_wight = weight;
        }
    }

    println!("! {}", max_wight);
    stdout().flush().unwrap();
}
