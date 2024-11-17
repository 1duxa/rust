use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
struct Node {
    name: String,
    edges: Vec<Edge>,
}

#[derive(Debug)]
struct Edge {
    destination: usize,
    cost: i32,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    fn add_node(&mut self, name: &str) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Node {
            name: name.to_string(),
            edges: Vec::new(),
        });
        index
    }

    fn add_edge(&mut self, src: usize, dest: usize, cost: i32) {
        self.nodes[src].edges.push(Edge {
            destination: dest,
            cost,
        });
    }

    fn dijkstra(&self, start: usize, goal: usize) -> Option<(i32, Vec<usize>)> {
        let mut dist: Vec<_> = (0..self.nodes.len()).map(|_| i32::MAX).collect();
        let mut prev: Vec<Option<usize>> = (0..self.nodes.len()).map(|_| None).collect();
        let mut heap = BinaryHeap::new();

        dist[start] = 0;
        heap.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position == goal {
                let mut path = Vec::new();
                let mut current = goal;
                while let Some(prev_node) = prev[current] {
                    path.push(current);
                    current = prev_node;
                }
                path.push(start);
                path.reverse();
                return Some((cost, path));
            }

            if cost > dist[position] {
                continue;
            }

            for edge in &self.nodes[position].edges {
                let next_cost = cost + edge.cost;
                let next_position = edge.destination;

                if next_cost < dist[next_position] {
                    heap.push(State {
                        cost: next_cost,
                        position: next_position,
                    });
                    dist[next_position] = next_cost;
                    prev[next_position] = Some(position);
                }
            }
        }

        None
    }

    fn adjacency_matrix(&self) -> Vec<Vec<i32>> {
        let size = self.nodes.len();
        let mut matrix = vec![vec![i32::MAX; size]; size];

        for (i, node) in self.nodes.iter().enumerate() {
            for edge in &node.edges {
                matrix[i][edge.destination] = edge.cost;
            }
        }

        matrix
    }

    fn display_adjacency_matrix(&self) {
        let matrix = self.adjacency_matrix();
        println!("Adjacency Matrix:");
        for row in &matrix {
            for &val in row {
                if val == i32::MAX {
                    print!("inf\t");
                } else {
                    print!("{}\t", val);
                }
            }
            println!();
        }
    }

    fn to_dot(&self) -> String {
        let mut dot = String::from("digraph G {\n");
        for (i, node) in self.nodes.iter().enumerate() {
            for edge in &node.edges {
                dot.push_str(&format!(
                    "\t\"{}\" -> \"{}\" [label = \"{}\"];\n",
                    self.nodes[i].name, self.nodes[edge.destination].name, edge.cost
                ));
            }
        }
        dot.push_str("}\n");
        dot
    }

    fn save_dot(&self, filename: &str) {
        let dot = self.to_dot();
        let mut file = File::create(filename).expect("Unable to create file");
        file.write_all(dot.as_bytes())
            .expect("Unable to write data");
    }
}

#[derive(Debug, Eq)]
struct State {
    cost: i32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.position == other.position
    }
}

fn main() {
    let mut graph = Graph::new();
    let kyiv = graph.add_node("Kyiv");
    let london = graph.add_node("London");
    let paris = graph.add_node("Paris");
    let nyc = graph.add_node("New York");
    let tokyo = graph.add_node("Tokyo");
    let sydney = graph.add_node("Sydney");

    graph.add_edge(kyiv, london, 150);
    graph.add_edge(london, paris, 100);
    graph.add_edge(paris, nyc, 200);
    graph.add_edge(nyc, tokyo, 400);
    graph.add_edge(tokyo, sydney, 300);
    graph.add_edge(sydney, kyiv, 500);

    graph.add_edge(london, kyiv, 150);
    graph.add_edge(paris, london, 100);
    graph.add_edge(nyc, paris, 200);
    graph.add_edge(tokyo, nyc, 400);
    graph.add_edge(kyiv, sydney, 500);

    graph.display_adjacency_matrix();

    if let Some((cost, path)) = graph.dijkstra(kyiv, tokyo) {
        println!(
            "Shortest path from Kyiv to Sydney: {:?} with total cost {}",
            path, cost
        );
    } else {
        println!("No path found from Kyiv to Sydney");
    }

    graph.save_dot("graph.dot");
}
