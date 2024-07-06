// Game
pub const GRID_SIZE: i32 = 15;

// Sim
pub const NUM_AGENTS: usize = 1000;
pub const NUM_STEPS: usize = 75;
pub const NUM_THREADS: usize = 8;

// Pop
pub const POP_RETAINED: f32 = 0.1;
pub const POP_RETAINED_MUTATED: f32 = 0.0;
pub const POP_ROULETTE: f32 = 0.6;
pub const POP_TOURNAMENT: f32 = 0.1;
pub const POP_NUM_RANDOM: f32 = 0.2;

// Data
pub const SAVE_FILE_NAME: &str = "data/net.json";
pub const LOAD_FILE_NAME: &str = "data/net-100-2.json";
pub const IS_LOAD_SAVED_DATA: bool = false;
pub const IS_SAVE_BEST_NET: bool = false;

// NN
pub const NN_ARCH: [usize; 4] = [24, 16, 8, 4];

// Viz
pub const IS_LOW_DETAIL_MODE: bool = false;
pub const USE_GAME_CANVAS: bool = false;
pub const VIZ_GAME_SCALE: i32 = 3;
pub const VIZ_OFFSET: i32 = 2;
pub const VIZ_UPDATE_FRAMES: u32 = 50;
pub const VIZ_GRAPHS_LEN: usize = 45;
