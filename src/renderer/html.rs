use crate::parser::Node;

pub fn render(nodes: &[Node]) -> String {
    let mut out = String::new();

    for node in nodes {
        match node {
            Node::Heading(text) => {
                out.push_str(&format!("<h1>{}</h1>\n", text));
            }
            Node::Paragraph(text) => {
                out.push_str(&format!("<p>{}</p>\n", text));
            }
        }
    }

    out
}
