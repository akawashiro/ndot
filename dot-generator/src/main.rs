use clap::Parser;
use rand::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// A tool to generate random Directed Acyclic Graphs (DAGs) in DOT format
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of nodes in the graph
    #[arg(short, long, default_value_t = 10)]
    nodes: usize,

    /// Number of edges in the graph
    #[arg(short, long, default_value_t = 15)]
    edges: usize,

    /// Output file path
    #[arg(short, long, default_value = "output.dot")]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Calculate maximum possible edges for a DAG with n nodes
    let max_edges = args.nodes * (args.nodes - 1) / 2;
    let edges = std::cmp::min(args.edges, max_edges);

    if args.edges > max_edges {
        println!(
            "Warning: Requested {} edges, but maximum possible for {} nodes is {}. Using {} edges.",
            args.edges, args.nodes, max_edges, edges
        );
    }

    let dot_content = generate_dag(args.nodes, edges);
    write_dot_file(&dot_content, &args.output)?;

    println!(
        "Generated DAG with {} nodes and {} edges. Saved to {:?}",
        args.nodes, edges, args.output
    );

    Ok(())
}

/// Generate a random DAG with the specified number of nodes and edges
fn generate_dag(nodes: usize, edges: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut dot_content = String::from("digraph {\n");

    // To ensure we have a DAG, we'll only create edges from lower-numbered nodes to higher-numbered nodes
    // This ensures a topological ordering and prevents cycles

    // Create a pool of all possible edges in a DAG
    let mut possible_edges = Vec::new();
    for i in 0..nodes {
        for j in (i + 1)..nodes {
            possible_edges.push((i, j));
        }
    }

    // Shuffle the possible edges
    possible_edges.shuffle(&mut rng);

    // Take the first 'edges' number of edges
    let selected_edges = possible_edges.iter().take(edges);

    // Add the edges to the DOT content
    for (src, dst) in selected_edges {
        dot_content.push_str(&format!("    {} -> {};\n", src, dst));
    }

    dot_content.push_str("}\n");
    dot_content
}

/// Write the DOT content to a file
fn write_dot_file(content: &str, path: &PathBuf) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
