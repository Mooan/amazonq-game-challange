use macroquad::prelude::*;

#[macroquad::main("Terra Lock")]
async fn main() {
    // 画面サイズ設定（800x600px）
    request_new_screen_size(800.0, 600.0);
    
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
        
        // TODO: ゲームロジックの実装
        
        next_frame().await;
    }
}
