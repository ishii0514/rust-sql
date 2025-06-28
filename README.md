# Rust SQL Parser

RustでPestパーサーライブラリを使用して実装したSQL92準拠のSQLパーサーです。

## 概要

このプロジェクトは、基本的なSQL文（SELECT、INSERT、UPDATE、DELETE）を解析し、抽象構文木（AST）として表現するRustライブラリです。SQL92標準に準拠し、大文字小文字を区別しない構文解析を提供します。

## 特徴

- **SQL92準拠**: SQL92標準に準拠した構文解析
- **基本的なSQL文対応**: SELECT、INSERT、UPDATE、DELETE文の解析
- **大文字小文字非依存**: SQLキーワードの大文字小文字を区別しない
- **日本語サポート**: 文字列リテラル内での日本語文字の使用
- **コメントサポート**: `--` で始まる行コメント
- **包括的テスト**: 様々なケースをカバーする単体テスト

## インストール

```bash
git clone <このリポジトリのURL>
cd rust-sql
cargo build
```

## 使用方法

### ライブラリとして使用

```rust
use rust_sql::parser::parse_sql;
use rust_sql::ast::Statement;

let sql = "SELECT * FROM users;";
match parse_sql(sql) {
    Ok(Statement::Select { table }) => {
        println!("テーブル名: {}", table);
    }
    Err(e) => {
        eprintln!("解析エラー: {}", e);
    }
}
```

### コマンドラインから実行

```bash
cargo run
```

## サポートするSQL文

### SELECT文
```sql
SELECT * FROM table_name;
```

### INSERT文
```sql
INSERT INTO table_name VALUES ('value1', 'value2');
```

### UPDATE文
```sql
UPDATE table_name SET column1 = 'value1', column2 = 'value2';
```

### DELETE文
```sql
DELETE FROM table_name;
```

## 開発

### 必要なツール

- Rust 1.70以上
- Cargo

### 開発コマンド

```bash
# ビルド
cargo build

# テスト実行
cargo test

# コード整形
cargo fmt

# 静的解析
cargo clippy

# リリースビルド
cargo build --release
```

### テスト駆動開発

このプロジェクトはt-wadaの推奨するテスト駆動開発（TDD）手法に基づいて開発されています：

1. まずテストを作成
2. テストの失敗を確認
3. 最小限の実装でテストをパス
4. リファクタリング

## プロジェクト構造

```
src/
├── lib.rs          # ライブラリルート
├── main.rs         # バイナリエントリーポイント
├── ast.rs          # 抽象構文木の定義
├── parser.rs       # SQLパーサーの実装
└── sql.pest        # Pest文法定義ファイル
```

## 依存関係

- [pest](https://github.com/pest-parser/pest) - パーサージェネレーター
- [pest_derive](https://github.com/pest-parser/pest) - pest用のderiveマクロ

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。

## 貢献

プルリクエストやイシューの報告を歓迎します。開発に参加する前に、プロジェクトのコーディング規約とTDD原則に従ってください。