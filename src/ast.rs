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
        return Err(format!("{}:{} No tokens", file!(), line!()));
    }
    if !valid_as_id(&tokens[0]) {
        return Err(format!("{}:{} Invalid id", file!(), line!()));
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

#[derive(Debug, PartialEq)]
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
    let try_rhs = parse_edge_stmt_rhs(&rest);
    if let Ok((edge_rhs, rest)) = try_rhs {
        return Ok((
            EdgeStmtRHS {
                edge_op,
                edge_egdge: edge_edge,
                edge_rhs: Some(Box::new(edge_rhs)),
            },
            rest,
        ));
    } else {
        return Ok((
            EdgeStmtRHS {
                edge_op,
                edge_egdge: edge_edge,
                edge_rhs: None,
            },
            rest,
        ));
    }
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
    let try_rhs = parse_edge_stmt_rhs(&rest);
    if let Ok((edge_rhs, rest)) = try_rhs {
        return Ok((
            EdgeStmt {
                edge_edge,
                edge_rhs: Some(Box::new(edge_rhs)),
            },
            rest,
        ));
    } else {
        return Ok((
            EdgeStmt {
                edge_edge,
                edge_rhs: None,
            },
            rest,
        ));
    }
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

    let tokens = tokenize("a -> b }".to_string());
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
                EdgeStmtOp::Directed => {}
                _ => panic!("expected directed"),
            }
            assert_eq!(rhs.edge_rhs, None);
        }
        None => panic!("expected edge_rhs"),
    }
    assert_eq!(rest, vec!["}".to_string()]);
}

#[derive(Debug, PartialEq)]
enum Stmt {
    IDEqStmt(IDEqStmt),
    EdgeStmt(EdgeStmt),
}

fn parse_stmt(tokens: &Vec<String>) -> Result<(Stmt, Vec<String>), String> {
    let try_id_eq_stmt = parse_id_eq_stmt(tokens);
    if let Ok((id_eq_stmt, rest)) = try_id_eq_stmt {
        return Ok((Stmt::IDEqStmt(id_eq_stmt), rest));
    }
    let try_edge_stmt = parse_edge_stmt(tokens);
    if let Ok((edge_stmt, rest)) = try_edge_stmt {
        return Ok((Stmt::EdgeStmt(edge_stmt), rest));
    }
    Err(format!("{}:{} Expected stmt. tokens={:?}", file!(), line!(), tokens))
}

#[test]
fn test_parse_stmt() {
    let tokens = tokenize("a = b".to_string());
    let (stmt, rest) = parse_stmt(&tokens).unwrap();
    match stmt {
        Stmt::IDEqStmt(id_eq_stmt) => {
            assert_eq!(id_eq_stmt.id_left.name, "a");
            assert_eq!(id_eq_stmt.id_right.name, "b");
        }
        _ => panic!("expected IDEqStmt"),
    }
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize("a -- b".to_string());
    let (stmt, rest) = parse_stmt(&tokens).unwrap();
    match stmt {
        Stmt::EdgeStmt(edge_stmt) => {
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
        }
        _ => panic!("expected EdgeStmt"),
    }
    assert_eq!(rest, vec![] as Vec<String>);
}

#[derive(Debug, PartialEq)]
struct StmtList {
    stmt: Stmt,
    stmt_list: Option<Box<StmtList>>,
}

fn parse_stmt_list(tokens: &Vec<String>) -> Result<(StmtList, Vec<String>), String> {
    let (stmt, rest) = parse_stmt(tokens)?;
    let try_stmt_list = parse_stmt_list(&rest);
    match try_stmt_list {
        Ok((stmt_list, rest)) => {
            return Ok((
                StmtList {
                    stmt,
                    stmt_list: Some(Box::new(stmt_list)),
                },
                rest,
            ));
        }
        Err(_) => {
            return Ok((StmtList {
                stmt,
                stmt_list: None,
            }, rest));
        }
    }
}

#[test]
fn test_parse_stmt_list() {
    let tokens = tokenize("a = b\na -- b".to_string());
    let (stmt_list, rest) = parse_stmt_list(&tokens).unwrap();
    match stmt_list.stmt {
        Stmt::IDEqStmt(id_eq_stmt) => {
            assert_eq!(id_eq_stmt.id_left.name, "a");
            assert_eq!(id_eq_stmt.id_right.name, "b");
        }
        _ => panic!("expected IDEqStmt"),
    }
    match stmt_list.stmt_list {
        Some(stmt_list) => {
            match stmt_list.stmt {
                Stmt::EdgeStmt(edge_stmt) => {
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
                }
                _ => panic!("expected EdgeStmt"),
            }
            assert_eq!(stmt_list.stmt_list, None);
        }
        None => panic!("expected stmt_list"),
    }
    assert_eq!(rest, vec![] as Vec<String>);
}

struct Graph {
    strict: bool,
    is_digraph: bool,
    stmt_list: StmtList,
}

fn parse_graph(tokens: &Vec<String>) -> Result<(Graph, Vec<String>), String> {
    let mut rest = tokens.clone();
    let mut strict = false;
    if rest.len() > 0 && rest[0].to_lowercase() == "strict" {
        strict = true;
        rest = rest[1..].to_vec();
    }
    if rest.len() == 0 {
        return Err("no tokens".to_string());
    }
    let is_digraph = match rest[0].to_lowercase().as_str() {
        "graph" => false,
        "digraph" => true,
        _ => return Err("expected graph or digraph".to_string()),
    };
    rest = rest[1..].to_vec();
    if rest.len() == 0 {
        return Err("no tokens".to_string());
    }
    match rest[0].as_str() {
        "{" => {}
        _ => return Err("expected {".to_string()),
    }
    rest = rest[1..].to_vec();
    if rest.len() == 0 {
        return Err("no tokens".to_string());
    }
    let (stmt_list, mut rest) = parse_stmt_list(&rest[0..].to_vec())?;
    if rest.len() == 0 {
        return Err("no tokens".to_string());
    }
    match rest[0].as_str() {
        "}" => {}
        _ => return Err("expected }".to_string()),
    }
    rest = rest[1..].to_vec();
    Ok((
        Graph {
            strict,
            is_digraph,
            stmt_list,
        },
        rest,
    ))
}

#[test]
fn test_parse_graph() {
    let tokens = tokenize(
        r#"graph {
    a = b
    a -- b
}"#
        .to_string(),
    );
    let (graph, rest) = parse_graph(&tokens).unwrap();
    assert_eq!(graph.strict, false);
    assert_eq!(graph.is_digraph, false);
    match graph.stmt_list.stmt {
        Stmt::IDEqStmt(id_eq_stmt) => {
            assert_eq!(id_eq_stmt.id_left.name, "a");
            assert_eq!(id_eq_stmt.id_right.name, "b");
        }
        _ => panic!("expected IDEqStmt"),
    }
    match graph.stmt_list.stmt_list {
        Some(stmt_list) => {
            match stmt_list.stmt {
                Stmt::EdgeStmt(edge_stmt) => {
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
                }
                _ => panic!("expected EdgeStmt"),
            }
            assert_eq!(stmt_list.stmt_list, None);
        }
        None => panic!("expected stmt_list"),
    }
    assert_eq!(rest, vec![] as Vec<String>);
}
