use super::ast::Node;

pub fn parse_blocks(src: &str) -> Vec<Node> {
    let mut nodes = Vec::new();

    for line in src.lines() {
        if line.starts_with("# ") {
            nodes.push(Node::Heading(line[2..].to_string()));
        } else if !line.trim().is_empty() {
            nodes.push(Node::Paragraph(line.to_string()));
        }
    }

    nodes
}
