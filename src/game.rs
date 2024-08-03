//! Snake Game
//! Handles all game related logic

use crate::{FourDirs, Point, GRID_SIZE};

#[derive(Clone)]
pub struct Game {
    pub head: Point,
    pub body: Vec<Point>,
    pub food: Point,
    pub dir: FourDirs,

    pub is_dead: bool,
    pub total_steps: usize,
    pub no_food_steps: usize,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    #[must_use]
    pub fn new() -> Self {
        let head = Point::new(GRID_SIZE / 2, GRID_SIZE / 2);
        let mut body = vec![head];
        body.push(Point::new(head.x - 1, head.y));
        body.push(Point::new(head.x - 2, head.y));

        Self {
            body,
            head,
            food: Point::rand(),
            dir: FourDirs::get_rand_dir(),
            is_dead: false,
            total_steps: 0,
            no_food_steps: 0,
        }
    }

    pub fn update(&mut self, dir: FourDirs) {
        if self.is_dead {
            return;
        }

        self.total_steps += 1;
        self.dir = dir;
        self.handle_food_collision();
        self.update_snake_positions();

        if self.is_wall(self.head) || self.is_snake_body(self.head) {
            self.is_dead = true;
        }
    }

    #[must_use]
    pub fn score(&self) -> usize {
        self.body.len()
    }

    #[must_use]
    pub fn is_wall(&self, pt: Point) -> bool {
        pt.x >= GRID_SIZE || pt.x <= 0 || pt.y >= GRID_SIZE || pt.y <= 0
    }

    #[must_use]
    pub fn is_snake_body(&self, pt: Point) -> bool {
        // skip head
        self.body[1..].contains(&pt)
    }

    fn update_snake_positions(&mut self) {
        self.head.x += self.dir.value().0;
        self.head.y += self.dir.value().1;

        for i in (1..self.body.len()).rev() {
            self.body[i] = self.body[i - 1];
        }
        self.body[0] = self.head;
    }

    fn handle_food_collision(&mut self) {
        if !self.head.equals(self.food) {
            self.no_food_steps += 1;
            return;
        }

        self.no_food_steps = 0;
        self.body.push(Point::new(self.head.x, self.head.y));
        self.food = Point::rand();
    }
}
