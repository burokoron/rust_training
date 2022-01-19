/*
petgraphライブラリを用いたグラフにおける最短経路探索
始点ノードからすべての点に対する最短経路を求める
コストが負だと使えるないが、高速

例題
Typical90_003：https://atcoder.jp/contests/typical90/tasks/typical90_c

*/

use petgraph::Graph;
use petgraph::graph::node_index;
use petgraph::algo::dijkstra;

fn main() {
    // グラフの構築
    let n = 3_usize;
    let ab: Vec<(usize, usize)> = vec![(1, 2), (2, 3)];

    let mut gr = Graph::new_undirected();
    let mut node = Vec::new();
    for _ in 0..n {
        node.push(gr.add_node(()));
    }
    for &(a, b) in &ab {
        gr.add_edge(node[a as usize -1], node[b as usize - 1], 1);
    }

    // node[0]を始点としたすべての点に対する最短経路探索
    // 変数4つ目の書き方が謎、また調べる
    // HashMap(NodeIndex, cost)
    let path = dijkstra(&gr, node[0], None, |e| *e.weight());

    // 始点node[0]から最も遠いノードを調べるなら
    let mut max_weight = 0;
    let mut start = node_index(0);
    for (i, weight) in path {
        if weight > max_weight {
            max_weight = weight;
            start = i;
        }
    }
}
