# AI learns to play Snake!
A neural network learns to play snakes in the terminal

Built with [Rust](https://www.rust-lang.org/) and [Ratatui](https://ratatui.rs/)

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

## Configurations
- The project configs file is located at `src/configs.rs`
- Update `IS_LOW_DETAIL_MODE` for a minimal UI, it runs faster
- Set `IS_SAVE_BEST_NET` to train your own network. Networks are saved in `data/net.json`, once saved you can use the trained network by setting `IS_LOAD_SAVED_NET`
- Use `data/net-100.json` to watch the AI complete the game for 15x15 board size
- Set `USE_GAME_CANVAS` to true and update `VIZ_GAME_SCALE` to scale the game if needed.

## Others
- I mostly post about my projects on - [Twitter - @BonesaiDev](https://x.com/BonesaiDev)
- Video of snake [completing the game](https://x.com/BonesaiDev/status/1806738659296891032)
- Check out my other projects here - [https://bones-ai.bearblog.dev/projects/](https://bones-ai.bearblog.dev/projects/)
- More about me - [https://bones-ai.bearblog.dev/](https://bones-ai.bearblog.dev/)
