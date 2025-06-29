# 蒼穹紅蓮隊風ロックオンレーザーゲーム仕様書

## プロジェクト概要

**プロジェクト名**: Terra Lock（テラロック）  
**開発期間**: 1日（8-10時間）  
**技術スタック**: Rust + WebAssembly (Macroquad)  
**目標**: 蒼穹紅蓮隊のN.A.L.S.システムを簡略化した2Dシューティングゲーム

## ゲーム仕様

### 基本コンセプト
マウス操作による自機制御と、エリア内ロックオン→一斉レーザー発射システムを核とした縦スクロールシューティングゲーム

### 画面仕様
- **解像度**: 800x600px（4:3比率）
- **フレームレート**: 60FPS
- **スクロール**: 固定画面（背景スクロールなし）
- **UI配置**: 左上にスコアとロックオン数表示

## 操作仕様

### 入力方式
- **マウス移動**: 自機とターゲティングサイトの移動
- **マウス左ボタン短押し**: 通常レーザー発射
- **マウス左ボタン長押し**: ワイヤーフレーム展開とロックオン
- **マウス左ボタンリリース**: ロックオン対象への一斉レーザー発射

### 操作フロー
1. マウス移動で自機が画面内を追従移動
2. 左クリックで前方に通常レーザー発射
3. 左ボタン長押しでマウス中心にワイヤーフレーム展開
4. ワイヤーフレーム内の敵機を自動ロックオン（最大6機）
5. ボタンリリースで全ロックオン敵にホーミングレーザー発射

### パフォーマンス要件
- **60FPS維持**: WebAssemblyの高速実行により安定フレームレート
- **低レイテンシ**: マウス入力からレンダリングまで16ms以内
- **バンドルサイズ**: 圧縮後500KB以下（macroquadの最適化適用）
- **クロスブラウザ対応**: Chrome, Firefox, Safari, Edgeでの動作保証
- **WebAssembly最適化**: ネイティブ性能の80%以上を維持

## ゲームオブジェクト仕様

## ゲームオブジェクト仕様

### 自機（Player）
```rust
struct Player {
    position: Vec2,
    texture: Texture2D,
    health: u8,
}
// 外観: 青い三角形（20x15px）
// 移動: マウス座標に追従（Vec2演算で効率化）
// 制約: 画面内のみ移動可能
// HP: 1（被弾で即ゲームオーバー）
```

### 敵機（Enemy）
```rust
struct Enemy {
    position: Vec2,
    velocity: Vec2,
    is_locked: bool,
    lock_timer: f32,
}
// 外観: 赤い円（直径20px）
// 移動: 画面上部から下部へ直線移動
// 速度: 120px/秒（2px/frame → 正確な時間ベース）
// 出現: 2-3秒間隔でランダム位置
// 最大同時数: 8機（メモリプール使用）
```

### レーザーシステム
```rust
struct NormalLaser {
    position: Vec2,
    velocity: Vec2,
    lifetime: f32,
}

struct LockOnLaser {
    start_pos: Vec2,
    target_pos: Vec2,
    progress: f32,
    speed: f32,
}
// 通常レーザー: 青い直線（幅2px、速度480px/秒）
// ロックオンレーザー: 黄色い追尾線（ベジェ曲線使用）
```

## ロックオンシステム仕様

### ワイヤーフレーム描画
```rust
fn draw_wireframe(center: Vec2, radius: f32, locked_count: u8) {
    // 点線の円を描画（WebGLベース高速レンダリング）
    draw_circle_lines(center.x, center.y, radius, 2.0, WHITE);
    
    // ロックオン数に応じた色変化
    let color = match locked_count {
        0 => WHITE,
        1..=3 => YELLOW,
        4..=6 => RED,
        _ => ORANGE,
    };
}
```

### 空間分割による効率的判定
```rust
struct LockOnSystem {
    active: bool,
    center: Vec2,
    radius: f32,
    locked_enemies: Vec<u32>,  // EntityIDベース
    max_targets: u8,
}

impl LockOnSystem {
    fn check_lock_targets(&mut self, enemies: &[Enemy]) {
        // 距離計算の最適化（平方根回避）
        for (id, enemy) in enemies.iter().enumerate() {
            let distance_sq = self.center.distance_squared(enemy.position);
            if distance_sq < self.radius * self.radius {
                self.add_target(id as u32);
            }
        }
    }
}
```

### ロックオン解除条件
- マウスボタンリリース
- 敵機がワイヤーフレーム外に移動
- 敵機の撃破
- 最大数を超えた場合は古いものから解除

## UI仕様

### 表示要素
```
スコア: 「SCORE: XXXXX」（左上、白文字）
ロックオン数: 「LOCK: X/6」（左上、黄文字）
ゲームオーバー: 「GAME OVER」（中央、赤文字）
操作説明: 「Mouse: Move | Click: Shoot | Hold: Lock-on」（下部、小文字）
```

### フォント仕様
```
スコア・ロックオン数: 16px monospace
ゲームオーバー: 48px monospace
操作説明: 12px sans-serif
```

## ゲームバランス

### スコアシステム
```
通常レーザー撃破: 100点
ロックオンレーザー撃破: 200点
同時ロックオン撃破ボーナス:
- 2機: +300点
- 3機: +600点
- 4機: +1000点
- 5機: +1500点
- 6機: +2100点
```

### 敵出現パターン
```
初期間隔: 3秒
最小間隔: 1秒（60秒後）
出現位置: X座標ランダム（50-750px）
同時最大数: 8機
```

## 技術仕様

### 開発環境とツールチェーン
```
Rust 1.85+               - 安定版コンパイラ
macroquad 0.3.x          - WebAssembly対応ゲームエンジン
wasm-pack 0.13.1         - WebAssembly最適化ツール
cargo-watch              - ホットリロード（1-3秒）
VS Code + rust-analyzer  - IDE環境
wasm-bindgen             - JavaScript連携（ブラウザAPI用）
```

### WebAssembly実行環境
```
Target: wasm32-unknown-unknown
Bundle Size: <500KB (gzip圧縮後)
Browser Support: Chrome 57+, Firefox 52+, Safari 11+, Edge 16+
Performance Target: ネイティブ性能の80%以上
Memory Usage: <50MB (ヒープ使用量)
```

### アーキテクチャ設計
```rust
// WebAssembly対応のコア構造体
struct GameState {
    player: Player,
    enemies: Vec<Enemy>,
    lasers: Vec<Laser>,
    lock_system: LockOnSystem,
    score: u32,
}

// WebAssembly環境検出とパフォーマンス監視
#[cfg(target_arch = "wasm32")]
mod web_support {
    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }
    
    pub fn performance_check(frame_time: f64) {
        if frame_time > 16.67 { // 60FPS threshold
            log(&format!("Performance warning: {:.2}ms", frame_time));
        }
    }
}

// デュアル環境対応のゲームループ
#[macroquad::main("Terra Lock")]
async fn main() {
    #[cfg(target_arch = "wasm32")]
    console_log!("WebAssembly environment initialized");
    
    loop {
        let start_time = get_time();
        
        clear_background(BLACK);
        update_game_logic();
        render_all_objects();
        
        #[cfg(target_arch = "wasm32")]
        web_support::performance_check((get_time() - start_time) * 1000.0);
        
        next_frame().await;
    }
}
```

// ゲームループ（60FPS保証）
#[macroquad::main("Terra Lock")]
async fn main() {
    loop {
        clear_background(BLACK);
        update_game_logic();
        render_all_objects();
        next_frame().await;
    }
}
```

### パフォーマンス最適化
```toml
[profile.release]
opt-level = "z"          # サイズ最適化
lto = true               # リンク時最適化
codegen-units = 1        # 単一コード生成
panic = "abort"          # パニック時即座終了
```



## 技術的優位性と制約

### Rust + WebAssemblyの利点
```
パフォーマンス: JavaScript比4-8倍高速な計算処理
メモリ安全性: 実行時エラーの大幅削減
型安全性: コンパイル時バグ検出による開発効率向上
ポータビリティ: デスクトップ・Web両対応のシングルコードベース
バンドルサイズ: 最適化後500KB以下達成可能
```

### 実装上の制約と対策
```rust
// WebAssembly制約への対応
// 1. シングルスレッド環境
//    → 軽量なゲームループ設計で回避

// 2. DOM操作の制限
//    → macroquadによる完全抽象化

// 3. ファイルシステムアクセス不可
//    → アセットのバイナリ埋め込み

// 4. デバッグ情報の制限
//    → console.log!マクロとwasm-bindgen使用
```

### パフォーマンス最適化技法
```rust
// スプライトバッチング
fn batch_render_enemies(enemies: &[Enemy]) {
    let mut batch = Vec::with_capacity(enemies.len());
    for enemy in enemies {
        batch.push(DrawParam::new().dest(enemy.position));
    }
    // 一括描画でGPU呼び出し最小化
}

// オブジェクトプール
struct GamePools {
    bullets: Pool<Bullet>,
    enemies: Pool<Enemy>,
    particles: Pool<Particle>,
}
```



## 開発環境とビルド設定

### 必要ツールとセットアップ
```bash
# Rustツールチェーン
rustup target add wasm32-unknown-unknown
cargo install wasm-pack cargo-watch

# プロジェクト初期化
cargo new --bin terra_lock
cd terra_lock

# Cargo.toml設定
[dependencies]
macroquad = "0.4"

[lib]
crate-type = ["cdylib"]

[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
```


