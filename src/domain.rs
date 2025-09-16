use std::ops::Index;

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
pub enum TileKind {
    Empty,
    Mine,
    Adjacent { no_of_mines: u8 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileState {
    Hidden,
    Revealed,
    Flagged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
    kind: TileKind,
    state: TileState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameStatus {
    Won,
    Lost,
    Going,
}

impl Tile {
    pub fn hidden_mine() -> Tile {
        Tile {
            kind: TileKind::Mine,
            state: TileState::Hidden,
        }
    }

    pub fn hidden_empty() -> Tile {
        Tile {
            kind: TileKind::Empty,
            state: TileState::Hidden,
        }
    }

    pub fn is_mine(&self) -> bool {
        self.kind == TileKind::Mine
    }

    pub fn is_empty(&self) -> bool {
        self.kind == TileKind::Empty
    }

    pub fn is_adjacent(&self) -> bool {
        matches!(self.kind, TileKind::Adjacent { .. })
    }

    pub fn no_of_adjacent_mine(&self) -> u8 {
        match self.kind {
            TileKind::Adjacent { no_of_mines } => no_of_mines,
            _ => 0,
        }
    }

    pub fn is_hidden(&self) -> bool {
        self.state == TileState::Hidden
    }

    pub fn is_revealed(&self) -> bool {
        self.state == TileState::Revealed
    }

    pub fn is_flagged(&self) -> bool {
        self.state == TileState::Flagged
    }

    pub fn toggle_flag(&mut self) {
        if self.is_flagged() {
            self.state = TileState::Hidden;
        } else {
            self.state = TileState::Flagged;
        }
    }

    pub fn reveal(&mut self) {
        // TODO perhaps return Result
        if self.is_hidden() && !self.is_flagged() {
            self.state = TileState::Revealed;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Minefield {
    h_size: u8,
    v_size: u8,
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
        let mut tiles = vec![Tile::hidden_empty(); h_size as usize * v_size as usize];
        for &(x, y) in mine_indices {
            tiles[tile_index(x, y, h_size)] = Tile::hidden_mine()
        }

        for x in 0..h_size {
            for y in 0..v_size {
                if tiles[tile_index(x, y, h_size)].is_empty() {
                    let no_of_adjacent_mines = neighbors(x, y, h_size, v_size)
                        .map(|(neighbor_x, neighbor_y)| {
                            &tiles[tile_index(neighbor_x, neighbor_y, h_size)]
                        })
                        .filter(|&tile| tile.is_mine())
                        .count();

                    if no_of_adjacent_mines > 0 {
                        tiles[tile_index(x, y, h_size)].kind = TileKind::Adjacent {
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

    pub fn h_size(&self) -> u8 {
        self.h_size
    }

    pub fn v_size(&self) -> u8 {
        self.v_size
    }

    pub fn reveal(&mut self, x: u8, y: u8) {
        let tile = &mut self.tiles[tile_index(x, y, self.h_size)];
        if tile.is_hidden() {
            tile.reveal();

            if tile.is_empty() {
                for (x, y) in neighbors(x, y, self.h_size, self.v_size) {
                    self.reveal(x, y);
                }
            }
        }
    }

    pub fn toggle_flag(&mut self, x: u8, y: u8) {
        self.tiles[tile_index(x, y, self.h_size)].toggle_flag();
    }

    pub fn game_status(&self) -> GameStatus {
        let mut status = GameStatus::Won;
        for tile in &self.tiles {
            if tile.is_mine() && tile.is_revealed() {
                return GameStatus::Lost;
            } else if !tile.is_mine() && tile.is_hidden() {
                status = GameStatus::Going;
            }
        }
        status
    }
}

impl Index<(u8, u8)> for Minefield {
    type Output = Tile;

    fn index(&self, (x, y): (u8, u8)) -> &Self::Output {
        &self.tiles[tile_index(x, y, self.h_size)]
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
            assert_eq!(minefield[mine], Tile::hidden_mine());
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
