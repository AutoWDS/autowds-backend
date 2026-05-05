# 后台管理系统

基于 React + TypeScript + Ant Design + Vite 构建的后台管理系统。

## 功能特性

- 🔐 用户登录认证
- 👥 用户管理
- 📋 任务管理（创建、编辑、删除、启动、停止）
- 📄 模板管理
- 📊 数据统计
- 🎨 响应式布局

## 技术栈

- React 18
- TypeScript
- Ant Design 5
- React Router 6
- Axios
- Zustand (状态管理)
- Vite

## 开发

### 安装依赖

```bash
npm install
```

### 启动开发服务器

```bash
npm run dev
```

访问 http://localhost:3001

### 构建生产版本

```bash
npm run build
```

## 项目结构

```
backend/
├── src/
│   ├── api/              # API 接口
│   │   ├── auth.ts
│   │   ├── user.ts
│   │   ├── task.ts
│   │   ├── template.ts
│   │   └── statistics.ts
│   ├── components/       # 公共组件
│   │   └── Layout/
│   ├── pages/           # 页面组件
│   │   ├── Login/
│   │   ├── Dashboard/
│   │   ├── User/
│   │   ├── Task/
│   │   ├── Template/
│   │   └── Statistics/
│   ├── store/           # 状态管理
│   │   └── auth.ts
│   ├── utils/           # 工具函数
│   │   └── request.ts
│   ├── App.tsx
│   ├── main.tsx
│   └── index.css
├── index.html
├── package.json
├── tsconfig.json
└── vite.config.ts
```

## API 接口

后端 API 地址：http://localhost:8080/api

主要接口：
- POST /api/token/login - 用户登录
- GET /api/user/list - 获取用户列表
- GET /api/task/list - 获取任务列表
- GET /api/template/list - 获取模板列表
- GET /api/statistics/overview - 获取统计数据

## 注意事项

- 确保后端服务已启动（端口 8080）
- 开发环境下会自动代理 /api 请求到后端服务
- 登录状态使用 localStorage 持久化存储
