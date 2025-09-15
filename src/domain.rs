use std::ops::{Index, IndexMut};

use rand::prelude::*;
use rand_chacha::ChaCha20Rng;
use uuid::Uuid;

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

fn tile_index(x: u8, y: u8, h_size: u8) -> usize {
    (y as usize) * (h_size as usize) + (x as usize)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Mine,
    Adjacent { no_of_mines: u8 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Minefield {
    pub h_size: u8,
    pub v_size: u8,
    tiles: Vec<Tile>,
}

impl Minefield {
    pub fn random(h_size: u8, v_size: u8, density: f32) -> Minefield {
        let seed = Uuid::new_v4().as_u64_pair().0;
        Minefield::from_seed(h_size, v_size, density, seed)
    }

    pub fn from_seed(h_size: u8, v_size: u8, density: f32, seed: u64) -> Minefield {
        let minefield_size = h_size as usize * v_size as usize;
        let mut rng: ChaCha20Rng = ChaCha20Rng::seed_from_u64(seed);
        let mine_count: usize = ((minefield_size as f32) * density).round() as usize;

        let mut mines: Vec<(u8, u8)> = (0..h_size)
            .flat_map(move |x| (0..v_size).map(move |y| (x, y)))
            .collect();
        mines.shuffle(&mut rng);

        Minefield::new(h_size, v_size, &mines[0..mine_count])
    }

    pub fn new(h_size: u8, v_size: u8, mine_indices: &[(u8, u8)]) -> Minefield {
        let mut tiles = vec![Tile::Empty; h_size as usize * v_size as usize];
        for &(x, y) in mine_indices {
            tiles[tile_index(x, y, h_size)] = Tile::Mine
        }

        for x in 0..h_size {
            for y in 0..v_size {
                if tiles[tile_index(x, y, h_size)] != Tile::Mine {
                    let no_of_adjacent_mines = neighbors(x, y, h_size, v_size)
                        .map(|(neighbor_x, neighbor_y)| {
                            &tiles[tile_index(neighbor_x, neighbor_y, h_size)]
                        })
                        .filter(|&tile| *tile == Tile::Mine)
                        .count();

                    if no_of_adjacent_mines > 0 {
                        tiles[tile_index(x, y, h_size)] = Tile::Adjacent {
                            no_of_mines: no_of_adjacent_mines as u8,
                        }
                    }
                }
            }
        }

        Minefield {
            h_size,
            v_size,
            tiles,
        }
    }
}

impl Index<(u8, u8)> for Minefield {
    type Output = Tile;

    fn index(&self, (x, y): (u8, u8)) -> &Self::Output {
        &self.tiles[tile_index(x, y, self.h_size)]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub minefield: Minefield,
    tile_states: Vec<TileState>,
}

impl GameState {
    pub fn new(minefield: Minefield) -> GameState {
        let h_size = minefield.h_size as usize;
        let v_size = minefield.v_size as usize;
        GameState {
            minefield,
            tile_states: vec![TileState::Hidden; h_size * v_size],
        }
    }
}

impl Index<(u8, u8)> for GameState {
    type Output = TileState;

    fn index(&self, (x, y): (u8, u8)) -> &Self::Output {
        &self.tile_states[tile_index(x, y, self.minefield.h_size)]
    }
}

impl IndexMut<(u8, u8)> for GameState {
    fn index_mut(&mut self, (x, y): (u8, u8)) -> &mut Self::Output {
        &mut self.tile_states[tile_index(x, y, self.minefield.h_size)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minefield_new_should_create_a_minefield_of_given_size() {
        let minefield = Minefield::new(16, 32, &[(0, 0)]);
        assert_eq!(minefield.h_size, 16);
        assert_eq!(minefield.v_size, 32);
    }

    #[test]
    fn minefield_new_should_create_a_mine_in_each_mine_index() {
        let mines = [(0, 4), (4, 8), (8, 16)];
        let minefield = Minefield::new(16, 32, &mines);
        for mine in mines {
            assert_eq!(minefield[mine], Tile::Mine);
        }
    }

    #[test]
    fn neighbors_should_return_empty_when_size_is_1x1() {
        let result: Vec<(u8, u8)> = neighbors(0, 0, 1, 1).collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn neighbors_should_return_8_tiles_when_size_is_3x3_and_xy_is_the_center() {
        let result: Vec<(u8, u8)> = neighbors(1, 1, 3, 3).collect();
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
        let result: Vec<(u8, u8)> = neighbors(0, 0, 3, 3).collect();
        assert_eq!(result, vec![(0, 1), (1, 0), (1, 1)]);
    }

    #[test]
    fn neighbors_should_return_3_tiles_when_size_is_3x3_and_xy_is_2_0() {
        let result: Vec<(u8, u8)> = neighbors(0, 2, 3, 3).collect();
        assert_eq!(result, vec![(0, 1), (1, 1), (1, 2)]);
    }
}
