
# Rust 短链接服务

这是一个使用 Rust + Axum 开发的简单高效的短链接服务，支持短链接生成、重定向和访问统计功能，数据持久化使用 MySQL，缓存使用 Redis，并通过 Docker 进行部署和管理。

## 功能特点

- 支持将任意合法 URL 缩短为短链接
- 支持短链接跳转到原始链接
- 支持查看访问统计（JSON 和 HTML 两种格式）
- 使用 MySQL 持久化存储，Redis 缓存提升性能
- 支持一键 Docker 部署

---

## 快速开始（Docker 版）

### 1. 克隆代码仓库

```bash
git clone https://github.com/Blyzes/url-shortener.git
cd url-shortener
````

### 2. 构建并启动服务

```bash
docker-compose up --build
```

服务默认会运行在：

```
http://localhost:3000
```

---

## 接口使用示例

### 1. 生成短链接

```bash
curl -X POST http://localhost:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://example.com"}'
```

**返回示例：**

```json
{
  "short_url": "http://localhost:3000/abc123"
}
```

---

### 2. 短链接跳转

浏览器访问：

```
http://localhost:3000/abc123
```

或者使用 curl：

```bash
curl -v http://localhost:3000/abc123
```

---

### 3. 查看访问统计（JSON）

```bash
curl http://localhost:3000/stats/abc123
```

### 4. 查看访问统计（HTML 页面）

浏览器访问：

```
http://localhost:3000/stats/html/abc123
```

---

## 环境变量示例（`.env` 文件）

```
DATABASE_URL=mysql://root:123456@mysql:3306/url-shortener
REDIS_URL=redis://redis:6379
```

---

## 数据库迁移

如果需要手动执行数据库迁移：

```bash
docker exec -it url_shortener bash
sqlx migrate run
```

---
