use macroquad::prelude::*;
use macroquad::rand::gen_range;

#[derive(Clone, Debug)]
enum GameState {
    Playing,
    GameOver,
}

// プレイヤー構造体
#[derive(Clone, Debug)]
struct Player {
    position: Vec2,
    health: u8,
}

impl Player {
    fn new() -> Self {
        Self {
            position: Vec2::new(400.0, 500.0),
            health: 1,
        }
    }
}

// 敵機構造体
#[derive(Clone, Debug)]
struct Enemy {
    position: Vec2,
    velocity: Vec2,
    is_locked: bool,
    lock_timer: f32,
}

// 通常レーザー構造体
#[derive(Clone, Debug)]
struct NormalLaser {
    position: Vec2,
    velocity: Vec2,
    lifetime: f32,
}

// ロックオンレーザー構造体
#[derive(Clone, Debug)]
struct LockOnLaser {
    start_pos: Vec2,
    target_pos: Vec2,
    progress: f32,
    speed: f32,
}

// 入力状態管理
#[derive(Debug)]
struct InputState {
    mouse_pos: Vec2,
    left_button_pressed: bool,
    left_button_just_pressed: bool,
    left_button_just_released: bool,
    left_button_hold_time: f32,
    prev_left_button_pressed: bool,
}

impl InputState {
    fn new() -> Self {
        Self {
            mouse_pos: Vec2::ZERO,
            left_button_pressed: false,
            left_button_just_pressed: false,
            left_button_just_released: false,
            left_button_hold_time: 0.0,
            prev_left_button_pressed: false,
        }
    }
    
    fn update(&mut self, delta_time: f32) {
        // マウス座標取得
        self.mouse_pos = mouse_position().into();
        
        // マウスボタン状態取得
        let current_pressed = is_mouse_button_down(MouseButton::Left);
        
        // ボタン状態の変化を検出
        self.left_button_just_pressed = current_pressed && !self.prev_left_button_pressed;
        self.left_button_just_released = !current_pressed && self.prev_left_button_pressed;
        self.left_button_pressed = current_pressed;
        
        // 長押し時間の更新
        if self.left_button_pressed {
            self.left_button_hold_time += delta_time;
        } else {
            self.left_button_hold_time = 0.0;
        }
        
        // 前フレームの状態を保存
        self.prev_left_button_pressed = current_pressed;
    }
    
    fn is_long_press(&self) -> bool {
        self.left_button_hold_time >= 0.2 // 0.2秒以上で長押し判定
    }
}

// ロックオンシステム
#[derive(Clone, Debug)]
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

// メインゲーム構造体
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
    enemy_spawn_timer: f32,
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            state: GameState::Playing,
            player: Player::new(),
            enemies: Vec::new(),
            normal_lasers: Vec::new(),
            lock_on_lasers: Vec::new(),
            lock_system: LockOnSystem::new(),
            score: 0,
            input: InputState::new(),
            enemy_spawn_timer: 0.0,
        };
        
        // テスト用敵機を追加（描画確認用）
        game.enemies.push(Enemy {
            position: Vec2::new(200.0, 100.0),
            velocity: Vec2::new(0.0, 120.0), // 120px/秒で下向き
            is_locked: false,
            lock_timer: 0.0,
        });
        
        game
    }
    
    fn update(&mut self, delta_time: f32) {
        // 入力状態更新
        self.input.update(delta_time);
        
        // プレイヤーの位置をマウス座標に更新（画面内制限付き）
        let screen_width = 800.0;
        let screen_height = 600.0;
        let player_half_width = 10.0;  // 自機の半分の幅（20px / 2）
        let player_half_height = 7.5;  // 自機の半分の高さ（15px / 2）
        
        // マウス座標を画面内に制限
        let clamped_x = self.input.mouse_pos.x.clamp(
            player_half_width, 
            screen_width - player_half_width
        );
        let clamped_y = self.input.mouse_pos.y.clamp(
            player_half_height, 
            screen_height - player_half_height
        );
        
        self.player.position = Vec2::new(clamped_x, clamped_y);
        
        // 通常レーザーの発射（左クリック）
        if self.input.left_button_just_pressed {
            self.fire_normal_laser();
        }
        
        // 敵機出現システム
        self.enemy_spawn_timer += delta_time;
        if self.enemy_spawn_timer >= 3.0 { // 3秒間隔で出現
            self.spawn_enemy();
            self.enemy_spawn_timer = 0.0;
        }
        
        // 通常レーザーの更新
        for laser in &mut self.normal_lasers {
            laser.position += laser.velocity * delta_time;
            laser.lifetime -= delta_time;
        }
        
        // 寿命切れまたは画面外のレーザーを削除
        self.normal_lasers.retain(|laser| laser.lifetime > 0.0 && laser.position.y > -50.0);
        
        // 敵機の更新
        for enemy in &mut self.enemies {
            enemy.position += enemy.velocity * delta_time;
        }
        
        // 画面外の敵機を削除
        self.enemies.retain(|enemy| enemy.position.y < screen_height + 50.0);
        
        // TODO: その他のゲームロジックの更新
    }
    
    fn fire_normal_laser(&mut self) {
        // プレイヤーの位置から上向きにレーザーを発射
        self.normal_lasers.push(NormalLaser {
            position: self.player.position,
            velocity: Vec2::new(0.0, -300.0), // 300px/秒で上向き
            lifetime: 3.0, // 3秒間の寿命
        });
    }
    
    fn spawn_enemy(&mut self) {
        let screen_width = 800.0;
        let enemy_radius = 10.0;
        
        // 画面上部のランダムな位置に敵機を生成
        let x = gen_range(enemy_radius, screen_width - enemy_radius);
        let y = -enemy_radius; // 画面上部の少し外側から出現
        
        self.enemies.push(Enemy {
            position: Vec2::new(x, y),
            velocity: Vec2::new(0.0, 120.0), // 120px/秒で下向き移動
            is_locked: false,
            lock_timer: 0.0,
        });
    }
    
    fn draw(&self) {
        // プレイヤー（自機）の描画 - 青い三角形（20x15px）
        let player_pos = self.player.position;
        let width = 20.0;   // 幅20px
        let height = 15.0;  // 高さ15px
        let vertices = [
            Vec2::new(player_pos.x, player_pos.y - height / 2.0),                    // 上頂点
            Vec2::new(player_pos.x - width / 2.0, player_pos.y + height / 2.0),     // 左下
            Vec2::new(player_pos.x + width / 2.0, player_pos.y + height / 2.0),     // 右下
        ];
        draw_triangle(vertices[0], vertices[1], vertices[2], BLUE);
        
        // 敵機の描画 - 赤い円（直径20px）
        for enemy in &self.enemies {
            draw_circle(enemy.position.x, enemy.position.y, 10.0, RED);
        }
        
        // 通常レーザーの描画 - シアンの線（幅3px）
        for laser in &self.normal_lasers {
            let laser_length = 15.0; // レーザーの長さ
            draw_line(
                laser.position.x, 
                laser.position.y - laser_length / 2.0,
                laser.position.x, 
                laser.position.y + laser_length / 2.0,
                3.0, 
                Color::new(0.0, 1.0, 1.0, 1.0) // CYAN
            );
        }
        
        // UI表示
        draw_text("SCORE: 0", 20.0, 30.0, 20.0, WHITE);
        draw_text("LOCK: 0/6", 20.0, 55.0, 16.0, YELLOW);
    }
    
    fn draw_debug_info(&self, fps: f32) {
        // FPS表示（パフォーマンス監視）
        let fps_color = if fps >= 60.0 { GREEN } else if fps >= 45.0 { YELLOW } else { RED };
        draw_text(&format!("FPS: {:.1}", fps), screen_width() - 100.0, 30.0, 20.0, fps_color);
        
        // マウス座標表示
        draw_text(
            &format!("Mouse: ({:.0}, {:.0})", self.input.mouse_pos.x, self.input.mouse_pos.y),
            screen_width() - 200.0, 55.0, 16.0, WHITE
        );
        
        // マウスボタン状態表示
        let button_status = if self.input.left_button_pressed {
            if self.input.is_long_press() {
                format!("LONG PRESS ({:.1}s)", self.input.left_button_hold_time)
            } else {
                "PRESSED".to_string()
            }
        } else {
            "RELEASED".to_string()
        };
        
        let button_color = if self.input.left_button_pressed {
            if self.input.is_long_press() { ORANGE } else { GREEN }
        } else { WHITE };
        
        draw_text(
            &format!("Button: {}", button_status),
            screen_width() - 250.0, 80.0, 16.0, button_color
        );
    }
}

// WebAssembly対応のメイン関数
pub async fn main() {
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
        
        // ゲーム更新
        game.update(delta_time);
        
        // ゲーム描画
        game.draw();
        
        // FPS計算と表示
        frame_count += 1;
        if current_time - last_time >= 1.0 {
            fps_display = frame_count as f32 / (current_time - last_time) as f32;
            frame_count = 0;
            last_time = current_time;
        }
        
        // デバッグ情報表示
        game.draw_debug_info(fps_display);
        
        next_frame().await;
    }
}
