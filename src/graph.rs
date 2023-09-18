use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

type GraphIndex = usize;
#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub struct NodeIndex(pub GraphIndex);

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub struct EdgeIndex(pub GraphIndex);
pub type EdgePair = (EdgeIndex, NodeIndex);
struct FrontierEntry<T>(u32, T);
impl<T> PartialEq for FrontierEntry<T> {
    fn eq(&self, other: &Self) -> bool {
        other.0.eq(&self.0)
    }
}

impl<T> Eq for FrontierEntry<T> {}

impl<T> PartialOrd for FrontierEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl<T> Ord for FrontierEntry<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

pub trait Edge {
    fn cost(&self) -> u32 {
        0
    }
}

#[derive(Debug)]
pub struct Graph<T, E: Edge> {
    nodes: Vec<T>,
    edges: Vec<E>,
    connections: HashMap<NodeIndex, HashSet<EdgePair>>,
}

impl<T, E: Edge> Graph<T, E> {
    pub fn new() -> Graph<T, E> {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            connections: HashMap::new(),
        }
    }

    pub fn insert_node(&mut self, node: T) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(node);
        NodeIndex(index)
    }

    pub fn get_node(&self, node: NodeIndex) -> Option<&T> {
        self.nodes.get(node.0)
    }

    /**
        Inserts a directed edge.
    */
    pub fn insert_edge(&mut self, edge: E, conn_a: NodeIndex, conn_b: NodeIndex) -> EdgeIndex {
        let index = self.edges.len();
        self.edges.push(edge);
        let set = self.connections.entry(conn_a).or_insert(HashSet::new());
        set.insert((EdgeIndex(index), conn_b));
        EdgeIndex(index)
    }

    pub fn insert_edge_undirected(
        &mut self,
        edge: E,
        conn_a: NodeIndex,
        conn_b: NodeIndex,
    ) -> EdgeIndex {
        let index = self.edges.len();
        self.edges.push(edge);
        let set1 = self.connections.entry(conn_a).or_insert(HashSet::new());
        set1.insert((EdgeIndex(index), conn_b));
        let set2 = self.connections.entry(conn_b).or_insert(HashSet::new());
        set2.insert((EdgeIndex(index), conn_a));
        EdgeIndex(index)
    }

    pub fn get_edge(&self, edge: EdgeIndex) -> Option<&E> {
        self.edges.get(edge.0)
    }

    pub fn get_connections(&self, node: NodeIndex) -> Option<&HashSet<EdgePair>> {
        self.connections.get(&node)
    }

    pub fn bfs(&self, from: NodeIndex, to: NodeIndex) -> Option<Vec<NodeIndex>> {
        let mut explored = HashMap::<NodeIndex, NodeIndex>::new(); // node, from
        let mut frontier = VecDeque::<NodeIndex>::new();
        explored.insert(from, from);
        frontier.push_back(from);

        while let Some(current) = frontier.pop_front() {
            if let Some(connections) = self.get_connections(current) {
                for (_, node) in connections {
                    if explored.contains_key(node) {
                        continue;
                    }
                    explored.insert(*node, current);
                    if *node == to {
                        let mut out: Vec<NodeIndex> = Vec::new();
                        let mut current = to;
                        while current != from {
                            out.push(current);
                            let from_node = explored.get(&current).unwrap();
                            current = *from_node
                        }
                        out.push(from);
                        out.reverse();
                        return Some(out);
                    } else {
                        frontier.push_back(*node);
                    }
                }
            }
        }
        None
    }

    pub fn astar(
        &self,
        from: NodeIndex,
        to: NodeIndex,
        h: impl Fn(&T, &T) -> u32,
    ) -> Option<Vec<NodeIndex>> {
        let mut frontier: BinaryHeap<FrontierEntry<NodeIndex>> = BinaryHeap::new();
        let mut explored = HashMap::<NodeIndex, (u64, NodeIndex)>::new(); // cost, from
        explored.insert(from, (0, from));
        frontier.push(FrontierEntry(0, from));

        while let Some(current) = frontier.pop() {
            let current_cost;
            {
                let (cc, _) = explored.get(&current.1).unwrap();
                current_cost = *cc;
            }
            if let Some(connections) = self.get_connections(current.1) {
                for (edge_index, node) in connections {
                    let edge = self.get_edge(*edge_index).unwrap();
                    let new_cost = current_cost + edge.cost() as u64;
                    {
                        let mut_node = explored.get_mut(node);
                        // Update the node if new route is faster than the old route
                        if let Some(entry) = mut_node {
                            if entry.0 > new_cost {
                                *entry = (new_cost, *node);
                            }
                            continue;
                        }
                    }
                    explored.insert(*node, (current_cost, current.1));
                    if *node == to {
                        let mut out: Vec<NodeIndex> = Vec::new();
                        let mut current = to;
                        while current != from {
                            out.push(current);
                            let (_, from_node) = explored.get(&current).unwrap();
                            current = *from_node
                        }
                        out.push(from);
                        out.reverse();
                        return Some(out);
                    } else {
                        frontier.push(FrontierEntry(
                            edge.cost()
                                + h(self.get_node(*node).unwrap(), self.get_node(to).unwrap()),
                            *node,
                        ));
                    }
                }
            }
        }
        None
    }

    pub fn dijkstra(&self, from: NodeIndex, to: NodeIndex) -> Option<Vec<NodeIndex>> {
        // I tested the extra overhead of even calling the extra function h. It seems rust basically removes the cost all-together
        self.astar(from, to, |_, _| 0)
    }

    pub fn to_mermaid_format(
        &self,
        format_node: impl Fn(&T, NodeIndex) -> String,
        format_edge: impl Fn(&E) -> String,
    ) -> String {
        let mut out = "graph TD;".to_string();
        for (from_node_index, edges) in self.connections.iter() {
            let from_node_label =
                format_node(self.get_node(*from_node_index).unwrap(), *from_node_index);
            for (edge_index, to_node_index) in edges.iter() {
                let edge = self.get_edge(*edge_index).unwrap();
                let to_node_label =
                    format_node(self.get_node(*to_node_index).unwrap(), *to_node_index);
                out.push_str(&format!(
                    "\n\t{}-->|{}|{};",
                    from_node_label,
                    format_edge(edge),
                    to_node_label
                ));
            }
        }
        out
    }

    pub fn to_mermaid(&self) -> String {
        self.to_mermaid_format(
            |_, index| index.0.to_string(),
            |edge| edge.cost().to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[derive(Debug)]
    struct EmptyEdge;
    impl Edge for EmptyEdge {}
    struct WeightedEdge(u32);

    impl Edge for WeightedEdge {
        fn cost(&self) -> u32 {
            self.0
        }
    }

    impl Edge for Box<dyn Edge> {
        fn cost(&self) -> u32 {
            self.as_ref().cost()
        }
    }

    #[bench]
    fn bench_dijkstra(bench: &mut Bencher) {
        let mut g: Graph<String, WeightedEdge> = Graph::new();

        let a = g.insert_node("A".into());
        let b = g.insert_node("B".into());
        let c = g.insert_node("C".into());
        let d = g.insert_node("D".into());
        let e = g.insert_node("E".into());
        let f = g.insert_node("F".into());

        g.insert_edge(WeightedEdge(2), a, b);
        g.insert_edge(WeightedEdge(1), a, c);
        g.insert_edge(WeightedEdge(3), b, d);
        g.insert_edge(WeightedEdge(5), c, d);
        g.insert_edge(WeightedEdge(4), c, e);
        g.insert_edge(WeightedEdge(2), d, e);
        g.insert_edge(WeightedEdge(1), e, f);
        g.insert_edge(WeightedEdge(6), b, f);

        bench.iter(|| {
            g.dijkstra(a, f);
        });
    }

    #[bench]
    fn bench_huge_dijkstra(bench: &mut Bencher) {
        let data = include_str!("../data/test/facebook_combined.txt")
            .split("\n")
            .map(|s| {
                s.split_whitespace()
                    .map(|s| s.parse::<u32>().ok())
                    .filter_map(|x| x)
                    .collect()
            })
            .collect::<Vec<Vec<u32>>>();

        let mut g: Graph<u32, EmptyEdge> = Graph::new();
        let mut nodes = HashMap::<u32, NodeIndex>::new();

        for vec in data {
            if vec.len() < 2 {
                continue;
            }
            let first = vec[0];
            let second = vec[1];
            if !nodes.contains_key(&first) {
                let idx = g.insert_node(first);
                nodes.insert(first, idx);
            }
            if !nodes.contains_key(&second) {
                let idx = g.insert_node(second);
                nodes.insert(second, idx);
            }
            g.insert_edge(
                EmptyEdge,
                *nodes.get(&first).unwrap(),
                *nodes.get(&second).unwrap(),
            );
        }

        bench.iter(|| {
            g.bfs(*nodes.get(&0).unwrap(), *nodes.get(&3080).unwrap());
        });

        println!(
            "{:?}",
            g.bfs(*nodes.get(&0).unwrap(), *nodes.get(&3080).unwrap())
                .unwrap()
                .iter()
                .map(|a| *g.get_node(*a).unwrap())
                .collect::<Vec<u32>>()
        );
    }

    #[bench]
    fn bfs_performs_well(bench: &mut Bencher) {
        let mut g: Graph<String, WeightedEdge> = Graph::new();

        let a = g.insert_node("A".into());
        let b = g.insert_node("B".into());
        let c = g.insert_node("C".into());
        let d = g.insert_node("D".into());
        let e = g.insert_node("E".into());
        let f = g.insert_node("F".into());

        g.insert_edge(WeightedEdge(2), a, b);
        g.insert_edge(WeightedEdge(1), a, c);
        g.insert_edge(WeightedEdge(3), b, d);
        g.insert_edge(WeightedEdge(5), c, d);
        g.insert_edge(WeightedEdge(4), c, e);
        g.insert_edge(WeightedEdge(2), d, e);
        g.insert_edge(WeightedEdge(1), e, f);
        g.insert_edge(WeightedEdge(6), b, f);

        bench.iter(|| {
            g.bfs(a, f);
        });
    }

    #[test]
    fn test_unweighted_bfs() {
        let mut g: Graph<String, EmptyEdge> = Graph::new();

        let a = g.insert_node("A".into());
        let b = g.insert_node("B".into());
        let c = g.insert_node("C".into());
        let d = g.insert_node("D".into());
        let e = g.insert_node("E".into());

        g.insert_edge(EmptyEdge, a, b);
        g.insert_edge(EmptyEdge, b, c);
        g.insert_edge(EmptyEdge, c, e);
        g.insert_edge(EmptyEdge, b, d);
        g.insert_edge(EmptyEdge, d, c);

        let result_bfs = g.bfs(a, e);
        let result_dijkstra = g.dijkstra(a, e);

        let path_bfs: Vec<String> = result_bfs
            .unwrap()
            .iter()
            .map(|n| g.get_node(*n).unwrap().clone())
            .collect();
        let path_dijkstra: Vec<String> = result_dijkstra
            .unwrap()
            .iter()
            .map(|n| g.get_node(*n).unwrap().clone())
            .collect();
        assert!(
            path_bfs.join("->") == path_dijkstra.join("->"),
            "Dijkstra and BFS results are not the same for simple undirected graphs"
        );
        assert_eq!(
            path_bfs.join("->"),
            "A->B->C->E",
            "Dijkstra path is incorrect"
        );
    }

    #[test]
    fn dijkstra_with_cycles() {
        let mut g: Graph<String, EmptyEdge> = Graph::new();

        let a = g.insert_node("A".into());
        let b = g.insert_node("B".into());
        let c = g.insert_node("C".into());
        let d = g.insert_node("D".into());
        let e = g.insert_node("E".into());
        let f = g.insert_node("F".into());

        g.insert_edge(EmptyEdge, a, b);
        g.insert_edge(EmptyEdge, b, c);
        g.insert_edge(EmptyEdge, c, e);
        g.insert_edge(EmptyEdge, b, d);
        g.insert_edge(EmptyEdge, d, c);
        g.insert_edge(EmptyEdge, d, f);
        g.insert_edge(EmptyEdge, f, a);

        let result_dijkstra = g.dijkstra(d, e);

        let path_strings: Vec<String> = result_dijkstra
            .expect("dijkstra did not resolve")
            .iter()
            .map(|n| g.get_node(*n).unwrap().clone())
            .collect();

        assert_eq!(
            path_strings.join("->"),
            "D->C->E",
            "Dijkstra path is incorrect"
        );
    }

    #[test]
    fn basic_weighted_graph() {
        let mut g: Graph<String, WeightedEdge> = Graph::new();

        let a = g.insert_node("A".into());
        let b = g.insert_node("B".into());
        let c = g.insert_node("C".into());

        g.insert_edge(WeightedEdge(1), a, b);
        g.insert_edge(WeightedEdge(2), b, c);
        g.insert_edge(WeightedEdge(4), a, c);

        let result_dijkstra = g.dijkstra(a, c);

        let path_strings: Vec<String> = result_dijkstra
            .expect("dijkstra did not resolve")
            .iter()
            .map(|n| g.get_node(*n).unwrap().clone())
            .collect();

        assert_eq!(
            path_strings.join("->"),
            "A->C",
            "Dijkstra path is incorrect"
        );
    }

    #[test]
    fn med_weighted_graph() {
        let mut g: Graph<String, WeightedEdge> = Graph::new();

        let a = g.insert_node("A".into());
        let b = g.insert_node("B".into());
        let c = g.insert_node("C".into());
        let d = g.insert_node("D".into());
        let e = g.insert_node("E".into());
        let f = g.insert_node("F".into());

        g.insert_edge(WeightedEdge(2), a, b);
        g.insert_edge(WeightedEdge(1), a, c);
        g.insert_edge(WeightedEdge(3), b, d);
        g.insert_edge(WeightedEdge(5), c, d);
        g.insert_edge(WeightedEdge(4), c, e);
        g.insert_edge(WeightedEdge(2), d, e);
        g.insert_edge(WeightedEdge(1), e, f);
        g.insert_edge(WeightedEdge(6), b, f);

        let result_dijkstra = g.dijkstra(a, f);

        let path_strings: Vec<String> = result_dijkstra
            .expect("dijkstra did not resolve")
            .iter()
            .map(|n| g.get_node(*n).unwrap().clone())
            .collect();

        println!("{}", path_strings.join("->"));

        println!(
            "{}",
            g.to_mermaid_format(|n, _| n.clone(), |e| e.cost().to_string())
        );
    }

    #[test]
    fn test_insert_node_and_get_node() {
        let mut graph = Graph::<String, EmptyEdge>::new();
        let node_data = "NodeData".to_string();
        let node_index = graph.insert_node(node_data.clone());

        assert_eq!(graph.get_node(node_index), Some(&node_data));
    }

    #[test]
    fn test_insert_edge_and_get_edge() {
        let mut graph = Graph::<String, WeightedEdge>::new();
        let node_a = graph.insert_node("Node A".into());
        let node_b = graph.insert_node("Node B".into());
        let edge = WeightedEdge(10); // Replace with your Edge type.

        let edge_index = graph.insert_edge(edge, node_a, node_b);

        assert_eq!(graph.get_edge(edge_index).map(|e| e.cost()), Some(10));
    }

    #[test]
    fn test_insert_edge_undirected() {
        let mut graph = Graph::<String, WeightedEdge>::new();
        let node_a = graph.insert_node("Node A".into());
        let node_b = graph.insert_node("Node B".into());
        let edge = WeightedEdge(10);

        let edge_index = graph.insert_edge_undirected(edge, node_a, node_b);

        assert_eq!(graph.get_edge(edge_index).map(|e| e.cost()), Some(10));

        // Check the reverse direction as well
        assert_eq!(graph.get_edge(edge_index).map(|e| e.cost()), Some(10));
    }

    #[test]
    fn works_with_both_weighted_and_unweighted_edges() {
        let mut graph = Graph::<String, Box<dyn Edge>>::new();
        let node_a = graph.insert_node("A".into());
        let node_b = graph.insert_node("B".into());
        let node_c = graph.insert_node("C".into());
        let node_d = graph.insert_node("D".into());
        let ab = Box::new(WeightedEdge(10));
        let ac = Box::new(EmptyEdge);
        let bd = Box::new(EmptyEdge);
        let cd = Box::new(WeightedEdge(20));

        graph.insert_edge_undirected(ab, node_a, node_b);
        graph.insert_edge_undirected(ac, node_b, node_c);
        graph.insert_edge_undirected(bd, node_b, node_d);
        graph.insert_edge_undirected(cd, node_c, node_d);

        let path_str = graph
            .dijkstra(node_a, node_d)
            .expect("dijkstra did not resolve")
            .iter()
            .map(|n| graph.get_node(*n).unwrap().clone())
            .collect::<Vec<String>>()
            .join("->");

        assert_eq!(path_str, "A->B->D");
    }
}
