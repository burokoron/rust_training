use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
    }

    let mut ah = vec![0; h];
    let mut aw = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            ah[i] += a[i][j];
            aw[j] += a[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{} ", ah[i] + aw[j] - a[i][j]);
        }
        println!("");
    }
}
