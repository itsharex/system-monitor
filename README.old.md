# System Monitor

[![Tauri](https://img.shields.io/badge/Tauri-2.2-blue.svg)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.5-green.svg)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![GitHub release](https://img.shields.io/github/release/xinggaoya/system-monitor.svg)](https://github.com/xinggaoya/system-monitor/releases)

A cross-platform desktop system monitoring application built with Tauri 2.2 + Vue 3.

## 🌐 Documentation

**简体中文** | **English**

---

### 📖 中文文档

👉 [README.zh.md](README.zh.md) - 完整的中文项目文档

包含：
- 📋 详细的功能介绍
- 🚀 快速开始指南
- 🏗️ 技术栈说明
- 📁 项目结构解析
- 🔧 配置和部署指南
- 🐛 故障排除
- 🤝 贡献指南

---

### 📖 English Documentation

👉 [README.en.md](README.en.md) - Complete English project documentation

Includes:
- 📋 Detailed feature descriptions
- 🚀 Quick start guide
- 🏗️ Technology stack overview
- 📁 Project structure analysis
- 🔧 Configuration and deployment guide
- 🐛 Troubleshooting
- 🤝 Contributing guidelines

---

## ✨ Key Features

- 🖥️ **Real-time Monitoring**: CPU, Memory, GPU, and Network usage
- 🎯 **Lightweight Design**: Minimal resource usage with floating window
- 🔄 **System Tray**: Complete tray integration with show/hide functionality
- 🎨 **Modern UI**: Transparent floating window with beautiful visual effects
- ⚡ **High Performance**: Rust backend + Vue frontend
- 🌐 **Cross-platform**: Windows, macOS, and Linux support

## 📥 Quick Download

Get the latest version from [GitHub Releases](https://github.com/xinggaoya/system-monitor/releases):

- **Windows**: `system_monitor_1.0.0_x64-setup.exe`
- **macOS**: `system_monitor_1.0.0_x64.dmg`
- **Linux**: `system_monitor_1.0.0_amd64.AppImage`

## 🚀 Quick Start

```bash
# Clone repository
git clone https://github.com/xinggaoya/system-monitor.git
cd system-monitor

# Install dependencies
pnpm install

# Start development
pnpm tauri dev

# Build release
pnpm tauri build
```

## 🏗️ Tech Stack

- **Frontend**: Vue 3.5 + TypeScript + Pinia + Vite
- **Backend**: Tauri 2.2 + Rust
- **Monitoring**: sysinfo 0.33 + nvml-wrapper

## 📊 Screenshot

```
CPU 45% | Memory 62% | GPU -- | Network
↓1.2MB ↑800KB
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

We welcome contributions! Please read our [Contributing Guidelines](CONTRIBUTING.md) for details.

---

<div align="center">

**⭐ If this project helps you, please give it a Star!**

Made with ❤️ by [System Monitor Team](https://github.com/xinggaoya/system-monitor)

</div>