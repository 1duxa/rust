#[allow(dead_code)]
mod graph_ops {
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct Graph {
        name: String,
        pub head: Rc<RefCell<Node>>,
    }

    impl Graph {
        pub fn new(name: impl Into<String>) -> Graph {
            Graph {
                name: name.into(),
                head: Rc::new(RefCell::new(Node::new(1))),
            }
        }
        fn walk_recursive(
            node: Rc<RefCell<Node>>,
            visited: &mut HashSet<i8>,
            path: &mut Vec<(i8, i32)>,
            paths: &mut Vec<Vec<(i8, i32)>>,
        ) {
            let curr_node = node.borrow();
            if visited.contains(&curr_node.val) {
                return;
            }
            visited.insert(curr_node.val);
            path.push((curr_node.val, 0)); // weight will be updated when returning

            if curr_node.val == 7 {
                paths.push(path.clone());
            } else {
                for edge in &curr_node.next_nodes {
                    if edge.direction {
                        let mut new_path = path.clone();
                        new_path.last_mut().unwrap().1 = edge.weight;
                        Graph::walk_recursive(Rc::clone(&edge.node), visited, &mut new_path, paths);
                    }
                }
            }

            visited.remove(&curr_node.val);
        }

        fn print_paths(&self, paths: &[Vec<(i8, i32)>]) {
            for path in paths {
                for &(node, weight) in path {
                    print!("{} ({} weight) -> ", node, weight);
                }
                println!();
            }
        }

        pub fn find_shortest_path(&self, from: i8, to: i8) -> Option<Vec<(i8, i32)>> {
            let mut visited = HashSet::new();
            let mut path = vec![];
            let mut paths = vec![];
            Graph::walk_recursive(Rc::clone(&self.head), &mut visited, &mut path, &mut paths);

            let mut shortest_path = None;
            let mut min_weight = i32::MAX;

            for path in paths {
                if path.first().unwrap().0 == from && path.last().unwrap().0 == to {
                    let total_weight: i32 = path.iter().map(|&(_, weight)| weight).sum();
                    if total_weight < min_weight {
                        min_weight = total_weight;
                        shortest_path = Some(path);
                    }
                }
            }

            shortest_path
        }

        pub fn create_weight_map(&self) -> HashMap<i8, HashMap<i8, i32>> {
            let mut map: HashMap<i8, HashMap<i8, i32>> = HashMap::new();
            let mut visited = HashMap::new();
            self.collect_weights(Rc::clone(&self.head), &mut map, &mut visited);
            map
        }

        fn collect_weights(
            &self,
            node: Rc<RefCell<Node>>,
            matrix: &mut HashMap<i8, HashMap<i8, i32>>,
            visited: &mut HashMap<i8, bool>,
        ) {
            let curr_node = node.borrow();
            if visited.contains_key(&curr_node.val) {
                return;
            }
            visited.insert(curr_node.val, true);

            let mut weights = HashMap::new();
            for edge in &curr_node.next_nodes {
                if edge.direction {
                    weights.insert(edge.node.borrow().val, edge.weight);
                    self.collect_weights(Rc::clone(&edge.node), matrix, visited);
                }
            }
            matrix.insert(curr_node.val, weights);
        }
    }

    #[derive(Debug)]
    pub struct Node {
        pub val: i8,
        pub next_nodes: Vec<Edge>,
    }

    #[derive(Debug)]
    pub struct Edge {
        pub node: Rc<RefCell<Node>>,
        pub weight: i32,
        pub direction: bool, // true -> , false <-
    }

    impl Node {
        pub fn new(val: i8) -> Self {
            Self {
                val,
                next_nodes: Vec::new(),
            }
        }

        pub fn add_edge(&mut self, node: Rc<RefCell<Node>>, weight: i32, direction: bool) {
            self.next_nodes.push(Edge {
                node,
                weight,
                direction,
            });
        }
    }
}

fn main() {
    use graph_ops::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    let node1 = Rc::new(RefCell::new(Node::new(1)));
    let node2 = Rc::new(RefCell::new(Node::new(2)));
    let node3 = Rc::new(RefCell::new(Node::new(3)));
    let node4 = Rc::new(RefCell::new(Node::new(4)));
    let node5 = Rc::new(RefCell::new(Node::new(5)));
    let node6 = Rc::new(RefCell::new(Node::new(6)));
    let node7 = Rc::new(RefCell::new(Node::new(7)));

    {
        let mut node2_borrow = node2.borrow_mut();
        node2_borrow.add_edge(Rc::clone(&node1), 16, false);
        node2_borrow.add_edge(Rc::clone(&node5), 2, true);
        node2_borrow.add_edge(Rc::clone(&node3), 2, false);
        node2_borrow.add_edge(Rc::clone(&node4), 4, true);
    }

    {
        let mut node3_borrow = node3.borrow_mut();
        node3_borrow.add_edge(Rc::clone(&node5), 5, true);
        node3_borrow.add_edge(Rc::clone(&node2), 2, true);
        node3_borrow.add_edge(Rc::clone(&node1), 11, false);
    }

    {
        let mut node4_borrow = node4.borrow_mut();
        node4_borrow.add_edge(Rc::clone(&node5), 3, false);
        node4_borrow.add_edge(Rc::clone(&node2), 4, false);
        node4_borrow.add_edge(Rc::clone(&node7), 1, true);
        node4_borrow.add_edge(Rc::clone(&node6), 9, true);
    }

    {
        let mut node5_borrow = node5.borrow_mut();
        node5_borrow.add_edge(Rc::clone(&node3), 5, false);
        node5_borrow.add_edge(Rc::clone(&node2), 2, false);
        node5_borrow.add_edge(Rc::clone(&node7), 6, true);
        node5_borrow.add_edge(Rc::clone(&node4), 3, true);
    }

    {
        let mut node6_borrow = node6.borrow_mut();
        node6_borrow.add_edge(Rc::clone(&node4), 9, false);
        node6_borrow.add_edge(Rc::clone(&node7), 10, true);
    }

    {
        let mut node7_borrow = node7.borrow_mut();
        node7_borrow.add_edge(Rc::clone(&node5), 6, false);
        node7_borrow.add_edge(Rc::clone(&node6), 10, false);
    }

    let graph = Graph::new("d93b5375-e1bf-4a59-a957-af0ebe8b57ea");
    {
        let mut graph_head_borrow = graph.head.borrow_mut();
        graph_head_borrow.add_edge(Rc::clone(&node3), 11, true);
        graph_head_borrow.add_edge(Rc::clone(&node2), 16, true);
    }

    //let weight_matrix = graph.create_weight_map();
    //    Graph::floyd_warshall(weight_matrix);

    let res = graph.find_shortest_path(2, 7);
    if let Some(sigma) = res {
        for skibidi in sigma {
            println!("{:?}", skibidi);
        }
    }
}
