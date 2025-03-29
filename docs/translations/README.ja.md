# scribe-crab
![Rust](https://img.shields.io/badge/rust-2024-orange)
![GitHub forks](https://img.shields.io/github/forks/blue-orange-yellow/scribe-crab?style=social)
[![GitHub stars](https://img.shields.io/github/stars/blue-orange-yellow/scribe-crab?style=social)](https://github.com/blue-orange-yellow/scribe-crab/stargazers)

Rust関数のドキュメントコメントを生成するMCPサーバー。

## 概要

scribe-crabはRust関数のドキュメントコメントを生成するMCP（Model Context Protocol）サーバーです。Rust関数のコードを入力として受け取り、カスタマイズ可能なフォーマットに従ってドキュメントコメントを生成します。

## 特徴

- Rust関数のドキュメントコメントを生成
- カスタマイズ可能なフォーマットテンプレートを使用
- MCPクライアント（CursorなどのIDE）と統合して使用

## インストール

リポジトリをクローンしてビルド：

```bash
git clone https://github.com/blue-orange-yellow/scribe-crab.git
cd scribe-crab
cargo build --release
```

## 設定

Cursorなどのクライアントで使用するためには、MCPサーバーとして設定する必要があります。Cursorでの設定例：

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

1. フォーマットファイルのパスを環境変数として設定します
2. MCPサーバーを起動します
3. MCPクライアント（Cursor等）からツールを使用します
4. Cursor Agentに「〇〇関数のdoc commentを生成して」のように指示します

## ドキュメントフォーマット

ドキュメントフォーマットは`.format.md`ファイルを編集することでカスタマイズできます。
例：

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

## 言語

このREADMEは以下の言語でも利用可能です：
- [English](../../README.md)
- [中文](README.zh.md)
- [Español](README.es.md)

## ライセンス

MIT

## 貢献

貢献は歓迎します！お気軽にPull Requestを提出してください。
