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

// 入力状態管理構造体
#[derive(Debug, Clone)]
struct InputState {
    mouse_pos: Vec2,
    left_button_pressed: bool,
    left_button_just_pressed: bool,
    left_button_just_released: bool,
    left_button_hold_time: f32,
}

impl InputState {
    fn new() -> Self {
        Self {
            mouse_pos: Vec2::ZERO,
            left_button_pressed: false,
            left_button_just_pressed: false,
            left_button_just_released: false,
            left_button_hold_time: 0.0,
        }
    }
    
    fn update(&mut self, delta_time: f32) {
        // マウス座標更新
        self.mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);
        
        // マウス左ボタン状態取得
        let current_pressed = is_mouse_button_down(MouseButton::Left);
        
        // 押下・リリース検出
        self.left_button_just_pressed = !self.left_button_pressed && current_pressed;
        self.left_button_just_released = self.left_button_pressed && !current_pressed;
        
        // 長押し時間更新
        if current_pressed {
            self.left_button_hold_time += delta_time;
        } else {
            self.left_button_hold_time = 0.0;
        }
        
        self.left_button_pressed = current_pressed;
    }
    
    fn is_long_press(&self) -> bool {
        self.left_button_hold_time >= 0.2 // 0.2秒以上で長押し判定
    }
}
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
    input: InputState,
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
            input: InputState::new(),
        }
    }
    
    fn update(&mut self, delta_time: f32) {
        // 入力状態更新
        self.input.update(delta_time);
        
        // TODO: マウス入力処理の実装
        // TODO: ゲームロジックの更新
    }
    
    fn draw(&self) {
        // 基本図形描画機能の確認
        // 円の描画（敵機用）
        draw_circle(200.0, 200.0, 10.0, RED);
        draw_circle_lines(200.0, 200.0, 15.0, 2.0, Color::new(0.5, 0.0, 0.0, 1.0)); // DARKRED
        
        // 三角形の描画（自機用）
        let player_pos = Vec2::new(400.0, 500.0);
        draw_triangle(
            Vec2::new(player_pos.x, player_pos.y - 10.0),
            Vec2::new(player_pos.x - 10.0, player_pos.y + 5.0),
            Vec2::new(player_pos.x + 10.0, player_pos.y + 5.0),
            BLUE
        );
        
        // 線の描画（レーザー用）
        draw_line(100.0, 300.0, 100.0, 250.0, 2.0, Color::new(0.0, 1.0, 1.0, 1.0)); // CYAN
        draw_line(120.0, 300.0, 150.0, 200.0, 3.0, YELLOW);
        
        // 矩形の描画（UI用）
        draw_rectangle_lines(50.0, 50.0, 200.0, 100.0, 2.0, WHITE);
        draw_rectangle(60.0, 60.0, 180.0, 80.0, Color::new(0.2, 0.2, 0.2, 0.8));
        
        // 図形描画確認テキスト
        draw_text("Basic Shapes Test:", 300.0, 300.0, 16.0, WHITE);
        draw_text("Red Circle (Enemy)", 300.0, 320.0, 14.0, RED);
        draw_text("Blue Triangle (Player)", 300.0, 340.0, 14.0, BLUE);
        draw_text("Cyan/Yellow Lines (Lasers)", 300.0, 360.0, 14.0, Color::new(0.0, 1.0, 1.0, 1.0)); // CYAN
        draw_text("White Rectangle (UI)", 300.0, 380.0, 14.0, WHITE);
        
        // Vec2数学ライブラリの動作確認
        let test_vec1 = Vec2::new(100.0, 50.0);
        let test_vec2 = Vec2::new(50.0, 25.0);
        let vec_sum = test_vec1 + test_vec2;
        let vec_distance = test_vec1.distance(test_vec2);
        let vec_length = test_vec1.length();
        
        // Vec2演算結果表示（デバッグ用）
        draw_text(
            &format!("Vec2 Test: ({:.0}, {:.0}) + ({:.0}, {:.0}) = ({:.0}, {:.0})", 
                     test_vec1.x, test_vec1.y, test_vec2.x, test_vec2.y, vec_sum.x, vec_sum.y),
            10.0, 130.0, 14.0, LIGHTGRAY
        );
        draw_text(
            &format!("Distance: {:.1}, Length: {:.1}", vec_distance, vec_length),
            10.0, 150.0, 14.0, LIGHTGRAY
        );
        
        // マウス座標表示（デバッグ用）
        draw_text(
            &format!("Mouse: ({:.0}, {:.0})", self.input.mouse_pos.x, self.input.mouse_pos.y),
            10.0, 50.0, 16.0, GRAY
        );
        
        // マウスボタン状態表示（デバッグ用）
        let button_status = if self.input.left_button_pressed {
            if self.input.is_long_press() {
                format!("Left Button: LONG PRESS ({:.1}s)", self.input.left_button_hold_time)
            } else {
                format!("Left Button: PRESSED ({:.1}s)", self.input.left_button_hold_time)
            }
        } else {
            "Left Button: RELEASED".to_string()
        };
        draw_text(&button_status, 10.0, 70.0, 16.0, YELLOW);
        
        // 押下・リリース瞬間表示
        if self.input.left_button_just_pressed {
            draw_text("JUST PRESSED!", 10.0, 90.0, 16.0, GREEN);
        }
        if self.input.left_button_just_released {
            draw_text("JUST RELEASED!", 10.0, 110.0, 16.0, RED);
        }
        
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
    let mut last_frame_time = get_time();
    
    loop {
        clear_background(BLACK);
        
        // デルタタイム計算
        let current_time = get_time();
        let delta_time = (current_time - last_frame_time) as f32;
        last_frame_time = current_time;
        
        // FPS計算と表示
        frame_count += 1;
        if current_time - last_time >= 1.0 {
            fps_display = frame_count as f64 / (current_time - last_time);
            frame_count = 0;
            last_time = current_time;
        }
        
        // ゲーム更新
        game.update(delta_time);
        
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
