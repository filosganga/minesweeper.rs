use log::info;
use macroquad::prelude::*;

mod domain;

use crate::domain::*;

const SPRITES_BYTES: &[u8] = include_bytes!("assets/sprites.png");
const FONT_SIZE: f32 = 64.0;
const TILE_SIZE: u32 = 64;

fn window_conf() -> Conf {
    Conf {
        window_title: "Minesweeper".to_owned(),
        window_width: 16 * TILE_SIZE as i32,  // width in pixels
        window_height: 16 * TILE_SIZE as i32, // height in pixels
        ..Default::default()
    }
}

fn draw_text_centered(
    text: &str,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    font_size: f32,
    color: Color,
) {
    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let text_x = x + (width - text_dimensions.width) / 2.0;
    let text_y = y + (height + text_dimensions.height) / 2.0; // y is baseline, so add height/2
    draw_text(text, text_x, text_y, font_size, color);
}

/// x 0 to 64 is 0, x 65 to 128 is 2, ....
fn screen_point_to_tile_index(point: Vec2) -> (u8, u8) {
    let x = point.x / TILE_SIZE as f32;
    let y = point.y / TILE_SIZE as f32;

    (x as u8, y as u8)
}

#[macroquad::main(window_conf)]
async fn main() {
    env_logger::init();
    info!("Avvio del gioco");

    let sprites = Image::from_file_with_format(SPRITES_BYTES, Some(ImageFormat::Png)).unwrap();
    let sprites_texture = Texture2D::from_image(&sprites);
    sprites_texture.set_filter(FilterMode::Nearest); // optional, for pixel-art

    let mine_rect = Rect::new(0.0, 0.0, TILE_SIZE as f32, TILE_SIZE as f32);
    let hidden_rect = Rect::new(TILE_SIZE as f32, 0.0, TILE_SIZE as f32, TILE_SIZE as f32);
    let flag_rect = Rect::new(0.0, TILE_SIZE as f32, TILE_SIZE as f32, TILE_SIZE as f32);
    let empty_rect = Rect::new(
        TILE_SIZE as f32,
        TILE_SIZE as f32,
        TILE_SIZE as f32,
        TILE_SIZE as f32,
    );

    let minefield = Minefield::random(16, 16, 0.2);
    let mut game_state = GameState::new(minefield);

    loop {
        clear_background(WHITE);

        let mouse_pos: Vec2 = mouse_position().into();

        for i in 0..game_state.minefield.h_size {
            for j in 0..game_state.minefield.v_size {
                let tile = &game_state.minefield[(i, j)];
                let tile_state = &game_state[(i, j)];

                let x = i as f32 * TILE_SIZE as f32;
                let y = j as f32 * TILE_SIZE as f32;

                let rect = if *tile_state == TileState::Hidden {
                    hidden_rect
                } else if *tile_state == TileState::Flagged {
                    flag_rect
                } else if *tile == Tile::Mine {
                    mine_rect
                } else {
                    empty_rect
                };

                draw_texture_ex(
                    &sprites_texture,
                    x,
                    y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)), // output size
                        source: Some(rect),
                        ..Default::default()
                    },
                );

                if *tile_state == TileState::Revealed {
                    if let Tile::Adjacent { no_of_mines } = *tile {
                        draw_text_centered(
                            &format!("{}", no_of_mines),
                            x,
                            y,
                            TILE_SIZE as f32,
                            TILE_SIZE as f32,
                            FONT_SIZE,
                            Color::from_rgba(74, 74, 74, 255),
                        )
                    }
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let (tile_x, tile_y) = screen_point_to_tile_index(mouse_pos);

            if tile_x < game_state.minefield.h_size && tile_y < game_state.minefield.v_size {
                let tile_state = &mut game_state[(tile_x, tile_y)];
                if *tile_state == TileState::Flagged {
                    *tile_state = TileState::Hidden;
                } else if *tile_state == TileState::Hidden {
                    *tile_state = TileState::Flagged;
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (tile_x, tile_y) = screen_point_to_tile_index(mouse_pos);

            if tile_x < game_state.minefield.h_size && tile_y < game_state.minefield.v_size {
                let tile_state = &mut game_state[(tile_x, tile_y)];
                if *tile_state == TileState::Hidden {
                    *tile_state = TileState::Revealed;
                }
            }
        }

        next_frame().await; // aggiorna lo schermo
    }
}
