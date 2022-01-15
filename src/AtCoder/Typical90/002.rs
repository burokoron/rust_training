use proconio::input;


struct DepthFirstSearch {
    n: i64,
    board: [i64; 2],
    record: Vec<char>
}

impl DepthFirstSearch {
    fn depth_first_search(v: &mut DepthFirstSearch, depth: i64) {
        if depth <= 0 {
            for &i in &v.record {
                print!("{}", i);
            }
            println!("");
            return;
        }

        let legal_moves = DepthFirstSearch::move_picker(v);
        for &(i, j) in &legal_moves {
            v.record.push(i);
            v.board[j] += 1;
            DepthFirstSearch::depth_first_search(v, depth - 1);
            v.board[j] -= 1;
            v.record.pop();
        }
    }

    fn move_picker(v: &DepthFirstSearch) -> Vec<(char, usize)>{
        let mut legal_moves: Vec<(char, usize)> = Vec::new();
        if v.board[0] < v.n / 2 {
            legal_moves.push(('(', 0));
        }
        if v.board[0] > v.board[1] && v.board[0] + v.board[1] < v.n {
            legal_moves.push((')', 1));
        }

        return legal_moves;
    }
}

fn main() {
    input! {
        n: i64,
    }
 
    let v = &mut DepthFirstSearch {
        n: n,
        board: [0, 0],
        record: Vec::new(),
    };
 
    DepthFirstSearch::depth_first_search(v, n);
}
