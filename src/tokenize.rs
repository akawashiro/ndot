macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

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
        vec_of_strings![
            "graph", "{", "\n", "a", "--", "b", ";", "\n", "b", "--", "c", ";", "\n", "a", "--",
            "c", ";", "\n", "d", "--", "c", ";", "\n", "e", "--", "c", ";", "\n", "e", "--", "a",
            ";", "\n", "}"
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
    let tokens = tokenize(dot_str.to_string());
    let tokens = remove_comments(tokens);
    assert_eq!(
        tokens,
        vec_of_strings![
            "graph", "{", "a", "--", "b", ";", "b", "--", "c", ";", "a", "--", "c", ";", "d", "--",
            "c", ";", "e", "--", "c", ";", "e", "--", "a", ";", "}"
        ]
    );
}

pub fn tokenize(dot_str: String) -> Vec<String> {
    let tokens = raw_tokenize_dot(dot_str);
    let tokens = remove_comments(tokens);
    tokens
}
