// Re-export modules needed by the library
mod ast;
mod graph;
mod layout;
mod svg;
mod tokenize;

/// Converts a DOT format string into an SVG representation of the graph.
///
/// This function takes a string in DOT language format and transforms it into an SVG
/// representation that can be rendered in web browsers or other SVG-compatible viewers.
/// The conversion process involves several steps:
///
/// 1. Tokenizing the DOT string to break it into meaningful tokens
/// 2. Parsing the tokens into an Abstract Syntax Tree (AST)
/// 3. Constructing a graph data structure from the AST
/// 4. Converting the graph to SVG format
///
/// # Parameters
///
/// * `dot` - A string containing a graph description in DOT language format
///
/// # Returns
///
/// * `Result<String, String>` - On success, returns the SVG representation as a string.
///   On failure, returns an error message describing what went wrong.
///
/// # Errors
///
/// This function may return errors in the following cases:
///
/// * If the DOT string contains syntax errors that prevent parsing
/// * If the parsed AST cannot be converted to a valid graph structure
///
/// # Examples
///
/// ```
/// use ndot::make_svg_from_dot;
///
/// let dot_string = r#"
/// digraph {
///     a -> b;
///     b -> c;
///     a -> c;
/// }
/// "#.to_string();
///
/// match make_svg_from_dot(dot_string) {
///     Ok(svg) => {
///         // Save the SVG to a file or display it
///         println!("Successfully generated SVG");
///     },
///     Err(e) => {
///         eprintln!("Error generating SVG: {}", e);
///     }
/// }
/// ```
pub fn make_svg_from_dot(dot: String) -> Result<String, String> {
    let tokens = tokenize::tokenize(dot);
    let (ast, _) = ast::parse_graph(&tokens)?;
    let graph = graph::construct_graph(&ast)?;
    return Ok(svg::graph_to_svg(&graph));
}

// Tests can remain in the library
#[cfg(test)]
pub mod e2e_test;
#[cfg(test)]
mod graph_test;
#[cfg(test)]
mod layout_test;
