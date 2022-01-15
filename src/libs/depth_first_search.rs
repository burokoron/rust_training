/*
深さ優先探索
全探索でも計算が間に合うような場合

例題
Typical90_002：https://atcoder.jp/contests/typical90/tasks/typical90_b
ABC190C：https://atcoder.jp/contests/abc190/tasks/abc190_c
ABC197C：https://atcoder.jp/contests/abc197/tasks/abc197_c
*/

struct DepthFirstSearch {
    // ここに探索で使用する変数を書く
    // 少なくとも現在の盤面情報が必要なはず
}

impl DepthFirstSearch {
    // 深さ優先探索
    fn depth_first_search(v: &mut DepthFirstSearch, depth: i64) -> i64 {
        // 最後まで探索したらスコアを返す
        if depth <= 0 {
            let score = 0;
            todo!(); // ここにスコアの計算などを書く
            return score;
        }

        // 合法手生成
        let legal_moves = DepthFirstSearch::move_picker(v);

        // 全合法手を調べる
        for &(i, j) in &legal_moves {
            todo!(); // ここに盤面を進める処理を書く
            // 深さ優先探索
            DepthFirstSearch::depth_first_search(v, depth - 1);
            todo!(); // ここに盤面を戻す処理を書く
        }
    }

    // 合法手生成
    fn move_picker(v: &DepthFirstSearch) -> Vec<usize>{
        let mut legal_moves: Vec<usize> = Vec::new();
        todo!(); // ここに全合法手を生成する処理を書く

        return legal_moves;
    }
}
