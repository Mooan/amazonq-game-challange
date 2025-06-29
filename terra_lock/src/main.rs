use macroquad::prelude::*;

#[macroquad::main("Terra Lock")]
async fn main() {
    loop {
        clear_background(BLACK);
        
        // TODO: ゲームロジックの実装
        
        next_frame().await;
    }
}
