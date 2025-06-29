use macroquad::prelude::*;

#[macroquad::main("Terra Lock")]
async fn main() {
    // 画面サイズ設定（800x600px）
    request_new_screen_size(800.0, 600.0);
    
    loop {
        clear_background(BLACK);
        
        // FPS表示（デバッグ用）
        #[cfg(debug_assertions)]
        draw_text(&format!("FPS: {}", get_fps()), 10.0, 30.0, 20.0, WHITE);
        
        // TODO: ゲームロジックの実装
        
        next_frame().await;
    }
}
