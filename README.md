<div align="center">

# 💰 DeepSeek 余额悬浮窗

> **始终看得见的 DeepSeek API 余额与用量趋势**

基于 **Tauri 2 + Vue 3 + ECharts** 构建的 Windows 桌面悬浮窗。它会定时查询 DeepSeek API 余额，在桌面上持续显示余额状态，并用本地历史曲线记录用量变化。

[![Release](https://img.shields.io/github/v/release/AmireuxGX/deepseek-balance-widget?style=flat-square)](https://github.com/AmireuxGX/deepseek-balance-widget/releases/latest)
[![Version](https://img.shields.io/badge/version-v0.3.0-4d6bfe?style=flat-square)](https://github.com/AmireuxGX/deepseek-balance-widget/releases/tag/v0.3.0)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%2F11-blue.svg?style=flat-square)](https://github.com/AmireuxGX/deepseek-balance-widget/releases/latest)
[![Tauri](https://img.shields.io/badge/Tauri-2-24C8D8?style=flat-square)](https://tauri.app)

</div>

---

## 📸 预览

![DeepSeek 余额悬浮窗 v0.3.0](docs/screenshot-v0.3.0.jpg)

## 🆕 v0.3.0 更新

- **全新毛玻璃界面**：重构主悬浮窗、余额曲线和设置窗口，统一半透明材质、边缘高光、层次阴影与控件状态
- **Windows 材质适配**：Windows 11 的常规窗口优先使用 Mica；不支持时自动回退到 CSS 玻璃效果
- **透明主窗口修复**：主悬浮窗使用透明原生窗口与圆角 CSS 表面，消除原生矩形底板和阴影裁切方框
- **可读性增强**：为玻璃底色设置最低可读强度，强化副文字、状态信息与操作图标在明暗背景下的对比度
- **交互与无障碍改进**：补充键盘焦点、按钮语义、减少动态效果偏好和更清晰的加载状态
- **版本统一**：前端、Rust、Tauri 配置和构建产物统一升级至 `0.3.0`

## ✨ 特性

- 🪟 **毛玻璃圆角悬浮卡片**：置顶显示、不占任务栏，透明原生窗口 + CSS 玻璃材质，兼顾质感、性能与兼容性
- ⏱️ **定时查询余额**：总余额 / 充值 / 赠送 / 可用状态一屏尽览，涨跌提示（↑ ↓）+ 千分位显示 + 相对更新时间
- 📈 **余额使用曲线**：本地自动记录快照，自动识别充值事件，ECharts 折线图 + 充值标记，支持 **24 小时 / 7 天 / 全部** 时间范围切换 —— 看着曲线往下走，花钱如流水
- 🎨 **自定义皮肤**：7 套预设皮肤 + 6 色自定义，设置窗口实时预览
- 🖥️ **系统托盘**：关闭窗口最小化到托盘，后台继续刷新；左键单击托盘图标显示悬浮窗，右键菜单可显示 / 退出
- 🚀 **开机自启**、深浅主题、可调不透明度、自定义刷新间隔
- 🖱️ **交互顺手**：拖拽移动、右下角缩放、双击刷新、右键菜单
- 🔒 **API Key 仅存本机**（`%APPDATA%\com.deepseek.balancewidget\config.json`），源码不硬编码

## ⬇️ 下载安装

前往 **[最新版本下载页](https://github.com/AmireuxGX/deepseek-balance-widget/releases/latest)**：

| 文件 | 说明 |
| --- | --- |
| `DeepSeekBalanceWidget_0.3.0_x64-setup.exe` | NSIS 安装包（推荐，安装到当前用户） |
| `deepseek-balance-widget.exe` | 便携版（免安装，直接运行） |

> 需要 **Windows 10 / 11**（自带 WebView2 Runtime）。

## 🚀 快速开始

1. 下载安装包并运行，首次启动会自动弹出**设置窗口**
2. 填入你的 [DeepSeek API Key](https://platform.deepseek.com/api_keys) 并保存
3. 完成！悬浮窗开始自动定时查询余额

## 🖱️ 使用说明

| 操作 | 功能 |
| --- | --- |
| 左键拖拽 | 移动悬浮窗位置（位置自动记忆） |
| 右下角拖拽 | 调整悬浮窗大小（260×160 ~ 900×600，自动记忆） |
| 双击卡片 | 立即刷新余额 |
| 右键菜单 | 立即刷新 / 查看余额曲线 / 设置 / 退出 |
| ✕ 按钮 | 最小化到托盘，后台继续刷新 |
| 托盘左键 | 显示悬浮窗 |
| 托盘右键 | 显示 / 退出 |

**设置窗口**可修改：API Key、刷新间隔（默认 5 分钟）、不透明度、主题皮肤（含自定义颜色）、开机自启、清空历史。

## 🪟 显示与兼容性

- 主悬浮窗不会对整个原生窗口启用 Mica，避免圆角外出现矩形底板；玻璃质感由透明 WebView 与 CSS 材质共同完成
- 曲线与设置窗口在 Windows 11 上优先使用 Mica，在 Windows 10 或效果不可用时自动使用 CSS 回退
- 不透明度较低时会保留必要的底色强度，避免文字在复杂或纯白壁纸上失去对比度
- 更新后若仍显示旧界面，请先从系统托盘退出旧实例，再启动新版本

## 🛠️ 开发环境

- [Node.js](https://nodejs.org) 18+ / npm
- [Rust](https://rustup.rs)（`stable-msvc` 工具链）
- Microsoft C++ Build Tools（勾选 **Desktop development with C++**）
- WebView2 Runtime（Win10/11 自带）

## 🔨 本地构建

```bash
# 安装前端依赖
npm install

# 开发调试（热更新）
npm run tauri dev

# 打包发布
npm run tauri build
```

打包产物：

```
src-tauri/target/release/deepseek-balance-widget.exe      # 主程序（便携版）
src-tauri/target/release/bundle/nsis/*-setup.exe          # NSIS 安装包
```

## 📁 目录结构

```
deepseek-balance-widget/
├── src/                    # Vue 3 前端
│   ├── views/              # MainCard / ChartView / SettingsView
│   ├── lib/                # invoke 封装、类型与窗口材质适配
│   ├── skins.ts            # 皮肤预设与应用
│   └── styles.css          # 皮肤 CSS 变量与全局样式
├── src-tauri/              # Rust 后端
│   ├── src/                # lib / config / history / api 模块
│   ├── tauri.conf.json     # 窗口 / 打包配置
│   └── capabilities/       # 权限配置
└── package.json
```

## 🔒 安全提示

- API Key 以明文保存在本机配置文件中（`%APPDATA%\com.deepseek.balancewidget\config.json`），**请勿分享该文件**
- 应用仅访问 `https://api.deepseek.com/user/balance` 一个接口，无其他网络行为

## 📄 License

[MIT](LICENSE) © DeepSeekBalanceWidget
