pub mod graph {
    pub mod graph_items;

    use std::collections::HashMap;

    use graph_items::{
        node::Node,
        edge::Edge,
    };

    #[derive(Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(self, nodes: &Vec<Node>) -> Self {
            Graph {
                nodes: nodes.to_vec(),
                edges: self.edges,
                attrs: self.attrs,
            }            
        }
        pub fn with_edges(self, edges: &Vec<Edge>) -> Self {
            Graph {
                nodes: self.nodes,
                edges: edges.to_vec(),
                attrs: self.attrs,
            } 
        }
        pub fn with_attrs(self, attrs: &[(&str, &str)]) -> Self {
            let mut new_attrs: HashMap<String, String> = HashMap::new(); 
            for (key, value) in attrs.iter() {
                new_attrs.insert(key.to_string(), value.to_string());
            }

            Graph {
                nodes: self.nodes,
                edges: self.edges,
                attrs: new_attrs,
            }
        }
        pub fn get_node(self, node_name: &str) -> Result<Node, &str> {
            if self.nodes.len() == 0 {
                return Err("No Nodes found on Graph. Use with_nodes to generate some");
            } 
            let mut into_iter = self.nodes.into_iter();
            let result = into_iter.find(|node| node.name == node_name).unwrap();
            Ok(result)
        }
    }
}
