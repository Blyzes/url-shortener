FROM rust:latest as builder

WORKDIR /app

# 安装 sqlx-cli
RUN cargo install sqlx-cli --no-default-features --features mysql

# 缓存依赖
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# 复制项目源码
COPY . .

# 构建 release
RUN cargo build --release

# 运行阶段
FROM debian:bullseye-slim

# 安装证书和运行时依赖
RUN apt-get update && apt-get install -y ca-certificates libssl-dev libpq-dev curl && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 拷贝构建好的二进制程序和资源
COPY --from=builder /app/target/release/url_shortener /app/url_shortener
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/templates /app/templates
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

ENV RUST_LOG=info

CMD ["bash", "-c", "sleep 10 && sqlx migrate run && ./url_shortener"]
