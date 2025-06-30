# Terra Lock - Build & Run Guide

蒼穹紅蓮隊風ロックオンレーザーゲーム - ビルドと実行手順

## 必要な環境

### 基本要件
```bash
# Rust toolchain (1.85以上推奨)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# WebAssembly target
rustup target add wasm32-unknown-unknown

# WebAssembly build tools
cargo install wasm-pack
```

### 開発環境 (推奨)
```bash
# ホットリロード用
cargo install cargo-watch

# VS Code + rust-analyzer (推奨IDE)
```

## ビルドと実行

### ネイティブ環境 (開発用)

#### デバッグビルド
```bash
# プロジェクトディレクトリに移動
cd terra_lock

# ビルドと実行
cargo run

# ホットリロード (開発時推奨)
cargo watch -x run
```

#### リリースビルド
```bash
# 最適化ビルド
cargo build --release

# 実行
./target/release/terra_lock
```

### WebAssembly環境 (本番用)

#### WebAssemblyビルド
```bash
# WebAssembly用ビルド
wasm-pack build --target web --out-dir web/pkg --features web

# ファイルサイズ確認
ls -lh web/pkg/terra_lock_bg.wasm
```

#### Web環境での実行
```bash
# 簡易HTTPサーバー起動
cd web
python3 -m http.server 8000

# ブラウザでアクセス
# http://localhost:8000/index.html
```

#### 本格的なWebサーバー (推奨)
```bash
# Node.js + serve
npm install -g serve
cd web
serve -s . -p 8000

# または nginx, Apache等の設定
```

## 開発ワークフロー

### 推奨開発手順
1. **ネイティブ環境で高速開発**
   ```bash
   cargo watch -x run
   ```

2. **定期的なWebAssembly検証**
   ```bash
   wasm-pack build --target web --out-dir web/pkg --features web
   cd web && python3 -m http.server 8000
   ```

3. **リリース前の最終確認**
   ```bash
   cargo build --release
   wasm-pack build --target web --out-dir web/pkg --features web --release
   ```

## プロジェクト構造

```
terra_lock/
├── src/
│   ├── main.rs          # ネイティブ環境エントリポイント
│   ├── lib.rs           # WebAssembly環境エントリポイント
│   └── game.rs          # 共通ゲームロジック
├── web/
│   ├── index.html       # WebAssembly テスト用HTML
│   └── pkg/             # wasm-pack生成ファイル (自動生成)
├── Cargo.toml           # プロジェクト設定
└── README.md            # このファイル
```

## パフォーマンス指標

### 現在の達成値
- **WebAssemblyファイルサイズ**: 36KB (目標: <500KB)
- **フレームレート**: 60FPS安定
- **ビルド時間**: ネイティブ <1秒, WebAssembly <15秒

### 最適化設定
```toml
[profile.release]
opt-level = "z"          # サイズ最適化
lto = true               # リンク時最適化
codegen-units = 1        # 単一コード生成
panic = "abort"          # パニック時即座終了
```

## トラブルシューティング

### よくある問題

#### 1. WebAssemblyビルドエラー
```bash
# wasm-packが見つからない場合
cargo install wasm-pack

# ターゲットが見つからない場合
rustup target add wasm32-unknown-unknown
```

#### 2. ネイティブビルドエラー
```bash
# 依存関係の問題
cargo clean
cargo build

# macOS固有の問題
xcode-select --install
```

#### 3. Web環境でのCORSエラー
```bash
# ローカルファイルアクセスではなくHTTPサーバーを使用
python3 -m http.server 8000
# または
npx serve web -p 8000
```

#### 4. パフォーマンス問題
```bash
# リリースビルドを使用
cargo build --release
wasm-pack build --release --target web --out-dir web/pkg --features web

# FPS監視機能でボトルネック特定
# 画面右上のFPS表示を確認
```

## ブラウザ対応状況

### 対応ブラウザ
- ✅ Chrome 57+
- ✅ Firefox 52+
- ✅ Safari 11+
- ✅ Edge 16+

### WebAssembly機能要件
- WebAssembly MVP対応
- Canvas API対応
- Mouse Events対応
- RequestAnimationFrame対応

## デバッグ機能

- **FPS表示**: 画面右上（緑=60fps, 黄=45-59fps, 赤=<45fps）
- **マウス座標**: リアルタイム表示
- **ボタン状態**: 押下・長押し・リリース検出
- **コンソールログ**: WebAssembly環境でのデバッグ出力

## 関連ドキュメント

プロジェクトの詳細情報は以下のドキュメントを参照してください：

- `../doc/Specification.md` - ゲーム仕様書
- `../doc/Task.md` - 開発タスク一覧
- `../doc/Rules.md` - 開発ルール
- `../doc/Report.md` - 実施報告書

## 関連リンク

- [macroquad公式ドキュメント](https://docs.rs/macroquad/)
- [WebAssembly公式サイト](https://webassembly.org/)
- [wasm-pack Book](https://rustwasm.github.io/wasm-pack/)

---

**最終更新**: 2025-06-30  
**バージョン**: Phase 1 Complete (v0.1.0)
