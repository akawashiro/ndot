use crate::graph::{Edge, Graph, Node};

// Constants for SVG generation
const SVG_WIDTH: i32 = 800;
const SVG_HEIGHT: i32 = 600;
const NODE_RADIUS: i32 = 20;
const NODE_SPACING: i32 = 100;
const ARROW_SIZE: i32 = 10;

// Function to generate SVG from a Graph
pub fn generate_svg(graph: &Graph) -> String {
    // Calculate node positions
    let node_positions = calculate_node_positions(graph);

    // Start SVG document
    let mut svg = format!(
        r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
        SVG_WIDTH, SVG_HEIGHT
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
        let start_x = x1 as f64 + nx * NODE_RADIUS as f64;
        let start_y = y1 as f64 + ny * NODE_RADIUS as f64;

        let end_x = x2 as f64 - nx * NODE_RADIUS as f64;
        let end_y = y2 as f64 - ny * NODE_RADIUS as f64;

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
            x, y, NODE_RADIUS
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

// Function to calculate node positions
fn calculate_node_positions(graph: &Graph) -> std::collections::HashMap<Node, (i32, i32)> {
    let mut positions = std::collections::HashMap::new();
    let node_count = graph.nodes.len();

    // Simple layout algorithm: place nodes in a circle
    if node_count > 0 {
        let radius = std::cmp::min(SVG_WIDTH, SVG_HEIGHT) as f64 / 3.0;
        let center_x = SVG_WIDTH as f64 / 2.0;
        let center_y = SVG_HEIGHT as f64 / 2.0;

        for (i, node) in graph.nodes.iter().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (node_count as f64);
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();

            positions.insert(node.clone(), (x as i32, y as i32));
        }
    }

    positions
}

// Function to export the graph as an SVG string
pub fn graph_to_svg(graph: &Graph) -> String {
    generate_svg(graph)
}
