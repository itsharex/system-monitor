# System Monitor 系统监控

[![Tauri](https://img.shields.io/badge/Tauri-2.2-blue.svg)](https://tauri.app/)
[![Vue](https://img.shields.io/badge/Vue-3.5-green.svg)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![GitHub release](https://img.shields.io/github/release/xinggaoya/system-monitor.svg)](https://github.com/xinggaoya/system-monitor/releases)

一个基于 Tauri 2.2 + Vue 3 构建的跨平台桌面系统监控应用，提供实时的 CPU、内存、GPU 和网络监控功能。

## ✨ 特性

- 🖥️ **实时系统监控**: CPU、内存、GPU 和网络使用情况
- 🎯 **轻量级设计**: 最小化资源占用，悬浮窗显示
- 🔄 **系统托盘集成**: 完整的托盘菜单功能，支持显示/隐藏和退出
- 🎨 **现代化界面**: 透明悬浮窗，美观的视觉效果
- ⚡ **高性能**: Rust 后端 + Vue 前端，响应迅速
- 🌐 **跨平台**: 支持 Windows、macOS 和 Linux
- 📊 **详细监控**: 实时网络流量监控，支持上下载速度显示
- 🎮 **GPU 监控**: 支持 NVIDIA GPU 监控（需要 NVML）

## 📸 截图

应用界面示例：
```
CPU 45% | 内存 62% | GPU -- | 网络
↓1.2MB ↑800KB
```

## 🚀 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- pnpm (推荐) 或 npm

### 安装依赖

```bash
# 克隆仓库
git clone https://github.com/xinggaoya/system-monitor.git
cd system-monitor

# 安装前端依赖
pnpm install

# 安装 Tauri CLI (如果尚未安装)
cargo install tauri-cli
```

### 开发模式

```bash
pnpm tauri dev
```

### 构建发布版本

```bash
pnpm tauri build
```

构建产物将生成在 `src-tauri/target/release/bundle/` 目录中。

## 📥 下载

从 [GitHub Releases](https://github.com/xinggaoya/system-monitor/releases) 下载最新的预编译版本：

- **Windows**: `system_monitor_1.0.0_x64-setup.exe`
- **macOS**: `system_monitor_1.0.0_x64.dmg`
- **Linux**: `system_monitor_1.0.0_amd64.AppImage`

## 🏗️ 技术栈

**前端:**
- **Vue 3.5** - 现代化前端框架，组合式 API
- **TypeScript** - 类型安全的 JavaScript
- **Pinia** - 轻量级状态管理
- **Vite** - 快速的前端构建工具

**后端:**
- **Tauri 2.2** - 跨平台桌面应用框架
- **Rust** - 系统级编程语言，内存安全
- **sysinfo 0.33** - 系统信息获取库
- **nvml-wrapper** - NVIDIA GPU 监控库

## 📁 项目结构

```
system-monitor/
├── README.md              # 项目导航（本文件）
├── README.zh.md           # 中文文档
├── README.en.md           # 英文文档
├── LICENSE                # MIT 许可证
├── CONTRIBUTING.md        # 贡献指南
├── src/                   # 前端源码
│   ├── components/        # Vue 组件
│   ├── composables/       # 组合式函数
│   ├── stores/           # Pinia 状态管理
│   └── assets/           # 静态资源
├── src-tauri/             # Rust 后端
│   ├── src/              # Rust 源码
│   │   ├── lib.rs        # 主要应用逻辑
│   │   ├── models.rs     # 数据模型
│   │   ├── monitor.rs    # 系统监控实现
│   │   └── gpu_monitor.rs # GPU 监控实现
│   ├── icons/            # 应用图标
│   └── tauri.conf.json   # Tauri 配置
├── .github/              # GitHub Actions
│   └── workflows/
│       └── ci.yml        # CI/CD 配置
└── package.json          # 项目依赖配置
```

## 🔧 配置说明

### 应用配置

应用支持以下配置选项：

- **刷新间隔**: 数据更新频率（默认 1000ms）
- **显示选项**: 选择要监控的系统指标
- **外观设置**: 窗口透明度和位置

### 开发配置

主要配置文件：
- `tauri.conf.json`: Tauri 应用配置
- `package.json`: 前端依赖和脚本
- `tsconfig.json`: TypeScript 配置
- `vite.config.ts`: Vite 构建配置

## 🎮 GPU 监控说明

### NVIDIA GPU 支持

应用支持 NVIDIA GPU 监控，需要：

1. **安装 NVIDIA 驱动**: 确保系统安装了最新的 NVIDIA 显卡驱动
2. **NVML 库**: 应用使用 nvml-wrapper 进行 GPU 监控
3. **权限**: 应用需要适当的系统权限来访问 GPU 信息

### 监控指标

- **GPU 使用率**: 实时 GPU 计算使用百分比
- **显存使用**: GPU 显存使用情况
- **温度**: GPU 温度监控（如果支持）
- **时钟频率**: GPU 核心时钟频率

## 🌐 网络监控

### 监控指标

- **下载速度**: 实时网络下载速度
- **上传速度**: 实时网络上传速度
- **网络接口**: 自动检测活动网络接口
- **流量统计**: 累计网络流量统计

### 显示格式

```
↓下载速度 ↑上传速度
示例: ↓1.2MB ↑800KB
```

## 🎨 界面特性

### 悬浮窗设计

- **透明背景**: 半透明背景，不遮挡桌面内容
- **始终置顶**: 可选择是否始终显示在最前面
- **可拖动**: 支持鼠标拖动到任意位置
- **最小化设计**: 占用最小的屏幕空间

### 系统托盘

- **托盘图标**: 系统托盘中显示应用图标
- **右键菜单**:
  - 显示/隐藏主窗口
  - 退出应用
- **左键点击**: 切换窗口显示/隐藏状态

## 🐛 故障排除

### 常见问题

**GPU 监控不可用:**
- 确保安装了 NVIDIA 驱动程序
- 检查 NVML 库是否正确安装
- 重启应用程序

**托盘图标不显示:**
- 检查系统托盘设置
- 重启应用程序
- 确认应用有适当的系统权限

**构建失败:**
- 确保所有依赖已正确安装
- 检查 Rust 和 Node.js 版本
- 清理缓存：`pnpm store prune` 和 `cargo clean`

**应用无法启动:**
- 检查系统日志获取详细错误信息
- 确认有足够的系统权限
- 尝试以管理员权限运行（Windows）

### 调试模式

启用调试模式获取详细日志：

```bash
# Windows
set RUST_LOG=debug && pnpm tauri dev

# macOS/Linux
RUST_LOG=debug pnpm tauri dev
```

## 🔒 安全性

### 隐私保护

- **本地处理**: 所有系统监控数据在本地处理，不上传到外部服务器
- **最小权限**: 应用只请求必要的系统权限
- **开源代码**: 所有源代码开源，可审查安全性

### 系统权限

应用请求以下系统权限：
- 系统信息访问（CPU、内存、GPU、网络）
- 文件系统访问（用于配置存储）
- 网络访问（用于更新检查，可选）

## 🚀 性能优化

### 资源占用

- **内存使用**: 通常小于 50MB
- **CPU 占用**: 空闲时小于 1%
- **启动时间**: 快速启动，通常小于 3 秒

### 优化建议

1. **调整刷新频率**: 根据需要调整数据更新间隔
2. **选择性监控**: 只启用需要的监控功能
3. **后台运行**: 最小化时自动降低更新频率

## 🤝 贡献

我们欢迎所有形式的贡献！请遵循以下步骤：

### 贡献类型

- 🐛 **Bug 报告**: 发现并报告问题
- 💡 **功能建议**: 提出新功能想法
- 📝 **文档改进**: 完善项目文档
- 🔧 **代码贡献**: 提交代码修复或新功能

### 开发流程

1. **Fork 仓库**: 点击 GitHub 页面右上角的 Fork 按钮
2. **创建分支**: `git checkout -b feature/AmazingFeature`
3. **提交更改**: `git commit -m '添加某个神奇功能'`
4. **推送分支**: `git push origin feature/AmazingFeature`
5. **创建 PR**: 在 GitHub 上创建 Pull Request

### 代码规范

- **Rust 代码**: 使用 `cargo fmt` 格式化，`cargo clippy` 检查
- **TypeScript 代码**: 遵循 ESLint 和 Prettier 规范
- **提交信息**: 使用清晰的提交信息格式
- **测试**: 确保新功能有适当的测试覆盖

## 📊 路线图

### v1.1.0 (计划中)

- [ ] 历史数据记录和图表显示
- [ ] 自定义监控指标
- [ ] 主题和外观自定义
- [ ] 插件系统支持

### v1.2.0 (计划中)

- [ ] 多显示器支持
- [ ] 远程监控功能
- [ ] 移动端配套应用
- [ ] 云端数据同步

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

感谢以下开源项目和贡献者：

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - 系统信息库

## 📞 联系方式

- **GitHub Issues**: [提交问题](https://github.com/xinggaoya/system-monitor/issues)
- **GitHub Discussions**: [参与讨论](https://github.com/xinggaoya/system-monitor/discussions)
- **Email**: [项目维护者邮箱]

---

<div align="center">

**⭐ 如果这个项目对你有帮助，请给个 Star！**

Made with ❤️ by [System Monitor Team](https://github.com/xinggaoya/system-monitor)

</div>