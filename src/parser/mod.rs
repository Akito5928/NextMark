pub mod ast;
pub mod block;
pub mod inline;

pub use ast::Node;

pub fn parse(src: &str) -> Vec<Node> {
    block::parse_blocks(src)
}
