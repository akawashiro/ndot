# ndot

ndot is a modern DOT language parser and SVG renderer implemented in Rust. This project provides tools for converting graph descriptions written in the DOT language into SVG visualizations, available as a command-line tool, a WebAssembly library, and a web-based editor.

## Project Structure

This repository consists of three main components:

- **ndot**: Core Rust library and CLI tool for parsing DOT language and generating SVG
- **ndot-wasm**: WebAssembly wrapper for the ndot library, enabling usage in web environments
- **ndot-client**: Web-based editor for creating and visualizing DOT graphs

## Components

### ndot

The core library provides functionality to parse DOT language and convert it to SVG. It includes:

- Tokenizer for DOT language
- Abstract Syntax Tree (AST) parser
- Graph construction
- Layout engine
- SVG generation

The CLI tool allows converting DOT files to SVG from the command line.

### ndot-wasm

A WebAssembly wrapper around the ndot library, making it accessible from JavaScript/TypeScript. This component:

- Exposes the core functionality through WebAssembly bindings
- Provides error handling and result types suitable for JavaScript
- Enables usage in web applications

### ndot-client

A React-based web application that provides a user interface for:

- Editing DOT language code
- Visualizing the resulting graph in real-time
- Exporting SVG output

## Installation

### Prerequisites

- Rust toolchain (for ndot and ndot-wasm)
- Node.js and npm (for ndot-client)
- wasm-pack (for building ndot-wasm)

### Building from Source

#### ndot (CLI tool)

```bash
cd ndot
cargo build --release
```

The binary will be available at `target/release/ndot`.

#### ndot-wasm

```bash
cd ndot-wasm
cargo install wasm-pack
wasm-pack build --target web
```

This will generate WebAssembly bindings in the `pkg` directory.

#### ndot-client

```bash
cd ndot-client
npm install
npm run build
```

The built web application will be available in the `dist` directory.

## Usage

### CLI Usage (ndot)

```bash
ndot --input-file input.dot --output-file output.svg
```

### Web Client

To run the web client locally:

```bash
cd ndot-client
npm run dev
```

Then open your browser to the URL shown in the terminal (typically http://localhost:5173).

### API Usage (for developers)

#### Rust API

```rust
use ndot::make_svg_from_dot;

let dot_string = r#"
digraph {
    a -> b;
    b -> c;
    a -> c;
}
"#.to_string();

match make_svg_from_dot(dot_string) {
    Ok(svg) => {
        // Use the SVG string
        println!("Successfully generated SVG");
    },
    Err(e) => {
        eprintln!("Error generating SVG: {}", e);
    }
}
```

#### JavaScript/TypeScript API (via WebAssembly)

```javascript
import { make_svg_from_dot } from 'ndot-wasm';

const dotString = `
digraph {
    a -> b;
    b -> c;
    a -> c;
}`;

const result = make_svg_from_dot(dotString);
if (result.svg) {
    // Use the SVG string
    console.log("Successfully generated SVG");
} else if (result.error) {
    console.error("Error generating SVG:", result.error);
}
```

## Development

### Testing

Run tests for the ndot library:

```bash
cd ndot
cargo test
```

Run tests for the ndot-wasm package:

```bash
cd ndot-wasm
wasm-pack test --headless --firefox
```

### Project Structure

```
ndot/               # Core Rust library and CLI
├── src/            # Source code
│   ├── ast.rs      # Abstract Syntax Tree
│   ├── graph.rs    # Graph data structures
│   ├── layout.rs   # Layout algorithms
│   ├── lib.rs      # Library entry point
│   ├── main.rs     # CLI entry point
│   ├── svg.rs      # SVG generation
│   └── tokenize.rs # DOT language tokenizer
│
ndot-wasm/          # WebAssembly wrapper
├── src/            # Source code
│   ├── lib.rs      # WebAssembly bindings
│   └── utils.rs    # Utility functions
│
ndot-client/        # Web client
├── src/            # Source code
│   ├── App.tsx     # Main application component
│   └── ...         # Other React components
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Author

Akira Kawata
