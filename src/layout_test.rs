use crate::ast;
use crate::graph::{construct_graph, Graph, Node};
use crate::layout::{
    calculate_circular_positions, calculate_sugiyama_positions, center_layout, is_dag, Position,
    NODE_RADIUS, SVG_HEIGHT, SVG_WIDTH,
};
use crate::tokenize;
use std::collections::HashMap;

// Helper function to create a graph from a DOT string
fn create_graph_from_dot(dot_string: &str) -> Graph {
    let token = tokenize::tokenize(dot_string.to_string());
    let (ast, _) = ast::parse_graph(&token).unwrap();
    construct_graph(&ast).unwrap()
}

#[test]
fn test_is_dag_with_dag() {
    // Create a simple DAG: a -> b -> c
    let graph = create_graph_from_dot("digraph { a -> b; b -> c; }");

    // Test that is_dag correctly identifies this as a DAG
    assert!(is_dag(&graph));
}

#[test]
fn test_is_dag_with_cycle() {
    // Create a graph with a cycle: a -> b -> c -> a
    let graph = create_graph_from_dot("digraph { a -> b; b -> c; c -> a; }");

    // Test that is_dag correctly identifies this as not a DAG
    assert!(!is_dag(&graph));
}

#[test]
fn test_is_dag_with_undirected_edge() {
    // Create a graph with an undirected edge
    let graph = create_graph_from_dot("graph { a -- b; b -> c; }");

    // Test that is_dag correctly identifies this as not a DAG
    assert!(!is_dag(&graph));
}

#[test]
fn test_calculate_sugiyama_positions_simple_dag() {
    // Create a simple DAG: a -> b -> c
    let graph = create_graph_from_dot("digraph { a -> b; b -> c; }");

    // Calculate positions
    let positions = calculate_sugiyama_positions(&graph);

    // Test that all nodes have positions
    assert_eq!(positions.len(), 3);

    // Test that nodes are in different layers (different y-coordinates)
    let a_pos = positions
        .iter()
        .find(|(node, _)| node.id.name == "a")
        .unwrap()
        .1;
    let b_pos = positions
        .iter()
        .find(|(node, _)| node.id.name == "b")
        .unwrap()
        .1;
    let c_pos = positions
        .iter()
        .find(|(node, _)| node.id.name == "c")
        .unwrap()
        .1;

    // In a Sugiyama layout for a -> b -> c, we expect a to be above b and c
    // The exact ordering of b and c might vary depending on the implementation details
    assert!(a_pos.y < b_pos.y);
    assert!(a_pos.y < c_pos.y);
}

#[test]
fn test_calculate_sugiyama_positions_complex_dag() {
    // Create a more complex DAG
    let graph = create_graph_from_dot("digraph { a -> b; a -> c; b -> d; c -> d; }");

    // Calculate positions
    let positions = calculate_sugiyama_positions(&graph);

    // Test that all nodes have positions
    assert_eq!(positions.len(), 4);

    // Get positions
    let a_pos = positions
        .iter()
        .find(|(node, _)| node.id.name == "a")
        .unwrap()
        .1;
    let b_pos = positions
        .iter()
        .find(|(node, _)| node.id.name == "b")
        .unwrap()
        .1;
    let c_pos = positions
        .iter()
        .find(|(node, _)| node.id.name == "c")
        .unwrap()
        .1;
    let d_pos = positions
        .iter()
        .find(|(node, _)| node.id.name == "d")
        .unwrap()
        .1;

    // Check layer ordering
    assert!(a_pos.y < b_pos.y || a_pos.y < c_pos.y); // a should be above at least one of b or c
    assert!(b_pos.y < d_pos.y); // b should be above d
    assert!(c_pos.y < d_pos.y); // c should be above d
}

#[test]
fn test_calculate_circular_positions() {
    // Create a graph with a cycle
    let graph = create_graph_from_dot("digraph { a -> b; b -> c; c -> a; }");

    // Calculate positions
    let positions = calculate_circular_positions(&graph);

    // Test that all nodes have positions
    assert_eq!(positions.len(), 3);

    // Test that all positions are within SVG bounds
    for (_, pos) in positions.iter() {
        assert!(pos.x >= 0 && pos.x <= SVG_WIDTH);
        assert!(pos.y >= 0 && pos.y <= SVG_HEIGHT);
    }

    // Test that nodes are placed in a roughly circular pattern
    // Get the center of the circle
    let center_x = SVG_WIDTH as f64 / 2.0;
    let center_y = SVG_HEIGHT as f64 / 2.0;

    // Calculate the distance from each node to the center
    let mut distances = Vec::new();
    for (_, pos) in positions.iter() {
        let dx = pos.x as f64 - center_x;
        let dy = pos.y as f64 - center_y;
        let distance = (dx * dx + dy * dy).sqrt();
        distances.push(distance);
    }

    // All distances should be approximately equal (within a small tolerance)
    let avg_distance = distances.iter().sum::<f64>() / distances.len() as f64;
    for distance in distances {
        assert!((distance - avg_distance).abs() < 1.0); // Allow for rounding errors
    }
}

#[test]
fn test_center_layout() {
    // Create a simple set of positions that are not centered
    let node_a = Node {
        id: ast::ID {
            name: "a".to_string(),
        },
    };
    let node_b = Node {
        id: ast::ID {
            name: "b".to_string(),
        },
    };
    let node_c = Node {
        id: ast::ID {
            name: "c".to_string(),
        },
    };

    let mut positions = HashMap::new();
    positions.insert(node_a, Position::new(10, 10));
    positions.insert(node_b, Position::new(20, 20));
    positions.insert(node_c, Position::new(30, 30));

    // Center the layout
    center_layout(&mut positions);

    // Calculate the bounds of the centered layout
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for pos in positions.values() {
        min_x = min_x.min(pos.x);
        max_x = max_x.max(pos.x);
        min_y = min_y.min(pos.y);
        max_y = max_y.max(pos.y);
    }

    // Calculate the center of the layout
    let center_x = (min_x + max_x) / 2;
    let center_y = (min_y + max_y) / 2;

    // The center of the layout should be close to the center of the SVG
    assert!((center_x - SVG_WIDTH / 2).abs() <= NODE_RADIUS);
    assert!((center_y - SVG_HEIGHT / 2).abs() <= NODE_RADIUS);
}

#[test]
fn test_is_dag_empty_graph() {
    // Create an empty graph
    let graph = Graph {
        nodes: vec![],
        edges: vec![],
    };

    // An empty graph is technically a DAG
    assert!(is_dag(&graph));
}

#[test]
fn test_is_dag_single_node() {
    // Create a graph with a single node
    let graph = create_graph_from_dot("digraph { a; }");

    // A single node with no edges is a DAG
    assert!(is_dag(&graph));
}

#[test]
fn test_is_dag_self_loop() {
    // Create a graph with a self-loop
    let graph = create_graph_from_dot("digraph { a -> a; }");

    // A graph with a self-loop is not a DAG
    assert!(!is_dag(&graph));
}

#[test]
fn test_calculate_sugiyama_positions_empty_graph() {
    // Create an empty graph
    let graph = Graph {
        nodes: vec![],
        edges: vec![],
    };

    // Calculate positions - this should not panic
    let positions = calculate_sugiyama_positions(&graph);

    // Test that no positions are returned for an empty graph
    assert_eq!(positions.len(), 0);
}

#[test]
fn test_calculate_circular_positions_empty_graph() {
    // Create an empty graph
    let graph = Graph {
        nodes: vec![],
        edges: vec![],
    };

    // Calculate positions
    let positions = calculate_circular_positions(&graph);

    // Test that no positions are returned
    assert_eq!(positions.len(), 0);
}

#[test]
fn test_center_layout_empty_positions() {
    // Create an empty positions map
    let mut positions: HashMap<Node, Position> = HashMap::new();

    // Center the layout (should not panic)
    center_layout(&mut positions);

    // Test that the positions map is still empty
    assert_eq!(positions.len(), 0);
}

#[test]
fn test_calculate_sugiyama_positions_diamond() {
    // Create a diamond-shaped DAG: a -> b -> d, a -> c -> d
    let graph = create_graph_from_dot("digraph { a -> b; a -> c; b -> d; c -> d; }");

    // Calculate positions
    let positions = calculate_sugiyama_positions(&graph);

    // Test that all nodes have positions
    assert_eq!(positions.len(), 4);

    // Get positions
    let a_pos = positions
        .iter()
        .find(|(node, _)| node.id.name == "a")
        .unwrap()
        .1;
    let b_pos = positions
        .iter()
        .find(|(node, _)| node.id.name == "b")
        .unwrap()
        .1;
    let c_pos = positions
        .iter()
        .find(|(node, _)| node.id.name == "c")
        .unwrap()
        .1;
    let d_pos = positions
        .iter()
        .find(|(node, _)| node.id.name == "d")
        .unwrap()
        .1;

    // Check layer ordering - in a diamond DAG:
    // a should be above b and c
    assert!(a_pos.y < b_pos.y);
    assert!(a_pos.y < c_pos.y);

    // d should be below b and c
    assert!(b_pos.y < d_pos.y);
    assert!(c_pos.y < d_pos.y);

    // Check that b and c are horizontally separated
    assert!(b_pos.x != c_pos.x);
}
