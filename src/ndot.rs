use crate::tokenize;
use crate::ast;

pub fn make_svg_from_dot(dot: String) -> String {
    let tokens = tokenize::tokenize(dot);
    let ast = ast::parse_graph(&tokens);
    assert!(ast.is_ok(), "Failed to parse graph ast:{:?}", ast);
    "Dummy SVG".to_string()
}
