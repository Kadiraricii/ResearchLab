# Stage 1: Build Frontend
FROM oven/bun:1 AS frontend-builder
WORKDIR /app
COPY package.json bun.lock ./
RUN bun install --frozen-lockfile
COPY . .
RUN bun run build

# Stage 2: Build Rust Backend
FROM rust:1.77-slim AS backend-builder
WORKDIR /app
RUN apt-get update && apt-get install -y libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# Sccache kurulumu (derleme hizlandirma icin)
RUN cargo install sccache
ENV RUSTC_WRAPPER=sccache

COPY src-tauri/Cargo.toml src-tauri/Cargo.lock ./src-tauri/
COPY src-tauri/src ./src-tauri/src
COPY src-tauri/tauri.conf.json ./src-tauri/
COPY src-tauri/build.rs ./src-tauri/
COPY src-tauri/icons ./src-tauri/icons
COPY src-tauri/capabilities ./src-tauri/capabilities

# Copy frontend dist to tauri build context
COPY --from=frontend-builder /app/dist ./dist

# Dummy build for caching dependencies
WORKDIR /app/src-tauri
RUN cargo build --release

# Stage 3: Minimal Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libwebkit2gtk-4.0-37 \
    libgtk-3-0 \
    libayatana-appindicator3-1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=backend-builder /app/src-tauri/target/release/vercel-hack-analysis ./app-binary
COPY configs ./configs

ENV RUST_LOG=info
CMD ["./app-binary"]
