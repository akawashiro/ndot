use crate::ast;
use crate::tokenize;

#[derive(Debug, PartialEq)]
struct Node {
    // Should we use a string or a number?
    id: String,
    label: String,
}

#[derive(Debug, PartialEq)]
struct Edge {
    source: Node,
    target: Node,
    label: String,
}

#[derive(Debug, PartialEq)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

fn construct_graph(ast: &ast::Graph) -> Result<Graph, String> {
    return Err("Not implemented".to_string());
}

#[test]
fn test_construct_graph() {
    let token = tokenize::tokenize("graph { a -> b; b -> c; }".to_string());
    let (ast, rest) = ast::parse_graph(&token).unwrap();
    assert_eq!(rest, vec![] as Vec<String>);
    let graph = construct_graph(&ast).unwrap();
    assert_eq!(
        graph,
        Graph {
            nodes: vec![
                Node {
                    id: "a".to_string(),
                    label: "a".to_string(),
                },
                Node {
                    id: "b".to_string(),
                    label: "b".to_string(),
                },
                Node {
                    id: "c".to_string(),
                    label: "c".to_string(),
                },
            ],
            edges: vec![
                Edge {
                    source: Node {
                        id: "a".to_string(),
                        label: "a".to_string(),
                    },
                    target: Node {
                        id: "b".to_string(),
                        label: "b".to_string(),
                    },
                    label: "".to_string(),
                },
                Edge {
                    source: Node {
                        id: "b".to_string(),
                        label: "b".to_string(),
                    },
                    target: Node {
                        id: "c".to_string(),
                        label: "c".to_string(),
                    },
                    label: "".to_string(),
                },
            ],
        }
    );
}
