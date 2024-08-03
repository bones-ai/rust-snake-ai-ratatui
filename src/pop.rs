//! Population
//! A group of simulation agents

use std::cmp::Ordering;

use rand::distributions::{Distribution, WeightedIndex};
use rand::Rng;
use rayon::prelude::*;

use crate::agent::Agent;
use crate::nn::Net;
use crate::{GRID_SIZE, IS_LOAD_SAVED_DATA, NN_ARCH, NUM_AGENTS, POP_NUM_RANDOM, POP_RETAINED, POP_RETAINED_MUTATED, POP_ROULETTE, POP_TOURNAMENT};

pub struct Population {
    pub mutation_magnitude: f64,
    pub mutation_rate: f64,

    agents: Vec<Agent>,
}

impl Default for Population {
    fn default() -> Self {
        Self::new()
    }
}

impl Population {
    #[must_use] pub fn new() -> Self {
        let mut agents = Vec::new();
        for _ in 0..NUM_AGENTS {
            agents.push(Agent::new(IS_LOAD_SAVED_DATA));
        }

        Self {
            // rate & mag will be reset before use
            mutation_rate: 0.1,
            mutation_magnitude: 0.1,
            agents,
        }
    }

    pub fn update(&mut self) -> usize {
        let agents_dead = self
            .agents
            .par_iter_mut()
            .map(|a| {
                a.update();
                if a.game.is_dead {
                    1
                } else {
                    0
                }
            })
            .sum::<usize>();

        NUM_AGENTS - agents_dead
    }

    pub fn reset(&mut self) {
        self.reset_pop();
    }

    #[must_use] pub fn get_gen_summary(&self) -> (Net, usize) {
        let mut max_score = 0;
        let mut best_net = None;

        for a in &self.agents {
            let score = a.game.score();
            if score > max_score {
                max_score = score;
                best_net = Some(&a.brain);
            }
        }

        if let Some(net) = best_net {
            return (net.to_owned(), max_score);
        }

        (Net::new(&NN_ARCH), max_score)
    }

    fn reset_pop(&mut self) {
        // Calc mutation rate and mag
        let gen_max_score = self
            .agents
            .iter()
            .map(|a| a.game.score())
            .max()
            .unwrap_or(0);
        let (mutation_mag, mutation_rate) = self.get_mutation_params(gen_max_score as f64);

        // Sort agents based on their fitness
        let mut agents_sorted = self.agents.clone();
        agents_sorted.sort_by(|a, b| b.partial_cmp(a).unwrap_or(Ordering::Equal));

        // Population Distribution
        let num_elite = (NUM_AGENTS as f32 * POP_RETAINED) as usize;
        let num_roulette = (NUM_AGENTS as f32 * POP_ROULETTE) as usize;
        let mut num_tournament = (NUM_AGENTS as f32 * POP_TOURNAMENT) as usize;
        let num_mutated = (NUM_AGENTS as f32 * POP_RETAINED_MUTATED) as usize;
        let num_random = (NUM_AGENTS as f32 * POP_NUM_RANDOM) as usize;

        // Elitism
        // Preserve best performing agents
        // Hels maintain high fitness levels within the population
        let mut new_agents: Vec<_> = agents_sorted
            .iter()
            .take(num_elite)
            .map(|agent| Agent::with_brain(agent.brain.clone()))
            .collect();

        new_agents.reserve(NUM_AGENTS - num_elite);

        // Roulette Selection (or Fitness Proportionate Selection)
        // Each agent is selected with a probability proportional to its fitness
        let gene_pool = self.generate_gene_pool();
        if let Some(pool) = gene_pool {
            let mut rng = rand::thread_rng();
            for _ in 0..num_roulette as i32 {
                let rand_parent_1 = &self.agents[pool.sample(&mut rng)];
                let rand_parent_2 = &self.agents[pool.sample(&mut rng)];
                let mut new_brain = rand_parent_1.brain.merge(&rand_parent_2.brain);
                new_brain.mutate(mutation_rate, mutation_mag);

                let new_agent = Agent::with_brain(new_brain);
                new_agents.push(new_agent);
            }
        } else {
            num_tournament += num_roulette;
        }

        // Tournament Selection
        // Fittest agents among a randomly selected group (tournament)
        // Tournament Size (TS) controls the balance between exploration and exploitation
        // Smaller TS -> More exploration
        let tournament_size = 5;
        for _ in 0..num_tournament {
            let winner = self.tournament_selection(tournament_size);
            let mut new_brain = winner.brain.clone();
            new_brain.mutate(mutation_rate, mutation_mag);
            new_agents.push(Agent::with_brain(new_brain));
        }

        // Mutational Elitism
        // Allows for incremental improvements to already good solutions
        new_agents.extend(agents_sorted.iter().take(num_mutated).map(|agent| {
            let mut old_brain = agent.brain.clone();
            old_brain.mutate(mutation_rate, mutation_mag);
            Agent::with_brain(old_brain)
        }));

        // Full random
        // Diversify the gene pool
        new_agents.extend(
            self.agents
                .iter()
                .take(num_random)
                .map(|_| Agent::new(false)),
        );

        self.agents = new_agents;
        self.mutation_magnitude = mutation_mag;
        self.mutation_rate = mutation_rate;
    }

    fn tournament_selection(&self, tournament_size: usize) -> &Agent {
        let mut rng = rand::thread_rng();
        let mut best_agent = &self.agents[rng.gen_range(0..self.agents.len())];

        for _ in 0..tournament_size {
            let agent = &self.agents[rng.gen_range(0..self.agents.len())];
            if agent.fitness() > best_agent.fitness() {
                best_agent = agent;
            }
        }

        best_agent
    }

    fn generate_gene_pool(&self) -> Option<WeightedIndex<f32>> {
        let mut max_fitness = 0.0;
        let mut weights = Vec::new();

        for a in &self.agents {
            let fitness = a.fitness();
            if fitness > max_fitness {
                max_fitness = fitness;
            }

            if fitness.is_finite() {
                weights.push(fitness);
            }
        }
        weights
            .iter_mut()
            .for_each(|i| *i = (*i / max_fitness) * 100.0);

        WeightedIndex::new(&weights).ok()
    }

    fn get_mutation_params(&self, gen_max: f64) -> (f64, f64) {
        let max_score = f64::from((GRID_SIZE - 1) * (GRID_SIZE - 1));
        if gen_max > 0.75 * max_score {
            (0.1, 0.15)
        } else if gen_max > 0.5 * max_score {
            (0.1, 0.25)
        } else {
            (0.5, 0.15)
        }
    }
}
