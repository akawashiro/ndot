use crate::ast;
use crate::tokenize;

#[derive(Debug, PartialEq)]
struct Node {
    // Should we use a string or a number?
    id: ast::ID,
}

#[derive(Debug, PartialEq)]
struct Edge {
    source: Node,
    target: Node,
}

#[derive(Debug, PartialEq)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

fn collect_nodes_from_edge_stmt_rhs(edge_rhs: &ast::EdgeStmtRHS) -> Vec<Node> {
    let mut nodes = vec![];
    match &edge_rhs.edge_egdge {
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

fn construct_graph(ast: &ast::Graph) -> Result<Graph, String> {
    let nodes = collect_nodes_from_stmtlist(&ast.stmt_list);
    let edges = vec![];
    Ok(Graph { nodes, edges })
}

fn hashset_of_ids(ids: Vec<&str>) -> std::collections::HashSet<ast::ID> {
    ids.iter()
        .map(|id| ast::ID {
            name: id.to_string(),
        })
        .collect()
}

#[test]
fn test_construct_graph() {
    let token = tokenize::tokenize("graph { a -> b; b -> c; }".to_string());
    let (ast, rest) = ast::parse_graph(&token).unwrap();
    assert_eq!(rest, vec![] as Vec<String>);
    let graph = construct_graph(&ast).unwrap();
    let nodes: std::collections::HashSet<ast::ID> =
        std::collections::HashSet::from_iter(graph.nodes.iter().map(|n| n.id.clone()));
    assert_eq!(nodes, hashset_of_ids(vec!["a", "b", "c"]),);
}
