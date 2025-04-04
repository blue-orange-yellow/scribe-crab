# scribe-crab
![Rust](https://img.shields.io/badge/rust-2024-orange)
![GitHub forks](https://img.shields.io/github/forks/blue-orange-yellow/scribe-crab?style=social)
[![GitHub stars](https://img.shields.io/github/stars/blue-orange-yellow/scribe-crab?style=social)](https://github.com/blue-orange-yellow/scribe-crab/stargazers)

The MCP server that generates documentation comments for Rust.
![ChatGPT Image 2025年3月29日 20_55_37](https://github.com/user-attachments/assets/c7cc9514-cac5-4fc9-9145-800375be6bb6)



## Overview

scribe-crab is an MCP (Model Context Protocol) server that helps developers generate documentation comments for their Rust functions. It takes Rust function code as input and produces documentation comments following a customizable format.

## Features

- Generate documentation comments for Rust functions
- Use customizable format templates
- Integrate with MCP clients (such as Cursor IDE)

## Installation

Clone the repository and build:

```bash
git clone https://github.com/blue-orange-yellow/scribe-crab.git
cd scribe-crab
cargo build --release
```

## Configuration

To use with clients like Cursor, you need to configure it as an MCP server. Example Cursor configuration:

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

## Usage

1. Set the format file path as an environment variable
2. Start the MCP server
3. Use the tool through an MCP client (like Cursor)
4. Ask Cursor Agent "Generate doc comment for XX function" or similar instructions

## Documentation Format

The documentation format can be customized by editing the `.format.md` file.
Example:

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

## Languages

This README is also available in:
- [日本語](docs/translations/README.ja.md)
- [中文](docs/translations/README.zh.md)
- [Español](docs/translations/README.es.md)
