# AutoWDS 官网

一个现代化的浏览器自动化插件介绍网站，参考 minirpa.net 和 webscraper.io 的设计风格。

## 技术栈

- **框架**: Next.js 13 (App Router)
- **语言**: TypeScript
- **样式**: Tailwind CSS
- **部署**: Vercel (推荐)

## 功能特性

### 主页 (/)
- 精美的 Hero 区域，突出产品价值
- 9 大核心功能展示
- 6 个主要使用场景
- 三步上手指南
- 灵活的定价方案（免费版、专业版、企业版）
- 响应式设计，完美支持移动端

### 教程页面 (/learn)
- 快速开始指南
- 6 大教程分类（基础、进阶、实战、自动化、API、最佳实践）
- 视频教程展示
- 常见问题解答
- 社区支持入口

### 云服务（整合在主页价格部分）
- 本地 vs 云端功能对比表
- 云服务核心优势（高速稳定、24/7运行、大规模采集、全球代理）
- 6 大云服务功能展示
- 云服务定价方案（入门版、专业版、企业版）
- 直接跳转到云服务平台 (https://autowds.dtiku.cn/)

## 开始使用

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run dev
```

访问 [http://localhost:3000](http://localhost:3000) 查看网站。

### 构建生产版本

```bash
npm run build
npm start
```

### 代码检查

```bash
npm run lint
```

## Changelog 管理

项目使用标准的 [Keep a Changelog](https://keepachangelog.com/) 格式管理版本更新记录。

### 可用命令

```bash
# 验证 changelog 格式
npm run changelog:validate

# 测试解析器功能
npm run changelog:test

# 测试集成效果
npm run changelog:test-integration

# 运行所有 changelog 测试
npm run changelog:test-all
```

### Changelog 文件

- `CHANGELOG.md` - 英文版更新日志
- `CHANGELOG.zh.md` - 中文版更新日志

### 添加新版本

1. 编辑对应的 changelog 文件
2. 遵循 Keep a Changelog 格式
3. 运行验证命令：`npm run changelog:validate`
4. 测试解析效果：`npm run changelog:test-all`

详细说明请参考 [CHANGELOG_README.md](./CHANGELOG_README.md) 和 [NPM_SCRIPTS.md](./NPM_SCRIPTS.md)。

## 项目结构

```
octopus-site/
├── src/
│   ├── app/
│   │   ├── [lang]/
│   │   │   ├── page.tsx            # 多语言主页
│   │   │   ├── learn/
│   │   │   │   └── page.tsx        # 教程页面
│   │   │   ├── changelog/
│   │   │   │   └── page.tsx        # 更新日志页面
│   │   │   └── docs/
│   │   │       └── [slug]/
│   │   │           └── page.tsx    # 文档页面
│   │   ├── layout.tsx              # 全局布局
│   │   └── globals.css             # 全局样式
│   ├── components/
│   │   ├── Header.tsx              # 页头组件
│   │   ├── Footer.tsx              # 页脚组件
│   │   └── ...                     # 其他组件
│   ├── utils/
│   │   └── changelog-parser.ts     # Changelog 解析器
│   └── i18n/                       # 国际化配置
├── scripts/                        # 工具脚本
│   ├── validate-changelog.js       # Changelog 格式验证
│   ├── test-changelog-parser.js    # 解析器测试
│   └── test-changelog-integration.js # 集成测试
├── public/                         # 静态资源
├── docs/                           # 文档
├── CHANGELOG.md                    # 英文更新日志
├── CHANGELOG.zh.md                 # 中文更新日志
└── README.md
```

## 设计特点

### 视觉风格
- 紫色到蓝色的渐变主题色
- 现代化的卡片式布局
- 流畅的动画和过渡效果
- 清晰的视觉层次

### 用户体验
- 响应式设计，适配各种设备
- 直观的导航结构
- 清晰的行动号召按钮
- 丰富的交互反馈

### 内容组织
- 功能特性突出展示
- 使用场景具体说明
- 定价方案清晰对比
- 教程分类完整全面

## 自定义配置

### 修改主题色

在 `tailwind.config.js` 中可以自定义颜色：

```js
theme: {
  extend: {
    colors: {
      primary: {...},
      secondary: {...}
    }
  }
}
```

### 修改内容

所有页面内容都在对应的 `.tsx` 文件中，可以直接修改文案、图标和数据。

## 部署

### Vercel (推荐)

1. 将代码推送到 GitHub
2. 在 Vercel 中导入项目
3. 自动部署完成

### 其他平台

```bash
npm run build
```

将 `.next` 目录部署到任何支持 Node.js 的平台。

## 待完善功能

- [ ] 添加真实的插件演示截图/视频
- [ ] 集成真实的下载链接
- [ ] 添加用户评价/案例展示
- [ ] 集成博客系统
- [x] 添加多语言支持（中文/英文）
- [x] 添加更新日志页面
- [x] 组件化重构（Header/Footer）
- [ ] 集成分析工具（Google Analytics）
- [ ] 添加在线客服系统
- [ ] SEO 优化
- [ ] 移动端菜单优化
- [ ] 添加搜索功能

## 开发工具

### NPM 脚本

| 命令 | 功能 |
|------|------|
| `npm run dev` | 启动开发服务器 |
| `npm run build` | 构建生产版本 |
| `npm run start` | 启动生产服务器 |
| `npm run lint` | 代码检查和格式化 |
| `npm run changelog:validate` | 验证 changelog 格式 |
| `npm run changelog:test` | 测试解析器功能 |
| `npm run changelog:test-integration` | 测试集成效果 |
| `npm run changelog:test-all` | 运行所有 changelog 测试 |

详细说明请参考 [NPM_SCRIPTS.md](./NPM_SCRIPTS.md)。

### 相关文档

- [CHANGELOG_README.md](./CHANGELOG_README.md) - Changelog 使用说明
- [COMPONENT_REFACTOR.md](./COMPONENT_REFACTOR.md) - 组件重构说明
- [CHANGELOG_FIX.md](./CHANGELOG_FIX.md) - 中文支持修复说明
- [NPM_SCRIPTS.md](./NPM_SCRIPTS.md) - NPM 脚本命令说明

## License

MIT
