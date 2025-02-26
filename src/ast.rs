use crate::tokenize::tokenize;

#[derive(Debug, PartialEq)]
struct ID {
    name: String,
}

const RESERVED_WORDS: [&str; 6] = ["node", "edge", "graph", "digraph", "subgraph", "strict"];

fn valid_as_id(token: &String) -> bool {
    for reserved_word in RESERVED_WORDS.iter() {
        if token.to_lowercase() == reserved_word.to_string() {
            return false;
        }
    }
    token.chars().all(|c| c.is_alphanumeric())
        && token.chars().next().unwrap_or(' ').is_alphabetic()
}

fn parse_id(tokens: &Vec<String>) -> Result<(ID, Vec<String>), String> {
    if tokens.len() == 0 {
        return Err("no tokens".to_string());
    }
    if !valid_as_id(&tokens[0]) {
        return Err("invalid id".to_string());
    }
    Ok((
        ID {
            name: tokens[0].clone(),
        },
        tokens[1..].to_vec(),
    ))
}

#[test]
fn test_parse_id() {
    let tokens = tokenize("a b".to_string());
    let (id, rest) = parse_id(&tokens).unwrap();
    assert_eq!(id.name, "a");
    assert_eq!(rest, vec!["b".to_string()]);

    let tokens = vec!["1".to_string(), "b".to_string()];
    let result = parse_id(&tokens);
    assert!(result.is_err());
}

struct IDEqStmt {
    id_left: ID,
    id_right: ID,
}

fn parse_id_eq_stmt(tokens: &Vec<String>) -> Result<(IDEqStmt, Vec<String>), String> {
    let (id_left, rest) = parse_id(tokens)?;
    if rest.len() == 0 {
        return Err("no tokens".to_string());
    }
    if rest[0] != "=" {
        return Err("expected =".to_string());
    }
    let (id_right, rest) = parse_id(&rest[1..].to_vec())?;
    Ok((IDEqStmt { id_left, id_right }, rest))
}

#[test]
fn test_parse_id_eq_stmt() {
    let tokens = tokenize("a = b".to_string());
    let (id_eq_stmt, rest) = parse_id_eq_stmt(&tokens).unwrap();
    assert_eq!(id_eq_stmt.id_left.name, "a");
    assert_eq!(id_eq_stmt.id_right.name, "b");
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = vec![
        "a".to_string(),
        "=".to_string(),
        "b".to_string(),
        "c".to_string(),
    ];
    let (id_eq_stmt, rest) = parse_id_eq_stmt(&tokens).unwrap();
    assert_eq!(id_eq_stmt.id_left.name, "a");
    assert_eq!(id_eq_stmt.id_right.name, "b");
    assert_eq!(rest, vec!["c".to_string()]);

    let tokens = tokenize("a b".to_string());
    let result = parse_id_eq_stmt(&tokens);
    assert!(result.is_err());
}

#[derive(Debug, PartialEq)]
enum EdgeStmtEdge {
    // TODO: We can take subgraph as the left side of the edge.
    NodeID(ID),
}

#[derive(Debug, PartialEq)]
enum EdgeStmtOp {
    Directed,
    Undirected,
}

#[derive(Debug, PartialEq)]
struct EdgeStmtRHS {
    edge_op: EdgeStmtOp,
    edge_egdge: EdgeStmtEdge,
    edge_rhs: Option<Box<EdgeStmtRHS>>,
}

#[derive(Debug, PartialEq)]
struct EdgeStmt {
    edge_edge: EdgeStmtEdge,
    edge_rhs: Option<Box<EdgeStmtRHS>>,
    // TODO attributes
}

fn parse_edge_stmt_edge(tokens: &Vec<String>) -> Result<(EdgeStmtEdge, Vec<String>), String> {
    let (id, rest) = parse_id(tokens)?;
    Ok((EdgeStmtEdge::NodeID(id), rest))
}

#[test]
fn test_parse_edge_stmt_edge() {
    let tokens = vec!["a".to_string()];
    let (edge_edge, rest) = parse_edge_stmt_edge(&tokens).unwrap();
    match edge_edge {
        EdgeStmtEdge::NodeID(id) => assert_eq!(id.name, "a"),
    }
    assert_eq!(rest, vec![] as Vec<String>);
}

fn parse_edge_stmt_op(tokens: &Vec<String>) -> Result<(EdgeStmtOp, Vec<String>), String> {
    if tokens.len() == 0 {
        return Err("no tokens".to_string());
    }
    match tokens[0].as_str() {
        "--" => Ok((EdgeStmtOp::Undirected, tokens[1..].to_vec())),
        "->" => Ok((EdgeStmtOp::Directed, tokens[1..].to_vec())),
        _ => Err("expected edge operator".to_string()),
    }
}

#[test]
fn test_parse_edge_stmt_op() {
    let tokens = vec!["--".to_string()];
    let (edge_op, rest) = parse_edge_stmt_op(&tokens).unwrap();
    match edge_op {
        EdgeStmtOp::Undirected => {}
        _ => panic!("expected undirected"),
    }
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = vec!["->".to_string()];
    let (edge_op, rest) = parse_edge_stmt_op(&tokens).unwrap();
    match edge_op {
        EdgeStmtOp::Directed => {}
        _ => panic!("expected directed"),
    }
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = vec!["a".to_string()];
    let result = parse_edge_stmt_op(&tokens);
    assert!(result.is_err());
}

fn parse_edge_stmt_rhs(tokens: &Vec<String>) -> Result<(EdgeStmtRHS, Vec<String>), String> {
    let (edge_op, rest) = parse_edge_stmt_op(tokens)?;
    let (edge_edge, rest) = parse_edge_stmt_edge(&rest)?;
    if rest.len() == 0 {
        return Ok((
            EdgeStmtRHS {
                edge_op,
                edge_egdge: edge_edge,
                edge_rhs: None,
            },
            rest,
        ));
    }
    let (edge_rhs, rest) = parse_edge_stmt_rhs(&rest)?;
    Ok((
        EdgeStmtRHS {
            edge_op,
            edge_egdge: edge_edge,
            edge_rhs: Some(Box::new(edge_rhs)),
        },
        rest,
    ))
}

#[test]
fn test_parse_edge_stmt_rhs() {
    let tokens = tokenize("-- a".to_string());
    let (edge_rhs, rest) = parse_edge_stmt_rhs(&tokens).unwrap();
    match edge_rhs.edge_egdge {
        EdgeStmtEdge::NodeID(id) => assert_eq!(id.name, "a"),
    }
    match edge_rhs.edge_op {
        EdgeStmtOp::Undirected => {}
        _ => panic!("expected undirected"),
    }
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize("-- a -- b".to_string());
    let (edge_rhs, rest) = parse_edge_stmt_rhs(&tokens).unwrap();
    match edge_rhs.edge_egdge {
        EdgeStmtEdge::NodeID(id) => assert_eq!(id.name, "a"),
    }
    match edge_rhs.edge_op {
        EdgeStmtOp::Undirected => {}
        _ => panic!("expected undirected"),
    }
    match edge_rhs.edge_rhs {
        Some(rhs) => {
            match rhs.edge_egdge {
                EdgeStmtEdge::NodeID(id) => assert_eq!(id.name, "b"),
            }
            match rhs.edge_op {
                EdgeStmtOp::Undirected => {}
                _ => panic!("expected undirected"),
            }
            assert_eq!(rhs.edge_rhs, None);
        }
        None => panic!("expected edge_rhs"),
    }
    assert_eq!(rest, vec![] as Vec<String>);
}

fn parse_edge_stmt(tokens: &Vec<String>) -> Result<(EdgeStmt, Vec<String>), String> {
    let (edge_edge, rest) = parse_edge_stmt_edge(tokens)?;
    if rest.len() == 0 {
        return Ok((
            EdgeStmt {
                edge_edge,
                edge_rhs: None,
            },
            rest,
        ));
    }
    let (edge_rhs, rest) = parse_edge_stmt_rhs(&rest)?;
    Ok((
        EdgeStmt {
            edge_edge,
            edge_rhs: Some(Box::new(edge_rhs)),
        },
        rest,
    ))
}

#[test]
fn test_parse_edge_stmt() {
    let tokens = tokenize("a -- b".to_string());
    let (edge_stmt, rest) = parse_edge_stmt(&tokens).unwrap();
    match edge_stmt.edge_edge {
        EdgeStmtEdge::NodeID(id) => assert_eq!(id.name, "a"),
    }
    match edge_stmt.edge_rhs {
        Some(rhs) => {
            match rhs.edge_egdge {
                EdgeStmtEdge::NodeID(id) => assert_eq!(id.name, "b"),
            }
            match rhs.edge_op {
                EdgeStmtOp::Undirected => {}
                _ => panic!("expected undirected"),
            }
            assert_eq!(rhs.edge_rhs, None);
        }
        None => panic!("expected edge_rhs"),
    }
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize("a -- b -- c".to_string());
    let (edge_stmt, rest) = parse_edge_stmt(&tokens).unwrap();
    match edge_stmt.edge_edge {
        EdgeStmtEdge::NodeID(id) => assert_eq!(id.name, "a"),
    }
    match edge_stmt.edge_rhs {
        Some(rhs) => {
            match rhs.edge_egdge {
                EdgeStmtEdge::NodeID(id) => assert_eq!(id.name, "b"),
            }
            match rhs.edge_op {
                EdgeStmtOp::Undirected => {}
                _ => panic!("expected undirected"),
            }
            match rhs.edge_rhs {
                Some(rhs) => {
                    match rhs.edge_egdge {
                        EdgeStmtEdge::NodeID(id) => assert_eq!(id.name, "c"),
                    }
                    match rhs.edge_op {
                        EdgeStmtOp::Undirected => {}
                        _ => panic!("expected undirected"),
                    }
                    assert_eq!(rhs.edge_rhs, None);
                }
                None => panic!("expected edge_rhs"),
            }
        }
        None => panic!("expected edge_rhs"),
    }
    assert_eq!(rest, vec![] as Vec<String>);
}
