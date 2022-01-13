use proconio::input;

struct BinarySearch {
    n: usize,
    l: i64,
    k: i64,
    a: Vec<i64>,
}

impl BinarySearch {
    // keyを満たすかどうか調べる問題固有の関数
    fn is_ok(v: &BinarySearch, key: i64) -> bool {
        let mut score = 0;
        let mut count = 0;
        for i in 0..v.n {
            if i == 0 {
                score += v.a[i];
            } else {
                score += v.a[i] - v.a[i-1];
            }
            if score >= key && v.k > count {
                count += 1;
                score = 0;
            }
        }
        score += v.l - v.a[v.n-1];
        if v.k == count && score >= key {
            return true;
        } else {
            return false;
        }
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
        l: i64,
        k: i64,
        a: [i64; n],
    }
    let v = &mut BinarySearch {
        n: n,
        l: l,
        k: k,
        a: a,
    };

    println!("{}", BinarySearch::binary_search(&v, l, 0));
}
