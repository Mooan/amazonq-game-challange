// ネイティブ環境用のエントリポイント
mod game;

#[macroquad::main("Terra Lock")]
async fn main() {
    game::main().await;
}
