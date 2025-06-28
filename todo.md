# TODO

## 完了済みタスク
- [x] linterの設定
- [x] formatterの設定
- [x] 必要なExtentionの設定
- [x] pre-commit時のチェックの設定
- [x] github workflowによるpull request時、merge時のチェックの設定
- [x] github workflowによるビルドの設定
- [x] cursor rulesの設定
- [x] SQLパーサーの実装 (pestを利用)
  - [x] 字句解析器（Lexer）の実装
    - [x] SQLキーワードのトークン化 (大文字・小文字を区別しない)
    - [x] 識別子のトークン化
    - [x] 演算子、リテラルのトークン化
  - [x] 抽象構文木（AST）のデータ構造の定義（基本版）
    - [x] Statement（SELECT, INSERT, UPDATE, DELETEなど）
  - [x] 構文解析器（Parser）の実装（基本版）
    - [x] SELECT文の解析 (`SELECT * FROM ...`)
    - [x] INSERT文の解析
    - [x] UPDATE文の解析
    - [x] DELETE文の解析
  - [x] エラーハンドリングの実装 (基本的なエラー処理)
  - [x] パーサーのテスト作成 (単体テスト)

## 高優先度タスク（次期実装対象）
- [ ] 式（Expression）のサポート追加
  - [ ] 比較演算子（=, !=, <, >, <=, >=）
  - [ ] 論理演算子（AND, OR, NOT）
  - [ ] 算術演算子（+, -, *, /）
  - [ ] リテラル（数値、文字列、NULL、真偽値）
  - [ ] カラム参照
- [ ] WHERE句のサポート
  - [ ] SELECT文でのWHERE句
  - [ ] UPDATE文でのWHERE句
  - [ ] DELETE文でのWHERE句
- [ ] SELECT文の機能拡張
  - [ ] 特定カラム選択（SELECT col1, col2 FROM ...）
  - [ ] ORDER BY句
  - [ ] GROUP BY句
  - [ ] LIMIT句

## 中優先度タスク
- [ ] JOIN句のサポート
  - [ ] INNER JOIN
  - [ ] LEFT JOIN
  - [ ] RIGHT JOIN
  - [ ] FULL OUTER JOIN
- [ ] INSERT文の機能拡張
  - [ ] カラム指定挿入（INSERT INTO table (col1, col2) VALUES ...）
  - [ ] 複数行挿入
- [ ] データ型の詳細サポート
  - [ ] 数値型（INTEGER, REAL）
  - [ ] 日付型（DATE, DATETIME）
  - [ ] NULL値の明示的サポート
- [ ] 文字列リテラルのエスケープ処理
- [ ] エラーハンドリングの改善（詳細なエラーメッセージ）

## 低優先度タスク（将来実装）
- [ ] サブクエリのサポート
- [ ] DDL文の追加
  - [ ] CREATE TABLE
  - [ ] DROP TABLE
  - [ ] ALTER TABLE
- [ ] 関数のサポート（COUNT, SUM, AVG等）
- [ ] ビューのサポート
- [ ] インデックス関連の構文

## 現在の制限事項
1. SELECT文は `SELECT *` のみ対応（特定カラム選択不可）
2. WHERE句が未実装（条件指定での絞り込み不可）
3. JOIN句が未実装（複数テーブルの結合不可）
4. データ型は文字列のみ（数値、日付等は文字列として扱われる）
5. 文字列リテラル内でのエスケープ処理未対応
6. サブクエリ未対応
7. 集約関数未対応