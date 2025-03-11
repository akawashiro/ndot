// Re-export modules needed by the library
mod ast;
mod graph;
mod layout;
mod ndot;
mod svg;
mod tokenize;

// Re-export the make_svg_from_dot function
pub use ndot::make_svg_from_dot;

// Tests can remain in the library
#[cfg(test)]
mod graph_test;
#[cfg(test)]
mod layout_test;
