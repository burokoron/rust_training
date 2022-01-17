use proconio::input;

struct DepthFirstSearch {
    a: Vec<u64>,
    record: Vec<bool>,
}

impl DepthFirstSearch {
    fn depth_first_search(v: &mut DepthFirstSearch, depth: usize) -> u64 {
        let mut best_value = 2_u64.pow(32);

        if v.a.len() <= v.record.len() {
            let mut or: Vec<u64> = vec![0; 1];
            for i in 0..v.a.len() {
                if v.record[i] {
                    or.push(0);
                }
                let index = or.len() - 1;
                or[index] |= v.a[i];
            }
            if or.len() == 1 {
                return or[0];
            } else {
                let mut xor = or[0];
                for &i in &or[1..] {
                    xor ^= i;
                }
                return xor;
            }
        }

        let legal_moves = DepthFirstSearch::move_picker(v);
        for &i in &legal_moves {
            v.record.push(i);
            let value = DepthFirstSearch::depth_first_search(v, depth + 1);
            v.record.pop();
            if best_value > value {
                best_value = value;
            }
        }

        return best_value;
    }

    fn move_picker(v: &DepthFirstSearch) -> Vec<bool> {
        let mut legal_moves: Vec<bool> = Vec::new();
        if v.record.len() == 0 {
            legal_moves.push(false);
        } else {
            legal_moves.push(true);
            legal_moves.push(false);
        }

        return legal_moves;
    }
}

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
 
    let v = &mut DepthFirstSearch {
        a: a,
        record: Vec::new(),
    };
 
    println!("{}", DepthFirstSearch::depth_first_search(v, 0));
}
