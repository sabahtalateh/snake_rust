use crate::coord::Coord;
use crate::draw;
use crate::MyResult;
use piston_window::*;
use std::fs;

pub type Cell = (Coord, Coord);

#[derive(Debug)]
pub enum CellType {
    WALL,
    EMPTY,
}

pub struct Field {
    field: Vec<Vec<CellType>>,
    width: u32,
    height: u32,
}

impl Field {
    fn new() -> Field {
        Field {
            field: Vec::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn get_cell_type(&self, (x, y): &Cell) -> Option<&CellType> {
        let y_idx: usize = y.clone().into();
        let x_idx: usize = x.clone().into();
        // return in reverse order because `y` states for `rows` and `x` for `cells`
        self.field.get(y_idx)?.get(x_idx)
    }

    pub fn insert_cell(&mut self, (x, y): Cell, cell_type: CellType) {
        let x_idx: usize = x.into();
        let y_idx: usize = y.into();

        let row = match self.field.get_mut(x_idx) {
            Some(x) => x,
            None => {
                self.field.insert(x_idx, Vec::new());
                self.field.get_mut(x_idx).unwrap()
            }
        };
        row.insert(y_idx, cell_type);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn load_from_file(file_path: &str) -> MyResult<Field> {
        let mut field = Field::new();

        let level = fs::read_to_string(file_path).unwrap();
        let lines: Vec<&str> = level.split("\n").collect();

        let mut width = 0;
        let mut height = 0;
        let mut x = 0;
        for &line in lines.iter() {
            let mut y = 0;

            let chars: Vec<char> = line.chars().collect();
            for character in chars.iter() {
                match character {
                    '-' => {
                        field.insert_cell((Coord::new(x), Coord::new(y)), CellType::WALL);
                    }
                    _ => {
                        field.insert_cell((Coord::new(x), Coord::new(y)), CellType::EMPTY);
                    }
                }
                y += 1;
                if y > width {
                    width = y;
                }
            }
            x += 1;
            height += 1;
        }

        field.width = width;
        field.height = height;

        Ok(field)
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                let cell: Cell = (Coord::new(x), Coord::new(y));
                let cell_type = self
                    .get_cell_type(&cell)
                    .unwrap_or_else(|| &CellType::EMPTY);
                match cell_type {
                    CellType::WALL => {
                        draw::draw_wall(&cell, context, graphics);
                    }
                    CellType::EMPTY => {
                        draw::draw_empty(&cell, context, graphics);
                    }
                }
            }
        }
    }

    pub fn get_empty_cells(&self) -> Vec<Cell> {
        let mut empty_cells: Vec<Cell> = Vec::new();
        for x in 0..self.height() {
            for y in 0..self.width() {
                let cell: Cell = (Coord::new(y), Coord::new(x));
                let cell_type = self
                    .get_cell_type(&cell)
                    .unwrap_or_else(|| &CellType::EMPTY);

                if let CellType::EMPTY = cell_type {
                    empty_cells.push(cell);
                }
            }
        }
        empty_cells
    }
}
