use std::collections::HashMap;

#[allow(dead_code)]
mod graph_ops {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct Graph {
        name: String,
        pub head: Rc<RefCell<Node>>,
    }

    impl Graph {
        pub fn new(name: impl Into<String>, head: Rc<RefCell<Node>>) -> Graph {
            Graph {
                name: name.into(),
                head,
            }
        }

        pub fn floyd_warshall(
            &self,
            weight_map: HashMap<i8, HashMap<i8, i32>>,
        ) -> HashMap<i8, HashMap<i8, i32>> {
            let mut distances = HashMap::new();
            for &src in weight_map.keys() {
                distances.entry(src).or_insert_with(HashMap::new);
                for (&dest, &weight) in &weight_map[&src] {
                    distances.get_mut(&src).unwrap().insert(dest, weight);
                }
            }

            for &node in weight_map.keys() {
                distances
                    .entry(node)
                    .or_insert_with(HashMap::new)
                    .insert(node, 0);
            }

            let nodes: Vec<i8> = weight_map.keys().copied().collect();
            for &k in &nodes {
                for &i in &nodes {
                    for &j in &nodes {
                        let through_k = distances
                            .get(&i)
                            .and_then(|m| m.get(&k))
                            .cloned()
                            .unwrap_or(i32::MAX)
                            .saturating_add(
                                distances
                                    .get(&k)
                                    .and_then(|m| m.get(&j))
                                    .cloned()
                                    .unwrap_or(i32::MAX),
                            );
                        let current_distance = distances
                            .get(&i)
                            .and_then(|m| m.get(&j))
                            .cloned()
                            .unwrap_or(i32::MAX);

                        if through_k < current_distance {
                            distances.get_mut(&i).unwrap().insert(j, through_k);
                        }
                    }
                }
            }
            distances
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
        pub direction: bool,
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

    let graph = Graph::new("d93b5375-e1bf-4a59-a957-af0ebe8b57ea", node1);
    {
        let mut graph_head_borrow = graph.head.borrow_mut();
        graph_head_borrow.add_edge(Rc::clone(&node3), 11, true);
        graph_head_borrow.add_edge(Rc::clone(&node2), 16, true);
    }

    let weight_matrix = graph.create_weight_map();
    println!("{:#?}", weight_matrix);
    let res = graph.floyd_warshall(weight_matrix);
    println!("{:#?}", res);
    find_from_to(res, 1, 6);
}

pub fn find_from_to(weights: HashMap<i8, HashMap<i8, i32>>, start: i8, end: i8) {
    println!("{}", weights[&start][&end]);
}
