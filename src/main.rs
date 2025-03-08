use clap::Parser;
use env_logger;
use log::info;
use std::env;
use std::io::Write;

mod ast;
mod graph;
mod layout;
mod ndot;
mod svg;
mod tokenize;

#[cfg(test)]
mod layout_test;

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
    let svg_content = match ndot::make_svg_from_dot(dot) {
        Ok(svg) => svg,
        Err(e) => {
            eprintln!("Error generating SVG: {}", e);
            std::process::exit(1);
        }
    };

    // Write the SVG to the output file
    match std::fs::write(&args.output_file, svg_content) {
        Ok(_) => info!("SVG saved to: {}", args.output_file),
        Err(e) => {
            eprintln!("Error writing SVG file: {}", e);
            std::process::exit(1);
        }
    }
}
