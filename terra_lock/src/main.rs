use macroquad::prelude::*;

// ゲーム状態の定義
#[derive(Debug, Clone, Copy, PartialEq)]
enum GameState {
    Playing,
    GameOver,
}

// プレイヤー構造体
#[derive(Debug, Clone)]
struct Player {
    position: Vec2,
    health: u8,
}

impl Player {
    fn new() -> Self {
        Self {
            position: Vec2::new(400.0, 500.0), // 画面下部中央
            health: 1,
        }
    }
}

// 敵機構造体
#[derive(Debug, Clone)]
struct Enemy {
    position: Vec2,
    velocity: Vec2,
    is_locked: bool,
    lock_timer: f32,
}

// レーザー構造体
#[derive(Debug, Clone)]
struct NormalLaser {
    position: Vec2,
    velocity: Vec2,
    lifetime: f32,
}

#[derive(Debug, Clone)]
struct LockOnLaser {
    start_pos: Vec2,
    target_pos: Vec2,
    progress: f32,
    speed: f32,
}

// ロックオンシステム構造体
#[derive(Debug, Clone)]
struct LockOnSystem {
    active: bool,
    center: Vec2,
    radius: f32,
    locked_enemies: Vec<usize>,
    max_targets: u8,
}

impl LockOnSystem {
    fn new() -> Self {
        Self {
            active: false,
            center: Vec2::ZERO,
            radius: 100.0,
            locked_enemies: Vec::new(),
            max_targets: 6,
        }
    }
}

// メインゲーム状態構造体
#[derive(Debug)]
struct Game {
    state: GameState,
    player: Player,
    enemies: Vec<Enemy>,
    normal_lasers: Vec<NormalLaser>,
    lock_on_lasers: Vec<LockOnLaser>,
    lock_system: LockOnSystem,
    score: u32,
}

impl Game {
    fn new() -> Self {
        Self {
            state: GameState::Playing,
            player: Player::new(),
            enemies: Vec::new(),
            normal_lasers: Vec::new(),
            lock_on_lasers: Vec::new(),
            lock_system: LockOnSystem::new(),
            score: 0,
        }
    }
    
    fn update(&mut self) {
        // TODO: ゲームロジックの更新
    }
    
    fn draw(&self) {
        // TODO: 描画処理
    }
}

#[macroquad::main("Terra Lock")]
async fn main() {
    // 画面サイズ設定（800x600px）
    request_new_screen_size(800.0, 600.0);
    
    // ゲーム状態の初期化
    let mut game = Game::new();
    
    // FPS計測用変数
    let mut frame_count = 0;
    let mut last_time = get_time();
    let mut fps_display = 60.0;
    
    loop {
        clear_background(BLACK);
        
        // FPS計算と表示
        frame_count += 1;
        let current_time = get_time();
        if current_time - last_time >= 1.0 {
            fps_display = frame_count as f64 / (current_time - last_time);
            frame_count = 0;
            last_time = current_time;
        }
        
        // ゲーム更新
        game.update();
        
        // ゲーム描画
        game.draw();
        
        // FPS表示（常時表示で60FPS維持確認）
        draw_text(&format!("FPS: {:.1}", fps_display), 10.0, 30.0, 20.0, WHITE);
        
        // 60FPS維持確認用の色表示（緑=60FPS、黄=45-59FPS、赤=45FPS未満）
        let fps_color = if fps_display >= 60.0 {
            GREEN
        } else if fps_display >= 45.0 {
            YELLOW
        } else {
            RED
        };
        draw_circle(750.0, 30.0, 10.0, fps_color);
        
        next_frame().await;
    }
}
