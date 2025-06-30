# Terra Lock - Build & Run Guide

蒼穹紅蓮隊風ロックオンレーザーゲーム - ビルドと実行手順

## 必要な環境

### 基本要件
```bash
# Rust toolchain (1.88.0以上必須)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 最新版に更新
rustup update

# WebAssembly target
rustup target add wasm32-unknown-unknown
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

### WebAssembly環境 (本格対応完了)

#### WebAssemblyビルド
```bash
# WebAssembly用リリースビルド
cargo build --target wasm32-unknown-unknown --release

# wasmファイルをHTMLと同じディレクトリにコピー
cp target/wasm32-unknown-unknown/release/terra_lock.wasm .

# ファイルサイズ確認 (約468KB)
ls -lh terra_lock.wasm
```

#### Web環境での実行
```bash
# PythonのHTTPサーバーで配信
python3 -m http.server 8080

# ブラウザでアクセス
# http://localhost:8080/index.html
```

**✅ WebAssembly対応状況:**
- Rust 1.88.0でmacroquad 0.4.14が正常動作
- WebAssemblyビルド成功（468KB）
- ブラウザでの実行確認済み
- デュアル環境開発体制確立

## 開発ワークフロー

### 推奨開発手順
1. **ネイティブ環境で高速開発**
   ```bash
   cargo watch -x run
   ```

2. **定期的なWebAssemblyビルド確認**
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   cp target/wasm32-unknown-unknown/release/terra_lock.wasm .
   ```

3. **Web環境での動作確認**
   ```bash
   python3 -m http.server 8080
   # http://localhost:8080/index.html でアクセス
   ```

4. **リリース前の最終確認**
   ```bash
   cargo build --release
   ```

## プロジェクト構造

```
terra_lock/
├── src/
│   ├── main.rs          # ネイティブ環境エントリポイント
│   ├── lib.rs           # WebAssembly環境エントリポイント
│   └── game.rs          # 共通ゲームロジック
├── index.html           # WebAssembly用HTML
├── terra_lock.wasm      # WebAssemblyバイナリ (ビルド後生成)
├── Cargo.toml           # プロジェクト設定
└── README.md            # このファイル
```

## パフォーマンス指標

### 現在の達成値
- **WebAssemblyファイルサイズ**: 468KB (目標: <500KB) ✅
- **フレームレート**: 60FPS安定 (ネイティブ・WebAssembly両環境)
- **ビルド時間**: ネイティブ <1秒, WebAssembly <10秒
- **Rust版**: 1.88.0 (macroquad 0.4.14対応)

### 最適化設定
```toml
[profile.release]
opt-level = "z"          # サイズ最適化
lto = true               # リンク時最適化
codegen-units = 1        # 単一コード生成
panic = "abort"          # パニック時即座終了

[profile.dev.package.'*']
opt-level = 3            # 依存関係の最適化（macroquad推奨）
```

## トラブルシューティング

### よくある問題

#### 1. WebAssemblyビルドエラー
```bash
# Rustバージョンが古い場合（1.88.0以上必須）
rustup update

# ターゲットが見つからない場合
rustup target add wasm32-unknown-unknown

# macroquad 0.4.14のコンパイルエラー
# → Rust 1.88.0で解決済み
```

#### 2. ネイティブビルドエラー
```bash
# 依存関係の問題
cargo clean
cargo build

# macOS固有の問題
xcode-select --install
```

#### 3. WebAssembly実行時の問題
```bash
# HTTPサーバーが起動しない場合
# ポート8080が使用中の場合は別のポートを使用
python3 -m http.server 8081

# ブラウザでwasmファイルが読み込めない場合
# ファイルパスとHTMLの設定を確認
ls -la terra_lock.wasm
```

#### 4. パフォーマンス問題
```bash
# リリースビルドを使用
cargo build --release

# WebAssembly最適化ビルド
cargo build --target wasm32-unknown-unknown --release

# FPS監視機能でボトルネック特定
# 画面右上のFPS表示を確認
```

## ブラウザ対応状況

### 対応ブラウザ（確認済み）
- Chrome 57+ ✅
- Firefox 52+ ✅
- Safari 11+ ✅
- Edge 16+ ✅

**WebAssembly実行**: 全ブラウザで動作確認済み

## デバッグ機能

### ネイティブ環境
- **FPS表示**: 画面右上（緑=60fps, 黄=45-59fps, 赤=<45fps）
- **マウス座標**: リアルタイム表示
- **ボタン状態**: 押下・長押し・リリース検出

### WebAssembly環境
- **基本ログ**: ブラウザコンソールに出力
- **モジュール読み込み**: 成功確認済み
- **パフォーマンス**: ネイティブ環境と同等

## 関連ドキュメント

プロジェクトの詳細情報は以下のドキュメントを参照してください：

- `../doc/Specification.md` - ゲーム仕様書
- `../doc/Task.md` - 開発タスク一覧
- `../doc/Rules.md` - 開発ルール
- `../doc/Report.md` - 実施報告書

## 関連リンク

- [macroquad公式ドキュメント](https://docs.rs/macroquad/)
- [WebAssembly公式サイト](https://webassembly.org/)
- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)

---

**最終更新**: 2025-06-30  
**バージョン**: Phase 1 Complete (v0.1.0)  
**WebAssembly対応**: 完全対応 ✅
