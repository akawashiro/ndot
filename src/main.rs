use clap::Parser;
use env_logger;
use log::info;
use std::env;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct NNDotArgs {
    #[arg(short, long)]
    input_file: String,
    #[arg(short, long)]
    output_file: String,
}

fn tokenize_dot(dot_str: String) -> Vec<String> {
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
fn test_tokenize_dot() {
    let dot_str = r#"graph {
    a -- b;
    b -- c;
    a -- c;
    d -- c;
    e -- c;
    e -- a;
}"#;
    let tokens = tokenize_dot(dot_str.to_string());
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

fn parse_dot(dot_str: String) {
    info!("parsing dot string");
    let tokens = tokenize_dot(dot_str);
    for token in tokens {
        info!("token: {}", token);
    }
}

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let ts = buf.timestamp();
            writeln!(
                buf,
                "[{} {} {} {}:{}] {}",
                ts,
                record.level(),
                record.target(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args(),
            )
        })
        .init();

    let args = NNDotArgs::parse();
    info!("input file: {}", args.input_file);

    let dot_str = std::fs::read_to_string(&args.input_file).unwrap();
    parse_dot(dot_str);
}
