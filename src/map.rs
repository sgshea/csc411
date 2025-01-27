use std::{collections::HashMap, fmt::Display, fs::File, io::{BufRead, BufReader, ErrorKind}};

use glam::IVec2;

use crate::action::Direction;

/**
 * Basic tile implementation.
 * This may be refactored into a trait if each tile requires complex behavior in the future.
 */
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum Tile {
    #[default]
    CLEAN,
    DIRTY,
    IMPASSABLE,
    TARGET,
}

#[derive(Clone)]
pub struct Map {
    tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let map = Map {
            tiles: vec![vec![Tile::default(); width]; height],
            width,
            height,
        };

        map
    }

    pub fn load_from_file(filename: &str) -> Result<Self, std::io::Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = Vec::new();

        for line_result in reader.lines() {
            let line = line_result?;
            // Trim leading and trailing whitespace
            let trimmed_line = line.trim();
            if !trimmed_line.is_empty() {
                lines.push(trimmed_line.to_string());
            }
        }

        if lines.is_empty() {
            return Err(std::io::Error::new(
                ErrorKind::Other,
                "The file is empty or contains only whitespace.",
            ));
        }

        // Determine width from the first line
        let width = lines[0].len();
        // Check that all lines have the same length as the first line
        for line in &lines[1..] {
            if line.len() != width {
                return Err(std::io::Error::new(
                    ErrorKind::Other,
                    "All lines must be of the same length.",
                ));
            }
        }

        let height = lines.len();
        let mut tiles = Vec::with_capacity(height);

        for line in lines {
            let mut row: Vec<Tile> = Vec::with_capacity(width);
            for c in line.chars() {
                match c {
                    'C' => row.push(Tile::CLEAN),
                    'W' => row.push(Tile::IMPASSABLE),
                    'T' => row.push(Tile::TARGET),
                    _ => {
                        // Return an error for unexpected characters
                        return Err(std::io::Error::new(
                            ErrorKind::Other,
                            format!("Unknown tile character: {}", c),
                        ));
                    }
                }
            }
            tiles.push(row);
        }

        Ok(Map {
            tiles,
            width,
            height,
        })
    }

    pub fn in_bounds(&self, pos: IVec2) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y >= 0 && pos.y < self.height as i32
    }

    pub fn get_tile(&self, pos: IVec2) -> &Tile {
        &self.tiles[pos.y as usize][pos.x as usize]
    }

    pub fn get_tile_mut(&mut self, pos: IVec2) -> &mut Tile {
        &mut self.tiles[pos.y as usize][pos.x as usize]
    }

    // Gets all of the tiles of a certain type
    pub fn get_all_of_type(&self, tile_type: Tile) -> HashMap<IVec2, &Tile> {
        let mut tiles_map = HashMap::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[y as usize][x as usize] == tile_type {
                    tiles_map.insert(IVec2::new(x as i32, y as i32), &self.tiles[y][x]);
                }
            }
        }
        tiles_map
    }

    // Returns neighbors of a given tile
    pub fn get_neighbors(&self, pos: &IVec2) -> HashMap<IVec2, (Direction, &Tile)> {
        let mut neighbors = HashMap::new();
        for direction in Direction::all() {
            let neighbor_pos = pos + direction.to_ivec2();
            if self.in_bounds(neighbor_pos) {
                neighbors.insert(neighbor_pos, (direction, &self.tiles[neighbor_pos.y as usize][neighbor_pos.x as usize]));
            }
        }
        neighbors
    }

    pub fn set_tile(&mut self, pos: IVec2, tile: Tile) {
        self.tiles[pos.y as usize][pos.x as usize] = tile;
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for row in &self.tiles {
            for tile in row {
                match tile {
                    Tile::IMPASSABLE => output.push('W'),
                    Tile::CLEAN => output.push('C'),
                    Tile::DIRTY => output.push('D'),
                    Tile::TARGET => output.push('T'),
                }
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}