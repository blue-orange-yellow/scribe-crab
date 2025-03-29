# scribe-crab
![Rust](https://img.shields.io/badge/rust-2024-orange)
![GitHub forks](https://img.shields.io/github/forks/blue-orange-yellow/scribe-crab?style=social)
[![GitHub stars](https://img.shields.io/github/stars/blue-orange-yellow/scribe-crab?style=social)](https://github.com/blue-orange-yellow/scribe-crab/stargazers)

一个用于生成Rust函数文档注释的MCP服务器。

## 概述

scribe-crab是一个MCP（Model Context Protocol）服务器，帮助开发者为其Rust函数生成文档注释。它接收Rust函数代码作为输入，并根据可自定义的格式生成文档注释。

## 特性

- 为Rust函数生成文档注释
- 使用可自定义的格式模板
- 与MCP客户端（如Cursor IDE）集成

## 安装

克隆仓库并构建：

```bash
git clone https://github.com/blue-orange-yellow/scribe-crab.git
cd scribe-crab
cargo build --release
```

## 配置

要与Cursor等客户端一起使用，您需要将其配置为MCP服务器。Cursor配置示例：

```json
{
  "mcpServers": {
    "scribe-crab": {
      "command": "/path/to/scribe-crab/target/release/scribe-crab",
      "args": [],
      "cwd": "/path/to/scribe-crab",
      "env": {
        "FORMAT_PATH": "/path/to/scribe-crab/.format.md"
      }
    }
  }
}
```

## 使用方法

1. 将格式文件路径设置为环境变量
2. 启动MCP服务器
3. 通过MCP客户端（如Cursor）使用该工具
4. 向Cursor Agent发送指令，如"为XX函数生成文档注释"

## 文档格式

可以通过编辑`.format.md`文件来自定义文档格式。
示例：

```rust
/// # Description
/// 
/// This function does XYZ.
/// 
/// # Arguments
/// 
/// * - Description of the first parameter
/// * - Description of the second parameter
/// 
/// # Returns
/// 
/// Description of the return value
```

## 语言

本README还提供以下语言版本：
- [English](../../README.md)
- [日本語](README.ja.md)
- [Español](README.es.md)

## 许可证

MIT

## 贡献

欢迎贡献！请随时提交Pull Request。 