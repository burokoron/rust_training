use proconio::input;

struct BinarySearch {
    n: usize,
    hs: Vec<(f64, f64)>,
}

impl BinarySearch {
    // keyを満たすかどうか調べる問題固有の関数
    fn is_ok(v: &BinarySearch, key: i64) -> bool {
        let mut t: Vec<f64> = Vec::new();
        for i in 0..v.n {
            t.push((key as f64 - v.hs[i].0) / v.hs[i].1);
        }
        t.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for i in 0..v.n {
            if i as f64 > t[i] {
                return false;
            }
        }
        return true;
    }

    // ng～ok区間を二分探索する汎用関数
    fn binary_search(v: &BinarySearch, mut ng: i64, mut ok: i64) -> i64 {
        while (ok - ng).abs() > 1 {
            let key = (ok + ng) / 2;
            if BinarySearch::is_ok(v, key) {
                ok = key;
            } else {
                ng = key;
            }
        }
        return ok;
    }
}

fn main() {
    input! {
        n: usize,
        hs: [(f64, f64); n],
    }
 
    let mut ok = 0;
    for i in 0..n {
        let score = hs[i].0 as i64 + hs[i].1 as i64 * n as i64 - 1;
        if ok < score {
            ok = score;
        }
    }
 
    let v = BinarySearch {
        n: n,
        hs: hs,
    };
 
    println!("{}", BinarySearch::binary_search(&v, 0, ok));
}
