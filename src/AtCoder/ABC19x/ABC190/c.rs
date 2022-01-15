use proconio::input;


struct DepthFirstSearch {
    ab: Vec<(usize, usize)>,
    cd: Vec<(usize, usize)>,
    board: Vec<i64>,
}

impl DepthFirstSearch {
    fn depth_first_search(v: &mut DepthFirstSearch, depth: usize) -> i64 {
        let mut best_value = -1;

        if v.cd.len() <= depth {
            let mut count = 0;
            for &(a, b) in &v.ab {
                if v.board[a-1] > 0 && v.board[b-1] > 0 {
                    count += 1;
                }
            }
            return count;
        }

        let legal_moves = DepthFirstSearch::move_picker(v, depth);
        for &i in &legal_moves {
            v.board[i] += 1;
            let value = DepthFirstSearch::depth_first_search(v, depth + 1);
            v.board[i] -= 1;

            if best_value < value {
                best_value = value;
            }
        }

        return best_value;
    }

    fn move_picker(v: &DepthFirstSearch, depth: usize) -> Vec<usize> {
        return vec![v.cd[depth].0 - 1, v.cd[depth].1 - 1];
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
        k: usize,
        cd: [(usize, usize); k],
    }
 
    let v = &mut DepthFirstSearch {
        ab: ab,
        cd: cd,
        board: vec![0; n],
    };
 
    println!("{}", DepthFirstSearch::depth_first_search(v, 0));
}
