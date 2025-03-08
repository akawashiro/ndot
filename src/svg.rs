use crate::graph::Graph;
use crate::layout;
use log::info;
use std::time::Instant;

// Constants for SVG generation
const ARROW_SIZE: i32 = 10;

// Function to generate SVG from a Graph with calculated node positions
pub fn generate_svg(graph: &Graph) -> String {
    info!(
        "generate_svg: Starting SVG generation for graph with {} nodes and {} edges",
        graph.nodes.len(),
        graph.edges.len()
    );
    let start_time = Instant::now();

    // Calculate node positions based on graph type
    info!("generate_svg: Checking if graph is a DAG");
    let is_dag_result = layout::is_dag(graph);
    info!("generate_svg: is_dag result: {}", is_dag_result);

    let node_positions = if is_dag_result {
        // Use Sugiyama algorithm for DAGs
        info!("generate_svg: Using Sugiyama algorithm for DAG");
        layout::calculate_sugiyama_positions(graph)
    } else {
        // Use circular layout for non-DAGs
        info!("generate_svg: Using circular layout for non-DAG");
        layout::calculate_circular_positions(graph)
    };

    info!(
        "generate_svg: Node positions calculated in {:?}",
        start_time.elapsed()
    );

    // Start SVG document
    let mut svg = format!(
        r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
        layout::SVG_WIDTH,
        layout::SVG_HEIGHT
    );

    // Add SVG definitions for arrowhead marker
    svg.push_str(
        r##"
  <defs>
    <marker id="arrowhead" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto">
      <polygon points="0 0 10 3.5 0 7" fill="#000" />
    </marker>
  </defs>
"##,
    );

    // Draw edges
    for edge in &graph.edges {
        let source_pos = node_positions.get(&edge.source).unwrap();
        let target_pos = node_positions.get(&edge.target).unwrap();

        // Calculate edge path
        let (x1, y1) = *source_pos;
        let (x2, y2) = *target_pos;

        // Calculate direction vector
        let dx = x2 - x1;
        let dy = y2 - y1;
        let length = ((dx * dx + dy * dy) as f64).sqrt();

        // Normalize direction vector
        let nx = dx as f64 / length;
        let ny = dy as f64 / length;

        // Adjust start and end points to be on the node boundaries
        let start_x = x1 as f64 + nx * layout::NODE_RADIUS as f64;
        let start_y = y1 as f64 + ny * layout::NODE_RADIUS as f64;

        let end_x = x2 as f64 - nx * layout::NODE_RADIUS as f64;
        let end_y = y2 as f64 - ny * layout::NODE_RADIUS as f64;

        // Draw the edge
        if edge.is_directed {
            svg.push_str(&format!(
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2" marker-end="url(#arrowhead)" />"#,
                start_x, start_y, end_x, end_y
            ));
        } else {
            svg.push_str(&format!(
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="2" />"#,
                start_x, start_y, end_x, end_y
            ));
        }
    }

    // Draw nodes
    for (node, (x, y)) in &node_positions {
        // Draw node circle
        svg.push_str(&format!(
            r#"  <circle cx="{}" cy="{}" r="{}" fill="white" stroke="black" stroke-width="2" />"#,
            x,
            y,
            layout::NODE_RADIUS
        ));

        // Draw node label
        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" text-anchor="middle" dominant-baseline="middle" font-family="Arial" font-size="14">{}</text>"#,
            x, y, node.id.name
        ));
    }

    // Close SVG document
    svg.push_str("\n</svg>");

    svg
}

// Function to export the graph as an SVG string
pub fn graph_to_svg(graph: &Graph) -> String {
    info!(
        "graph_to_svg: Starting SVG generation for graph with {} nodes and {} edges",
        graph.nodes.len(),
        graph.edges.len()
    );
    let start_time = Instant::now();

    let result = generate_svg(graph);

    info!(
        "graph_to_svg: SVG generation completed in {:?}, SVG size: {} bytes",
        start_time.elapsed(),
        result.len()
    );

    result
}
