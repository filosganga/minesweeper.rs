use log::info;
use macroquad::prelude::*;

mod domain;

#[macroquad::main("Hello World")]
async fn main() {
    env_logger::init();
    info!("Avvio del gioco");

    loop {
        clear_background(WHITE); // sfondo bianco

        let mouse_pos: Vec2 = mouse_position().into();

        draw_text(
            &format!("Hello Minesweeper! {}", mouse_pos), // testo
            20.0,                                         // x
            50.0,                                         // y
            30.0,                                         // font size
            BLACK,                                        // colore
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            info!("Click sinistro in {:?}", mouse_pos);
        }

        next_frame().await; // aggiorna lo schermo
    }
}
