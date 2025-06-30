# Terra Lock - ロックオンレーザーゲーム

[![GitHub Pages](https://img.shields.io/badge/Play%20Online-GitHub%20Pages-blue)](https://kojiyasuda.github.io/amazonq-game-challenge/)
[![Rust](https://img.shields.io/badge/Rust-1.88.0-orange)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-482KB-green)](https://webassembly.org/)

蒼穹紅蓮隊のN.A.L.S.システムを簡略化した2Dシューティングゲーム「Terra Lock」です。  
Rust + macroquadで開発し、WebAssemblyでブラウザ上でプレイできます。

## 🎮 Web版でプレイ

**[🚀 今すぐプレイ（GitHub Pages）](https://kojiyasuda.github.io/amazonq-game-challenge/)**

対応ブラウザ: Chrome, Firefox, Safari, Edge

## 🎯 ゲーム概要

### コンセプト
マウス操作による自機制御と、エリア内ロックオン→一斉レーザー発射システムを核とした縦スクロールシューティングゲーム

### 操作方法
- **マウス移動**: 自機とターゲティングサイトの移動
- **左クリック短押し**: 通常レーザー発射
- **左ボタン長押し**: ワイヤーフレーム展開とロックオン
- **左ボタンリリース**: ロックオン対象への一斉ホーミングレーザー発射

### ゲーム特徴
- **4種類の敵機**: 直線型、ジグザグ型、円弧型、追尾型
- **加速ホーミングレーザー**: 時間経過で加速する追尾レーザー
- **難易度カーブ**: 30秒ごとに段階的に難易度上昇
- **最大6機同時ロックオン**: 戦略的な一斉攻撃システム

## 🛠️ ローカル開発

### 必要環境
- Rust 1.88.0以上
- WebAssembly用ターゲット: `wasm32-unknown-unknown`

### ビルド手順

#### ネイティブ版（開発用）
```bash
cd terra_lock
cargo run
```

#### WebAssembly版
```bash
cd terra_lock
cargo build --target wasm32-unknown-unknown --release
```

#### Web版テスト
```bash
cd terra_lock/web
python3 -m http.server 8000
# http://localhost:8000 でアクセス
```

### プロジェクト構造
```
terra_lock/
├── src/
│   ├── main.rs          # ネイティブ版エントリポイント
│   ├── lib.rs           # WebAssembly版エントリポイント
│   └── game.rs          # ゲームロジック
└── Cargo.toml
docs/                    # GitHub Pages配信用
├── index.html
├── terra_lock.wasm
└── README.md
```

## 🎨 技術仕様

### 開発技術
- **言語**: Rust 1.88.0
- **ゲームエンジン**: macroquad 0.4
- **WebAssembly**: wasm32-unknown-unknown
- **配信**: GitHub Pages

### パフォーマンス
- **フレームレート**: 60FPS
- **解像度**: 800x600px
- **WebAssemblyサイズ**: 482KB（目標500KB以下達成）
- **レスポンス**: マウス入力から描画まで16ms以内

### ゲームシステム
- **敵機AI**: 4種類の動作パターン
- **物理演算**: ベジェ曲線によるホーミング軌道
- **難易度システム**: 時間ベースの動的調整
- **スコアシステム**: 同時撃破ボーナス

## 📈 開発経緯

### プロジェクト背景
[Amazon Q CLI でゲームを作ろう Tシャツキャンペーン](https://aws.amazon.com/jp/blogs/news/build-games-with-amazon-q-cli-and-score-a-t-shirt/)への応募作品として開発。

蒼穹紅蓮隊のN.A.L.S.（Numerical Approach & Lock-on Sight）システムに着想を得て、ロックオンレーザーシューティングゲームを制作。

### 開発手法
1. **要件定義**: Claude Sonnet 4との対話による仕様策定
2. **実装**: Amazon Q CLIとのペアプログラミング
3. **段階的開発**: Phase 1-5の5段階に分けた計画的開発

### 開発フェーズ
- **Phase 1**: 基盤構築（プロジェクト初期化、基本ゲームループ）
- **Phase 2**: コアゲームプレイ（自機、敵機、レーザーシステム）
- **Phase 3**: ロックオンシステム（ワイヤーフレーム、ホーミングレーザー）
- **Phase 4**: 最適化・仕上げ（UI改善、難易度カーブ、敵機バリエーション）
- **Phase 5**: デプロイメント（WebAssembly最適化、GitHub Pages公開）

### 技術的挑戦
- **WebAssembly最適化**: 500KB以下のバンドルサイズ達成
- **60FPS維持**: ネイティブ性能の80%以上を維持
- **クロスプラットフォーム**: デスクトップ・Web両対応
- **リアルタイム物理**: ベジェ曲線による滑らかなホーミング

## 🏆 成果

### 技術的成果
- **WebAssemblyサイズ**: 482KB（目標500KB以下達成）
- **パフォーマンス**: 60FPS安定動作
- **クロスブラウザ対応**: 主要ブラウザで動作確認済み

### ゲームプレイ成果
- **戦略性**: 4種類の敵機による多様な戦術
- **爽快感**: 加速ホーミングレーザーによる迫力ある演出
- **挑戦要素**: 段階的難易度上昇による長時間プレイ対応

## 📄 ライセンス

MIT License

## 🤝 開発協力

- **Amazon Q CLI**: ペアプログラミングパートナー
- **Claude Sonnet 4**: 要件定義・設計支援
- **macroquad**: Rustゲーム開発フレームワーク

---

**🎮 [今すぐプレイ！](https://kojiyasuda.github.io/amazonq-game-challenge/)**
