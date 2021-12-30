use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; 2_usize.pow(n as u32)],
    }

    let person = 2_usize.pow(n as u32);
    let mut rate_a = 0;
    let mut id_a = 0;
    for i in 0..person/2 {
        if rate_a < a[i] {
            rate_a = a[i];
            id_a = i;
        }
    }
    let mut rate_b = 0;
    let mut id_b = 0;
    for i in person/2..person {
        if rate_b < a[i] {
            rate_b = a[i];
            id_b = i;
        }
    }

    if rate_a > rate_b {
        println!("{}", id_b + 1)
    } else {
        println!("{}", id_a + 1)
    }
}
