# Hyakki (百鬼) 開発用 justfile

# デフォルトタスク: ヘルプを表示
default:
    @just --list

# ビルド
build:
    cargo build

# リリースビルド
release:
    cargo build --release

# テスト実行
test:
    cargo test

# テスト実行（出力付き）
test-verbose:
    cargo test -- --nocapture

# 特定のテストを実行
test-one NAME:
    cargo test {{NAME}} -- --nocapture

# 実行
run:
    cargo run

# リリースモードで実行
run-release:
    cargo run --release

# クリッピー（リント）
clippy:
    cargo clippy --all-targets --all-features --fix --allow-dirty -- -D warnings

# フォーマット
fmt:
    cargo fmt

# フォーマットチェック（変更なし）
fmt-check:
    cargo fmt -- --check

# すべてのチェックを実行（CI用）
check: fmt-check clippy test

# クリーン
clean:
    cargo clean

# ドキュメント生成
doc:
    cargo doc --open

# 依存関係の更新
update:
    cargo update

# ウォッチモードでテスト（cargo-watchが必要）
watch-test:
    cargo watch -x test

# ウォッチモードで実行（cargo-watchが必要）
watch-run:
    cargo watch -x run
