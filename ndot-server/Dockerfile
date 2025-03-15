# Multi-stage build Dockerfile for ndot-server
# This Dockerfile builds all components from source and creates a minimal final image

# Stage 1: Rust Builder - Build ndot and ndot-wasm
FROM rust:1.85 as rust-builder

# Install wasm-pack for WebAssembly compilation
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Set working directory
WORKDIR /build

# Copy Rust code
COPY ndot/ ./ndot/
COPY ndot-wasm/ ./ndot-wasm/

# Build ndot-wasm
RUN cd ndot-wasm && wasm-pack build --target web

# Stage 2: Node.js Builder - Build ndot-client
FROM node:20 as node-builder

# Set working directory
WORKDIR /build

# Copy ndot-client code
COPY ndot-client/ ./ndot-client/

# Copy built wasm package from rust-builder
COPY --from=rust-builder /build/ndot-wasm/pkg/ ./ndot-wasm/pkg/

# Install dependencies and build
WORKDIR /build/ndot-client
RUN npm ci
RUN npm run build

# Stage 3: Python Builder - Set up ndot-server dependencies
FROM python:3.13 as python-builder

# Install uv
RUN pip install --no-cache-dir uv

# Set working directory
WORKDIR /build

# Copy Python project files
COPY ndot-server/pyproject.toml ndot-server/uv.lock ./

# Install dependencies using uv
RUN uv pip install --system .

# Copy server code
COPY ndot-server/main.py ./

# Create static directory
RUN mkdir -p static

# Copy built frontend from node-builder
COPY --from=node-builder /build/ndot-client/dist/ ./static/

# Stage 4: Final - Create minimal production image
FROM python:3.13-slim

# Create non-root user
RUN useradd -m appuser

# Set working directory
WORKDIR /app

# Copy Python packages and application code
COPY --from=python-builder /usr/local/lib/python3.13/site-packages /usr/local/lib/python3.13/site-packages
COPY --from=python-builder /build/main.py /app/
COPY --from=python-builder /build/static /app/static/

# Create entrypoint script for handling configuration
RUN echo '#!/bin/sh\n\
PORT=${PORT:-30080}\n\
SAVE_DIR=${SAVE_DIR:-/app/saved_data}\n\
\n\
# Create save directory if it doesn'\''t exist\n\
mkdir -p "$SAVE_DIR"\n\
\n\
# Run the server with the specified options\n\
exec python main.py --port "$PORT" --save-dir "$SAVE_DIR" "$@"\n\
' > /app/entrypoint.sh && chmod +x /app/entrypoint.sh

# Create default save directory
RUN mkdir -p /app/saved_data && chown -R appuser:appuser /app

# Switch to non-root user
USER appuser

# Expose default port (can be overridden)
EXPOSE 30080

# Set environment variables
ENV PYTHONUNBUFFERED=1
ENV PORT=30080
ENV SAVE_DIR=/app/saved_data

# Use the entrypoint script
ENTRYPOINT ["/app/entrypoint.sh"]
