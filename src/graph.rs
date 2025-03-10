use crate::ast;
use crate::tokenize;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Node {
    // Should we use a string or a number?
    pub id: ast::ID,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Edge {
    pub is_directed: bool,
    pub source: Node,
    pub target: Node,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

fn collect_node_from_edge_stmt_edge(edge_edge: &ast::EdgeStmtEdge) -> Vec<Node> {
    match edge_edge {
        ast::EdgeStmtEdge::NodeID(id) => {
            vec![Node { id: id.id.clone() }]
        }
        ast::EdgeStmtEdge::Subgraph(subgraph) => collect_node_from_subgraph(subgraph),
    }
}

fn collect_nodes_from_edge_stmt_rhs(edge_rhs: &ast::EdgeStmtRHS) -> Vec<Node> {
    let mut nodes = collect_node_from_edge_stmt_edge(&edge_rhs.edge_edge);
    if let Some(tail) = edge_rhs.edge_rhs.as_ref() {
        nodes.extend(collect_nodes_from_edge_stmt_rhs(&tail));
    }
    nodes
}

fn collect_nodes_from_edgestmt(edge: &ast::EdgeStmt) -> Vec<Node> {
    let mut nodes = collect_node_from_edge_stmt_edge(&edge.edge_edge);
    if let Some(tail) = edge.edge_rhs.as_ref() {
        nodes.extend(collect_nodes_from_edge_stmt_rhs(&tail));
    }
    nodes
}

fn collect_nodes_from_stmtlist(stmt_list: &ast::StmtList) -> Vec<Node> {
    let mut nodes = vec![];
    let mut added: std::collections::HashSet<ast::ID> = std::collections::HashSet::new();
    match &stmt_list.stmt {
        ast::Stmt::NodeStmt(node_stmt) => {
            if added.insert(node_stmt.id.clone()) {
                nodes.push(Node {
                    id: node_stmt.id.clone(),
                });
            }
        }
        ast::Stmt::EdgeStmt(edge_stmt) => {
            let nodes_from_edge = collect_nodes_from_edgestmt(&edge_stmt);
            for node in nodes_from_edge {
                if added.insert(node.id.clone()) {
                    nodes.push(node);
                }
            }
        }
        _ => {}
    }
    if let Some(tail) = stmt_list.stmt_list.as_ref() {
        let nodes_from_tail = collect_nodes_from_stmtlist(&tail);
        for node in nodes_from_tail {
            if added.insert(node.id.clone()) {
                nodes.push(node);
            }
        }
    }

    nodes
}

fn collect_edge_from_edge_stmt_rhs(
    edge_rhs: &ast::EdgeStmtRHS,
    left_nodes: &Vec<Node>,
) -> Vec<Edge> {
    let is_directed = if edge_rhs.edge_op == ast::EdgeStmtOp::Directed {
        true
    } else {
        false
    };
    let mut edges = vec![];
    let mut right_nodes = vec![];

    match &edge_rhs.edge_edge {
        ast::EdgeStmtEdge::NodeID(id) => {
            for left_node in left_nodes {
                edges.push(Edge {
                    is_directed: is_directed,
                    source: left_node.clone(),
                    target: Node { id: id.id.clone() },
                });
            }
            right_nodes = vec![Node { id: id.id.clone() }];
        }
        ast::EdgeStmtEdge::Subgraph(subgraph) => {
            let right_nodes = collect_node_from_subgraph(subgraph);
            for left_node in left_nodes {
                for right_node in &right_nodes {
                    edges.push(Edge {
                        is_directed: is_directed,
                        source: left_node.clone(),
                        target: right_node.clone(),
                    });
                }
            }
        }
    }
    if let Some(tail) = edge_rhs.edge_rhs.as_ref() {
        edges.extend(collect_edge_from_edge_stmt_rhs(&tail, &right_nodes));
    }
    edges
}

fn collect_node_from_subgraph(subgraph: &ast::Subgraph) -> Vec<Node> {
    // We should check all stmts in stmt_list are NodeStmt.
    collect_nodes_from_stmtlist(&subgraph.stmt_list)
}

fn collect_edge_from_stmtlist(stmt_list: &ast::StmtList) -> Vec<Edge> {
    let mut edges = vec![];
    match &stmt_list.stmt {
        ast::Stmt::EdgeStmt(edge_stmt) => match edge_stmt.edge_edge {
            ast::EdgeStmtEdge::NodeID(ref id) => {
                if let Some(ref edge_rhs) = edge_stmt.edge_rhs {
                    edges.extend(collect_edge_from_edge_stmt_rhs(
                        &edge_rhs,
                        &vec![Node { id: id.id.clone() }],
                    ));
                }
            }
            ast::EdgeStmtEdge::Subgraph(ref subgraph) => {
                let left_nodes = collect_node_from_subgraph(subgraph);
                if let Some(ref edge_rhs) = edge_stmt.edge_rhs {
                    edges.extend(collect_edge_from_edge_stmt_rhs(&edge_rhs, &left_nodes));
                }
            }
        },
        _ => {}
    }
    if let Some(tail) = stmt_list.stmt_list.as_ref() {
        edges.extend(collect_edge_from_stmtlist(&tail));
    }
    edges
}

pub fn construct_graph(ast: &ast::Graph) -> Result<Graph, String> {
    let nodes = collect_nodes_from_stmtlist(&ast.stmt_list);
    let edges = collect_edge_from_stmtlist(&ast.stmt_list);
    Ok(Graph { nodes, edges })
}
