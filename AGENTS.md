# AGENTS.md

## 项目定位

你正在 `autowds-backend` 仓库中工作。这是 **AutoWDS** 平台的后端服务，为网页数据采集提供完整的 API 支撑，包括用户认证、任务管理、模板管理、支付系统、积分系统和后台管理。

这不是一个单纯的 API 服务。仓库同时包含：

- **Rust 后端**：基于 Summer 框架的插件化 Web 服务
- **Frontend (cloud)**：React + TypeScript 用户控制台
- **Backend (admin)**：React + TypeScript 管理后台
- **Site**：Next.js 营销站点
- **WASM 智能抽取**：与 `autowds-browser` 共享的 Rust/WASM 核心

做改动前，先确认自己修改的是哪一层：`Rust API`、`frontend`、`backend` 还是 `site`，不要混淆各层的职责和构建方式。

## 技术栈

### 后端核心
- **语言**：Rust 2021 Edition
- **Web 框架**：Summer (基于 Axum 的插件化框架)
- **ORM**：Sea-ORM (PostgreSQL)
- **缓存**：Redis
- **任务队列**：Apalis (基于 Redis)
- **邮件**：SMTP (summer-mail)
- **支付**：支付宝、微信支付、Paddle
- **API 文档**：OpenAPI + Scalar

### 前端子项目
- **cloud (frontend/)**：React 18 + TypeScript + Ant Design，Vite 构建
- **admin (backend/)**：React 18 + TypeScript + Ant Design，Vite 构建
- **site (site/)**：Next.js + React + TypeScript

### 基础设施
- **数据库**：PostgreSQL 15
- **容器化**：Docker + Docker Compose
- **构建工具**：Cargo (Rust)、npm (前端)、Just (命令快捷方式)

## 常用命令

### 后端开发
```bash
# 启动开发环境（PostgreSQL + Redis）
docker compose up -d

# 运行后端服务
cargo run

# 类型检查
cargo check

# 构建 release
cargo build --release

# 格式化代码
cargo fmt

# Clippy 检查
cargo clippy

# 生成 Sea-ORM 实体（需先安装 sea-orm-cli）
just gen-model
# 或：sea-orm-cli generate entity --with-serde both --output-dir src/model/_entities --enum-extra-derives strum::EnumString
```

### 前端开发
```bash
# cloud 控制台
cd frontend && npm install && npm start

# admin 管理后台
cd backend && npm install && npm start

# site 营销站点
cd site && npm install && npm run dev
```

### 数据库
```bash
# 启动数据库
docker compose up -d postgres redis

# DDL 在 sql/ddl.sql 中，启动时会自动执行
```

## 目录地图

### Rust 后端 (`src/`)

- `src/main.rs`
  应用入口。使用 Summer 的插件系统加载 Web、ORM、Redis、邮件、任务队列、支付等插件。

- `src/router/`
  路由处理模块。使用 `#[get_api]`, `#[post_api]`, `#[patch_api]`, `#[put_api]`, `#[delete_api]` 宏自动注册路由。
  - `user.rs` — 用户注册、登录、积分、签到
  - `task.rs` — 采集任务 CRUD、调度
  - `template.rs` — 采集模板管理
  - `pay.rs` — 支付创建、回调
  - `pay_query.rs` — 支付查询
  - `statistics.rs` — 统计数据
  - `admin.rs` — 管理后台 API
  - `token.rs` — JWT Token 相关

- `src/views/`
  请求/响应数据结构（DTO）。使用 `serde` 序列化，`schemars` 生成 OpenAPI 文档，`validator` 做参数校验。

- `src/model/`
  数据库模型层。
  - `_entities/` — Sea-ORM 自动生成的实体（**优先视为生成文件**）
  - `account_user.rs`, `scraper_task.rs`, `pay_order.rs` 等 — 自定义模型扩展

- `src/service/`
  业务服务层。
  - `credit.rs` — 积分扣减/增加逻辑
  - `pay.rs` — 支付渠道封装（支付宝、微信、Paddle）
  - `user.rs` — 用户相关服务

- `src/utils/`
  工具模块。
  - `jwt.rs` — JWT 签发与验证
  - `mail.rs` — 邮件发送
  - `pay_plugin.rs` — 支付插件初始化
  - `validate_code.rs` — 验证码生成与校验

- `src/config/`
  配置结构体，对应 `config/app.toml` 中的配置段。

- `src/task.rs`
  Apalis 任务队列配置。使用 Redis 作为存储后端。

### 前端子项目

- `frontend/`
  用户控制台（cloud 界面）。Vite + React + Ant Design。

- `backend/`
  管理后台（admin 界面）。Vite + React + Ant Design。

- `site/`
  营销站点。Next.js。

### 配置与部署

- `config/app.toml` / `config/app-prod.toml`
  应用配置文件，支持环境变量替换（如 `${DATABASE_URL:默认值}`）。

- `sql/ddl.sql`
  数据库初始化脚本，包含表、序列、枚举类型定义。

- `Dockerfile`
  多阶段构建：分别构建三个前端子项目，再构建 Rust 后端，最终合并到 runner 镜像。

- `compose.yaml`
  本地开发环境：PostgreSQL 15 + Redis 7。

## 关键架构事实

### 1. 插件化架构

后端基于 **Summer 框架**，采用插件化设计。所有核心能力都以插件形式注册：

```rust
App::new()
    .add_plugin(WebPlugin)
    .add_plugin(SeaOrmPlugin)
    .add_plugin(MailPlugin)
    .add_plugin(RedisPlugin)
    .add_plugin(JobPlugin)
    .add_plugin(ApalisPlugin)
    .add_plugin(PayPlugin)
    .add_router(router::router())
    .add_worker(task::add_storage)
    .run()
```

新增插件或修改插件初始化顺序时，要检查组件依赖关系。

### 2. 自动路由与 OpenAPI

路由使用宏自动注册，无需手动 `Router::new().route(...)`：

```rust
/// # 接口标题
/// @tag 分组名
#[post_api("/path")]
async fn handler(...) -> Result<Json<Resp>> { ... }
```

- 所有 API 路径以 `/api` 为前缀（在 `router/mod.rs` 中统一 nest）
- 文档注释 `/// # 标题` 会生成 OpenAPI 文档
- `@tag` 用于 API 分组

### 3. 数据库与模型生成

- 使用 **Sea-ORM** 进行数据库操作
- 实体文件在 `src/model/_entities/` 中，通过 `sea-orm-cli generate entity` 自动生成
- 自定义逻辑放在 `src/model/` 下的同名文件中（如 `scraper_task.rs`）
- DDL 源文件在 `sql/ddl.sql`，修改表结构后需同步更新 DDL 并重新生成实体

**注意**：`src/model/_entities/` 是生成目录，手工修改会被覆盖。

### 4. 依赖注入

使用 Summer 的 `Component<T>` 提取器进行依赖注入：

```rust
async fn handler(
    Component(db): Component<DbConn>,
    Component(redis): Component<Redis>,
    Component(mailer): Component<Mailer>,
) -> Result<...> { ... }
```

常用组件：`DbConn`, `Redis`, `Mailer`, `Config<T>`, `TaskPublisher`。

### 5. 任务队列 (Apalis)

基于 Redis 的异步任务队列，用于：
- 支付状态检查
- 其他后台异步处理

任务定义在 `src/task.rs`，消费者通过 `add_worker` 注册。

### 6. 支付系统

支持三种支付渠道：
- **支付宝**：沙箱/生产环境切换通过配置控制
- **微信支付**：配置全为环境变量
- **Paddle**：海外支付

支付配置在 `config/app.toml` 的 `[pay]` 段，敏感信息通过环境变量注入。

### 7. 错误处理

- 使用 `anyhow::Result` 和 `anyhow::Context` 进行错误传播
- 使用 `KnownWebError` 返回标准 HTTP 错误（如 `bad_request`, `forbidden`）
- 使用 `problemdetails` 生成符合 RFC 7807 的错误响应
- 中间件 `problem_middleware` 会将错误统一转换为 Problem Details 格式

### 8. 多前端服务

生产环境由 Rust 后端统一 serving 静态资源：
- `/cloud/` → frontend (用户控制台)
- `/admin/` → backend (管理后台)
- `/` → site (营销站点)

开发环境分别 serve，通过 `router/mod.rs` 中的 `Env::Dev` 判断。

## 修改约束

### 1. 先理解再改

不要看到一个模块就直接局部重写。先判断它是否参与：
- 认证/授权链路（JWT、Claims）
- 数据库事务（`TransactionTrait`）
- 积分扣减（`CreditService`）
- 支付回调（幂等性、状态机）
- 任务调度（Apalis、Redis）

### 2. 优先做小而完整的改动

除非需求明确要求重构，否则避免大范围重写。优先保持：
- 现有路由注册方式（宏）
- 现有错误处理模式（`KnownWebError` + `anyhow`）
- 现有依赖注入风格（`Component<T>`）
- 现有类型结构（views / model）

### 3. 数据库变更要谨慎

修改数据库相关代码时：
- 更新 `sql/ddl.sql`
- 重新生成 `_entities`（`just gen-model`）
- 检查枚举类型在 Rust 中的映射
- 考虑旧数据兼容性

### 4. 不要随意修改生成文件

以下内容应视为生成物或构建产物，除非任务明确要求，否则不要手工维护：
- `src/model/_entities/`
- `target/`
- `frontend/build/`, `backend/dist/`, `site/out/`

### 5. 支付相关改动需额外谨慎

任何涉及支付的改动都需要：
- 检查幂等性
- 确认回调验证逻辑
- 测试沙箱环境
- 不要修改生产环境配置

## 测试与验证

默认验证顺序：

1. `cargo check` — 类型和编译检查
2. `cargo clippy` — Lint 检查
3. `cargo fmt` — 格式化检查
4. 涉及数据库改动时，确认 `sql/ddl.sql` 可正常执行
5. 涉及支付改动时，说明验证方式（沙箱/模拟）

## 编码建议

- 保持 Rust 类型完整，不要为了省事大量使用 `anyhow::Error` 抹除类型
- API 参数校验使用 `validator`，校验消息使用中文
- 数据库操作使用 `.context("...")` 添加上下文
- 异步代码中注意 `?` 和 `.await` 的顺序
- 敏感配置（密码、密钥）必须走环境变量，不要硬编码
- 新增 API 时同步更新 OpenAPI 文档注释

## 完成定义

一个合格的改动应尽量满足：

- 改动范围与需求直接相关
- 涉及数据库时，DDL 和实体文件已同步
- 必要的编译检查已通过 (`cargo check` / `cargo clippy`)
- 如果有未验证项（如支付回调），明确说明原因和风险
- 配置变更已说明需要设置的环境变量

## 给代理的工作方式要求

在这个仓库里工作时，请默认遵守以下流程：

1. 先读相关 `views/` 和 `router/` 了解接口契约，再开始改代码
2. 如果需求涉及数据库变更，先检查 `sql/ddl.sql` 和 `src/model/_entities/`
3. 如果需求涉及支付，确认涉及的支付渠道和配置项
4. 如果需求跨 Rust 后端和前端，确认两端接口对齐
5. 完成后优先做最小必要验证，不要只停留在"看起来没问题"

如果你必须在速度和风险之间取舍，优先选择可验证、可回退、对现有用户数据更安全的方案。
