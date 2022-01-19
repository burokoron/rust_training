use proconio::input;
use petgraph::Graph;
use petgraph::graph::node_index;
use petgraph::algo::dijkstra;

fn main() {
    input! {
        n: usize,
        ab: [(i32, i32); n-1],
    }
 
    let mut gr = Graph::new_undirected();
    let mut node = Vec::new();
    for _ in 0..n {
        node.push(gr.add_node(()));
    }
    for &(a, b) in &ab {
        gr.add_edge(node[a as usize -1], node[b as usize - 1], 1.);
    }

    let path = dijkstra(&gr, node[0], None, |_| 1);
    let mut max_weight = 0;
    let mut start = node_index(0);
    for (i, weight) in path {
        if weight > max_weight {
            max_weight = weight;
            start = i;
        }
    }

    let path = dijkstra(&gr, start, None, |_| 1);
    let mut max_weight = 0;
    for (_, weight) in path {
        if weight > max_weight {
            max_weight = weight;
        }
    }
    println!("{}", max_weight as i64 + 1);
}
