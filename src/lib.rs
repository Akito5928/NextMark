pub mod parser;
pub mod renderer;

pub fn render(src: &str) -> String {
    let ast = parser::parse(src);
    renderer::html::render(&ast)
}
