//! Simulation
//! Manages the evolution of population over multiple generations

use std::io;
use std::time::Instant;

use crate::pop::Population;
use crate::viz::Viz;

pub struct Simulation {
    gen_count: usize,
    pop: Population,
    viz: Viz,
    gen_start_ts: Instant,
    max_score: usize,
}

#[derive(Default, Clone, Copy)]
pub struct GenerationSummary {
    pub gen_count: usize,
    pub time_elapsed_secs: f32,
    pub gen_max_score: usize,
    pub sim_max_score: usize,
}

impl Simulation {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            gen_count: 0,
            pop: Population::new(),
            viz: Viz::new()?,
            gen_start_ts: Instant::now(),
            max_score: 0,
        })
    }

    pub fn terminate(&self) -> io::Result<()> {
        Viz::restore_terminal()
    }

    pub fn update(&mut self) {
        let games_alive = self.pop.update();
        if games_alive <= 0 {
            self.end_current_genration();
            self.start_new_generation();
        }

        self.viz.update();
        self.viz.draw();
    }

    pub fn start_new_generation(&mut self) {
        self.gen_count += 1;
        self.pop.reset();
    }

    pub fn end_current_genration(&mut self) {
        let (best_net, gen_max_score) = self.pop.get_gen_summary();
        if gen_max_score > self.max_score {
            self.max_score = gen_max_score;
            best_net.save();
            self.viz.update_brain(best_net);
        }

        let stats = GenerationSummary {
            gen_count: self.gen_count,
            time_elapsed_secs: self.gen_start_ts.elapsed().as_secs_f32(),
            gen_max_score,
            sim_max_score: self.max_score,
        };
        self.viz.update_summary(stats);
        self.gen_start_ts = Instant::now();
    }
}
