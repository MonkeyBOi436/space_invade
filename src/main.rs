/*
By: Austin Smith
Date: 2026-10-08
Program Details: Space Invaders clone
*/

mod ui;
mod utils;
mod menu;
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
    let last_switch = get_time() - 0.02;
    let mut score = 0;
    let mut high_score = 0;



    loop {
        if get_time() - last_switch > 0.01 {
          (current_screen, score, high_score) = match current_screen.as_str() {
                "menu" => menu::run(score, high_score).await,
                "game" => game::run(score, high_score).await,
                "lose" => lose::run(score, high_score).await,
                "end" => break,
                _ => break,
            };
        }
        next_frame().await;

    }
}



