<div align="center">

# 💰 DeepSeek 余额悬浮窗

> **实时感受花钱如流水 —— DeepSeek API 余额 & 用量悬浮小插件**

基于 **Tauri 2 + Vue 3 + ECharts** 构建的半透明置顶悬浮窗，定时查询 DeepSeek API 余额，用实时曲线记录每一分钱的流逝。

[![Release](https://img.shields.io/github/v/release/你的用户名/deepseek-balance-widget?style=flat-square)](https://github.com/你的用户名/deepseek-balance-widget/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%2F11-blue.svg?style=flat-square)](https://github.com/你的用户名/deepseek-balance-widget/releases/latest)
[![Tauri](https://img.shields.io/badge/Tauri-2-24C8D8?style=flat-square)](https://tauri.app)

</div>

---

## 📸 预览

![DeepSeek 余额悬浮窗](docs/screenshot.png)

## ✨ 特性

- 🪟 **半透明圆角悬浮卡片**：置顶显示、不占任务栏，CSS 圆角 + WebView2 硬件渲染，边缘天然抗锯齿无毛边
- ⏱️ **定时查询余额**：总余额 / 充值 / 赠送 / 可用状态一屏尽览，涨跌提示（↑ ↓）+ 千分位显示 + 相对更新时间
- 📈 **余额使用曲线**：本地自动记录快照，自动识别充值事件，ECharts 折线图 + 充值标记，支持 **24 小时 / 7 天 / 全部** 时间范围切换 —— 看着曲线往下走，花钱如流水
- 🎨 **自定义皮肤**：7 套预设皮肤 + 6 色自定义，设置窗口实时预览
- 🖥️ **系统托盘**：关闭窗口最小化到托盘，后台继续刷新；左键单击托盘图标显示悬浮窗，右键菜单可显示 / 退出
- 🚀 **开机自启**、深浅主题、可调不透明度、自定义刷新间隔
- 🖱️ **交互顺手**：拖拽移动、右下角缩放、双击刷新、右键菜单
- 🔒 **API Key 仅存本机**（`%APPDATA%\com.deepseek.balancewidget\config.json`），源码不硬编码

## ⬇️ 下载安装

前往 **[Releases](https://github.com/你的用户名/deepseek-balance-widget/releases/latest)** 页面下载：

| 文件 | 说明 |
| --- | --- |
| `DeepSeekBalanceWidget_<版本>_x64-setup.exe` | NSIS 安装包（推荐，安装到当前用户） |
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
│   ├── lib/api.ts          # invoke 封装与类型
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
