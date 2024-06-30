// Game
pub const GRID_SIZE: i32 = 15;

// Sim
pub const NUM_AGENTS: usize = 500;
pub const NUM_STEPS: usize = 50;
pub const NUM_THREADS: usize = 8;

// Pop
pub const POP_RETAINED: f32 = 0.1;
pub const POP_RETAINED_MUTATED: f32 = 0.0;
pub const POP_ROULETTE: f32 = 0.6;
pub const POP_TOURNAMENT: f32 = 0.2;
pub const POP_NUM_RANDOM: f32 = 0.1;

// Data
pub const SAVE_FILE_NAME: &str = "data/net.json";
pub const LOAD_FILE_NAME: &str = "data/net.json";
pub const IS_LOAD_SAVED_DATA: bool = true;
pub const IS_SAVE_BEST_NET: bool = true;

// NN
pub const BRAIN_MUTATION_RATE: f64 = 0.1;
pub const BRAIN_MUTATION_VARIATION: f64 = 0.1;
pub const NN_ARCH: [usize; 4] = [24, 16, 8, 4];

// Viz
pub const IS_LOW_DETAIL_MODE: bool = false;
pub const USE_GAME_CANVAS: bool = true;
pub const VIZ_GAME_SCALE: i32 = 3;
pub const VIZ_OFFSET: i32 = 2;
pub const VIZ_UPDATE_FRAMES: u32 = 50;
pub const VIZ_GRAPHS_LEN: usize = 45;
