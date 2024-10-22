#[allow(dead_code)]
pub mod graph {
    use std::collections::HashMap;

    use graph_items::{edge::Edge, node::Node};
    
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
    
            #[derive(Debug, PartialEq, Eq,Clone)]
            pub struct Edge {
                pub node_a: String,
                pub node_b: String,
                pub attrs: HashMap<String, String>,
            }
    
            impl Edge {
                pub fn new(node_a: &str, node_b: &str) -> Self {
                    Self {
                        node_a: node_a.to_string(),
                        node_b: node_b.to_string(),
                        attrs: HashMap::new(),
                    }
                }
    
                pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> &mut Self {
                    for attr in attrs {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }
    
                pub fn attr(&self, key: &str) -> Option<&str> {
                    self.attrs.get(key).map(|s| s.as_str())
                }
            }
        }
    
        pub mod node {
            use std::collections::HashMap;
    
            #[derive(Debug, PartialEq, Eq, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }
    
            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
    
                pub fn with_attrs(&mut self, attrs: &[(&str, &str)]) -> &mut Self {
                    for attr in attrs {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }
                    self
                }
            }
        }
    }

    pub trait AsNode {
        fn name(&self) -> &str;
    }

    impl AsNode for Node {
        fn name(&self) -> &str {
            &self.name
        }
    }

    impl AsNode for &mut Node {
        fn name(&self) -> &str {
            &self.name
        }
    }
    pub trait AsEdge {
        fn as_edge(&self) -> Edge;
    }

    impl AsEdge for Edge {
        fn as_edge(&self) -> Edge {
            self.clone()
        }
    }

    impl AsEdge for &Edge {
        fn as_edge(&self) -> Edge {
            (*self).clone()
        }
    }

    impl AsEdge for &mut Edge {
        fn as_edge(&self) -> Edge {
            (*self).clone()
        }
    }


    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
    
        pub fn with_nodes<N: AsNode>(mut self, nodes: &Vec<N>) -> Self {
            for node in nodes {
                self.nodes.push(Node::new(node.name()));
            }
            self
        }
        

        pub fn with_edges<T: AsEdge>(mut self, edges: &[T]) -> Self {
            for edge in edges {
                self.edges.push(edge.as_edge());
            }
            self
        }
    
        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for attr in attrs {
                self.attrs.insert(attr.0.to_string(), attr.1.to_string());
            }
            self
        }

        // Method to find a node by name
        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }
    }
}
