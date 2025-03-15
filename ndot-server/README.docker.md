# Docker Support for ndot-server

This document explains how to build and run the ndot-server using Docker.

## Overview

The included Dockerfile provides a multi-stage build that:

1. Builds the ndot Rust library
2. Compiles ndot-wasm to WebAssembly
3. Builds the ndot-client frontend
4. Sets up the ndot-server with all dependencies
5. Creates a minimal final image

## Building the Docker Image

From the root directory of the project (where the `ndot`, `ndot-wasm`, `ndot-client`, and `ndot-server` directories are located), run:

```bash
docker build -t ndot-server -f ndot-server/Dockerfile .
```

## Running the Container

### Basic Usage

Run with default settings (port 30080, internal save directory):

```bash
docker run -p 30080:30080 ndot-server
```

### Configuration Options

The container can be configured in several ways:

#### 1. Environment Variables

Set the port and save directory using environment variables:

```bash
docker run -p 8080:8080 -e PORT=8080 -e SAVE_DIR=/data ndot-server
```

#### 2. Command-line Arguments

Pass arguments directly to the server (these override environment variables):

```bash
docker run -p 8888:8888 ndot-server --port 8888 --save-dir /custom/path
```

#### 3. Persistent Storage

Mount a volume for persistent storage of saved DOT files:

```bash
docker run -p 30080:30080 -v /host/path/to/data:/app/saved_data ndot-server
```

Or specify a custom save directory with a volume:

```bash
docker run -p 30080:30080 -e SAVE_DIR=/data -v /host/path/to/data:/data ndot-server
```


## Notes

- The server runs as a non-root user inside the container for security
- The default port is 30080
- The default save directory is `/app/saved_data` inside the container
- All components are built from source during the Docker build process
