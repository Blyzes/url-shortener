# ---------- 构建阶段 ----------
FROM rust:1.85-slim as builder

WORKDIR /app

# 安装 sqlx-cli（用于运行阶段的数据库迁移）
RUN cargo install sqlx-cli --no-default-features --features mysql

# 开启 SQLX 离线模式，避免构建阶段访问数据库
ENV SQLX_OFFLINE=true

# 复制依赖文件并缓存构建依赖
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# 复制完整源码和 .sqlx（用于 SQL 编译验证）
COPY . .
COPY .sqlx .sqlx

# 构建项目
RUN cargo build --release

# ---------- 运行阶段 ----------
FROM debian:bookworm-slim

# 安装系统运行所需依赖（包括 TLS/SSL）
RUN apt-get update && apt-get install -y \
  ca-certificates \
  libssl-dev \
  libpq-dev \
  curl \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 拷贝构建好的可执行程序和相关资源
COPY --from=builder /app/target/release/url_shortener /app/url_shortener
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/templates /app/templates
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# 设置日志级别（可选）
ENV RUST_LOG=info

# 容器启动命令
CMD ["bash", "-c", "sleep 10 && sqlx migrate run && ./url_shortener"]
