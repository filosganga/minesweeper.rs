use log::info;
use macroquad::prelude::*;

#[macroquad::main("Hello World")]
async fn main() {
    env_logger::init();
    info!("Avvio del gioco");

    loop {
        clear_background(WHITE); // sfondo bianco

        draw_text(
            "Hello Minesweeper!", // testo
            20.0,                 // x
            50.0,                 // y
            30.0,                 // font size
            BLACK,                // colore
        );

        next_frame().await; // aggiorna lo schermo
    }
}
