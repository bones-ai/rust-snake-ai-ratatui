# AI learns to play Snake!
A neural network learns to play snakes in the terminal

It was built from scratch using [Rust](https://www.rust-lang.org/) and [Ratatui](https://ratatui.rs/)

Check out [this](https://github.com/bones-ai/rust-snake-ai) for an older version of the AI that uses macroquad for GUI

![screenshot](/screenshot.png)


## Usage
- Clone the repo
```bash
git clone git@github.com:bones-ai/rust-snake-ai-ratatui.git
cd rust-snake-ai-ratatui
```
- Run the simulation
```bash
cargo run --release
```

## Configs
- The project configs file is located at `src/configs.rs`
- Update `IS_LOW_DETAIL_MODE` for a minimal UI, it runs faster
- Set `IS_SAVE_BEST_NET` to train your own network. Networks are saved in `data/net.json`, once saved you can use the trained network by setting `IS_LOAD_SAVED_NET`
- Use `data/net-100.json` to watch the AI complete the game for 15x15 board size
- Set `USE_GAME_CANVAS` to true and update `VIZ_GAME_SCALE` to scale the game if needed.


## Resources
Here are some excellent resources to learn more about genetic algorithms:
- **Video Series**: [Genetic Algorithm](https://www.youtube.com/watch?v=9zfeTw-uFCw&list=PLRqwX-V7Uu6bJM3VgzjNV5YxVxUwzALHV&ab_channel=TheCodingTrain) by The Coding Train.
- **Book**: [Nature of Code](http://natureofcode.com/book/) by Daniel Shiffman for those who prefer reading.
- I highly recommend checking out [Joshka's fork](https://github.com/joshka/rust-snake-ai-ratatui/tree/jm/refactor) for more idiomatic Rust code.

## Algorithm
1. **Initialization**: 
   - The simulation begins at `Generation 0`. 
   - A new population of snakes is created, each with a neural network initialized with random weights and biases.
2. **Game Update**: 
   - Each step, every game is updated by passing vision inputs to the neural network to decide the snake's action.
   - A game is flagged as complete if:
     - The snake collides with walls or itself.
     - The snake fails to eat food within a certain number of steps, preventing indefinite looping.
3. **Generation Completion**: 
   - The generation continues updating each game until all games are complete.
4. **Fitness Evaluation**: 
   - At the end of each generation, snakes are ranked based on their performance.
5. **Parent Selection**: 
   - Parents for the next generation are chosen based on rankings. Higher-ranked snakes have a higher probability of being selected as parents.
6. **Reproduction**: 
   - Techniques such as roulette wheel selection, elitism, and other methods are used to generate children for the next generation.
7. **New Generation**: 
   - A new population is created, and the process repeats from step 2 until the simulation is manually stopped.

This iterative process leads to snakes fine-tuning their strategies, resulting in longer snakes over time.


## Extras
- I mostly post about my projects on - [Twitter - @BonesaiDev](https://x.com/BonesaiDev)
- Video of snake [completing](https://x.com/BonesaiDev/status/1806738659296891032) the game
- Check out my other projects on github [https://github.com/bones-ai](https://github.com/bones-ai)