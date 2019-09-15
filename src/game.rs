use crate::field::Cell;
use crate::field::{CellType, Field};
use crate::snake::{Direction, Snake};
use crate::{draw, MyResult};
use piston_window::*;
use rand::Rng;

const MOVING_PERIOD: f64 = 0.2;

pub struct Game {
    waiting_time: f64,
    game_over: bool,
    collision: Option<Cell>,
    field: Field,
    snake: Snake,
    food: Option<Cell>,
    empty_cells: Vec<Cell>,
    direction_buffer: Direction,
}

impl Game {
    pub fn create_form_level_file(level_file: &str) -> MyResult<Game> {
        let field = Field::load_from_file(level_file)?;
        let snake = Snake::load_from_file(level_file)?;
        let empty_cells = field.get_empty_cells();
        let direction_buffer = snake.direction();

        let mut game = Game {
            waiting_time: 0.0,
            game_over: false,
            collision: None,
            field,
            snake,
            food: None,
            empty_cells,
            direction_buffer,
        };

        game.food = game.generate_food();

        Ok(game)
    }

    pub fn field_width(&self) -> u32 {
        self.field.width()
    }

    pub fn field_height(&self) -> u32 {
        self.field.height()
    }

    pub fn update(&mut self, delta_time: f64) {
        if self.is_over() {
            return;
        }

        if let Some(collision) = self.get_collision() {
            self.collision = Some(collision);
            self.game_over = true;
            return;
        }

        if self.food.is_none() {
            self.food = self.generate_food();
        }

        if let Some(eating) = self.get_eating() {
            self.snake.put_food(eating);
            self.food = self.generate_food();
        }

        self.waiting_time += delta_time;
        if self.waiting_time >= MOVING_PERIOD {
            self.snake.update_direction(self.direction_buffer);
            self.snake.update();
            self.waiting_time = 0.0;
        }
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        self.field.draw(context, graphics);
        self.snake.draw(context, graphics);

        if let Some(food_cell) = self.food {
            draw::draw_food(food_cell, context, graphics);
        }

        if let Some(collision_cell) = self.collision {
            draw::draw_collision(collision_cell, context, graphics);
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        let direction = match key {
            Key::Up => Direction::UP,
            Key::Down => Direction::DOWN,
            Key::Right => Direction::RIGHT,
            Key::Left => Direction::LEFT,
            _ => self.snake.direction(),
        };
        self.direction_buffer = direction;
    }

    pub fn is_over(&self) -> bool {
        self.game_over
    }

    fn generate_food(&self) -> Option<Cell> {
        let mut empty_cells: Vec<Cell> = self.empty_cells.to_vec();
        for snake_cell in self.snake.body().iter() {
            if let Some(position) = empty_cells.iter().position(|cell| cell == snake_cell) {
                empty_cells.remove(position);
            }
        }

        if empty_cells.is_empty() {
            None
        } else {
            let mut rng = rand::thread_rng();
            let random_number: usize = rng.gen_range(0, empty_cells.len());
            Some(*empty_cells.get(random_number).unwrap())
        }
    }

    fn get_collision(&self) -> Option<Cell> {
        let head = self.snake.body().front().unwrap();
        for (idx, snake_cell) in self.snake.body().iter().enumerate() {
            if let CellType::WALL = self.field.get_cell_type(*snake_cell).unwrap() {
                return Some(*snake_cell);
            }

            // Don't check if the head interleave the rest snake
            if idx == 0 {
                continue;
            }
            if head == snake_cell {
                return Some(*snake_cell);
            }
        }
        None
    }

    fn get_eating(&self) -> Option<Cell> {
        for snake_cell in self.snake.body() {
            if let Some(food) = &self.food {
                if food == snake_cell {
                    return Some(*food);
                }
            }
        }
        None
    }
}
