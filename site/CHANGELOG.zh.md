# 更新日志

本项目的所有重要变更都将记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/) 规范。

## [未发布]

### 新增

- 数据导出到Google Sheet
- 数据导出到数据库

## [0.4.0] - 2026-04-25

### 新增
- 采集任务页面添加停止采集按钮
- 新增工具箱，提供XPath选择器和正则工具

### 变更
- 采用更现代化的用户界面设计
- 优化多个ShadowDOM间样式无法注入的问题
- 优化单元素XPath生成算法
- 优化翻页智能识别算法

## [0.2.0] - 2024-12-01

### 新增
- 全新的可视化规则编辑器，支持拖拽式操作
- 增加了 AI 智能识别功能，自动识别页面结构

### 变更
- 重构了核心采集引擎，大幅提升稳定性和性能
- 更新了用户界面，提供更直观的操作体验

### 移除
- 移除了旧版本的兼容模式，专注于现代浏览器支持

## [0.1.1] - 2024-11-15

### 修复
- 修复了 Chrome 扩展在最新版本中的兼容性问题
- 解决了定时任务在某些情况下无法正常执行的 bug
- 修复了导出 Excel 文件时中文乱码的问题

### 变更
- 优化了内存使用，减少了长时间运行时的内存占用

## [0.1.0] - 2024-10-30

### 新增
- 新增定时任务系统，支持自动化数据采集
- 增强的错误报告和调试工具

### 变更
- 改进了分页检测算法
- 更新扩展清单至 v3 版本，提升安全性

### 修复
- 修复了 AJAX 内容检测问题
- 解决了某些网站的 Cookie 处理问题

## [0.0.2] - 2024-10-15

### 修复
- 数据导出功能的关键安全修复
- 修复了长时间采集会话中的内存泄漏问题
- 解决了与基于 React 的网站的兼容性问题

### 变更
- 重新设计设置面板，提升可用性
- 改进元素选择器准确性
- 改进了错误消息，提供更好的用户指导

## [0.0.1] - 2024-09-10

### 新增
- 浏览器扩展首次发布
- 基础网页采集功能
- 支持 Chrome、Firefox 和 Edge 浏览器
- CSV 和 JSON 导出格式
- 可视化元素选择工具

[未发布]: https://github.com/octopus-crawler/octopus-crawler/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/octopus-crawler/octopus-crawler/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/octopus-crawler/octopus-crawler/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/octopus-crawler/octopus-crawler/compare/v0.0.2...v0.1.0
[0.0.2]: https://github.com/octopus-crawler/octopus-crawler/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/octopus-crawler/octopus-crawler/releases/tag/v0.0.1