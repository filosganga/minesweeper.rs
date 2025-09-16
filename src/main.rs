use std::sync::LazyLock;

use clap::Parser;
use log::info;
use macroquad::prelude::*;

mod domain;

use crate::domain::*;

const SPRITES_BYTES: &[u8] = include_bytes!("assets/sprites.png");
const FONT_SIZE: f32 = 64.0;
const FONT_SIZE_XL: f32 = 80.0;
const TILE_SIZE: u32 = 64;

const TEXT_COLOR: Color = Color::from_rgba(74, 74, 74, 255);

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'x', long, default_value_t = 8_u8)]
    width: u8,

    #[arg(short = 'y', long, default_value_t = 8_u8)]
    height: u8,

    #[arg(short, long, default_value = "0.15")]
    density: f32,
}

static args: LazyLock<Args> = LazyLock::new(|| Args::parse());

fn window_conf() -> Conf {
    Conf {
        window_title: "Minesweeper".to_owned(),
        window_width: args.width as i32 * TILE_SIZE as i32, // width in pixels
        window_height: args.height as i32 * TILE_SIZE as i32, // height in pixels
        window_resizable: false,
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

    let mut minefield = Minefield::random(args.width, args.height, args.density);

    loop {
        // TODO extract render_minefield
        clear_background(WHITE);
        for i in 0..minefield.h_size() {
            for j in 0..minefield.v_size() {
                let tile = &minefield[(i, j)];

                let x = i as f32 * TILE_SIZE as f32;
                let y = j as f32 * TILE_SIZE as f32;

                let rect = if tile.is_hidden() {
                    hidden_rect
                } else if tile.is_flagged() {
                    flag_rect
                } else if tile.is_mine() {
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

                if tile.is_revealed() && tile.is_adjacent() {
                    draw_text_centered(
                        &format!("{}", tile.no_of_adjacent_mine()),
                        x,
                        y,
                        TILE_SIZE as f32,
                        TILE_SIZE as f32,
                        FONT_SIZE,
                        TEXT_COLOR,
                    )
                }
            }
        }

        match minefield.game_status() {
            GameStatus::Lost => draw_text_centered(
                "YOU LOST!",
                0.0,
                0.0,
                screen_width(),
                screen_width(),
                FONT_SIZE_XL,
                RED,
            ),
            GameStatus::Won => draw_text_centered(
                "YOU WON!",
                0.0,
                0.0,
                screen_width(),
                screen_width(),
                FONT_SIZE_XL,
                RED,
            ),
            GameStatus::Going => {
                if is_mouse_button_pressed(MouseButton::Right) {
                    let mouse_pos: Vec2 = mouse_position().into();
                    let (tile_x, tile_y) = screen_point_to_tile_index(mouse_pos);
                    if tile_x < minefield.h_size() && tile_y < minefield.v_size() {
                        minefield.toggle_flag(tile_x, tile_y);
                    }
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    let mouse_pos: Vec2 = mouse_position().into();
                    let (tile_x, tile_y) = screen_point_to_tile_index(mouse_pos);
                    if tile_x < minefield.h_size() && tile_y < minefield.v_size() {
                        minefield.reveal(tile_x, tile_y);
                    }
                }
            }
        }

        next_frame().await;
    }
}
