use proconio::input;

struct BinarySearch {
    n: usize,
    m: usize,
    xyr: Vec<(f64, f64, f64)>,
    xy: Vec<(f64, f64)>,
}

impl BinarySearch {
    // keyを満たすかどうか調べる問題固有の関数
    fn is_ok(v: &BinarySearch, key: f64) -> bool {
        for i in 0..v.m {
            for j in 0..v.n {
                let r = (v.xy[i].0 - v.xyr[j].0).hypot(v.xy[i].1 - v.xyr[j].1);
                if r < key + v.xyr[j].2 {
                    return false;
                }
            }
            for j in i+1..v.m {
                let r = (v.xy[i].0 - v.xy[j].0).hypot(v.xy[i].1 - v.xy[j].1);
                if r < key * 2. {
                    return false;
                }
            }
        }
        return true;
    }

    // ng～ok区間を二分探索する汎用関数
    fn binary_search(v: &BinarySearch, mut ng: f64, mut ok: f64) -> f64 {
        while (ok - ng).abs() > 1e-7 {
            let key = (ok + ng) / 2.;
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
        m: usize,
        xyr: [(f64, f64, f64); n],
        xy: [(f64, f64); m],
    }
 
    let mut ans = 200_f64.hypot(200_f64);
    for i in 0..n {
        if ans > xyr[i].2 {
            ans = xyr[i].2;
        }
    }
 
    let v = BinarySearch {
        n: n,
        m: m,
        xyr: xyr,
        xy: xy,
    };
    let ng = 200_f64.hypot(200_f64);
 
    ans = ans.min(BinarySearch::binary_search(&v, ng, 0.));
    println!("{}", ans);
}
