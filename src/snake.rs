use crate::coord::Coord;
use crate::draw;
use crate::error::MyError;
use crate::field::Cell;
use crate::MyResult;
use piston_window::*;
use std::collections::LinkedList;
use std::fs;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct Snake {
    body: LinkedList<Cell>,
    direction: Direction,
    food: LinkedList<Cell>,
    prev_tail: Cell,
}

impl Snake {
    pub fn load_from_file(file_path: &str) -> MyResult<Snake> {
        let file = fs::read_to_string(file_path).unwrap();
        let lines: Vec<&str> = file.split("\n").collect();

        let mut body = LinkedList::new();
        let mut direction = None;
        let mut y = 0;
        for &line in lines.iter() {
            let mut x = 0;

            let chars: Vec<char> = line.chars().collect();
            for character in chars.iter() {
                match character {
                    'r' => {
                        body.push_front((Coord::new(x), Coord::new(y)));
                        direction = Some(Direction::RIGHT);
                    }
                    'l' => {
                        body.push_front((Coord::new(x), Coord::new(y)));
                        direction = Some(Direction::LEFT);
                    }
                    'u' => {
                        body.push_front((Coord::new(x), Coord::new(y)));
                        direction = Some(Direction::UP);
                    }
                    'd' => {
                        body.push_front((Coord::new(x), Coord::new(y)));
                        direction = Some(Direction::DOWN);
                    }
                    '=' => {
                        body.push_front((Coord::new(x), Coord::new(y)));
                    }
                    _ => (),
                }
                x += 1;
            }
            y += 1;
        }

        let direction = match direction {
            Some(d) => d,
            None => return Err(MyError::NoDirection.into()),
        };

        let prev_tail = body.back().unwrap().clone();

        Ok(Snake {
            body,
            direction,
            food: LinkedList::new(),
            prev_tail,
        })
    }

    pub fn update(&mut self) {
        if let Some(grow) = self.food.pop_back() {
            if self.prev_tail == grow {
                self.body.push_back(grow);
            } else {
                self.food.push_back(grow);
            }
        }

        let head = self.body.front().unwrap().clone();
        match self.direction {
            Direction::UP => {
                self.body.push_front((head.0, head.1 - 1));
            }
            Direction::DOWN => {
                self.body.push_front((head.0, head.1 + 1));
            }
            Direction::LEFT => {
                self.body.push_front((head.0 - 1, head.1));
            }
            Direction::RIGHT => {
                self.body.push_front((head.0 + 1, head.1));
            }
        };
        self.prev_tail = self.body.pop_back().unwrap();
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        for block in &self.body {
            draw::draw_snake(block, context, graphics);
        }
        for block in &self.food {
            draw::draw_snake_eating(block, context, graphics);
        }
    }

    pub fn put_food(&mut self, food_cell: Cell) {
        self.food.push_front(food_cell);
    }

    fn opposite_direction(&self, direction: Direction) -> bool {
        match self.direction {
            Direction::UP => direction == Direction::DOWN,
            Direction::DOWN => direction == Direction::UP,
            Direction::LEFT => direction == Direction::RIGHT,
            Direction::RIGHT => direction == Direction::LEFT,
        }
    }

    pub fn update_direction(&mut self, direction: Direction) {
        if !self.opposite_direction(direction) {
            self.direction = direction;
        }
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn body(&self) -> &LinkedList<Cell> {
        &self.body
    }
}
