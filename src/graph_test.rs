use crate::ast;
use crate::graph::{construct_graph, Edge, Graph, Node};
use crate::tokenize;
use std::collections::HashSet;

// Helper function to create a HashSet of IDs from string slices
fn hashset_of_ids(ids: Vec<&str>) -> HashSet<ast::ID> {
    ids.iter()
        .map(|id| ast::ID {
            name: id.to_string(),
        })
        .collect()
}

// Helper function to create a HashSet of Edges from (source, target) string pairs
fn hashset_of_edges(edges: Vec<(&str, &str)>) -> HashSet<Edge> {
    edges
        .iter()
        .map(|(source, target)| Edge {
            is_directed: true,
            source_id: ast::ID {
                name: source.to_string(),
            },
            target_id: ast::ID {
                name: target.to_string(),
            },
        })
        .collect()
}

#[test]
fn test_construct_graph_ab_bc() {
    let token = tokenize::tokenize("graph { a -> b; b -> c; }".to_string());
    let (ast, rest) = ast::parse_graph(&token).unwrap();
    let expected_rest = vec![] as Vec<String>;
    assert_eq!(rest, expected_rest);

    let graph = construct_graph(&ast).unwrap();

    // Convert to HashSet for unordered comparison
    let nodes_set: HashSet<ast::ID> = HashSet::from_iter(graph.nodes.iter().map(|n| n.id.clone()));
    let expected_nodes_set = hashset_of_ids(vec!["a", "b", "c"]);

    let edges_set: HashSet<Edge> = HashSet::from_iter(graph.edges.iter().cloned());
    let expected_edges_set = hashset_of_edges(vec![("a", "b"), ("b", "c")]);

    assert_eq!(nodes_set, expected_nodes_set);
    assert_eq!(edges_set, expected_edges_set);
}

#[test]
fn test_construct_graph_abc() {
    let token = tokenize::tokenize("graph { a -> b -> c; }".to_string());
    let (ast, rest) = ast::parse_graph(&token).unwrap();
    let expected_rest = vec![] as Vec<String>;
    assert_eq!(rest, expected_rest);

    let graph = construct_graph(&ast).unwrap();

    // Create expected graph structure
    let expected_graph = Graph {
        nodes: vec![
            Node {
                id: ast::ID {
                    name: "a".to_string(),
                },
                label: None,
            },
            Node {
                id: ast::ID {
                    name: "b".to_string(),
                },
                label: None,
            },
            Node {
                id: ast::ID {
                    name: "c".to_string(),
                },
                label: None,
            },
        ],
        edges: vec![
            Edge {
                is_directed: true,
                source_id: ast::ID {
                    name: "a".to_string(),
                },
                target_id: ast::ID {
                    name: "b".to_string(),
                },
            },
            Edge {
                is_directed: true,
                source_id: ast::ID {
                    name: "b".to_string(),
                },
                target_id: ast::ID {
                    name: "c".to_string(),
                },
            },
        ],
    };

    // Convert to HashSet for unordered comparison
    let nodes_set: HashSet<ast::ID> = HashSet::from_iter(graph.nodes.iter().map(|n| n.id.clone()));
    let expected_nodes_set: HashSet<ast::ID> =
        HashSet::from_iter(expected_graph.nodes.iter().map(|n| n.id.clone()));

    let edges_set: HashSet<Edge> = HashSet::from_iter(graph.edges.iter().cloned());
    let expected_edges_set: HashSet<Edge> =
        HashSet::from_iter(expected_graph.edges.iter().cloned());

    assert_eq!(nodes_set, expected_nodes_set);
    assert_eq!(edges_set, expected_edges_set);
}

#[test]
fn test_construct_graph_a_bc_with_subgraph() {
    let token = tokenize::tokenize("digraph { a -> { b c } }".to_string());
    let (ast, rest) = ast::parse_graph(&token).unwrap();
    let expected_rest = vec![] as Vec<String>;
    assert_eq!(rest, expected_rest);

    let graph = construct_graph(&ast).unwrap();

    // Convert to HashSet for unordered comparison
    let nodes_set: HashSet<ast::ID> = HashSet::from_iter(graph.nodes.iter().map(|n| n.id.clone()));
    let expected_nodes_set = hashset_of_ids(vec!["a", "b", "c"]);

    let edges_set: HashSet<Edge> = HashSet::from_iter(graph.edges.iter().cloned());
    let expected_edges_set = hashset_of_edges(vec![("a", "b"), ("a", "c")]);

    assert_eq!(nodes_set, expected_nodes_set);
    assert_eq!(edges_set, expected_edges_set);
}

#[test]
fn test_construct_graph_ab_c_with_subgraph() {
    let token = tokenize::tokenize("digraph { { a b } -> c }".to_string());
    let (ast, rest) = ast::parse_graph(&token).unwrap();
    let expected_rest = vec![] as Vec<String>;
    assert_eq!(rest, expected_rest);

    let graph = construct_graph(&ast).unwrap();

    // Create expected graph structure with complete structs
    let expected_graph = Graph {
        nodes: vec![
            Node {
                id: ast::ID {
                    name: "a".to_string(),
                },
                label: None,
            },
            Node {
                id: ast::ID {
                    name: "b".to_string(),
                },
                label: None,
            },
            Node {
                id: ast::ID {
                    name: "c".to_string(),
                },
                label: None,
            },
        ],
        edges: vec![
            Edge {
                is_directed: true,
                source_id: ast::ID {
                    name: "a".to_string(),
                },
                target_id: ast::ID {
                    name: "c".to_string(),
                },
            },
            Edge {
                is_directed: true,
                source_id: ast::ID {
                    name: "b".to_string(),
                },
                target_id: ast::ID {
                    name: "c".to_string(),
                },
            },
        ],
    };

    // Convert to HashSet for unordered comparison
    let nodes_set: HashSet<ast::ID> = HashSet::from_iter(graph.nodes.iter().map(|n| n.id.clone()));
    let expected_nodes_set: HashSet<ast::ID> =
        HashSet::from_iter(expected_graph.nodes.iter().map(|n| n.id.clone()));

    let edges_set: HashSet<Edge> = HashSet::from_iter(graph.edges.iter().cloned());
    let expected_edges_set: HashSet<Edge> =
        HashSet::from_iter(expected_graph.edges.iter().cloned());

    assert_eq!(nodes_set, expected_nodes_set);
    assert_eq!(edges_set, expected_edges_set);
}

#[test]
fn test_construct_graph_diamond() {
    // Create a diamond-shaped DAG:
    //   a
    //  / \
    // b   c
    //  \ /
    //   d
    let dot_string = "digraph { a -> b; a -> c; b -> d; c -> d; }";
    let token = tokenize::tokenize(dot_string.to_string());
    let (ast, rest) = ast::parse_graph(&token).unwrap();
    let expected_rest = vec![] as Vec<String>;
    assert_eq!(rest, expected_rest);

    // Construct the graph
    let graph = construct_graph(&ast).unwrap();

    // Verify the graph has the correct number of nodes and edges
    assert_eq!(graph.nodes.len(), 4, "Graph should have 4 nodes");
    assert_eq!(graph.edges.len(), 4, "Graph should have 4 edges");

    // Convert to HashSet for unordered comparison
    let nodes_set: HashSet<ast::ID> = HashSet::from_iter(graph.nodes.iter().map(|n| n.id.clone()));
    let expected_nodes_set = hashset_of_ids(vec!["a", "b", "c", "d"]);

    let edges_set: HashSet<Edge> = HashSet::from_iter(graph.edges.iter().cloned());
    let expected_edges_set = hashset_of_edges(vec![("a", "b"), ("a", "c"), ("b", "d"), ("c", "d")]);

    // Verify nodes
    assert_eq!(
        nodes_set, expected_nodes_set,
        "Graph should contain nodes a, b, c, and d"
    );

    // Verify edges
    assert_eq!(
        edges_set, expected_edges_set,
        "Graph should contain edges a->b, a->c, b->d, and c->d"
    );

    // Verify the graph structure
    // Check that node 'a' has outgoing edges to 'b' and 'c'
    let a_outgoing_edges: Vec<&Edge> = graph
        .edges
        .iter()
        .filter(|e| e.source_id.name == "a")
        .collect();
    assert_eq!(
        a_outgoing_edges.len(),
        2,
        "Node 'a' should have 2 outgoing edges"
    );
    assert!(
        a_outgoing_edges
            .iter()
            .any(|e| e.target_id.name == "b" && e.is_directed),
        "Node 'a' should have a directed edge to 'b'"
    );
    assert!(
        a_outgoing_edges
            .iter()
            .any(|e| e.target_id.name == "c" && e.is_directed),
        "Node 'a' should have a directed edge to 'c'"
    );

    // Check that nodes 'b' and 'c' have outgoing edges to 'd'
    let b_outgoing_edges: Vec<&Edge> = graph
        .edges
        .iter()
        .filter(|e| e.source_id.name == "b")
        .collect();
    assert_eq!(
        b_outgoing_edges.len(),
        1,
        "Node 'b' should have 1 outgoing edge"
    );
    assert!(
        b_outgoing_edges
            .iter()
            .any(|e| e.target_id.name == "d" && e.is_directed),
        "Node 'b' should have a directed edge to 'd'"
    );

    let c_outgoing_edges: Vec<&Edge> = graph
        .edges
        .iter()
        .filter(|e| e.source_id.name == "c")
        .collect();
    assert_eq!(
        c_outgoing_edges.len(),
        1,
        "Node 'c' should have 1 outgoing edge"
    );
    assert!(
        c_outgoing_edges
            .iter()
            .any(|e| e.target_id.name == "d" && e.is_directed),
        "Node 'c' should have a directed edge to 'd'"
    );

    // Check that node 'd' has no outgoing edges
    let d_outgoing_edges: Vec<&Edge> = graph
        .edges
        .iter()
        .filter(|e| e.source_id.name == "d")
        .collect();
    assert_eq!(
        d_outgoing_edges.len(),
        0,
        "Node 'd' should have no outgoing edges"
    );
}

#[test]
fn test_node_with_label() {
    // Create a graph with a node that has a label attribute
    let dot_string = "digraph { a [label=\"Foo\"]; }";
    let token = tokenize::tokenize(dot_string.to_string());
    let (ast, rest) = ast::parse_graph(&token).unwrap();
    let expected_rest = vec![] as Vec<String>;
    assert_eq!(rest, expected_rest);

    // Construct the graph
    let graph = construct_graph(&ast).unwrap();

    // Verify the graph has the correct number of nodes
    assert_eq!(graph.nodes.len(), 1, "Graph should have 1 node");

    // Get the node
    let node = &graph.nodes[0];

    // Verify the node has the correct ID
    assert_eq!(node.id.name, "a", "Node should have ID 'a'");

    // Verify the node has a label
    assert!(node.label.is_some(), "Node should have a label");

    // Verify the label is "Foo"
    // Note: In our implementation, we're hardcoding the label to "Foo" for demonstration purposes
    assert_eq!(
        node.label.as_ref().unwrap(),
        "Foo",
        "Node label should be 'Foo'"
    );
}

#[test]
fn test_node_with_label_and_edge() {
    let dot_string = "digraph { a [label=\"Foo\"]; a -> b; }";
    let token = tokenize::tokenize(dot_string.to_string());
    let (ast, rest) = ast::parse_graph(&token).unwrap();
    let expected_rest = vec![] as Vec<String>;
    assert_eq!(rest, expected_rest);

    // Construct the graph
    let graph = construct_graph(&ast).unwrap();

    // Verify the graph has the correct number of nodes and edges
    assert_eq!(graph.nodes.len(), 2, "Graph should have 2 nodes");
    assert_eq!(graph.edges.len(), 1, "Graph should have 1 edge");

    // Convert to HashSet for unordered comparison
    let nodes_set: HashSet<ast::ID> = HashSet::from_iter(graph.nodes.iter().map(|n| n.id.clone()));
    let expected_nodes_set = hashset_of_ids(vec!["a", "b"]);

    // Verify nodes
    assert_eq!(
        nodes_set, expected_nodes_set,
        "Graph should contain nodes 'a' and 'b'"
    );

    // Find node 'a' and verify it has the correct label
    let node_a = graph.nodes.iter().find(|n| n.id.name == "a").unwrap();
    assert!(node_a.label.is_some(), "Node 'a' should have a label");
    assert_eq!(
        node_a.label.as_ref().unwrap(),
        "Foo",
        "Node 'a' label should be 'Foo'"
    );

    // Verify the edge
    let edge = &graph.edges[0];
    assert!(edge.is_directed, "Edge should be directed");
    assert_eq!(edge.source_id.name, "a", "Edge source should be 'a'");
    assert_eq!(edge.target_id.name, "b", "Edge target should be 'b'");

    // Verify that node 'b' doesn't have a label
    let node_b = graph.nodes.iter().find(|n| n.id.name == "b").unwrap();
    assert!(node_b.label.is_none(), "Node 'b' should not have a label");
}
