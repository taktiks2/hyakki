# CLAUDE.md - Hyakki (百鬼)

## プロジェクト概要

Hyakki (百鬼) は、陰陽師を操作して日本の妖怪と戦うターミナルベースのローグライクゲームです。

- **ジャンル:** ターン制ローグライク
- **テーマ:** 日本の妖怪
- **プレイヤー:** 陰陽師
- **UI:** ratatuiを使用したターミナルベース
- **言語:** 英語（ゲーム内）

## 技術スタック

- **言語:** Rust（エディション2024）
- **UIフレームワーク:** ratatui 0.30.0（crosstermは内蔵）
- **乱数生成:** rand 0.8
- **テスト:** Rust組み込みテスト

## 開発ガイドライン

### TDD（テスト駆動開発）

このプロジェクトはTDDで進めます：

1. まずテストを作成する
2. テストを実行し、失敗を確認する
3. テストが正しいことを確認できたらコミットする
4. テストをパスさせる実装を進める
5. 実装中はテストを変更せず、コードを修正し続ける
6. すべてのテストが通過するまで繰り返す

### テスト対象（TDDアプローチ）

- コアゲームロジック（戦闘ダメージ、移動検証、FOV）
- ダンジョン生成（部屋配置、接続性）
- AI動作
- 符術効果

### テスト後（非TDD）

- UI描画（ビジュアルテスト）
- 入力処理（統合テスト）
- ゲームループ

## コマンド

```bash
# ビルド
cargo build

# テスト実行
cargo test

# 実行
cargo run

# クリッピー（リント）
cargo clippy

# フォーマット
cargo fmt
```

## モジュール構成

```
src/
├── lib.rs                      # ライブラリルート、モジュール公開
├── main.rs                     # エントリーポイント、メインループ
│
├── game.rs                     # ゲーム状態機械の統合
├── game/
│   ├── state.rs               # GameState列挙型
│   └── config.rs              # ゲーム定数
│
├── world.rs                    # ワールドモジュールの統合
├── world/
│   ├── dungeon.rs             # ダンジョンデータ構造
│   ├── generator.rs           # 部屋と通路の生成
│   ├── tile.rs                # タイル型
│   └── fov.rs                 # 視野計算
│
├── entity.rs                   # エンティティシステムの統合
├── entity/
│   ├── player.rs              # プレイヤー構造体
│   ├── yokai.rs               # 妖怪の敵定義
│   ├── position.rs            # 位置コンポーネント
│   └── combat.rs              # 戦闘ステータス
│
├── system.rs                   # システムの統合
├── system/
│   ├── turn.rs                # ターンベースのアクションキュー
│   ├── combat_system.rs       # ダメージ計算
│   ├── movement.rs            # 移動の検証
│   └── ai.rs                  # 敵AI
│
├── spell.rs                    # 符術システムの統合
├── spell/
│   └── ofuda.rs               # 基本符術
│
├── item.rs                     # アイテムシステムの統合
├── item/
│   ├── item_types.rs          # アイテム定義
│   └── inventory.rs           # インベントリ
│
├── ui.rs                       # UIモジュールの統合
├── ui/
│   ├── renderer.rs            # Ratatui描画
│   ├── input.rs               # キーボード入力処理
│   ├── hud.rs                 # HUD
│   └── log.rs                 # メッセージログ
│
└── data.rs                     # データ定義の統合
    └── yokai_data.rs           # 妖怪テンプレート
```

**重要:** `mod.rs`は使用しない現代的なモジュール構造を採用。各`foo.rs`ファイルは対応する`foo/`ディレクトリのサブモジュールを統合します。

## コア型

### Position

```rust
struct Position {
    x: i32,
    y: i32,
}
```

### TileType

```rust
enum TileType {
    Wall,
    Floor,
    StairsDown,
}
```

### GameState

```rust
enum GameState {
    MainMenu,
    Playing,
    ShowInventory,
    SelectSpell,
    GameOver,
}
```

### CombatStats

```rust
struct CombatStats {
    max_hp: i32,
    hp: i32,
    attack: i32,
    defense: i32,
    max_mana: i32,
    mana: i32,
}
```

## ゲーム定数

- ダンジョンサイズ: 80x50
- 最大部屋数: 8-12
- 最大階層: 10
- FOV半径: 8
- インベントリ最大: 10アイテム
- マナ回復: 毎ターン1

## 妖怪（5種類）

| 名前 | シンボル | 色 | 特徴 |
|------|----------|-----|------|
| Hitotsume-kozo | h | 緑 | 弱い、受動的 |
| Kasa-obake | u | シアン | 遅い |
| Kappa | k | 青 | 攻撃的 |
| Tengu | T | マゼンタ | 速い |
| Oni | O | 赤 | 強い |

## 符術（3種類）

| 名前 | 効果 | マナ | 射程 |
|------|------|------|------|
| Fire Ofuda (火符) | 単体ダメージ10 | 5 | 5 |
| Lightning Ofuda (雷符) | 直線ダメージ5 | 8 | ∞ |
| Healing Ofuda (回復符) | HP15回復 | 6 | 自分 |

## アイテム（3種類）

| 名前 | 効果 |
|------|------|
| Healing Charm | HP20回復 |
| Mana Potion | マナ10回復 |
| Attack Talisman | 攻撃力+2（階終了まで） |

## 操作方法

| キー | アクション |
|------|----------|
| 矢印 / hjkl | 移動 |
| > | 階段を下る |
| g | アイテムを拾う |
| i | インベントリ |
| f | 符術メニュー |
| q | メニューに戻る |
| Esc | キャンセル |

## ダメージ計算式

```
damage = max(1, attacker.attack - defender.defense + rand(-1, 1))
```

## 開発フェーズ

1. **基盤** - 構造、基本描画、移動
2. **ランダム生成** - ランダムダンジョン生成
3. **FOV** - 視野、探索
4. **敵** - 妖怪、AI
5. **戦闘** - ターンシステム、戦闘、レベルアップ
6. **符術** - 符術システム
7. **アイテム** - インベントリ、アイテム

## コーディング規約

- Rustの標準スタイルに従う（`cargo fmt`）
- `cargo clippy`の警告を解消する
- パブリックAPIにはドキュメントコメントを付ける
- テストは各モジュールの`#[cfg(test)]`ブロック内に配置
