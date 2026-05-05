# AutoWDS 双语网站

这是一个支持中英双语的 AutoWDS 介绍网站，基于 Next.js 13+ App Router 构建。

## 功能特性

✅ **中英双语支持**
- 自动路由：`/zh` (中文) 和 `/en` (英文)
- 语言切换器组件
- 默认语言：中文

✅ **文档系统**
- Markdown 文档支持
- 双语文档：`docs/filename.zh.md` 和 `docs/filename.en.md`
- 自动渲染和样式化

✅ **响应式设计**
- 移动端友好
- Tailwind CSS 样式

## 项目结构

```
site/
├── src/
│   ├── app/
│   │   ├── [lang]/              # 国际化路由
│   │   │   ├── layout.tsx       # 语言布局
│   │   │   ├── page.tsx         # 首页
│   │   │   ├── learn/           # 学习中心
│   │   │   │   └── page.tsx
│   │   │   └── docs/            # 文档页面
│   │   │       └── [slug]/
│   │   │           └── page.tsx
│   │   ├── layout.tsx           # 根布局
│   │   └── page.tsx             # 根页面（重定向）
│   ├── components/
│   │   ├── LanguageSwitcher.tsx # 语言切换器
│   │   └── MarkdownRenderer.tsx # Markdown 渲染器
│   ├── i18n/
│   │   ├── config.ts            # 国际化配置
│   │   ├── utils.ts             # 翻译工具函数
│   │   └── locales/
│   │       ├── zh.json          # 中文翻译
│   │       └── en.json          # 英文翻译
│   └── middleware.ts            # 路由中间件
└── docs/                        # 文档文件
    ├── installation.zh.md       # 中文安装指南
    ├── installation.en.md       # 英文安装指南
    ├── getting-started.zh.md    # 中文快速开始
    └── getting-started.en.md    # 英文快速开始
```

## 如何添加新的翻译

### 1. 添加页面翻译

编辑 `src/i18n/locales/zh.json` 和 `src/i18n/locales/en.json`：

```json
{
  "newSection": {
    "title": "标题",
    "description": "描述"
  }
}
```

### 2. 在页面中使用翻译

```tsx
import { useTranslations } from '@/i18n/utils';

export default function Page({ params }: { params: { lang: Locale } }) {
  const t = useTranslations(params.lang);
  
  return (
    <div>
      <h1>{t('newSection.title')}</h1>
      <p>{t('newSection.description')}</p>
    </div>
  );
}
```

## 如何添加新文档

### 1. 创建双语 Markdown 文件

在 `docs/` 目录下创建：
- `your-doc.zh.md` (中文版本)
- `your-doc.en.md` (英文版本)

### 2. 文档会自动生成路由

- 中文：`/zh/docs/your-doc`
- 英文：`/en/docs/your-doc`

### 3. 在学习中心添加链接

编辑 `src/app/[lang]/learn/page.tsx` 添加文档链接。

## 开发

```bash
# 安装依赖
npm install

# 开发模式
npm run dev

# 构建
npm run build

# 启动生产服务器
npm start
```

## 访问路径

- 首页（自动重定向到中文）: `/`
- 中文首页: `/zh`
- 英文首页: `/en`
- 中文学习中心: `/zh/learn`
- 英文学习中心: `/en/learn`
- 中文文档: `/zh/docs/installation`
- 英文文档: `/en/docs/installation`

## 语言检测和切换

### 自动语言检测

当用户访问根路径 `/` 时，系统会自动检测浏览器语言：
- 如果浏览器语言是中文（zh-*），自动重定向到 `/zh`
- 其他语言自动重定向到 `/en`

检测逻辑在两个地方：
1. **客户端**：`src/app/page.tsx` - 使用 `navigator.language` 检测
2. **中间件**：`src/middleware.ts` - 使用 `Accept-Language` 请求头检测

### 手动切换语言

页面右上角有语言切换器，点击可以在中英文之间切换。切换时会保持当前页面路径。

## 注意事项

1. 所有新页面都应该放在 `src/app/[lang]/` 目录下
2. 使用 `params.lang` 获取当前语言
3. 使用 `useTranslations(params.lang)` 获取翻译函数
4. 文档文件命名格式：`filename.{lang}.md`
5. 如果某个语言的文档不存在，会尝试使用通用文档 `filename.md`
