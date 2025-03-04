use crate::ast;
use crate::graph;
use crate::svg;
use crate::tokenize;

pub fn make_svg_from_dot(dot: String) -> Result<String, String> {
    let tokens = tokenize::tokenize(dot);
    let (ast, _) = ast::parse_graph(&tokens)?;
    let graph = graph::construct_graph(&ast)?;
    return Ok(svg::graph_to_svg(&graph));
}

#[test]
fn test_digraph_dot() {
    let dot = std::fs::read_to_string("digraph.dot").unwrap();
    let svg = make_svg_from_dot(dot);
    assert!(svg.is_ok());
}

#[test]
fn test_large_graphs_dot() {
    let dot = std::fs::read_to_string("large_graphs.dot").unwrap();
    let svg = make_svg_from_dot(dot);
    assert!(svg.is_ok());
}

#[test]
fn test_full_digraph_dot() {
    let dot = std::fs::read_to_string("full_digraph.dot").unwrap();
    let svg = make_svg_from_dot(dot);
    assert!(svg.is_ok());
}

#[test]
fn test_subgraphs_dot() {
    let dot = std::fs::read_to_string("subgraphs.dot").unwrap();
    let svg = make_svg_from_dot(dot);
    assert!(svg.is_ok());
}

#[test]
fn test_showing_a_path_dot() {
    let dot = std::fs::read_to_string("showing_a_path.dot").unwrap();
    let svg = make_svg_from_dot(dot);
    assert!(svg.is_ok());
}

#[test]
fn test_port_dot() {
    let dot = std::fs::read_to_string("port.dot").unwrap();
    let svg = make_svg_from_dot(dot);
    assert!(svg.is_ok());
}
