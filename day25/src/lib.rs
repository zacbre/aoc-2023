mod input;

use std::collections::HashMap;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::NodeIndex;
use rustworkx_core::petgraph::graph::UnGraph;

fn solve(input: &str) -> usize {
    let mut graph: UnGraph<&str, u32> = UnGraph::new_undirected();
    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();
    input.lines().map(|line| line.split_at(3)).for_each(|(a, b)| {
        for b in b.split_ascii_whitespace().skip(1) {
            let a = *nodes.entry(a).or_insert_with(|| graph.add_node(a));
            let b = *nodes.entry(b).or_insert_with(|| graph.add_node(b));
            graph.add_edge(a, b, 1);
            graph.add_edge(b, a, 1);
        }
    });

    let cut: rustworkx_core::Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    let cut = cut.unwrap().unwrap();
    let total_nodes = nodes.len();
    (total_nodes - cut.1.len()) * cut.1.len()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_example() {
        let result = solve(input::EXAMPLE_INPUT);
        assert_eq!(result, 54);
    }

    #[test]
    fn can_solve() {
        let result = solve(input::INPUT);
        assert_eq!(result, 554064);
    }
}
