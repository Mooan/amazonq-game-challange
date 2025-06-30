// WebAssembly用のエントリポイント
#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

// ゲームロジックモジュール
mod game;

// WebAssembly用のエントリポイント
#[cfg(feature = "web")]
#[wasm_bindgen(start)]
pub fn main_web() {
    // WebAssembly環境でのゲーム開始
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Debug).expect("error initializing log");
    
    wasm_bindgen_futures::spawn_local(async {
        game::main().await;
    });
}

// WebAssembly用のコンソールログ設定
#[cfg(feature = "web")]
extern crate console_error_panic_hook;

#[cfg(feature = "web")]
extern crate console_log;
