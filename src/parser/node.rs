use std::io::{self, Write};

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
    pub is_term: bool
}

impl Node {
    pub fn new(name: &str, children: Vec<Node>) -> Node {
        Node { name: name.to_string(), children, is_term: false }
    }

    pub fn print<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        type Edge = (usize, usize);
        type NodeInfo = (String, bool);

        fn flatten_node_tree(n: &Node, ed: &mut Vec<Edge>, nodes: &mut Vec<NodeInfo>) {
            let cur = nodes.len();
            nodes.push((n.name.clone(), n.is_term));
            for c in &n.children {
                ed.push((cur, nodes.len()));
                flatten_node_tree(c, ed, nodes);
            }
        }

        let mut edges = Vec::new();
        let mut nodes = Vec::new();
        flatten_node_tree(&self, &mut edges, &mut nodes);

        writeln!(writer, "digraph parsed {{")?;
        for (i, (name, is_term)) in nodes.iter().enumerate() {
            let fill_color = if *is_term {"azure3"} else {"azure2"};
            writeln!(writer, "N{}[label=\"{}\",style=filled,fillcolor={}]", i, name, fill_color)?;
        }
        for (src, dest) in edges {
            writeln!(writer, "N{} -> {{ N{} }}", src, dest)?;
        }
        writeln!(writer, "}}")?;

        Ok(())
    }
}