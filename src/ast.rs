struct ID {
    name: String,
}

fn valid_as_id(token: &String) -> bool {
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
    let tokens = vec!["a".to_string(), "b".to_string()];
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
    let tokens = vec!["a".to_string(), "=".to_string(), "b".to_string()];
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

    let tokens = vec!["a".to_string(), "b".to_string()];
    let result = parse_id_eq_stmt(&tokens);
    assert!(result.is_err());
}
