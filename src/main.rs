/*
By: Austin Smith
Date: 2026-10-08
Program Details: Space Invaders clone
*/

mod ui;
mod utils;
mod menu;
mod tutorial;
mod game;
mod lose;
mod custom;


use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "space_invade".to_string(),
        window_width: 1624,
        window_height: 968,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
    
async fn main() {
    let mut current_screen = "menu".to_string();
    let mut last_switch = get_time() - 0.02;

    loop {
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "menu" => menu::run().await,
                "tutorial" => tutorial::run().await,
                "game" => game::run().await,
                "lose" => lose::run().await,
                "end" => break,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}

