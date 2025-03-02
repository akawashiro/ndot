use crate::ast;
use crate::tokenize;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    // Should we use a string or a number?
    id: ast::ID,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Edge {
    is_directed: bool,
    source: Node,
    target: Node,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

fn collect_nodes_from_edge_stmt_rhs(edge_rhs: &ast::EdgeStmtRHS) -> Vec<Node> {
    let mut nodes = vec![];
    match &edge_rhs.edge_edge {
        ast::EdgeStmtEdge::NodeID(id) => {
            nodes.push(Node { id: id.id.clone() });
        }
        _ => {}
    }
    if let Some(tail) = edge_rhs.edge_rhs.as_ref() {
        nodes.extend(collect_nodes_from_edge_stmt_rhs(&tail));
    }
    nodes
}

fn collect_nodes_from_edgestmt(edge: &ast::EdgeStmt) -> Vec<Node> {
    let mut nodes = vec![];
    match &edge.edge_edge {
        ast::EdgeStmtEdge::NodeID(id) => {
            nodes.push(Node { id: id.id.clone() });
        }
        _ => {}
    }
    if let Some(tail) = edge.edge_rhs.as_ref() {
        nodes.extend(collect_nodes_from_edge_stmt_rhs(&tail));
    }
    nodes
}

fn collect_nodes_from_stmtlist(stmt_list: &ast::StmtList) -> Vec<Node> {
    let mut nodes = vec![];
    match &stmt_list.stmt {
        ast::Stmt::NodeStmt(node_stmt) => {
            nodes.push(Node {
                id: node_stmt.id.clone(),
            });
        }
        ast::Stmt::EdgeStmt(edge_stmt) => {
            nodes.extend(collect_nodes_from_edgestmt(&edge_stmt));
        }
        _ => {}
    }
    if let Some(tail) = stmt_list.stmt_list.as_ref() {
        nodes.extend(collect_nodes_from_stmtlist(&tail));
    }
    nodes
}

fn collect_edge_from_edge_stmt_rhs(edge_rhs: &ast::EdgeStmtRHS, left_node: &Node) -> Vec<Edge> {
    let mut edges = vec![];
    match &edge_rhs.edge_edge {
        ast::EdgeStmtEdge::NodeID(id) => {
            let is_directed = if edge_rhs.edge_op == ast::EdgeStmtOp::Directed {
                true
            } else {
                false
            };
            edges.push(Edge {
                is_directed: is_directed,
                source: left_node.clone(),
                target: Node { id: id.id.clone() },
            });
            if let Some(tail) = edge_rhs.edge_rhs.as_ref() {
                let new_left_node = Node { id: id.id.clone() };
                edges.extend(collect_edge_from_edge_stmt_rhs(&tail, &new_left_node));
            }
        }
        ast::EdgeStmtEdge::Subgraph(_) => {
            todo!("Subgraph is not implemented yet");
        }
    }
    edges
}

fn collect_edge_from_stmtlist(stmt_list: &ast::StmtList) -> Vec<Edge> {
    let mut edges = vec![];
    match &stmt_list.stmt {
        ast::Stmt::EdgeStmt(edge_stmt) => match edge_stmt.edge_edge {
            ast::EdgeStmtEdge::NodeID(ref id) => {
                if let Some(ref edge_rhs) = edge_stmt.edge_rhs {
                    edges.extend(collect_edge_from_edge_stmt_rhs(
                        &edge_rhs,
                        &Node { id: id.id.clone() },
                    ));
                }
            }
            ast::EdgeStmtEdge::Subgraph(ref subgraph) => {
                todo!("Subgraph is not implemented yet");
            }
        },
        _ => {}
    }
    if let Some(tail) = stmt_list.stmt_list.as_ref() {
        edges.extend(collect_edge_from_stmtlist(&tail));
    }
    edges
}

fn construct_graph(ast: &ast::Graph) -> Result<Graph, String> {
    let nodes = collect_nodes_from_stmtlist(&ast.stmt_list);
    let edges = collect_edge_from_stmtlist(&ast.stmt_list);
    Ok(Graph { nodes, edges })
}

fn hashset_of_ids(ids: Vec<&str>) -> std::collections::HashSet<ast::ID> {
    ids.iter()
        .map(|id| ast::ID {
            name: id.to_string(),
        })
        .collect()
}

fn hashset_of_edges(edges: Vec<(&str, &str)>) -> std::collections::HashSet<Edge> {
    edges
        .iter()
        .map(|(source, target)| Edge {
            is_directed: true,
            source: Node {
                id: ast::ID {
                    name: source.to_string(),
                },
            },
            target: Node {
                id: ast::ID {
                    name: target.to_string(),
                },
            },
        })
        .collect()
}

#[test]
fn test_construct_graph_ab_bc() {
    let token = tokenize::tokenize("graph { a -> b; b -> c; }".to_string());
    let (ast, rest) = ast::parse_graph(&token).unwrap();
    assert_eq!(rest, vec![] as Vec<String>);
    let graph = construct_graph(&ast).unwrap();
    let nodes: std::collections::HashSet<ast::ID> =
        std::collections::HashSet::from_iter(graph.nodes.iter().map(|n| n.id.clone()));
    assert_eq!(nodes, hashset_of_ids(vec!["a", "b", "c"]),);
    let edges: std::collections::HashSet<Edge> =
        std::collections::HashSet::from_iter(graph.edges.iter().map(|e| e.clone()));
    assert_eq!(edges, hashset_of_edges(vec![("a", "b"), ("b", "c")]));
}

#[test]
fn test_construct_graph_abc() {
    let token = tokenize::tokenize("graph { a -> b -> c; }".to_string());
    let (ast, rest) = ast::parse_graph(&token).unwrap();
    assert_eq!(rest, vec![] as Vec<String>);
    let graph = construct_graph(&ast).unwrap();
    let nodes: std::collections::HashSet<ast::ID> =
        std::collections::HashSet::from_iter(graph.nodes.iter().map(|n| n.id.clone()));
    assert_eq!(nodes, hashset_of_ids(vec!["a", "b", "c"]),);
    let edges: std::collections::HashSet<Edge> =
        std::collections::HashSet::from_iter(graph.edges.iter().map(|e| e.clone()));
    assert_eq!(edges, hashset_of_edges(vec![("a", "b"), ("b", "c")]));
}
