use proconio::input;
use petgraph::Graph;
use petgraph::graph::node_index;
use petgraph::algo::dijkstra;
 
fn main() {
    input! {
        k: usize,
    }

    let mut gr = Graph::new();
    let mut node = Vec::new();
    for _ in 0..k {
        node.push(gr.add_node(()));
    }
    for i in 1..k {
        let index = (i + 1) % k;
        gr.add_edge(node[i as usize], node[index as usize], 1);
        let index = (i * 10) % k;
        gr.add_edge(node[i as usize], node[index as usize], 0);
    }

    let path = dijkstra(&gr, node[1], None, |e| *e.weight());

    println!("{}", path.get(&node_index(0)).unwrap() + 1);
}
