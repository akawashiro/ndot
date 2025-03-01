use crate::tokenize::tokenize;

#[derive(Debug, PartialEq)]
struct ID {
    name: String,
}

const RESERVED_WORDS: [&str; 6] = ["node", "edge", "graph", "digraph", "subgraph", "strict"];

fn is_reserved_word(token: &String) -> bool {
    for reserved_word in RESERVED_WORDS.iter() {
        if token.to_lowercase() == reserved_word.to_string() {
            return true;
        }
    }
    false
}

fn is_double_quoted(token: &String) -> bool {
    if token.len() < 2 {
        return false;
    }
    if !(token.chars().next().unwrap() == '"' && token.chars().last().unwrap() == '"') {
        return false;
    }
    let content = token
        .chars()
        .skip(1)
        .take(token.len() - 2)
        .collect::<String>();
    let mut last_char = ' ';
    for c in content.chars() {
        if last_char != '\\' && c == '"' {
            return false;
        }
        last_char = c;
    }
    true
}

fn valid_as_id(token: &String) -> bool {
    if is_reserved_word(token) {
        return false;
    }
    if is_double_quoted(token) {
        return true;
    }
    token.chars().all(|c| c.is_alphanumeric())
        && token.chars().next().unwrap_or(' ').is_alphabetic()
}

fn parse_id(tokens: &Vec<String>) -> Result<(ID, Vec<String>), String> {
    if tokens.len() == 0 {
        return Err(format!("{}:{} No tokens", file!(), line!()));
    }
    if !valid_as_id(&tokens[0]) {
        return Err(format!("{}:{} Invalid id: {}", file!(), line!(), tokens[0]));
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
    let (_, rest) = parse_keyword(&rest, "=")?;
    let (id_right, rest) = parse_id(&rest)?;
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

    let tokens = tokenize("a=\"b\"".to_string());
    let (id_eq_stmt, rest) = parse_id_eq_stmt(&tokens).unwrap();
    assert_eq!(id_eq_stmt.id_left.name, "a");
    assert_eq!(id_eq_stmt.id_right.name, "\"b\"");
    assert_eq!(rest, vec![] as Vec<String>);
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
    attr_list: Option<AttrList>,
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
    let (keyword, rest) = parse_keyword_list_or(tokens, &["--", "->"].to_vec())?;
    if keyword == "--" {
        return Ok((EdgeStmtOp::Undirected, rest));
    } else if keyword == "->" {
        return Ok((EdgeStmtOp::Directed, rest));
    } else {
        return Err(format!(
            "{}:{} Invalid edge op: {}",
            file!(),
            line!(),
            keyword
        ));
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
    let (edge_rhs, rest) = if let Ok((edge_rhs, rest)) = try_rhs {
        (Some(Box::new(edge_rhs)), rest)
    } else {
        (None, rest)
    };
    let try_attr_list = parse_attr_list(&rest);
    let (attr_list, rest) = if let Ok((attr_list, rest)) = try_attr_list {
        (Some(attr_list), rest)
    } else {
        (None, rest)
    };
    Ok((
        EdgeStmt {
            edge_edge,
            edge_rhs,
            attr_list,
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

    let tokens = tokenize("a -> b[label=\"0.2\"];".to_string());
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
            match edge_stmt.attr_list {
                Some(attr_list) => {
                    match attr_list.a_list {
                        Some(a_list) => {
                            assert_eq!(a_list.id_left.name, "label");
                            assert_eq!(a_list.id_right.name, "\"0.2\"");
                            assert_eq!(a_list.a_list, None);
                        }
                        None => panic!("expected a_list"),
                    }
                    assert_eq!(attr_list.attr_list, None);
                }
                None => panic!("expected attr_list. rest={:?}", rest),
            }
        }
        None => panic!("expected edge_rhs"),
    }
    assert_eq!(rest, vec![";".to_string()]);
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
    Err(format!(
        "{}:{} Expected stmt. tokens={:?}",
        file!(),
        line!(),
        tokens
    ))
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

fn parse_semicolon(tokens: &Vec<String>) -> Result<(String, Vec<String>), String> {
    parse_keyword(tokens, ";")
}

fn parse_stmt_list(tokens: &Vec<String>) -> Result<(StmtList, Vec<String>), String> {
    let (stmt, rest) = parse_stmt(tokens)?;
    let rest = parse_skip(&rest, parse_semicolon);
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
            return Ok((
                StmtList {
                    stmt,
                    stmt_list: None,
                },
                rest,
            ));
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

    let tokens = tokenize(
        r#"a = b;
a -- b;"#
            .to_string(),
    );
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

#[derive(Debug, PartialEq)]
pub struct Graph {
    strict: bool,
    id: Option<ID>,
    is_digraph: bool,
    stmt_list: StmtList,
}

fn parse_keyword(tokens: &Vec<String>, keyword: &str) -> Result<(String, Vec<String>), String> {
    if tokens.len() == 0 {
        return Err(format!("{}:{} No tokens", file!(), line!()));
    }
    if tokens[0].to_lowercase() != keyword {
        return Err(format!("{}:{} Expected {}", file!(), line!(), keyword));
    }
    Ok((keyword.to_string(), tokens[1..].to_vec()))
}

fn parse_keyword_list_or(
    tokens: &Vec<String>,
    keywords: &Vec<&str>,
) -> Result<(String, Vec<String>), String> {
    for keyword in keywords.iter() {
        let try_rest = parse_keyword(tokens, keyword);
        if let Ok((_, rest)) = try_rest {
            return Ok((keyword.to_string(), rest));
        }
    }
    Err(format!(
        "{}:{} Expected one of {:?}",
        file!(),
        line!(),
        keywords
    ))
}

pub fn parse_graph(tokens: &Vec<String>) -> Result<(Graph, Vec<String>), String> {
    let (try_strict, rest) = parse_try(tokens, |tokens| parse_keyword(tokens, "strict"))?;
    let strict = if let Some(_) = try_strict {
        true
    } else {
        false
    };
    let (graph_or_digraph, rest) =
        parse_keyword_list_or(&rest, &(["graph", "digraph"]).to_vec())?;
    let is_digraph = graph_or_digraph == "digraph";
    let (try_id, rest) = parse_try(&rest, parse_id)?;
    let id = if let Some(id) = try_id {
        Some(id)
    } else {
        None
    };
    let (_, rest) = parse_keyword(&rest, "{")?;
    let (stmt_list, rest) = parse_stmt_list(&rest)?;
    let (_, rest) = parse_keyword(&rest, "}")?;

    Ok((
        Graph {
            strict,
            id,
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

    let tokens = tokenize(
        r#"strict graph hoge {
    a = b
}"#
        .to_string(),
    );
    let (graph, rest) = parse_graph(&tokens).unwrap();
    assert_eq!(graph.strict, true);
    assert_eq!(graph.is_digraph, false);
    match graph.stmt_list.stmt {
        Stmt::IDEqStmt(id_eq_stmt) => {
            assert_eq!(id_eq_stmt.id_left.name, "a");
            assert_eq!(id_eq_stmt.id_right.name, "b");
        }
        _ => panic!("expected IDEqStmt"),
    }
    assert_eq!(graph.stmt_list.stmt_list, None);
    assert_eq!(rest, vec![] as Vec<String>);
}

#[derive(Debug, PartialEq)]
struct AList {
    id_left: ID,
    id_right: ID,
    a_list: Option<Box<AList>>,
}

fn parse_skip<T>(
    tokens: &Vec<String>,
    parse_fn: fn(&Vec<String>) -> Result<(T, Vec<String>), String>,
) -> Vec<String> {
    if let Ok((_, rest)) = parse_fn(tokens) {
        return rest;
    } else {
        return tokens.clone();
    }
}

fn parse_a_list(tokens: &Vec<String>) -> Result<(AList, Vec<String>), String> {
    let (id_left, rest) = parse_id(tokens)?;
    let (_, rest) = parse_keyword(&rest, "=")?;
    let (id_right, rest) = parse_id(&rest)?;

    let parse_semicolon_or_camma = |tokens: &Vec<String>| -> Result<(String, Vec<String>), String> {
        parse_keyword_list_or(tokens, &[";", ","].to_vec())
    };
    let rest = parse_skip(&rest, parse_semicolon_or_camma);
    let try_a_list = parse_a_list(&rest);
    match try_a_list {
        Ok((a_list, rest)) => {
            return Ok((
                AList {
                    id_left,
                    id_right,
                    a_list: Some(Box::new(a_list)),
                },
                rest,
            ));
        }
        Err(_) => {
            return Ok((
                AList {
                    id_left,
                    id_right,
                    a_list: None,
                },
                rest,
            ));
        }
    }
}

#[test]
fn test_parse_a_list() {
    let tokens = tokenize("a = b".to_string());
    let (a_list, rest) = parse_a_list(&tokens).unwrap();
    assert_eq!(a_list.id_left.name, "a");
    assert_eq!(a_list.id_right.name, "b");
    assert_eq!(a_list.a_list, None);
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize("a = b, c = d".to_string());
    let (a_list, rest) = parse_a_list(&tokens).unwrap();
    assert_eq!(a_list.id_left.name, "a");
    assert_eq!(a_list.id_right.name, "b");
    match a_list.a_list {
        Some(a_list) => {
            assert_eq!(a_list.id_left.name, "c");
            assert_eq!(a_list.id_right.name, "d");
            assert_eq!(a_list.a_list, None);
        }
        None => panic!("expected a_list"),
    }
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize("a = b; c = d".to_string());
    let (a_list, rest) = parse_a_list(&tokens).unwrap();
    assert_eq!(a_list.id_left.name, "a");
    assert_eq!(a_list.id_right.name, "b");
    match a_list.a_list {
        Some(a_list) => {
            assert_eq!(a_list.id_left.name, "c");
            assert_eq!(a_list.id_right.name, "d");
            assert_eq!(a_list.a_list, None);
        }
        None => panic!("expected a_list"),
    }
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize("a = b c = d".to_string());
    let (a_list, rest) = parse_a_list(&tokens).unwrap();
    assert_eq!(a_list.id_left.name, "a");
    assert_eq!(a_list.id_right.name, "b");
    match a_list.a_list {
        Some(a_list) => {
            assert_eq!(a_list.id_left.name, "c");
            assert_eq!(a_list.id_right.name, "d");
            assert_eq!(a_list.a_list, None);
        }
        None => panic!("expected a_list"),
    }
    assert_eq!(rest, vec![] as Vec<String>);
}

#[derive(Debug, PartialEq)]
struct AttrList {
    a_list: Option<AList>,
    attr_list: Option<Box<AttrList>>,
}

fn parse_try<T>(
    tokens: &Vec<String>,
    parse_fn: fn(&Vec<String>) -> Result<(T, Vec<String>), String>,
) -> Result<(Option<T>, Vec<String>), String> {
    let try_result = parse_fn(tokens);
    match try_result {
        Ok((result, rest)) => {
            return Ok((Some(result), rest));
        }
        Err(_) => {
            return Ok((None, tokens.clone()));
        }
    }
}

fn parse_attr_list(tokens: &Vec<String>) -> Result<(AttrList, Vec<String>), String> {
    let (_, rest) = parse_keyword(&tokens, "[")?;
    let (head_a_list, rest) = parse_try(&rest, parse_a_list)?;
    let (_, rest) = parse_keyword(&rest, "]")?;
    let (attr_list, rest) = parse_try(&rest, parse_attr_list)?;
    if let Some(attr_list) = attr_list {
        return Ok((
            AttrList {
                a_list: head_a_list,
                attr_list: Some(Box::new(attr_list)),
            },
            rest,
        ));
    } else {
        return Ok((
            AttrList {
                a_list: head_a_list,
                attr_list: None,
            },
            rest,
        ));
    }
}

#[test]
fn test_parse_attr_list() {
    let tokens = tokenize(r#"[a = b]"#.to_string());
    let (attr_list, rest) = parse_attr_list(&tokens).unwrap();
    match attr_list.a_list {
        Some(a_list) => {
            assert_eq!(a_list.id_left.name, "a");
            assert_eq!(a_list.id_right.name, "b");
            assert_eq!(a_list.a_list, None);
        }
        None => panic!("expected a_list"),
    }
    assert_eq!(attr_list.attr_list, None);
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize(r#"[a = b, c = d]"#.to_string());
    let (attr_list, rest) = parse_attr_list(&tokens).unwrap();
    match attr_list.a_list {
        Some(a_list) => {
            assert_eq!(a_list.id_left.name, "a");
            assert_eq!(a_list.id_right.name, "b");
            match a_list.a_list {
                Some(a_list) => {
                    assert_eq!(a_list.id_left.name, "c");
                    assert_eq!(a_list.id_right.name, "d");
                    assert_eq!(a_list.a_list, None);
                }
                None => panic!("expected a_list"),
            }
        }
        None => panic!("expected a_list"),
    }
    assert_eq!(attr_list.attr_list, None);
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize(r#"[]"#.to_string());
    let (attr_list, rest) = parse_attr_list(&tokens).unwrap();
    assert_eq!(attr_list.a_list, None);
    assert_eq!(attr_list.attr_list, None);
    assert_eq!(rest, vec![] as Vec<String>);
}

#[derive(Debug, PartialEq)]
struct Subgraph {
    id: Option<ID>,
    stmt_list: StmtList,
}

fn parse_subgraph(tokens: &Vec<String>) -> Result<(Subgraph, Vec<String>), String> {
    let (try_subgraph, rest) = parse_try(tokens, |tokens| parse_keyword(tokens, "subgraph"))?;
    // When and only when we have subgraph keyword, we have to try to parse ID.
    let (try_id, rest) = if let Some(_) = try_subgraph {
        parse_try(&rest, parse_id)?
    } else {
        (None, rest)
    };
    let (_, rest) = parse_keyword(&rest, "{")?;
    let (stmt_list, rest) = parse_stmt_list(&rest)?;
    let (_, rest) = parse_keyword(&rest, "}")?;
    Ok((
        Subgraph {
            id: try_id,
            stmt_list,
        },
        rest,
    ))
}

#[test]
fn test_parse_subgraph() {
    let tokens = tokenize(
        r#"subgraph sub { a = b }"#
            .to_string());
    let (subgraph, rest) = parse_subgraph(&tokens).unwrap();
    assert_eq!(subgraph.id.unwrap().name, "sub");
    match subgraph.stmt_list.stmt {
        Stmt::IDEqStmt(id_eq_stmt) => {
            assert_eq!(id_eq_stmt.id_left.name, "a");
            assert_eq!(id_eq_stmt.id_right.name, "b");
        }
        _ => panic!("expected IDEqStmt"),
    }
    assert_eq!(subgraph.stmt_list.stmt_list, None);
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize(
        r#"subgraph { a = b }"#
            .to_string());
    let (subgraph, rest) = parse_subgraph(&tokens).unwrap();
    assert_eq!(subgraph.id, None);
    match subgraph.stmt_list.stmt {
        Stmt::IDEqStmt(id_eq_stmt) => {
            assert_eq!(id_eq_stmt.id_left.name, "a");
            assert_eq!(id_eq_stmt.id_right.name, "b");
        }
        _ => panic!("expected IDEqStmt"),
    }
    assert_eq!(subgraph.stmt_list.stmt_list, None);
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize(
        r#"{ a = b }"#
            .to_string());
    let (subgraph, rest) = parse_subgraph(&tokens).unwrap();
    assert_eq!(subgraph.id, None);
    match subgraph.stmt_list.stmt {
        Stmt::IDEqStmt(id_eq_stmt) => {
            assert_eq!(id_eq_stmt.id_left.name, "a");
            assert_eq!(id_eq_stmt.id_right.name, "b");
        }
        _ => panic!("expected IDEqStmt"),
    }
    assert_eq!(subgraph.stmt_list.stmt_list, None);
    assert_eq!(rest, vec![] as Vec<String>);
}
