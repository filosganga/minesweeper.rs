use std::ops::{Index, IndexMut};

use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

#[derive(Clone)]
enum Tile {
    Empty,
    Mine,
    Adjacent { no_of_mines: u8 },
}

#[derive(Clone)]
struct Minefield {
    pub h_size: u8,
    pub v_size: u8,
    tiles: Vec<Tile>,
}

impl Minefield {
    fn empty(h_size: u8, v_size: u8) -> Minefield {
        let tiles = vec![Tile::Empty; (h_size * v_size).into()];
        Minefield {
            h_size,
            v_size,
            tiles,
        }
    }

    fn init(minefield: &mut Minefield, density: f32, seed: u64) {
        let minefield_size = minefield.tiles.len();
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        let mine_count = ((minefield_size as f32) * density).round() as usize;

        let mut mine_indices: Vec<usize> = (0..minefield_size).collect();
        for &i in &mine_indices[..mine_count] {
            minefield.tiles[i] = Tile::Mine
        }
    }

    pub fn new(h_size: u8, v_size: u8, density: f32, seed: u64) -> Minefield {
        let mut minefield = Minefield::empty(h_size, v_size);
        Minefield::init(&mut minefield, density, seed);
        minefield
    }

    fn tile_index(&self, x: u8, y: u8) -> usize {
        (y as usize) * (self.h_size as usize) + (x as usize)
    }

    fn neighbors(x: u8, y: u8, h_size: u8, v_size: u8) -> impl Iterator<Item = (u8, u8)> {
        let x_as_i8 = x as i8;
        let y_as_i8 = y as i8;
        let h_size_as_i8 = h_size as i8;
        let v_size_as_i8 = v_size as i8;
        (-1_i8..=1).flat_map(move |dx| {
            (-1_i8..=1).filter_map(move |dy| {
                if dx == 0 && dy == 0 {
                    None
                } else if x_as_i8 + dx >= h_size_as_i8
                    || y_as_i8 + dy >= v_size_as_i8
                    || x_as_i8 + dx < 0
                    || y_as_i8 + dy < 0
                {
                    None
                } else {
                    Some(((x_as_i8 + dx) as u8, (y_as_i8 + dy) as u8))
                }
            })
        })
    }
}

impl Index<u8> for Minefield {
    type Output = [Tile];

    fn index(&self, index: u8) -> &Self::Output {
        &self.tiles[0..5]
    }
}

impl IndexMut<u8> for Minefield {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.tiles[0..5]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbors_should_return_empty_when_size_is_1x1() {
        let result: Vec<(u8, u8)> = Minefield::neighbors(0, 0, 1, 1).collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn neighbors_should_return_8_tiles_when_size_is_3x3_and_xy_is_the_center() {
        let result: Vec<(u8, u8)> = Minefield::neighbors(1, 1, 3, 3).collect();
        assert_eq!(
            result,
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ]
        );
    }

    #[test]
    fn neighbors_should_return_3_tiles_when_size_is_3x3_and_xy_is_0_0() {
        let result: Vec<(u8, u8)> = Minefield::neighbors(0, 0, 3, 3).collect();
        assert_eq!(result, vec![(0, 1), (1, 0), (1, 1)]);
    }

    #[test]
    fn neighbors_should_return_3_tiles_when_size_is_3x3_and_xy_is_2_0() {
        let result: Vec<(u8, u8)> = Minefield::neighbors(0, 2, 3, 3).collect();
        assert_eq!(result, vec![(0, 1), (1, 1), (1, 2)]);
    }
}
