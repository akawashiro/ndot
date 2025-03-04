use clap::Parser;
use env_logger;
use log::info;
use std::env;
use std::io::Write;

mod ast;
mod graph;
mod ndot;
mod svg;
mod tokenize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct NDotArgs {
    #[arg(short, long)]
    input_file: String,
    #[arg(short, long)]
    output_file: String,
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

    let args = NDotArgs::parse();
    info!("input file: {}", args.input_file);

    let dot = std::fs::read_to_string(&args.input_file).unwrap();
    ndot::make_svg_from_dot(dot);
}
