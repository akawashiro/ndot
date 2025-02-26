use clap::Parser;
use env_logger;
use log::info;
use std::env;
use std::io::Write;

mod ast;
mod tokenize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct NNDotArgs {
    #[arg(short, long)]
    input_file: String,
    #[arg(short, long)]
    output_file: String,
}

fn parse_dot(dot_str: String) {
    info!("parsing dot string");
    let tokens = tokenize::tokenize_dot(dot_str);
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
