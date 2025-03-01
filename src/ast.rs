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

// Any string of alphabetic ([a-zA-Z\200-\377]) characters, underscores ('_') or digits([0-9]), not
// beginning with a digit;
fn is_alphanumeric_id(token: &String) -> bool {
    token.chars().all(|c| c.is_alphanumeric() || c == '_')
        && token.chars().next().unwrap_or(' ').is_alphabetic()
}

// a numeral [-]?(.[0-9]⁺ | [0-9]⁺(.[0-9]*)? );
fn is_number(token: &String) -> bool {
    let mut chars = token.chars();
    let first_char = chars.next().unwrap();
    if first_char == '-' {
        return is_number(&chars.collect::<String>());
    }
    let mut has_dot = false;
    let mut has_number = false;
    for c in token.chars() {
        if c == '.' {
            if has_dot {
                return false;
            }
            has_dot = true;
        } else if c.is_numeric() {
            has_number = true;
        } else {
            return false;
        }
    }
    has_number
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
    if is_alphanumeric_id(token) {
        return true;
    }
    if is_number(token) {
        return true;
    }
    false
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

    let tokens = tokenize("cluster_0".to_string());
    let (id, rest) = parse_id(&tokens).unwrap();
    assert_eq!(id.name, "cluster_0");
    assert_eq!(rest, vec![] as Vec<String>);
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
struct NodeID {
    id: ID,
    port: Option<Port>,
}

fn parse_node_id(tokens: &Vec<String>) -> Result<(NodeID, Vec<String>), String> {
    let (id, rest) = parse_id(tokens)?;
    let try_port = parse_port(&rest);
    if let Ok((port, rest)) = try_port {
        return Ok((
            NodeID {
                id,
                port: Some(port),
            },
            rest,
        ));
    } else {
        return Ok((NodeID { id, port: None }, rest));
    }
}

#[derive(Debug, PartialEq)]
enum EdgeStmtEdge {
    NodeID(NodeID),
    Subgraph(Box<Subgraph>),
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
    let (id, rest) = parse_try(tokens, parse_node_id)?;
    if let Some(id) = id {
        return Ok((EdgeStmtEdge::NodeID(id), rest));
    } else {
        let (subgraph, rest) = parse_subgraph(tokens)?;
        return Ok((EdgeStmtEdge::Subgraph(Box::new(subgraph)), rest));
    }
}

#[test]
fn test_parse_edge_stmt_edge() {
    let tokens = vec!["a".to_string()];
    let (edge_edge, rest) = parse_edge_stmt_edge(&tokens).unwrap();
    match edge_edge {
        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "a"),
        _ => panic!("expected NodeID"),
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
        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "a"),
        _ => panic!("expected NodeID"),
    }
    match edge_rhs.edge_op {
        EdgeStmtOp::Undirected => {}
        _ => panic!("expected undirected"),
    }
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize("-- a -- b".to_string());
    let (edge_rhs, rest) = parse_edge_stmt_rhs(&tokens).unwrap();
    match edge_rhs.edge_egdge {
        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "a"),
        _ => panic!("expected NodeID"),
    }
    match edge_rhs.edge_op {
        EdgeStmtOp::Undirected => {}
        _ => panic!("expected undirected"),
    }
    match edge_rhs.edge_rhs {
        Some(rhs) => {
            match rhs.edge_egdge {
                EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "b"),
                _ => panic!("expected NodeID"),
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
        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "a"),
        _ => panic!("expected NodeID"),
    }
    match edge_stmt.edge_rhs {
        Some(rhs) => {
            match rhs.edge_egdge {
                EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "b"),
                _ => panic!("expected NodeID"),
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
        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "a"),
        _ => panic!("expected NodeID"),
    }
    match edge_stmt.edge_rhs {
        Some(rhs) => {
            match rhs.edge_egdge {
                EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "b"),
                _ => panic!("expected NodeID"),
            }
            match rhs.edge_op {
                EdgeStmtOp::Undirected => {}
                _ => panic!("expected undirected"),
            }
            match rhs.edge_rhs {
                Some(rhs) => {
                    match rhs.edge_egdge {
                        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "c"),
                        _ => panic!("expected NodeID"),
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
        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "a"),
        _ => panic!("expected NodeID"),
    }
    match edge_stmt.edge_rhs {
        Some(rhs) => {
            match rhs.edge_egdge {
                EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "b"),
                _ => panic!("expected NodeID"),
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
        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "a"),
        _ => panic!("expected NodeID"),
    }
    match edge_stmt.edge_rhs {
        Some(rhs) => {
            match rhs.edge_egdge {
                EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "b"),
                _ => panic!("expected NodeID"),
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
    NodeStmt(NodeStmt),
    EdgeStmt(EdgeStmt),
    AttrStmt(AttrStmt),
    IDEqStmt(IDEqStmt),
    Subgraph(Box<Subgraph>),
}

fn parse_stmt(tokens: &Vec<String>) -> Result<(Stmt, Vec<String>), String> {
    let try_attr_stmt = parse_attr_stmt(tokens);
    if let Ok((attr_stmt, rest)) = try_attr_stmt {
        return Ok((Stmt::AttrStmt(attr_stmt), rest));
    }
    let try_subgraph = parse_subgraph(tokens);
    if let Ok((subgraph, rest)) = try_subgraph {
        return Ok((Stmt::Subgraph(Box::new(subgraph)), rest));
    }
    let try_id_eq_stmt = parse_id_eq_stmt(tokens);
    if let Ok((id_eq_stmt, rest)) = try_id_eq_stmt {
        return Ok((Stmt::IDEqStmt(id_eq_stmt), rest));
    }
    let try_edge_stmt = parse_edge_stmt(tokens);
    if let Ok((edge_stmt, rest)) = try_edge_stmt {
        return Ok((Stmt::EdgeStmt(edge_stmt), rest));
    }
    // TODO: We cannot recognize difference between node_stmt and id_eq_stmt.
    let try_node_stmt = parse_node_stmt(tokens);
    if let Ok((node_stmt, rest)) = try_node_stmt {
        return Ok((Stmt::NodeStmt(node_stmt), rest));
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
                EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "a"),
                _ => panic!("expected NodeID"),
            }
            match edge_stmt.edge_rhs {
                Some(rhs) => {
                    match rhs.edge_egdge {
                        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "b"),
                        _ => panic!("expected NodeID"),
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

    let tokens = tokenize("subgraph sub { a = b }".to_string());
    let (stmt, rest) = parse_stmt(&tokens).unwrap();
    match stmt {
        Stmt::Subgraph(subgraph) => {
            assert_eq!(subgraph.id.unwrap().name, "sub");
            match subgraph.stmt_list.stmt {
                Stmt::IDEqStmt(id_eq_stmt) => {
                    assert_eq!(id_eq_stmt.id_left.name, "a");
                    assert_eq!(id_eq_stmt.id_right.name, "b");
                }
                _ => panic!("expected IDEqStmt"),
            }
            assert_eq!(subgraph.stmt_list.stmt_list, None);
        }
        _ => panic!("expected Subgraph {:?}", stmt),
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
                        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "a"),
                        _ => panic!("expected NodeID"),
                    }
                    match edge_stmt.edge_rhs {
                        Some(rhs) => {
                            match rhs.edge_egdge {
                                EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "b"),
                                _ => panic!("expected NodeID"),
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
                        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "a"),
                        _ => panic!("expected NodeID"),
                    }
                    match edge_stmt.edge_rhs {
                        Some(rhs) => {
                            match rhs.edge_egdge {
                                EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "b"),
                                _ => panic!("expected NodeID"),
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
        return Err(format!(
            "{}:{} Expected {}. tokens={:?}",
            file!(),
            line!(),
            keyword,
            tokens
        ));
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
    let (graph_or_digraph, rest) = parse_keyword_list_or(&rest, &(["graph", "digraph"]).to_vec())?;
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
                        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "a"),
                        _ => panic!("expected NodeID"),
                    }
                    match edge_stmt.edge_rhs {
                        Some(rhs) => {
                            match rhs.edge_egdge {
                                EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "b"),
                                _ => panic!("expected NodeID"),
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

    let tokens = tokenize(r#"digraph { subgraph sub { a -> b } }"#.to_string());
    let (graph, rest) = parse_graph(&tokens).unwrap();
    assert_eq!(graph.strict, false);
    assert_eq!(graph.is_digraph, true);
    match graph.stmt_list.stmt {
        Stmt::Subgraph(subgraph) => {
            assert_eq!(subgraph.id.unwrap().name, "sub");
            match subgraph.stmt_list.stmt {
                Stmt::EdgeStmt(edge_stmt) => {
                    match edge_stmt.edge_edge {
                        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "a"),
                        _ => panic!("expected NodeID"),
                    }
                    match edge_stmt.edge_rhs {
                        Some(rhs) => {
                            match rhs.edge_egdge {
                                EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "b"),
                                _ => panic!("expected NodeID"),
                            }
                            match rhs.edge_op {
                                EdgeStmtOp::Directed => {}
                                _ => panic!("expected directed"),
                            }
                            assert_eq!(rhs.edge_rhs, None);
                        }
                        None => panic!("expected edge_rhs"),
                    }
                }
                _ => panic!("expected EdgeStmt"),
            }
            assert_eq!(subgraph.stmt_list.stmt_list, None);
        }
        _ => panic!("expected Subgraph"),
    }
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize(
        r#"digraph {
    subgraph cluster_0 {
        label="Subgraph A";
        a -> b;
    }}"#
        .to_string(),
    );
    let (graph, rest) = parse_graph(&tokens).unwrap();
    assert_eq!(graph.strict, false);
    assert_eq!(graph.is_digraph, true);
    match graph.stmt_list.stmt {
        Stmt::Subgraph(subgraph) => {
            assert_eq!(subgraph.id.unwrap().name, "cluster_0");
            match subgraph.stmt_list.stmt {
                Stmt::IDEqStmt(id_eq_stmt) => {
                    assert_eq!(id_eq_stmt.id_left.name, "label");
                    assert_eq!(id_eq_stmt.id_right.name, "\"Subgraph A\"");
                }
                _ => panic!("expected IDEqStmt"),
            }
            match subgraph.stmt_list.stmt_list {
                Some(stmt_list) => {
                    match stmt_list.stmt {
                        Stmt::EdgeStmt(edge_stmt) => {
                            match edge_stmt.edge_edge {
                                EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "a"),
                                _ => panic!("expected NodeID"),
                            }
                            match edge_stmt.edge_rhs {
                                Some(rhs) => {
                                    match rhs.edge_egdge {
                                        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "b"),
                                        _ => panic!("expected NodeID"),
                                    }
                                    match rhs.edge_op {
                                        EdgeStmtOp::Directed => {}
                                        _ => panic!("expected directed"),
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
        }
        _ => panic!("expected Subgraph"),
    }
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize(
        r#"
graph {
    a -- b[color=red,penwidth=3.0];
    }"#
        .to_string(),
    );
    let (graph, rest) = parse_graph(&tokens).unwrap();
    assert_eq!(graph.strict, false);
    assert_eq!(graph.is_digraph, false);
    match graph.stmt_list.stmt {
        Stmt::EdgeStmt(edge_stmt) => {
            match edge_stmt.edge_edge {
                EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "a"),
                _ => panic!("expected NodeID"),
            }
            match edge_stmt.edge_rhs {
                Some(rhs) => {
                    match rhs.edge_egdge {
                        EdgeStmtEdge::NodeID(id) => assert_eq!(id.id.name, "b"),
                        _ => panic!("expected NodeID"),
                    }
                    match rhs.edge_op {
                        EdgeStmtOp::Undirected => {}
                        _ => panic!("expected undirected"),
                    }
                    assert_eq!(rhs.edge_rhs, None);
                    match edge_stmt.attr_list {
                        Some(attr_list) => {
                            match attr_list.a_list {
                                Some(a_list) => {
                                    assert_eq!(a_list.id_left.name, "color");
                                    assert_eq!(a_list.id_right.name, "red");
                                    match a_list.a_list {
                                        Some(a_list) => {
                                            assert_eq!(a_list.id_left.name, "penwidth");
                                            assert_eq!(a_list.id_right.name, "3.0");
                                            assert_eq!(a_list.a_list, None);
                                        }
                                        None => panic!("expected a_list"),
                                    }
                                }
                                None => panic!("expected a_list"),
                            }
                            assert_eq!(attr_list.attr_list, None);
                        }
                        None => panic!("expected attr_list"),
                    }
                }
                None => panic!("expected edge_rhs"),
            }
        }
        _ => panic!("expected EdgeStmt"),
    }
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
    let tokens = tokenize("penwidth=3.0".to_string());
    let (a_list, rest) = parse_a_list(&tokens).unwrap();
    assert_eq!(a_list.id_left.name, "penwidth");
    assert_eq!(a_list.id_right.name, "3.0");
    assert_eq!(a_list.a_list, None);
    assert_eq!(rest, vec![] as Vec<String>);

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
    let tokens = tokenize(r#"subgraph sub { a = b }"#.to_string());
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

    let tokens = tokenize(r#"subgraph { a = b }"#.to_string());
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

    let tokens = tokenize(r#"{ a = b }"#.to_string());
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

#[derive(Debug, PartialEq)]
struct NodeStmt {
    id: ID,
    attr_list: Option<AttrList>,
}

fn parse_node_stmt(tokens: &Vec<String>) -> Result<(NodeStmt, Vec<String>), String> {
    let (id, rest) = parse_id(tokens)?;
    let try_attr_list = parse_attr_list(&rest);
    let (attr_list, rest) = if let Ok((attr_list, rest)) = try_attr_list {
        (Some(attr_list), rest)
    } else {
        (None, rest)
    };
    Ok((NodeStmt { id, attr_list }, rest))
}

#[test]
fn test_parse_node_stmt() {
    let tokens = tokenize("a".to_string());
    let (node_stmt, rest) = parse_node_stmt(&tokens).unwrap();
    assert_eq!(node_stmt.id.name, "a");
    assert_eq!(node_stmt.attr_list, None);
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize("a [label=\"0.2\"]".to_string());
    let (node_stmt, rest) = parse_node_stmt(&tokens).unwrap();
    assert_eq!(node_stmt.id.name, "a");
    match node_stmt.attr_list {
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
        None => panic!("expected attr_list"),
    }
    assert_eq!(rest, vec![] as Vec<String>);
}

#[derive(Debug, PartialEq)]
enum AttrStmtType {
    Graph,
    Node,
    Edge,
}

#[derive(Debug, PartialEq)]
struct AttrStmt {
    attr_type: AttrStmtType,
    attr_list: AttrList,
}

fn parse_attr_stmt(tokens: &Vec<String>) -> Result<(AttrStmt, Vec<String>), String> {
    let (attr_type, rest) = parse_keyword_list_or(&tokens, &["graph", "node", "edge"].to_vec())?;
    let (attr_list, rest) = parse_attr_list(&rest)?;
    let attr_type = match attr_type.as_str() {
        "graph" => AttrStmtType::Graph,
        "node" => AttrStmtType::Node,
        "edge" => AttrStmtType::Edge,
        _ => panic!("unexpected attr_type"),
    };
    Ok((
        AttrStmt {
            attr_type,
            attr_list,
        },
        rest,
    ))
}

#[test]
fn test_parse_attr_stmt() {
    let tokens = tokenize(r#"graph [label="0.2"]"#.to_string());
    let (attr_stmt, rest) = parse_attr_stmt(&tokens).unwrap();
    match attr_stmt.attr_type {
        AttrStmtType::Graph => {}
        _ => panic!("expected graph"),
    }
    match attr_stmt.attr_list.a_list {
        Some(a_list) => {
            assert_eq!(a_list.id_left.name, "label");
            assert_eq!(a_list.id_right.name, "\"0.2\"");
            assert_eq!(a_list.a_list, None);
        }
        None => panic!("expected a_list"),
    }
    assert_eq!(attr_stmt.attr_list.attr_list, None);
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize(r#"node [label="0.2"]"#.to_string());
    let (attr_stmt, rest) = parse_attr_stmt(&tokens).unwrap();
    match attr_stmt.attr_type {
        AttrStmtType::Node => {}
        _ => panic!("expected node"),
    }
    match attr_stmt.attr_list.a_list {
        Some(a_list) => {
            assert_eq!(a_list.id_left.name, "label");
            assert_eq!(a_list.id_right.name, "\"0.2\"");
            assert_eq!(a_list.a_list, None);
        }
        None => panic!("expected a_list"),
    }
    assert_eq!(attr_stmt.attr_list.attr_list, None);
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize(r#"edge [label="0.2"]"#.to_string());
    let (attr_stmt, rest) = parse_attr_stmt(&tokens).unwrap();
    match attr_stmt.attr_type {
        AttrStmtType::Edge => {}
        _ => panic!("expected edge"),
    }
    match attr_stmt.attr_list.a_list {
        Some(a_list) => {
            assert_eq!(a_list.id_left.name, "label");
            assert_eq!(a_list.id_right.name, "\"0.2\"");
            assert_eq!(a_list.a_list, None);
        }
        None => panic!("expected a_list"),
    }
    assert_eq!(attr_stmt.attr_list.attr_list, None);
    assert_eq!(rest, vec![] as Vec<String>);
}

#[derive(Debug, PartialEq)]
enum CompassPoint {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    C,
    UNDERSORE,
}

fn parse_compass_point(tokens: &Vec<String>) -> Result<(CompassPoint, Vec<String>), String> {
    let (_, rest) = parse_keyword(&tokens, ":")?;
    let (keyword, rest) = parse_keyword_list_or(
        &rest,
        &["n", "ne", "e", "se", "s", "sw", "w", "nw", "c", "_"].to_vec(),
    )?;
    let compass_point = match keyword.as_str() {
        "n" => CompassPoint::N,
        "ne" => CompassPoint::NE,
        "e" => CompassPoint::E,
        "se" => CompassPoint::SE,
        "s" => CompassPoint::S,
        "sw" => CompassPoint::SW,
        "w" => CompassPoint::W,
        "nw" => CompassPoint::NW,
        "c" => CompassPoint::C,
        "_" => CompassPoint::UNDERSORE,
        _ => panic!("unexpected compass point"),
    };
    Ok((compass_point, rest))
}

#[test]
fn test_parse_compass_point() {
    let tokens = tokenize(":n".to_string());
    let (compass_point, rest) = parse_compass_point(&tokens).unwrap();
    assert_eq!(compass_point, CompassPoint::N);
    assert_eq!(rest, vec![] as Vec<String>);
}

#[derive(Debug, PartialEq)]
enum Port {
    IDPort((ID, Option<CompassPoint>)),
    CompassPointPort(CompassPoint),
}

fn parse_port(tokens: &Vec<String>) -> Result<(Port, Vec<String>), String> {
    let try_compass_point = parse_try(tokens, parse_compass_point);
    if let Ok((Some(compass_point), rest)) = try_compass_point {
        return Ok((Port::CompassPointPort(compass_point), rest));
    }
    let (_, rest) = parse_keyword(&tokens, ":")?;
    let (id, rest) = parse_id(&rest)?;
    let try_compass_point = parse_try(&rest, parse_compass_point);
    if let Ok((compass_point, rest)) = try_compass_point {
        return Ok((Port::IDPort((id, compass_point)), rest));
    } else {
        return Ok((Port::IDPort((id, None)), rest));
    }
}

#[test]
fn test_parse_port() {
    let tokens = tokenize(":a".to_string());
    let (port, rest) = parse_port(&tokens).unwrap();
    match port {
        Port::IDPort((id, compass_point)) => {
            assert_eq!(id.name, "a");
            assert_eq!(compass_point, None);
        }
        _ => panic!("expected IDPort"),
    }
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize(":a:n".to_string());
    let (port, rest) = parse_port(&tokens).unwrap();
    match port {
        Port::IDPort((id, compass_point)) => {
            assert_eq!(id.name, "a");
            assert_eq!(compass_point.unwrap(), CompassPoint::N);
        }
        _ => panic!("expected IDPort"),
    }
    assert_eq!(rest, vec![] as Vec<String>);

    let tokens = tokenize(":n".to_string());
    let (port, rest) = parse_port(&tokens).unwrap();
    match port {
        Port::CompassPointPort(compass_point) => {
            assert_eq!(compass_point, CompassPoint::N);
        }
        _ => panic!("expected CompassPointPort"),
    }
    assert_eq!(rest, vec![] as Vec<String>);
}
