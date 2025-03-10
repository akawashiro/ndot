use crate::ast;
use crate::graph;
use crate::svg;
use crate::tokenize;
use log::info;
use std::fs;
use std::path::{Path, PathBuf};

pub fn make_svg_from_dot(dot: String) -> Result<String, String> {
    let tokens = tokenize::tokenize(dot);
    let (ast, _) = ast::parse_graph(&tokens)?;
    let graph = graph::construct_graph(&ast)?;
    return Ok(svg::graph_to_svg(&graph));
}

/// Process a DOT file and save the SVG in the same directory
///
/// # Arguments
///
/// * `dot_path` - Path to the DOT file
///
/// # Returns
///
/// * `Ok(String)` - Path to the generated SVG file
/// * `Err(String)` - Error message
pub fn process_dot_file<P: AsRef<Path>>(dot_path: P) -> Result<String, String> {
    // Get the absolute path to the DOT file
    let dot_path = dot_path.as_ref();
    let absolute_dot_path = match fs::canonicalize(dot_path) {
        Ok(path) => path,
        Err(e) => {
            return Err(format!(
                "Failed to get absolute path for {}: {}",
                dot_path.display(),
                e
            ))
        }
    };

    // Read the DOT file
    let dot_content = match fs::read_to_string(&absolute_dot_path) {
        Ok(content) => content,
        Err(e) => {
            return Err(format!(
                "Failed to read DOT file {}: {}",
                absolute_dot_path.display(),
                e
            ))
        }
    };

    // Generate SVG
    let svg_content = make_svg_from_dot(dot_content)?;

    // Create the output path in the same directory as the DOT file
    let mut svg_path = PathBuf::from(&absolute_dot_path);
    svg_path.set_extension("svg");

    // Write the SVG file
    match fs::write(&svg_path, svg_content) {
        Ok(_) => {
            info!("SVG saved to: {}", svg_path.display());
            Ok(svg_path.to_string_lossy().to_string())
        }
        Err(e) => Err(format!(
            "Failed to write SVG file {}: {}",
            svg_path.display(),
            e
        )),
    }
}

#[test]
fn test_digraph_dot() {
    let result = process_dot_file("digraph.dot");
    assert!(result.is_ok());
}

#[test]
fn test_large_graphs_dot() {
    let result = process_dot_file("large_graphs.dot");
    assert!(result.is_ok());
}

#[test]
fn test_full_digraph_dot() {
    let result = process_dot_file("full_digraph.dot");
    assert!(result.is_ok());
}

#[test]
fn test_subgraphs_dot() {
    let result = process_dot_file("subgraphs.dot");
    assert!(result.is_ok());
}

#[test]
fn test_showing_a_path_dot() {
    let result = process_dot_file("showing_a_path.dot");
    assert!(result.is_ok());
}

#[test]
fn test_port_dot() {
    let result = process_dot_file("port.dot");
    assert!(result.is_ok());
}

#[test]
fn test_large_diamond_dot() {
    let result = process_dot_file("large_diamond.dot");
    assert!(result.is_ok());
}

#[test]
fn test_label_dot() {
    let result = process_dot_file("label.dot");
    assert!(result.is_ok());
}
