#![allow(dead_code)]

mod action;
mod audio;
mod enemy;
mod game;
mod hud;
mod player;
mod projectile;
mod render;
mod spawner;
mod upgrades;
mod util;
mod xp;

pub use game::RequiemGame;

pub fn run_game() {
    engine::run::<RequiemGame>();
}

#[cfg(target_arch = "wasm32")]
mod wasm_entry {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn wasm_main() {
        engine::run_wasm::<super::RequiemGame>();
    }
}
