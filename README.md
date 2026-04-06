# 💣 扫雷 (Minesweeper)

> 使用 Rust 和 Bevy 游戏引擎开发的经典扫雷游戏

[![Rust](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Bevy](https://img.shields.io/badge/Bevy-0.18.1-blue.svg)](https://bevyengine.org/)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

## 📖 简介

这是一个基于 ECS（Entity-Component-System）架构模式开发的扫雷游戏。项目采用 Rust 语言和 Bevy 游戏引擎，实现了完整的扫雷游戏功能，包括多种难度级别、首次点击安全保护、连通区域自动展开等经典特性。

## ✨ 特性

- 🎯 **三种难度级别**
  - 初级：9×9 网格，10 个地雷
  - 中级：16×16 网格，40 个地雷
  - 专家：16×30 网格，99 个地雷

- 🛡️ **首次点击安全** - 第一次点击的位置及其周围保证没有地雷

- 🌊 **连通区域自动展开** - 点击空白区域时自动展开所有相连的安全区域

- ⏱️ **实时计时** - 显示游戏用时

- 🚩 **旗帜标记** - 右键标记可疑位置

- 📊 **HUD 信息面板** - 实时显示难度、地雷数、旗帜数、剩余雷数、游戏时间

## 🚀 快速开始

### 环境要求

- **Rust**: 1.80+ (Edition 2024)
- **Cargo**: 最新版本

### 安装 Rust

如果你还没有安装 Rust，可以使用以下命令安装：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 克隆项目

```bash
git clone https://github.com/woruo03/Minesweeper.git
cd Minesweeper
```

### 运行游戏

```bash
# 开发模式运行
cargo run

# 发布模式运行（更好的性能）
cargo run --release
```

### 构建项目

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release
```

### 运行测试

```bash
cargo test
```

## 🎮 操作指南

| 操作 | 按键 | 说明 |
|------|------|------|
| 揭开单元格 | 鼠标左键 | 揭开当前单元格 |
| 标记/取消旗帜 | 鼠标右键 | 标记可疑位置 |
| 重新开始 | `R` 键 | 重置当前游戏 |
| 切换难度 - 初级 | `1` 键 | 9×9, 10雷 |
| 切换难度 - 中级 | `2` 键 | 16×16, 40雷 |
| 切换难度 - 专家 | `3` 键 | 16×30, 99雷 |

## 🏗️ 项目结构

```
Minesweeper/
├── Cargo.toml                 # 项目配置与依赖
├── README.md                  # 项目说明文档
├── LICENSE                    # 许可证
├── docs/
│   └── TECHNICAL_ARCHITECTURE.md  # 技术架构文档
├── src/
│   ├── main.rs                # 程序入口
│   ├── lib.rs                 # 库根，插件注册
│   ├── core/                  # 核心模块（组件、事件、资源）
│   ├── game/                  # 游戏逻辑（地雷布置、规则判定）
│   ├── state/                 # 状态管理
│   ├── systems/               # 系统（输入、逻辑、计时器）
│   ├── ui/                    # 用户界面（网格渲染、HUD）
│   └── utils/                 # 工具函数
└── tests/                     # 集成测试
```

## 📐 架构设计

本项目采用 Bevy 的 ECS 架构模式：

- **Component（组件）**: 纯数据容器，如 `Cell`、`MainCamera`
- **Entity（实体）**: 组件的容器，通过 Entity ID 引用
- **System（系统）**: 处理游戏逻辑的函数
- **Resource（资源）**: 全局共享数据，如 `Board`、`GameSession`
- **Event（事件）**: 系统间通信机制
- **State（状态）**: 游戏状态机

详细的架构设计请参考 [技术架构文档](docs/TECHNICAL_ARCHITECTURE.md)。

## 📦 依赖

| 依赖 | 版本 | 用途 |
|------|------|------|
| [bevy](https://bevyengine.org/) | 0.18.1 | 游戏引擎 |
| [rand](https://docs.rs/rand/) | 0.9 | 随机数生成 |

## 🔧 配置

### 开发配置

在 `Cargo.toml` 中，默认启用了动态链接以加快开发编译速度：

```toml
[features]
default = ["dev-dynamic-linking"]
dev-dynamic-linking = ["bevy/dynamic_linking"]
```

### 发布优化

发布构建已进行优化，包括薄 LTO、代码生成质量提升、符号表移除等：

```toml
[profile.release]
lto = "thin"
codegen-units = 1
strip = true
panic = "abort"
```

## 🧪 测试

项目包含单元测试和集成测试：

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test flood_fill
cargo test minefield
cargo test rules
```

## 📝 许可证

本项目采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建你的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交你的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启一个 Pull Request

## 📚 相关链接

- [Bevy 官方文档](https://bevyengine.org/learn/)
- [Rust 编程语言](https://www.rust-lang.org/)
- [技术架构文档](docs/TECHNICAL_ARCHITECTURE.md)

## 👤 作者

**woruo03** - [GitHub](https://github.com/woruo03)
