pub mod graph {
    use std::collections::HashMap;
    use graph::graph_items::edge::Edge;
    use graph::graph_items::node::Node;

    pub struct Graph {
        pub edges: Vec<Edge>,
        pub nodes: Vec<Node>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                edges: Vec::new(),
                nodes: Vec::new(),
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
            for node in nodes {
                self.nodes.push(node.clone());
            }
            self
        }

        pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
            for edge in edges {
                self.edges.push(edge.clone());
            }
            self
        }

        pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
            for (k, v) in attrs {
                self.attrs.insert(k.to_string(), v.to_string());
            }
            self
        }
    }

    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Edge {
                left: String,
                right: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(left: &str, right: &str) -> Self {
                    Edge {
                        left: left.to_string(),
                        right: right.to_string(),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, Debug, PartialEq)]
            pub struct Node {
                value: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(value: &str) -> Self {
                    Node {
                        value: value.to_string(),
                        attrs: HashMap::new()
                    }
                }

                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    for (k, v) in attrs {
                        self.attrs.insert(k.to_string(), v.to_string());
                    }
                    self
                }
            }
        }
    }
}
