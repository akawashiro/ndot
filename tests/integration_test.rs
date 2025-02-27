extern crate ndot;

#[test]
fn test_digraph_dot() {
    let dot = std::fs::read_to_string("tests/digraph.dot").unwrap();
    let svg = ndot::make_svg_from_dot(dot);
    assert!(svg.is_ok());
}
