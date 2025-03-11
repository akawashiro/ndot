// Re-export modules needed by the library
mod ast;
mod graph;
mod layout;
mod svg;
mod tokenize;

pub fn make_svg_from_dot(dot: String) -> Result<String, String> {
    let tokens = tokenize::tokenize(dot);
    let (ast, _) = ast::parse_graph(&tokens)?;
    let graph = graph::construct_graph(&ast)?;
    return Ok(svg::graph_to_svg(&graph));
}

// Tests can remain in the library
#[cfg(test)]
mod graph_test;
#[cfg(test)]
mod layout_test;
#[cfg(test)]
pub mod e2e_test;
