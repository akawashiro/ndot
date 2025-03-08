use crate::graph::{Graph, Node};
use log::{debug, info};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

// Constants for layout
pub const SVG_WIDTH: i32 = 800;
pub const SVG_HEIGHT: i32 = 600;
pub const NODE_RADIUS: i32 = 20;
pub const NODE_SPACING_X: i32 = 100;
pub const NODE_SPACING_Y: i32 = 80;

// Function to check if a graph is a DAG (Directed Acyclic Graph)
pub fn is_dag(graph: &Graph) -> bool {
    info!("is_dag: Checking if graph is a DAG");
    let start_time = Instant::now();

    // If any edge is not directed, it's not a DAG
    if graph.edges.iter().any(|e| !e.is_directed) {
        info!(
            "is_dag: Found undirected edge, not a DAG. Took {:?}",
            start_time.elapsed()
        );
        return false;
    }

    // Check for cycles using DFS
    let mut visited = HashSet::new();
    let mut path = HashSet::new();

    // Create adjacency list for faster traversal
    info!("is_dag: Creating adjacency list");
    let mut adj_list: HashMap<&Node, Vec<&Node>> = HashMap::new();
    for edge in &graph.edges {
        adj_list
            .entry(&edge.source)
            .or_insert_with(Vec::new)
            .push(&edge.target);
    }
    info!(
        "is_dag: Adjacency list created with {} entries",
        adj_list.len()
    );

    // DFS function to detect cycles
    fn has_cycle(
        node: &Node,
        adj_list: &HashMap<&Node, Vec<&Node>>,
        visited: &mut HashSet<String>,
        path: &mut HashSet<String>,
    ) -> bool {
        // Mark current node as visited and add to recursion path
        debug!("has_cycle: Visiting node {}", node.id.name);
        visited.insert(node.id.name.clone());
        path.insert(node.id.name.clone());

        // Check all adjacent nodes
        if let Some(neighbors) = adj_list.get(node) {
            debug!(
                "has_cycle: Node {} has {} neighbors",
                node.id.name,
                neighbors.len()
            );
            for &neighbor in neighbors {
                // If neighbor not visited, check if it leads to a cycle
                if !visited.contains(&neighbor.id.name) {
                    debug!(
                        "has_cycle: Checking unvisited neighbor {}",
                        neighbor.id.name
                    );
                    if has_cycle(neighbor, adj_list, visited, path) {
                        debug!("has_cycle: Found cycle through {}", neighbor.id.name);
                        return true;
                    }
                }
                // If neighbor is in current recursion path, we found a cycle
                else if path.contains(&neighbor.id.name) {
                    debug!(
                        "has_cycle: Found cycle - {} is already in path",
                        neighbor.id.name
                    );
                    return true;
                }
            }
        }

        // Remove node from current path
        debug!("has_cycle: Removing {} from path", node.id.name);
        path.remove(&node.id.name);
        false
    }

    // Check each unvisited node
    info!("is_dag: Starting cycle detection");
    for node in &graph.nodes {
        if !visited.contains(&node.id.name) {
            info!("is_dag: Checking unvisited node {}", node.id.name);
            if has_cycle(node, &adj_list, &mut visited, &mut path) {
                info!(
                    "is_dag: Found cycle, not a DAG. Took {:?}",
                    start_time.elapsed()
                );
                return false;
            }
        }
    }

    info!(
        "is_dag: No cycles found, is a DAG. Took {:?}",
        start_time.elapsed()
    );
    true
}

// Function to calculate node positions using the Sugiyama algorithm for DAGs
pub fn calculate_sugiyama_positions(graph: &Graph) -> HashMap<Node, (i32, i32)> {
    info!("calculate_sugiyama_positions: Starting Sugiyama layout");
    let start_time = Instant::now();

    let mut positions = HashMap::new();

    // Step 1: Assign layers to nodes
    info!("calculate_sugiyama_positions: Assigning layers to nodes");
    let layer_start = Instant::now();
    let layers = assign_layers(graph);
    info!(
        "calculate_sugiyama_positions: Layers assigned in {:?}, {} layers created",
        layer_start.elapsed(),
        layers.len()
    );

    // Step 2: Assign x-coordinates to minimize crossings
    info!("calculate_sugiyama_positions: Minimizing crossings");
    let crossing_start = Instant::now();
    let x_positions = minimize_crossings(graph, &layers);
    info!(
        "calculate_sugiyama_positions: Crossings minimized in {:?}",
        crossing_start.elapsed()
    );

    // Step 3: Calculate final positions
    for (layer_idx, layer) in layers.iter().enumerate() {
        let layer_y = 50 + layer_idx as i32 * NODE_SPACING_Y;

        for node in layer {
            let node_x = if let Some(&x_pos) = x_positions.get(node) {
                50 + x_pos * NODE_SPACING_X
            } else {
                // Fallback if node doesn't have an x-position
                50 + (layer.iter().position(|n| n == node).unwrap_or(0) as i32) * NODE_SPACING_X
            };

            positions.insert(node.clone(), (node_x, layer_y));
        }
    }

    // Center the layout
    center_layout(&mut positions);

    positions
}

// Function to assign layers to nodes (topological sorting)
fn assign_layers(graph: &Graph) -> Vec<Vec<Node>> {
    info!("assign_layers: Starting layer assignment");
    let start_time = Instant::now();

    let mut layers: Vec<Vec<Node>> = Vec::new();
    let mut node_to_layer: HashMap<String, usize> = HashMap::new();

    // Calculate in-degree for each node
    info!("assign_layers: Calculating in-degree for each node");
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for node in &graph.nodes {
        in_degree.insert(node.id.name.clone(), 0);
    }

    for edge in &graph.edges {
        *in_degree.entry(edge.target.id.name.clone()).or_insert(0) += 1;
    }

    info!(
        "assign_layers: In-degree calculated for {} nodes",
        in_degree.len()
    );

    // Queue for nodes with no incoming edges
    let mut queue: VecDeque<Node> = VecDeque::new();

    // Add nodes with no incoming edges to the first layer
    info!("assign_layers: Adding nodes with no incoming edges to first layer");
    for node in &graph.nodes {
        if in_degree.get(&node.id.name).unwrap_or(&0) == &0 {
            info!(
                "assign_layers: Node {} has no incoming edges, adding to first layer",
                node.id.name
            );
            queue.push_back(node.clone());
        }
    }

    info!("assign_layers: {} nodes added to first layer", queue.len());

    // Process nodes in topological order
    let mut current_layer = 0;
    layers.push(Vec::new());

    info!("assign_layers: Processing nodes in topological order");
    let mut processed_count = 0;

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();
        processed_count += 1;

        info!(
            "assign_layers: Processing node {} in layer {}",
            node.id.name, current_layer
        );

        // Add node to current layer
        layers[current_layer].push(node.clone());
        node_to_layer.insert(node.id.name.clone(), current_layer);

        // Find outgoing edges
        let outgoing = graph
            .edges
            .iter()
            .filter(|e| e.source.id.name == node.id.name)
            .map(|e| e.target.clone())
            .collect::<Vec<_>>();

        info!(
            "assign_layers: Node {} has {} outgoing edges",
            node.id.name,
            outgoing.len()
        );

        // Update in-degree and add to queue if in-degree becomes 0
        for target in outgoing {
            let in_deg = in_degree.get_mut(&target.id.name).unwrap();
            *in_deg -= 1;

            info!(
                "assign_layers: Reduced in-degree of {} to {}",
                target.id.name, in_deg
            );

            if *in_deg == 0 {
                info!(
                    "assign_layers: Adding {} to queue (in-degree is now 0)",
                    target.id.name
                );
                queue.push_back(target);
            }
        }

        // If queue is empty but we haven't processed all nodes,
        // we might have a cycle (shouldn't happen for DAGs)
        if queue.is_empty() && node_to_layer.len() < graph.nodes.len() {
            info!("assign_layers: Queue empty but not all nodes processed. Starting new layer.");
            // Start a new layer
            current_layer += 1;
            layers.push(Vec::new());

            // Find nodes not yet assigned to a layer
            for node in &graph.nodes {
                if !node_to_layer.contains_key(&node.id.name) {
                    info!(
                        "assign_layers: Adding unprocessed node {} to queue",
                        node.id.name
                    );
                    queue.push_back(node.clone());
                    break;
                }
            }
        }
    }

    info!(
        "assign_layers: Processed {} nodes in {} layers",
        processed_count,
        layers.len()
    );

    // Remove empty layers
    layers.retain(|layer| !layer.is_empty());
    info!(
        "assign_layers: After removing empty layers, {} layers remain",
        layers.len()
    );

    // Optimize layer assignment to minimize edge lengths
    info!("assign_layers: Optimizing layer assignment");
    let optimize_start = Instant::now();
    optimize_layers(graph, &mut layers);
    info!(
        "assign_layers: Layer optimization completed in {:?}",
        optimize_start.elapsed()
    );

    info!(
        "assign_layers: Layer assignment completed in {:?}",
        start_time.elapsed()
    );
    layers
}

// Function to optimize layer assignment to minimize edge lengths
fn optimize_layers(graph: &Graph, layers: &mut Vec<Vec<Node>>) {
    info!("optimize_layers: Starting layer optimization");
    let start_time = Instant::now();

    // Create a map from node name to layer index
    let mut node_to_layer: HashMap<String, usize> = HashMap::new();
    for (i, layer) in layers.iter().enumerate() {
        for node in layer {
            node_to_layer.insert(node.id.name.clone(), i);
        }
    }

    info!(
        "optimize_layers: Created node-to-layer map with {} entries",
        node_to_layer.len()
    );

    // Try to move nodes to better layers
    let mut changed = true;
    let mut iteration = 0;
    while changed {
        iteration += 1;
        info!("optimize_layers: Starting iteration {}", iteration);
        let iter_start = Instant::now();
        changed = false;

        for layer_idx in 0..layers.len() {
            info!(
                "optimize_layers: Processing layer {}/{}",
                layer_idx + 1,
                layers.len()
            );
            let mut i = 0;
            while i < layers[layer_idx].len() {
                let node = &layers[layer_idx][i];
                debug!(
                    "optimize_layers: Processing node {} in layer {}",
                    node.id.name, layer_idx
                );

                // Calculate median layer of neighbors
                let mut incoming: Vec<usize> = Vec::new();
                let mut outgoing: Vec<usize> = Vec::new();

                for edge in &graph.edges {
                    if edge.target.id.name == node.id.name {
                        if let Some(&src_layer) = node_to_layer.get(&edge.source.id.name) {
                            incoming.push(src_layer);
                        }
                    } else if edge.source.id.name == node.id.name {
                        if let Some(&tgt_layer) = node_to_layer.get(&edge.target.id.name) {
                            outgoing.push(tgt_layer);
                        }
                    }
                }

                debug!(
                    "optimize_layers: Node {} has {} incoming and {} outgoing connections",
                    node.id.name,
                    incoming.len(),
                    outgoing.len()
                );

                // Calculate best layer
                let mut best_layer = layer_idx;

                if !incoming.is_empty() && !outgoing.is_empty() {
                    // If node has both incoming and outgoing edges,
                    // try to place it at the median position
                    let mut all_neighbors = incoming.clone();
                    all_neighbors.extend(outgoing.clone());
                    all_neighbors.sort();

                    let median = all_neighbors[all_neighbors.len() / 2];
                    if median != layer_idx
                        && (median > layer_idx && incoming.iter().all(|&l| l < median)
                            || median < layer_idx && outgoing.iter().all(|&l| l > median))
                    {
                        best_layer = median;
                        debug!(
                            "optimize_layers: Median layer {} is better for node {}",
                            median, node.id.name
                        );
                    }
                } else if !incoming.is_empty() {
                    // If node only has incoming edges, place it as low as possible
                    let max_incoming = *incoming.iter().max().unwrap_or(&0);
                    if max_incoming + 1 != layer_idx {
                        best_layer = max_incoming + 1;
                        debug!(
                            "optimize_layers: Layer {} is better for node {} (incoming edges only)",
                            best_layer, node.id.name
                        );
                    }
                } else if !outgoing.is_empty() {
                    // If node only has outgoing edges, place it as high as possible
                    if let Some(&min_outgoing) = outgoing.iter().min() {
                        // Ensure we don't overflow when calculating the best layer
                        if min_outgoing > 0 && min_outgoing != layer_idx + 1 {
                            best_layer = if min_outgoing > 1 {
                                min_outgoing - 1
                            } else {
                                0
                            };
                            debug!("optimize_layers: Layer {} is better for node {} (outgoing edges only)", 
                                  best_layer, node.id.name);
                        }
                    }
                }

                // Move node if better layer found
                if best_layer != layer_idx {
                    info!(
                        "optimize_layers: Moving node {} from layer {} to {}",
                        node.id.name, layer_idx, best_layer
                    );
                    let node_clone = node.clone();

                    // Ensure the target layer exists
                    while layers.len() <= best_layer {
                        layers.push(Vec::new());
                    }

                    // Move node to new layer
                    layers[best_layer].push(node_clone.clone());
                    layers[layer_idx].remove(i);
                    node_to_layer.insert(node_clone.id.name.clone(), best_layer);

                    changed = true;
                } else {
                    i += 1;
                }
            }
        }

        info!(
            "optimize_layers: Iteration {} completed in {:?}, changed: {}",
            iteration,
            iter_start.elapsed(),
            changed
        );

        // Remove empty layers
        let before_len = layers.len();
        layers.retain(|layer| !layer.is_empty());
        info!(
            "optimize_layers: Removed {} empty layers",
            before_len - layers.len()
        );

        // Safety check to prevent infinite loops
        if iteration > 100 {
            info!("optimize_layers: Reached maximum iterations (100), breaking loop");
            break;
        }
    }

    info!(
        "optimize_layers: Layer optimization completed in {:?} after {} iterations",
        start_time.elapsed(),
        iteration
    );
}

// Function to minimize edge crossings
fn minimize_crossings(graph: &Graph, layers: &Vec<Vec<Node>>) -> HashMap<Node, i32> {
    info!("minimize_crossings: Starting crossing minimization");
    let start_time = Instant::now();

    let mut x_positions = HashMap::new();

    // Initialize positions with simple ordering
    for (layer_idx, layer) in layers.iter().enumerate() {
        for (pos, node) in layer.iter().enumerate() {
            x_positions.insert(node.clone(), pos as i32);
        }
    }

    info!(
        "minimize_crossings: Initialized positions for {} nodes",
        x_positions.len()
    );

    // Create adjacency lists
    info!("minimize_crossings: Creating adjacency lists");
    let mut outgoing: HashMap<String, Vec<Node>> = HashMap::new();
    let mut incoming: HashMap<String, Vec<Node>> = HashMap::new();

    for edge in &graph.edges {
        outgoing
            .entry(edge.source.id.name.clone())
            .or_insert_with(Vec::new)
            .push(edge.target.clone());

        incoming
            .entry(edge.target.id.name.clone())
            .or_insert_with(Vec::new)
            .push(edge.source.clone());
    }

    info!(
        "minimize_crossings: Created adjacency lists with {} outgoing and {} incoming entries",
        outgoing.len(),
        incoming.len()
    );

    // Perform several iterations of crossing minimization
    for _ in 0..3 {
        // Top-down pass
        for layer_idx in 1..layers.len() {
            let current_layer = &layers[layer_idx];
            let _prev_layer = &layers[layer_idx - 1];

            // Calculate barycenter for each node in current layer
            let mut barycenters: Vec<(Node, f64)> = Vec::new();

            for node in current_layer {
                let mut sum = 0.0;
                let mut count = 0;

                if let Some(sources) = incoming.get(&node.id.name) {
                    for source in sources {
                        if let Some(&pos) = x_positions.get(source) {
                            sum += pos as f64;
                            count += 1;
                        }
                    }
                }

                let barycenter = if count > 0 { sum / count as f64 } else { 0.0 };
                barycenters.push((node.clone(), barycenter));
            }

            // Sort nodes by barycenter
            barycenters.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            // Update positions
            for (i, (node, _)) in barycenters.iter().enumerate() {
                x_positions.insert(node.clone(), i as i32);
            }
        }

        // Bottom-up pass
        for layer_idx in (0..layers.len() - 1).rev() {
            let current_layer = &layers[layer_idx];
            let _next_layer = &layers[layer_idx + 1];

            // Calculate barycenter for each node in current layer
            let mut barycenters: Vec<(Node, f64)> = Vec::new();

            for node in current_layer {
                let mut sum = 0.0;
                let mut count = 0;

                if let Some(targets) = outgoing.get(&node.id.name) {
                    for target in targets {
                        if let Some(&pos) = x_positions.get(target) {
                            sum += pos as f64;
                            count += 1;
                        }
                    }
                }

                let barycenter = if count > 0 { sum / count as f64 } else { 0.0 };
                barycenters.push((node.clone(), barycenter));
            }

            // Sort nodes by barycenter
            barycenters.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            // Update positions
            for (i, (node, _)) in barycenters.iter().enumerate() {
                x_positions.insert(node.clone(), i as i32);
            }
        }
    }

    x_positions
}

// Function to center the layout
pub fn center_layout(positions: &mut HashMap<Node, (i32, i32)>) {
    info!(
        "center_layout: Starting layout centering for {} nodes",
        positions.len()
    );
    let start_time = Instant::now();

    if positions.is_empty() {
        info!("center_layout: No positions to center, returning");
        return;
    }

    // Find bounds
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for &(x, y) in positions.values() {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    // Calculate offsets to center
    let width = max_x - min_x;
    let height = max_y - min_y;
    let offset_x = (SVG_WIDTH - width) / 2 - min_x;
    let offset_y = (SVG_HEIGHT - height) / 2 - min_y;

    info!(
        "center_layout: Bounds calculated - width: {}, height: {}, offsets: ({}, {})",
        width, height, offset_x, offset_y
    );

    // Apply offsets
    for (_, pos) in positions.iter_mut() {
        pos.0 += offset_x;
        pos.1 += offset_y;
    }

    info!(
        "center_layout: Layout centered in {:?}",
        start_time.elapsed()
    );
}

// Function to calculate node positions using circular layout (for non-DAGs)
pub fn calculate_circular_positions(graph: &Graph) -> HashMap<Node, (i32, i32)> {
    info!(
        "calculate_circular_positions: Starting circular layout for {} nodes",
        graph.nodes.len()
    );
    let start_time = Instant::now();

    let mut positions = HashMap::new();
    let node_count = graph.nodes.len();

    // Simple layout algorithm: place nodes in a circle
    if node_count > 0 {
        let radius = std::cmp::min(SVG_WIDTH, SVG_HEIGHT) as f64 / 3.0;
        let center_x = SVG_WIDTH as f64 / 2.0;
        let center_y = SVG_HEIGHT as f64 / 2.0;

        info!(
            "calculate_circular_positions: Using radius {}, center ({}, {})",
            radius, center_x, center_y
        );

        for (i, node) in graph.nodes.iter().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (node_count as f64);
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();

            debug!("calculate_circular_positions: Placing node {} at angle {:.2} rad, position ({:.2}, {:.2})", 
                  node.id.name, angle, x, y);
            positions.insert(node.clone(), (x as i32, y as i32));
        }
    }

    info!(
        "calculate_circular_positions: Circular layout completed in {:?}",
        start_time.elapsed()
    );
    positions
}
