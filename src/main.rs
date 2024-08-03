use std::time::Duration;
use std::{io, time::Instant};

use crossterm::event::{self, Event, KeyCode};

use sim::Simulation;
use snake_tui::{NUM_THREADS, sim};

fn main() -> io::Result<()> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(NUM_THREADS)
        .build_global()
        .unwrap();

    let mut sim = Simulation::new()?;
    let mut last_poll = Instant::now();

    loop {
        if last_poll.elapsed() > Duration::from_millis(15) {
            if event::poll(Duration::ZERO)? {
                last_poll = Instant::now();
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Esc | KeyCode::Char('q') = key.code {
                        break;
                    }
                }
            }
            sim.draw();
        }

        sim.update();
    }

    sim.stop()
}
