fn raw_tokenize_dot(dot_str: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut in_quote = false;
    let mut last_char = ' ';
    for c in dot_str.chars() {
        match c {
            ' ' | '\t' | '\n' | ';' => {
                if in_quote {
                    token.push(c);
                } else {
                    if !token.is_empty() {
                        tokens.push(token.clone());
                        token.clear();
                    }
                    // We need '\n' to parse C++ style comments
                    if c == ';' || c == '\n' {
                        let t = c.to_string();
                        tokens.push(t);
                    }
                }
            }
            '"' => {
                if last_char != '\\' {
                    in_quote = !in_quote;
                }
                token.push(c);
            }
            _ => {
                token.push(c);
            }
        }
        last_char = c;
    }
    if !token.is_empty() {
        tokens.push(token);
    }
    tokens
}

#[test]
fn test_raw_tokenize_dot() {
    let dot_str = r#"graph {
    a -- b;
    b -- c;
    a -- c;
    d -- c;
    e -- c;
    e -- a;
}"#;
    let tokens = raw_tokenize_dot(dot_str.to_string());
    assert_eq!(
        tokens,
        vec![
            "graph".to_string(),
            "{".to_string(),
            "\n".to_string(),
            "a".to_string(),
            "--".to_string(),
            "b".to_string(),
            ";".to_string(),
            "\n".to_string(),
            "b".to_string(),
            "--".to_string(),
            "c".to_string(),
            ";".to_string(),
            "\n".to_string(),
            "a".to_string(),
            "--".to_string(),
            "c".to_string(),
            ";".to_string(),
            "\n".to_string(),
            "d".to_string(),
            "--".to_string(),
            "c".to_string(),
            ";".to_string(),
            "\n".to_string(),
            "e".to_string(),
            "--".to_string(),
            "c".to_string(),
            ";".to_string(),
            "\n".to_string(),
            "e".to_string(),
            "--".to_string(),
            "a".to_string(),
            ";".to_string(),
            "\n".to_string(),
            "}".to_string(),
        ]
    );
}

// Because newline characters are used to parse C++ style comments, we remove them here.
fn remove_comments(tokens: Vec<String>) -> Vec<String> {
    let mut new_tokens = Vec::new();
    let mut in_cpp_comment = false;
    let mut in_c_comment = false;
    for token in tokens {
        if in_cpp_comment {
            if token == "\n" {
                in_cpp_comment = false;
            }
            continue;
        }
        if in_c_comment {
            if token == "*/" {
                in_c_comment = false;
            }
            continue;
        }
        if token == "//" {
            in_cpp_comment = true;
            continue;
        }
        if token == "/*" {
            in_c_comment = true;
            continue;
        }
        new_tokens.push(token);
    }

    new_tokens = new_tokens
        .into_iter()
        .filter(|t| t != &"\n".to_string())
        .collect();
    new_tokens
}

#[test]
fn test_remove_comments() {
    let dot_str = r#"graph {
    // This is a comment
    a -- b;
    /* This is a comment */
    b -- c;
    a -- c;
    /* // This is a comment */
    d -- c;
    // "This is a comment"
    e -- c;
    // /* This is a comment */
    e -- a;
    /* "This is a comment" */
}"#;
    let tokens = tokenize_dot(dot_str.to_string());
    let tokens = remove_comments(tokens);
    assert_eq!(
        tokens,
        vec![
            "graph".to_string(),
            "{".to_string(),
            "a".to_string(),
            "--".to_string(),
            "b".to_string(),
            ";".to_string(),
            "b".to_string(),
            "--".to_string(),
            "c".to_string(),
            ";".to_string(),
            "a".to_string(),
            "--".to_string(),
            "c".to_string(),
            ";".to_string(),
            "d".to_string(),
            "--".to_string(),
            "c".to_string(),
            ";".to_string(),
            "e".to_string(),
            "--".to_string(),
            "c".to_string(),
            ";".to_string(),
            "e".to_string(),
            "--".to_string(),
            "a".to_string(),
            ";".to_string(),
            "}".to_string(),
        ]
    );
}

pub fn tokenize_dot(dot_str: String) -> Vec<String> {
    let tokens = raw_tokenize_dot(dot_str);
    let tokens = remove_comments(tokens);
    tokens
}
