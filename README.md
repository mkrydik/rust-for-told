# Rust for Told

[Google 日本語入力 CGI API](https://www.google.co.jp/ime/cgiapi.html) を使って、与えられた文字列を変換する CLI ツールです。


## 使い方

このプロジェクトは以下の環境で動作確認しました。

- macOS Monterey v12.4 : Rust v1.63.0

Rust 製なので `cargo` コマンドでビルド・実行してみてください。

```bash
$ git clone https://github.com/mkrydik/rust-for-told.git && cd "$(basename "$_" .git)"

# ソースを直接実行する
$ cargo run 'ひきょう'
秘境
# ↑ このように第1引数で与えた文字列が漢字変換されます

# 開発ビルド
$ cargo build
$ ./target/debug/rust-for-told '1949ねんこうかいのせいぶげき'
１９４９年公開の西部劇

# 本番ビルド
$ cargo build --release
$ ./target/release/rust-for-told 'ごらんのすぽんさーのていきょうでおおくりします'
ご覧のスポンサーの提供でお送りします
```


## コーディング関連コマンド

```bash
# 静的チェック
$ cargo check

# フォーマット
$ cargo fmt

# テスト
$ cargo test

# ベンチマークテスト
$ cargo bench

# ドキュメンテーション生成
$ cargo doc
```
