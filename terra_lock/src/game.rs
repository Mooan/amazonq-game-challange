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
    target_enemy_id: Option<usize>, // 追跡対象の敵機ID
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
    
    // ロックオン解除システム
    fn clear_all_locks(&mut self, enemies: &mut [Enemy]) {
        // 全ての敵機のロックオン状態をクリア
        for &enemy_idx in &self.locked_enemies {
            if enemy_idx < enemies.len() {
                enemies[enemy_idx].is_locked = false;
            }
        }
        
        // ロックオンリストをクリア
        self.locked_enemies.clear();
        
        // ワイヤーフレームを非表示
        self.active = false;
        
        println!("All lock-on targets cleared");
    }
    
    // ワイヤーフレーム外に移動した敵機の解除
    fn remove_out_of_range_targets(&mut self, enemies: &mut [Enemy]) {
        let mut removed_count = 0;
        
        // 後ろから削除して、インデックスのずれを防ぐ
        for i in (0..self.locked_enemies.len()).rev() {
            let enemy_idx = self.locked_enemies[i];
            
            if enemy_idx < enemies.len() {
                let enemy = &enemies[enemy_idx];
                
                // 距離計算（平方根回避最適化）
                let distance_squared = (enemy.position.x - self.center.x).powi(2)
                                     + (enemy.position.y - self.center.y).powi(2);
                let radius_squared = self.radius.powi(2);
                
                // ワイヤーフレーム外に移動した場合
                if distance_squared > radius_squared {
                    // 敵機のロックオン状態を解除
                    enemies[enemy_idx].is_locked = false;
                    
                    // ロックオンリストから削除
                    self.locked_enemies.remove(i);
                    removed_count += 1;
                }
            }
        }
        
        if removed_count > 0 {
            println!("Removed {} targets that moved out of wireframe", removed_count);
        }
    }
    
    // 撃破された敵機のロックオン解除
    fn remove_destroyed_enemies(&mut self, destroyed_indices: &[usize]) {
        if destroyed_indices.is_empty() {
            return;
        }
        
        let mut removed_count = 0;
        
        // 後ろから削除して、インデックスのずれを防ぐ
        for i in (0..self.locked_enemies.len()).rev() {
            let enemy_idx = self.locked_enemies[i];
            
            // 撃破された敵機のインデックスと一致するかチェック
            if destroyed_indices.contains(&enemy_idx) {
                self.locked_enemies.remove(i);
                removed_count += 1;
            }
        }
        
        // インデックス調整：撃破された敵機より後ろの敵機のインデックスを調整
        for destroyed_idx in destroyed_indices.iter().rev() {
            for locked_idx in &mut self.locked_enemies {
                if *locked_idx > *destroyed_idx {
                    *locked_idx -= 1;
                }
            }
        }
        
        if removed_count > 0 {
            println!("Removed {} destroyed enemies from lock-on list", removed_count);
        }
    }
}

// メインゲーム構造体
#[derive(Debug, Clone)]
struct BonusDisplay {
    text: String,
    position: Vec2,
    timer: f32,
    max_time: f32,
}

impl BonusDisplay {
    fn new(text: String, position: Vec2) -> Self {
        Self {
            text,
            position,
            timer: 0.0,
            max_time: 2.0, // 2秒間表示
        }
    }
    
    fn update(&mut self, delta_time: f32) {
        self.timer += delta_time;
        // フェードアウト効果のために上に移動
        self.position.y -= 30.0 * delta_time;
    }
    
    fn is_expired(&self) -> bool {
        self.timer >= self.max_time
    }
    
    fn get_alpha(&self) -> f32 {
        // フェードアウト効果
        (1.0 - (self.timer / self.max_time)).max(0.0)
    }
}

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
    bonus_displays: Vec<BonusDisplay>,
}

impl Game {
    // ホーミングレーザーのターゲットID調整
    fn update_homing_laser_targets(&mut self, destroyed_indices: &[usize]) {
        for laser in &mut self.lock_on_lasers {
            if let Some(target_id) = laser.target_enemy_id {
                // 撃破された敵機をターゲットにしている場合、IDをクリア
                if destroyed_indices.contains(&target_id) {
                    laser.target_enemy_id = None;
                } else {
                    // インデックス調整：撃破された敵機より後ろの敵機のIDを調整
                    let mut adjusted_id = target_id;
                    for &destroyed_idx in destroyed_indices.iter().rev() {
                        if adjusted_id > destroyed_idx {
                            adjusted_id -= 1;
                        }
                    }
                    laser.target_enemy_id = Some(adjusted_id);
                }
            }
        }
    }
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
            bonus_displays: Vec::new(),
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
        // ゲーム状態に応じた処理分岐
        match self.state {
            GameState::Playing => {
                self.update_playing(delta_time);
            }
            GameState::GameOver => {
                // ゲームオーバー状態でのリスタート処理
                self.input.update(delta_time);
                if self.input.left_button_just_pressed {
                    self.restart_game();
                }
            }
        }
    }
    
    fn restart_game(&mut self) {
        // ゲーム状態をリセット
        self.state = GameState::Playing;
        self.score = 0;
        self.enemy_spawn_timer = 0.0;
        
        // プレイヤーを初期位置に戻す
        self.player.position = Vec2::new(400.0, 500.0);
        
        // 全てのオブジェクトをクリア
        self.enemies.clear();
        self.normal_lasers.clear();
        self.lock_on_lasers.clear();
        self.bonus_displays.clear();
        
        println!("Game Restarted!");
    }
    
    fn update_playing(&mut self, delta_time: f32) {
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
        
        // ロックオンシステムの更新
        self.update_lock_on_system();
        
        // 通常レーザーの発射（左クリック短押し）
        if self.input.left_button_just_pressed && !self.lock_system.active {
            self.fire_normal_laser();
        }
        
        // 敵機出現システム（ロックオン動作確認用に強化）
        self.enemy_spawn_timer += delta_time;
        if self.enemy_spawn_timer >= 1.5 { // 1.5秒間隔で出現（短縮）
            // 30%の確率で2機同時出現、10%の確率で3機同時出現
            let spawn_count = if gen_range(0.0, 1.0) < 0.1 {
                3 // 10%の確率で3機
            } else if gen_range(0.0, 1.0) < 0.3 {
                2 // 30%の確率で2機
            } else {
                1 // 60%の確率で1機
            };
            
            for _ in 0..spawn_count {
                self.spawn_enemy();
            }
            self.enemy_spawn_timer = 0.0;
        }
        
        // 通常レーザーの更新
        for laser in &mut self.normal_lasers {
            laser.position += laser.velocity * delta_time;
            laser.lifetime -= delta_time;
        }
        
        // 寿命切れまたは画面外のレーザーを削除
        self.normal_lasers.retain(|laser| laser.lifetime > 0.0 && laser.position.y > -50.0);
        
        // ホーミングレーザーの更新（動的ターゲット追跡）
        for laser in &mut self.lock_on_lasers {
            // 対象敵機が存在する場合、ターゲット位置を更新
            if let Some(enemy_id) = laser.target_enemy_id {
                if enemy_id < self.enemies.len() {
                    // 敵機の現在位置にターゲットを更新
                    laser.target_pos = self.enemies[enemy_id].position;
                } else {
                    // 対象敵機が削除された場合、IDをクリア
                    laser.target_enemy_id = None;
                }
            }
            
            laser.progress += laser.speed * delta_time / 1000.0; // 進行度を0-1で管理
            laser.progress = laser.progress.min(1.0);
        }
        
        // 完了したホーミングレーザーのスコア加算と削除
        let completed_lasers = self.lock_on_lasers.iter().filter(|laser| laser.progress >= 1.0).count();
        if completed_lasers > 0 {
            // ロックオンレーザー撃破スコア（200点 × 完了数）
            let base_score = completed_lasers as u32 * 200;
            self.score += base_score;
            
            // 同時ロックオン撃破ボーナス計算
            let bonus_score = match completed_lasers {
                2 => 300,
                3 => 600,
                4 => 1000,
                5 => 1500,
                6 => 2100,
                _ => 0,
            };
            
            if bonus_score > 0 {
                self.score += bonus_score;
                
                // ボーナススコア表示を追加
                let bonus_text = format!("BONUS +{}", bonus_score);
                let display_pos = Vec2::new(400.0, 300.0); // 画面中央
                self.bonus_displays.push(BonusDisplay::new(bonus_text, display_pos));
                
                println!("Lock-on laser hits: {} enemies, +{} points (base) + {} points (bonus) = {} total", 
                         completed_lasers, base_score, bonus_score, base_score + bonus_score);
            } else {
                println!("Lock-on laser hits: {} enemies, +{} points", completed_lasers, base_score);
            }
        }
        
        self.lock_on_lasers.retain(|laser| laser.progress < 1.0);
        
        // 敵機の更新
        for enemy in &mut self.enemies {
            enemy.position += enemy.velocity * delta_time;
        }
        
        // レーザーと敵機の当たり判定
        self.check_laser_enemy_collision();
        
        // 敵機と自機の当たり判定
        self.check_player_enemy_collision();
        
        // 画面外の敵機を削除
        self.enemies.retain(|enemy| enemy.position.y < screen_height + 50.0);
        
        // ボーナス表示の更新
        for bonus_display in &mut self.bonus_displays {
            bonus_display.update(delta_time);
        }
        
        // 期限切れのボーナス表示を削除
        self.bonus_displays.retain(|display| !display.is_expired());
        
        // TODO: その他のゲームロジックの更新
    }
    
    fn update_lock_on_system(&mut self) {
        // マウス長押し検出でワイヤーフレーム展開
        if self.input.is_long_press() {
            self.lock_system.active = true;
            self.lock_system.center = self.input.mouse_pos;
            
            // ワイヤーフレーム内の敵機検出
            self.detect_enemies_in_wireframe();
            
            // ワイヤーフレーム外に移動した敵機の解除
            self.lock_system.remove_out_of_range_targets(&mut self.enemies);
        } else if self.input.left_button_just_released && self.lock_system.active {
            // マウスボタンリリース時の処理
            if !self.lock_system.locked_enemies.is_empty() {
                // ロックオン対象がある場合は一斉発射
                self.fire_lock_on_lasers();
            }
            
            // マウスボタンリリース時の完全解除
            self.lock_system.clear_all_locks(&mut self.enemies);
        } else if !self.input.left_button_pressed {
            // マウスボタンが押されていない場合も解除
            if self.lock_system.active {
                self.lock_system.clear_all_locks(&mut self.enemies);
            }
        }
    }
    
    fn fire_lock_on_lasers(&mut self) {
        let player_pos = self.player.position;
        
        for &enemy_idx in &self.lock_system.locked_enemies {
            if enemy_idx < self.enemies.len() {
                let target_pos = self.enemies[enemy_idx].position;
                
                self.lock_on_lasers.push(LockOnLaser {
                    start_pos: player_pos,
                    target_pos,
                    target_enemy_id: Some(enemy_idx), // 敵機IDを設定
                    progress: 0.0,
                    speed: 400.0, // 400px/秒でホーミング
                });
            }
        }
        
        println!("Fired {} lock-on lasers!", self.lock_system.locked_enemies.len());
    }
    
    fn detect_enemies_in_wireframe(&mut self) {
        // 全ての敵機のロックオン状態をリセット
        for enemy in &mut self.enemies {
            enemy.is_locked = false;
        }
        
        self.lock_system.locked_enemies.clear();
        
        for (enemy_idx, enemy) in self.enemies.iter_mut().enumerate() {
            // 距離計算による判定（平方根回避最適化）
            let distance_squared = (enemy.position.x - self.lock_system.center.x).powi(2)
                                 + (enemy.position.y - self.lock_system.center.y).powi(2);
            let radius_squared = self.lock_system.radius.powi(2);
            
            if distance_squared <= radius_squared {
                // 最大6機までのロックオン制限
                if self.lock_system.locked_enemies.len() < self.lock_system.max_targets as usize {
                    self.lock_system.locked_enemies.push(enemy_idx);
                    enemy.is_locked = true;
                }
            }
        }
    }
    
    fn check_laser_enemy_collision(&mut self) {
        let mut lasers_to_remove = Vec::new();
        let mut enemies_to_remove = Vec::new();
        
        for (laser_idx, laser) in self.normal_lasers.iter().enumerate() {
            for (enemy_idx, enemy) in self.enemies.iter().enumerate() {
                // 円と点の当たり判定（レーザーは点、敵機は半径10pxの円）
                let distance_squared = (laser.position.x - enemy.position.x).powi(2) 
                                     + (laser.position.y - enemy.position.y).powi(2);
                let enemy_radius: f32 = 10.0;
                
                if distance_squared <= enemy_radius.powi(2) {
                    // 当たり判定発生
                    lasers_to_remove.push(laser_idx);
                    enemies_to_remove.push(enemy_idx);
                    
                    // 通常レーザー撃破時のスコア加算（100点）
                    self.score += 100;
                    
                    break; // このレーザーは1つの敵にのみ当たる
                }
            }
        }
        
        // 重複を除去してソート（逆順で削除）
        lasers_to_remove.sort_unstable();
        lasers_to_remove.dedup();
        enemies_to_remove.sort_unstable();
        enemies_to_remove.dedup();
        
        // 撃破された敵機のロックオン解除（敵機削除前に実行）
        if !enemies_to_remove.is_empty() {
            self.lock_system.remove_destroyed_enemies(&enemies_to_remove);
            // ホーミングレーザーのターゲットID調整
            self.update_homing_laser_targets(&enemies_to_remove);
        }
        
        // 逆順で削除（インデックスのずれを防ぐ）
        for &idx in lasers_to_remove.iter().rev() {
            if idx < self.normal_lasers.len() {
                self.normal_lasers.remove(idx);
            }
        }
        
        for &idx in enemies_to_remove.iter().rev() {
            if idx < self.enemies.len() {
                self.enemies.remove(idx);
            }
        }
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
        // 既存の敵機がある場合は、近い位置に配置する確率を上げる
        let x = if !self.enemies.is_empty() && gen_range(0.0, 1.0) < 0.4 {
            // 40%の確率で既存の敵機の近くに配置
            let existing_enemy = &self.enemies[gen_range(0, self.enemies.len())];
            let offset = gen_range(-100.0, 100.0);
            (existing_enemy.position.x + offset).clamp(enemy_radius, screen_width - enemy_radius)
        } else {
            // 通常のランダム配置
            gen_range(enemy_radius, screen_width - enemy_radius)
        };
        
        let y = -enemy_radius; // 画面上部の少し外側から出現
        
        self.enemies.push(Enemy {
            position: Vec2::new(x, y),
            velocity: Vec2::new(0.0, 120.0), // 120px/秒で下向き移動
            is_locked: false,
            lock_timer: 0.0,
        });
    }
    
    fn draw(&self) {
        // ゲーム状態に応じた描画処理
        match self.state {
            GameState::Playing => {
                self.draw_playing();
            }
            GameState::GameOver => {
                self.draw_game_over();
            }
        }
    }
    
    fn draw_playing(&self) {
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
        
        // 敵機の描画 - 赤い円（直径20px）、ロックオン時は黄色
        for enemy in &self.enemies {
            let color = if enemy.is_locked { YELLOW } else { RED };
            draw_circle(enemy.position.x, enemy.position.y, 10.0, color);
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
        
        // ホーミングレーザーの描画 - 黄色い追尾線
        for laser in &self.lock_on_lasers {
            // ベジェ曲線による軌道計算
            let current_pos = self.calculate_homing_position(laser);
            
            // レーザーの軌跡を描画（複数の線分で曲線を表現）
            let segments = 10;
            for i in 0..segments {
                let t1 = (i as f32) / (segments as f32) * laser.progress;
                let t2 = ((i + 1) as f32) / (segments as f32) * laser.progress;
                
                if t2 <= laser.progress {
                    let pos1 = self.calculate_bezier_point(laser.start_pos, laser.target_pos, t1);
                    let pos2 = self.calculate_bezier_point(laser.start_pos, laser.target_pos, t2);
                    
                    draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 2.0, YELLOW);
                }
            }
        }
        
        // ワイヤーフレーム描画（ロックオンシステム）
        if self.lock_system.active {
            self.draw_wireframe();
        }
        
        // UI表示
        draw_text(&format!("SCORE: {}", self.score), 20.0, 30.0, 20.0, WHITE);
        
        // ロックオン数表示（ロックオン数に応じた色変化）
        let lock_count = self.lock_system.locked_enemies.len();
        let lock_color = match lock_count {
            0 => GRAY,            // ロックオンなし: グレー
            1..=2 => GREEN,       // 1-2機: 緑
            3..=4 => YELLOW,      // 3-4機: 黄
            5..=6 => ORANGE,      // 5-6機: オレンジ
            _ => RED,             // 7機以上: 赤
        };
        draw_text(&format!("LOCK: {}/6", lock_count), 20.0, 55.0, 16.0, lock_color);
        
        // ボーナススコア表示
        for bonus_display in &self.bonus_displays {
            let alpha = bonus_display.get_alpha();
            let color = Color::new(1.0, 1.0, 0.0, alpha); // 黄色でフェードアウト
            draw_text(
                &bonus_display.text,
                bonus_display.position.x - 50.0, // 中央揃え調整
                bonus_display.position.y,
                24.0, // フォントサイズ
                color
            );
        }
    }
    
    fn draw_game_over(&self) {
        // 背景を暗くする
        draw_rectangle(0.0, 0.0, 800.0, 600.0, Color::new(0.0, 0.0, 0.0, 0.7));
        
        // ゲームオーバー表示（中央、赤文字、48px）
        let game_over_text = "GAME OVER";
        let text_width = 48.0 * game_over_text.len() as f32 * 0.6; // 概算幅
        draw_text(
            game_over_text,
            (800.0 - text_width) / 2.0,
            300.0,
            48.0,
            RED
        );
        
        // 最終スコア表示
        let score_text = format!("FINAL SCORE: {}", self.score);
        let score_width = 24.0 * score_text.len() as f32 * 0.6; // 概算幅
        draw_text(
            &score_text,
            (800.0 - score_width) / 2.0,
            350.0,
            24.0,
            WHITE
        );
        
        // リスタート指示
        let restart_text = "Click to Restart";
        let restart_width = 20.0 * restart_text.len() as f32 * 0.6; // 概算幅
        draw_text(
            restart_text,
            (800.0 - restart_width) / 2.0,
            400.0,
            20.0,
            YELLOW
        );
    }
    
    fn draw_wireframe(&self) {
        // ワイヤーフレーム円の描画（点線、ロックオン数に応じた色変化）
        let segments = 32; // 円を32個の線分で描画
        let radius = self.lock_system.radius;
        let center = self.lock_system.center;
        
        // ロックオン数に応じた色変化
        let wireframe_color = match self.lock_system.locked_enemies.len() {
            0 => WHITE,           // ロックオンなし: 白
            1..=2 => GREEN,       // 1-2機: 緑
            3..=4 => YELLOW,      // 3-4機: 黄
            5..=6 => ORANGE,      // 5-6機: オレンジ
            _ => RED,             // 7機以上（通常発生しない）: 赤
        };
        
        for i in 0..segments {
            // 点線効果のため、偶数番目の線分のみ描画
            if i % 2 == 0 {
                let angle1 = (i as f32) * 2.0 * std::f32::consts::PI / (segments as f32);
                let angle2 = ((i + 1) as f32) * 2.0 * std::f32::consts::PI / (segments as f32);
                
                let x1 = center.x + radius * angle1.cos();
                let y1 = center.y + radius * angle1.sin();
                let x2 = center.x + radius * angle2.cos();
                let y2 = center.y + radius * angle2.sin();
                
                draw_line(x1, y1, x2, y2, 2.0, wireframe_color);
            }
        }
        
        // 中心点の描画
        draw_circle(center.x, center.y, 3.0, wireframe_color);
    }
    
    fn calculate_homing_position(&self, laser: &LockOnLaser) -> Vec2 {
        self.calculate_bezier_point(laser.start_pos, laser.target_pos, laser.progress)
    }
    
    fn calculate_bezier_point(&self, start: Vec2, target: Vec2, t: f32) -> Vec2 {
        // 2次ベジェ曲線による軌道計算
        // 制御点は開始点と目標点の中間の上方に設定
        let mid_x = (start.x + target.x) / 2.0;
        let mid_y = (start.y + target.y) / 2.0 - 100.0; // 上方に100px
        let control_point = Vec2::new(mid_x, mid_y);
        
        // ベジェ曲線の計算: B(t) = (1-t)²P₀ + 2(1-t)tP₁ + t²P₂
        let t_inv = 1.0 - t;
        let t_inv_sq = t_inv * t_inv;
        let t_sq = t * t;
        let t_2_inv = 2.0 * t * t_inv;
        
        Vec2::new(
            t_inv_sq * start.x + t_2_inv * control_point.x + t_sq * target.x,
            t_inv_sq * start.y + t_2_inv * control_point.y + t_sq * target.y,
        )
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
    
    fn check_player_enemy_collision(&mut self) {
        let player_half_width = 10.0;  // 自機の半分の幅
        let player_half_height = 7.5;  // 自機の半分の高さ
        let enemy_radius: f32 = 10.0;   // 敵機の半径
        
        for enemy in &self.enemies {
            // 矩形（自機）と円（敵機）の当たり判定
            // 自機の矩形の境界を計算
            let player_left = self.player.position.x - player_half_width;
            let player_right = self.player.position.x + player_half_width;
            let player_top = self.player.position.y - player_half_height;
            let player_bottom = self.player.position.y + player_half_height;
            
            // 敵機の円の中心から自機の矩形への最短距離を計算
            let closest_x = enemy.position.x.clamp(player_left, player_right);
            let closest_y = enemy.position.y.clamp(player_top, player_bottom);
            
            let distance_squared = (enemy.position.x - closest_x).powi(2) 
                                 + (enemy.position.y - closest_y).powi(2);
            
            if distance_squared <= enemy_radius.powi(2) {
                // 自機と敵機が衝突した場合、ゲーム状態をGameOverに変更
                self.state = GameState::GameOver;
                println!("Player hit by enemy! Game Over!");
                break;
            }
        }
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
