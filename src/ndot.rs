use crate::ast;
use crate::tokenize;

pub fn make_svg_from_dot(dot: String) -> String {
    let tokens = tokenize::tokenize(dot);
    let ast = ast::parse_graph(&tokens);
    assert!(ast.is_ok(), "Failed to parse graph ast:{:?}", ast);
    "Dummy SVG".to_string()
}

#[test]
fn test_digraph_dot() {
    let dot = std::fs::read_to_string("digraph.dot").unwrap();
    make_svg_from_dot(dot);
}

#[test]
fn test_large_graphs_dot() {
    let dot = std::fs::read_to_string("large_graphs.dot").unwrap();
    make_svg_from_dot(dot);
}

#[test]
fn test_full_digraph_dot() {
    let dot = std::fs::read_to_string("full_digraph.dot").unwrap();
    make_svg_from_dot(dot);
}

#[test]
fn test_subgraphs_dot() {
    let dot = std::fs::read_to_string("subgraphs.dot").unwrap();
    make_svg_from_dot(dot);
}

#[test]
fn test_showing_a_path_dot() {
    let dot = std::fs::read_to_string("showing_a_path.dot").unwrap();
    make_svg_from_dot(dot);
}

#[test]
fn test_port_dot() {
    let dot = std::fs::read_to_string("port.dot").unwrap();
    make_svg_from_dot(dot);
}
