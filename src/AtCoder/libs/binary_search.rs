
/*
二分探索
最大値の最小化や、最小値の最大化をする場合などがキーワード

例題
Typical90_001：https://atcoder.jp/contests/typical90/tasks/typical90_a
ABC023D：https://atcoder.jp/contests/abc023/tasks/abc023_d
s8pc5B：https://atcoder.jp/contests/s8pc-5/tasks/s8pc_5_b
*/

struct BinarySearch {
    // ここに探索で使用する変数を書く
}

impl BinarySearch {
    // keyを満たすかどうか調べる問題固有の関数
    fn is_ok(v: &BinarySearch, key: i64) -> bool {
        todo!(); // ここに判定方法を書く

        if true {
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
    let v = &mut BinarySearch {
        // ここに探索で使用する変数を書く
    };
    let mut ng = 0;
    let mut ok = 100;

    printin!("{}", BinarySearch::binary_search(&v, ng, ok))
}