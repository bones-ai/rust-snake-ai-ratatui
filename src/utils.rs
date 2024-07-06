use rand::Rng;

use crate::*;

#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum FourDirs {
    #[default]
    Left,
    Right,
    Bottom,
    Top,
}

pub fn get_eight_dirs() -> [(i32, i32); 8] {
    [
        FourDirs::Left.value(),
        FourDirs::Right.value(),
        FourDirs::Bottom.value(),
        FourDirs::Top.value(),
        (-1, 1),
        (1, 1),
        (1, -1),
        (-1, -1),
    ]
}

impl FourDirs {
    pub fn get_rand_dir() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Self::Left,
            1 => Self::Right,
            2 => Self::Bottom,
            _ => Self::Top,
        }
    }

    pub fn value(&self) -> (i32, i32) {
        match self {
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::Top => (0, 1),
            Self::Bottom => (0, -1),
        }
    }

    pub fn get_one_hot_dir(&self) -> Vec<f64> {
        match self {
            FourDirs::Left => vec![1.0, 0.0, 0.0, 0.0],
            FourDirs::Right => vec![0.0, 1.0, 0.0, 0.0],
            FourDirs::Bottom => vec![0.0, 0.0, 1.0, 0.0],
            FourDirs::Top => vec![0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn equals(&self, other: Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }

    pub fn rand() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(1..GRID_SIZE - 1),
            y: rng.gen_range(1..GRID_SIZE - 1),
        }
    }
}

// Tuple to point
impl Into<Point> for (i32, i32) {
    fn into(self) -> Point {
        Point {
            x: self.0,
            y: self.1,
        }
    }
}

// Get a tuple from a point,
// let (fx, fy) = self.game.food.into();
impl From<Point> for (i32, i32) {
    fn from(point: Point) -> Self {
        (point.x, point.y)
    }
}
