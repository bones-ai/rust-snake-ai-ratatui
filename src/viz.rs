//! Visualization
//! Responsible for rendering the game state and neural network on the terminal

use std::io::{self, stdout, Stdout};
use std::time::Instant;

use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::*;
use ratatui::widgets::canvas::{Canvas, Painter, Shape};
use ratatui::widgets::{
    Block, BorderType, Borders, Gauge, List, ListItem, Padding, Paragraph, Sparkline,
};
use symbols::Marker;

use crate::agent::Agent;
use crate::game::Game;
use crate::nn::Net;
use crate::sim::GenerationSummary;
use crate::*;

const COLOR_WALLS: Color = Color::Indexed(137);
const COLOR_BODY: Color = Color::Indexed(140);
const COLOR_HEAD: Color = Color::White;
const COLOR_DEAD: Color = Color::Indexed(205);
const COLOR_FOOD: Color = Color::LightGreen;

pub struct Viz {
    frame_count: u32,
    data: VizData,
    term: Terminal<CrosstermBackend<Stdout>>,
}

struct TermViz;

struct GameRender<'a> {
    game: &'a Game,
}

struct NNColors {
    disabled_color: Color,
    inp_colors: Vec<Color>,
    hidden_1_colors: Vec<Color>,
    hidden_2_colors: Vec<Color>,
    out_colors: Vec<Color>,
}

struct VizData {
    agent: Option<Agent>,
    stats: GenerationSummary,
    sim_start_ts: Instant,
    scores: Vec<u64>,
    gen_times: Vec<u64>,
    mutation_rate: f64,
    mutation_magnitude: f64,
}

impl Viz {
    pub fn new() -> io::Result<Self> {
        Ok(Self {
            frame_count: 0,
            data: VizData::default(),
            term: TermViz::init_terminal()?,
        })
    }

    pub fn update_brain(&mut self, new_brain: Net) {
        self.data.agent = Some(Agent::with_brain(new_brain));
    }

    pub fn update_summary(&mut self, stats: GenerationSummary, mr: f64, mg: f64) {
        self.data.stats = stats;
        self.data.mutation_rate = mr;
        self.data.mutation_magnitude = mg;

        self.data.scores.push(stats.gen_max_score as u64);
        self.data
            .gen_times
            .push((stats.time_elapsed_secs * 1000.0) as u64);
        if self.data.scores.len() > VIZ_GRAPHS_LEN {
            self.data.scores.remove(0);
        }
        if self.data.gen_times.len() > VIZ_GRAPHS_LEN {
            self.data.gen_times.remove(0);
        }
    }

    pub fn update(&mut self) {
        if self.data.agent.is_none() {
            return;
        }

        self.frame_count = (self.frame_count + 1) % 1000;
        if self.frame_count % VIZ_UPDATE_FRAMES != 0 {
            return;
        }

        // Update game agent
        // TODO find a better way to do the update
        let agent = self.data.agent.as_mut().unwrap();
        let is_alive = agent.update();
        if !is_alive {
            self.data.agent = Some(Agent::with_brain(agent.brain.clone()));
        }
    }

    pub fn draw(&mut self) {
        let _ = self.term.draw(|f| TermViz::draw(f, &self.data));
    }

    pub fn restore_terminal() -> io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}

// Handles rataui terminal rendering
impl TermViz {
    fn init_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        Terminal::new(CrosstermBackend::new(stdout()))
    }

    fn draw(f: &mut Frame, viz: &VizData) {
        // Gen-0, Viz agent not available yet
        if viz.agent.is_none() {
            f.render_widget(
                TermViz::widget_raw_text("Running Gen 0. Please Wait.".to_string()),
                f.size(),
            );
            return;
        }

        if IS_LOW_DETAIL_MODE {
            f.render_widget(
                TermViz::widget_raw_text(TermViz::get_simple_render_text(&viz)),
                f.size(),
            );
            return;
        }

        let agent = viz.agent.as_ref().unwrap();
        let root = Layout::horizontal([
            Constraint::Percentage(35),
            Constraint::Percentage(40),
            Constraint::Percentage(25),
        ]);
        let net_viz_vertical =
            Layout::vertical([Constraint::Percentage(75), Constraint::Percentage(25)]);
        let game_viz_vertical =
            Layout::vertical([Constraint::Percentage(100), Constraint::Percentage(0)]);
        let stats_viz_vertical = Layout::vertical([
            Constraint::Percentage(25),
            Constraint::Percentage(15),
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ]);

        let [game_lane, net_lane, stats_lane] = root.areas(f.size());
        let [nn_viz_area, about_area] = net_viz_vertical.areas(net_lane);
        let [game_area, _] = game_viz_vertical.areas(game_lane);
        let [sim_summary, viz_summary, viz_score_gauge, max_score_gauge, gen_times_graph, score_graph] =
            stats_viz_vertical.areas(stats_lane);

        f.render_widget(TermViz::render_about(), about_area);
        f.render_widget(
            TermViz::render_viz_score_gauge(agent.game.score()),
            viz_score_gauge,
        );
        f.render_widget(
            TermViz::render_max_score_gauge(viz.stats.sim_max_score),
            max_score_gauge,
        );
        f.render_widget(TermViz::render_score_graph(&viz.scores), score_graph);
        f.render_widget(
            TermViz::render_gen_times_graph(&viz.gen_times),
            gen_times_graph,
        );
        f.render_widget(
            TermViz::render_sim_stats(
                &viz.stats,
                &viz.sim_start_ts,
                viz.mutation_rate,
                viz.mutation_magnitude,
            ),
            sim_summary,
        );
        f.render_widget(TermViz::render_viz_stats(agent), viz_summary);
        f.render_widget(TermViz::render_nn(agent), nn_viz_area);

        if USE_GAME_CANVAS {
            f.render_widget(TermViz::render_game_canvas(&agent.game), game_area);
        } else {
            f.render_widget(TermViz::display_game_blocks(&agent.game), game_area);
        }
    }

    fn render_game_canvas<'a>(game: &'a Game) -> impl Widget + 'a {
        Canvas::default()
            .block(Block::new())
            .marker(Marker::HalfBlock)
            .paint(move |ctx| {
                ctx.draw(&GameRender { game: &game });
            })
            .x_bounds([0.0, 100.0])
            .y_bounds([0.0, 100.0])
    }

    fn render_viz_stats(agent: &Agent) -> impl Widget {
        let title = "  V I Z    S T A T S  ";
        let current_score = format!(
            "  Score: {:?}/{:?}",
            agent.game.score(),
            ((GRID_SIZE - 1) * (GRID_SIZE - 1))
        );
        let fitness = format!("Fitness: {:.2?}", agent.fitness());
        let fsteps = format!(
            "FSteps: {:?}/{:?}",
            agent.game.no_food_steps,
            agent.get_step_limit()
        );
        let items = vec![current_score, fitness, fsteps];

        TermViz::widget_stats_block(title, items)
    }

    fn render_sim_stats(
        stats: &GenerationSummary,
        sim_start_ts: &Instant,
        mutation_rate: f64,
        mutation_magnitude: f64,
    ) -> impl Widget {
        let title = "  S I M    S T A T S  ";
        let elapsed = sim_start_ts.elapsed().as_secs_f32() / 60.0;
        let max_score = (GRID_SIZE - 1) * (GRID_SIZE - 1);
        let items = vec![
            format!("Gen: {0}", stats.gen_count),
            format!("Sim Max: {0}/{1}", stats.sim_max_score, max_score),
            format!("Gen Max: {0}/{1}", stats.gen_max_score, max_score),
            format!("Mutation Rate: {0}", mutation_rate),
            format!("Mutation Magnitude: {0}", mutation_magnitude),
            format!("Gen Max: {0}/{1}", stats.gen_max_score, max_score),
            format!("Gen Ts: {:.2} secs", stats.time_elapsed_secs),
            format!("Sim Ts: {:.2} mins", elapsed),
        ];

        TermViz::widget_stats_block(title, items)
    }

    fn render_about() -> impl Widget {
        let title = "  S N A K E   A I  ";
        let items = vec![
            format!("Num Agents: {NUM_AGENTS}"),
            format!("Step Limit: {NUM_STEPS}"),
            format!("Net Arch: {:?}", NN_ARCH),
            format!("Save Net: {IS_SAVE_BEST_NET}"),
            format!("Load Net: {IS_LOAD_SAVED_DATA}"),
            "".to_string(),
            "Press [ESC] to quit".to_string(),
        ];

        TermViz::widget_stats_block(title, items)
    }

    fn render_score_graph<'a>(data: &'a [u64]) -> impl Widget + 'a {
        TermViz::widget_sparkline(data, "  G E N    S C O R E S  ", Color::LightGreen)
    }

    fn render_gen_times_graph<'a>(data: &'a [u64]) -> impl Widget + 'a {
        TermViz::widget_sparkline(data, "  G E N    T I M E S  ", Color::LightCyan)
    }

    fn render_viz_score_gauge(score: usize) -> impl Widget {
        let ratio = score as f64 / ((GRID_SIZE - 1) * (GRID_SIZE - 1)) as f64;
        let ratio = ratio.min(1.0).max(0.0);
        let title = "  V I Z    S C O R E  ";
        TermViz::widget_gauge(ratio, title, Color::LightMagenta)
    }

    fn render_max_score_gauge(score: usize) -> impl Widget {
        let ratio = score as f64 / ((GRID_SIZE - 1) * (GRID_SIZE - 1)) as f64;
        let ratio = ratio.min(1.0).max(0.0);
        let title = "  M A X    S C O R E  ";
        TermViz::widget_gauge(ratio, title, Color::LightRed)
    }

    fn widget_stats_block<'a>(title: &'a str, items: Vec<String>) -> impl Widget + 'a {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .title(title.bold().into_centered_line().green());
        let mut list_items = Vec::new();
        for item in items {
            list_items.push(ListItem::new(Text::from(item).alignment(Alignment::Center)));
        }

        List::new(list_items).block(block)
    }

    fn widget_sparkline<'a>(data: &'a [u64], title: &'a str, color: Color) -> impl Widget + 'a {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .title(title.bold().into_centered_line().yellow());
        Sparkline::default()
            .block(block)
            .data(data)
            .style(Style::default().fg(color))
    }

    fn widget_gauge<'a>(ratio: f64, title: &'a str, color: Color) -> impl Widget + 'a {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Plain)
            .title(title.bold().into_centered_line().yellow());

        Gauge::default()
            .block(block)
            .gauge_style(Style::default().fg(color).add_modifier(Modifier::BOLD))
            .ratio(ratio)
    }

    fn widget_raw_text(message: String) -> impl Widget {
        Paragraph::new(message)
    }

    fn get_simple_render_text(viz: &VizData) -> String {
        let max_score = (GRID_SIZE - 1) * (GRID_SIZE - 1);
        let mut message = format!(
            "Gen: {:?}, Max: {:?}/{:?}, Gen_Max: {:?}/{:?}, Ts: {:.2?}, Sim_Ts: {:.2?}\nMR: {:.2?}, MG: {:.2?}\n\n",
            viz.stats.gen_count,
            viz.stats.sim_max_score,
            max_score,
            viz.stats.gen_max_score,
            max_score,
            viz.stats.time_elapsed_secs,
            (viz.sim_start_ts.elapsed().as_secs_f32() / 60.0),
            viz.mutation_rate,
            viz.mutation_magnitude
        );

        // Game Render
        if let Some(agent) = &viz.agent {
            let game = &agent.game;
            message.push_str(&TermViz::get_block_game_string(game));

            // Viz stats
            message.push_str("\n\n");
            message.push_str(
                format!(
                    "Score: {:?}/{:?}, Fitness: {:.2?}, FSteps: {:?}/{:?}\n",
                    agent.game.score(),
                    ((GRID_SIZE - 1) * (GRID_SIZE - 1)),
                    agent.fitness(),
                    agent.game.no_food_steps,
                    agent.get_step_limit()
                )
                .as_str(),
            );
        }

        message
    }

    fn get_block_game_string(game: &Game) -> String {
        let mut game_grid = String::new();
        for x in 0..=GRID_SIZE {
            for y in 0..=GRID_SIZE {
                let pt = (x, y).into();
                if game.food == pt {
                    game_grid.push_str("▒▒");
                    continue;
                }
                if game.is_wall(pt) {
                    game_grid.push_str("▓▓");
                    continue;
                }
                if game.is_snake_body(pt) {
                    game_grid.push_str("██");
                    continue;
                }
                if game.head == pt {
                    game_grid.push_str("██");
                    continue;
                }
                game_grid.push_str("  ");
            }
            game_grid.push('\n');
        }

        game_grid
    }
}

// NN Viz
impl TermViz {
    #[rustfmt::skip]
    fn get_network_text<'a>() -> Vec<Vec<&'a str>> {
        // This is the network that will be drawn on the terminal
        // TODO make this work for any network size
        vec![
            vec!["LF S ● ━━ · "],
            vec!["LF F ● ━━ · ╲"],
            vec!["RT S ● ━━ · ╲╲"],
            vec!["RT F ● ━━ · ╲╲╲"],
            vec!["BT S ● ━━ · ╲╲╲", " · ━━ ● ━━ · "],
            vec!["BT F ● ━━ · ╲╲╲", " · ━━ ● ━━ · ╲"],
            vec!["TP S ● ━━ · ╲╲╲", " · ━━ ● ━━ · ╲╲"],
            vec!["TP F ● ━━ · ╲╲╲", " · ━━ ● ━━ · ╲╲╲"],
            vec!["TL S ● ━━ · ╲╲╲", " · ━━ ● ━━ · ╲╲╲ ", "· ━━ ● ━━ ·"],
            vec!["TL F ● ━━ · ╲╲╲", " · ━━ ● ━━ · ╲╲╲ ", "· ━━ ● ━━ · ╲╲"],
            vec!["TR S ● ━━ · ╲╲╲", " · ━━ ● ━━ · ╲╲╲ ", "· ━━ ● ━━ · ╲╲╲", " · ━━ ● LEFT"],
            vec!["TR F ● ━━ · ╲╲╲", " · ━━ ● ━━ · ╲╲╲ ", "· ━━ ● ━━ · ╲╲╲", " · ━━ ● RIGHT"],
            vec!["BR S ● ━━ · ╱╱╱", " · ━━ ● ━━ · ╱╱╱ ", "· ━━ ● ━━ · ╱╱╱", " · ━━ ● BOTTOM"],
            vec!["BR F ● ━━ · ╱╱╱", " · ━━ ● ━━ · ╱╱╱ ", "· ━━ ● ━━ · ╱╱╱", " · ━━ ● TOP"],
            vec!["BL S ● ━━ · ╱╱╱", " · ━━ ● ━━ · ╱╱╱ ", "· ━━ ● ━━ · ╱╱"],
            vec!["BL F ● ━━ · ╱╱╱", " · ━━ ● ━━ · ╱╱╱ ", "· ━━ ● ━━ ·"],
            vec!["HE L ● ━━ · ╱╱╱", " · ━━ ● ━━ · ╱╱╱"],
            vec!["HE R ● ━━ · ╱╱╱", " · ━━ ● ━━ · ╱╱"],
            vec!["HE B ● ━━ · ╱╱╱", " · ━━ ● ━━ · ╱"],
            vec!["HE T ● ━━ · ╱╱╱", " · ━━ ● ━━ · "],
            vec!["TA L ● ━━ · ╱╱╱"],
            vec!["TA R ● ━━ · ╱╱"],
            vec!["TA B ● ━━ · ╱"],
            vec!["TA T ● ━━ · "],
        ]
    }

    fn get_node_colors(agent: &Agent) -> NNColors {
        let disabled_color = Color::DarkGray;
        let nn_input = agent.get_brain_input();
        let nn_output = agent.get_brain_output();

        // Process the input to get a list of colors for the input layer
        let mut inp_colors = Vec::new();
        for (i, val) in nn_input.iter().enumerate() {
            // These are 1-hot encoded head and tail directions
            if i >= (NN_ARCH[0] - 8) {
                if *val >= 1.0 {
                    inp_colors.push(Color::Cyan);
                } else {
                    inp_colors.push(disabled_color);
                }
                continue;
            }

            // Odd nodes are food booleans
            if i % 2 != 0 {
                if *val >= 1.0 {
                    inp_colors.push(Color::White);
                } else {
                    inp_colors.push(disabled_color);
                }
                continue;
            }

            // Even nodes - solid collision values
            if *val >= 1.0 {
                inp_colors.push(Color::LightMagenta);
            } else if *val >= 0.5 {
                inp_colors.push(Color::Indexed(104));
            } else if *val >= 0.15 {
                inp_colors.push(Color::Indexed(104));
            } else {
                inp_colors.push(disabled_color);
            }
        }

        // Process Layer 2 - Hidden layer 1
        let mut hidden_1_colors = Vec::new();
        let mut hidden_2_colors = Vec::new();
        for i in agent.brain.get_bias(0) {
            if i <= 0.5 {
                hidden_1_colors.push(Color::Indexed(248));
            } else {
                hidden_1_colors.push(Color::Indexed(240));
            }
        }
        for i in agent.brain.get_bias(1) {
            if i <= 0.3 {
                hidden_2_colors.push(Color::Indexed(242));
            } else {
                hidden_2_colors.push(Color::Indexed(237));
            }
        }

        // Process output colors
        let mut out_colors = vec![
            Color::Indexed(242),
            Color::Indexed(242),
            Color::Indexed(242),
            Color::Indexed(242),
        ];
        let result_color = Color::LightMagenta;
        match nn_output {
            FourDirs::Left => {
                out_colors[0] = result_color;
            }
            FourDirs::Right => {
                out_colors[1] = result_color;
            }
            FourDirs::Bottom => {
                out_colors[3] = result_color;
            }
            FourDirs::Top => {
                out_colors[2] = result_color;
            }
        }

        NNColors {
            disabled_color,
            inp_colors,
            hidden_1_colors,
            hidden_2_colors,
            out_colors,
        }
    }

    fn render_nn(agent: &Agent) -> impl Widget {
        if NN_ARCH != [24, 16, 8, 4] {
            let block = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain);
            return Paragraph::new("Can only visualize network with arch [24, 16, 8, 4]")
                .block(block);
        }

        let network: Vec<Vec<&str>> = TermViz::get_network_text();
        let colors = TermViz::get_node_colors(agent);

        let mut lines = Vec::new();
        let mut layer_1_idx = 0;
        let mut layer_2_idx = 0;
        let mut layer_3_idx = 0;
        let mut layer_4_idx = 0;

        for parts in network.iter() {
            let mut line_spans = Vec::new();
            let parts_len = parts.len();
            for (i, part) in parts.iter().enumerate() {
                let mut color = colors.disabled_color;

                // Layer 1
                if i == 0 {
                    color = colors.inp_colors[layer_1_idx];
                }
                // Layer 2
                if i == 1 {
                    color = colors.hidden_1_colors[layer_2_idx];
                }
                // Layer 3
                if i == 2 {
                    color = colors.hidden_2_colors[layer_3_idx];
                }
                // Layer 4
                if i == 3 {
                    color = colors.out_colors[layer_4_idx];
                }

                if parts_len >= 1 && i == 0 {
                    layer_1_idx += 1;
                }
                if parts_len >= 2 && i == 1 {
                    layer_2_idx += 1;
                }
                if parts_len >= 3 && i == 2 {
                    layer_3_idx += 1;
                }
                if parts_len >= 4 && i == 3 {
                    layer_4_idx += 1;
                }

                line_spans.push(Span::styled(*part, Style::default().fg(color)));
            }

            lines.push(Line::from(line_spans));
        }

        let block = Block::default().padding(Padding::new(0, 0, 5, 0));
        Paragraph::new(lines).block(block)
    }
}

impl TermViz {
    fn display_game_blocks(game: &Game) -> impl Widget {
        let mut lines = Vec::new();
        let body_color = match game.is_dead {
            true => COLOR_DEAD,
            false => COLOR_BODY,
        };
        let head_color = match game.is_dead {
            true => COLOR_DEAD,
            false => COLOR_HEAD,
        };

        for x in 0..=GRID_SIZE {
            let mut line_spans = Vec::new();
            for y in 0..=GRID_SIZE {
                let pt = (x, y).into();
                if game.food == pt {
                    line_spans.push(Span::styled("██", Style::default().fg(COLOR_FOOD)));
                    continue;
                }
                if game.is_wall(pt) {
                    line_spans.push(Span::styled("██", Style::default().fg(COLOR_WALLS)));
                    continue;
                }
                if game.is_snake_body(pt) {
                    line_spans.push(Span::styled("██", Style::default().fg(body_color)));
                    continue;
                }
                if game.head == pt {
                    line_spans.push(Span::styled("██", Style::default().fg(head_color)));
                    continue;
                }
                line_spans.push(Span::styled("  ", Style::default()));
            }
            lines.push(Line::from(line_spans));
        }

        let block = Block::default().padding(Padding::new(8, 0, 8, 0));
        Paragraph::new(lines).block(block)
    }
}

impl<'a> GameRender<'a> {
    fn draw_rect(&self, painter: &mut Painter, point: Point, color: Color) {
        let scale = VIZ_GAME_SCALE;
        let offset = ((VIZ_OFFSET) * scale, (VIZ_OFFSET + 2) * scale);
        for dx in 0..scale {
            for dy in 0..scale {
                let x = point.x * scale + dx + offset.0;
                let y = point.y * scale + dy + offset.1;
                painter.paint(x as usize, y as usize, color);
            }
        }
    }

    fn draw_border(&self, painter: &mut Painter) {
        for i in 0..=GRID_SIZE {
            for j in 0..=GRID_SIZE {
                if i == 0 || i == GRID_SIZE || j == 0 || j == GRID_SIZE {
                    self.draw_rect(painter, Point { x: i, y: j }, COLOR_WALLS);
                }
            }
        }
    }

    fn draw_snake(&self, painter: &mut Painter) {
        let body_color = match self.game.is_dead {
            true => COLOR_DEAD,
            false => COLOR_BODY,
        };
        let head_color = match self.game.is_dead {
            true => COLOR_DEAD,
            false => COLOR_HEAD,
        };
        for &segment in &self.game.body {
            self.draw_rect(painter, segment.into(), body_color);
        }
        self.draw_rect(painter, self.game.head.into(), head_color);
    }

    fn draw_food(&self, painter: &mut Painter) {
        self.draw_rect(painter, self.game.food.into(), COLOR_FOOD);
    }
}

impl Shape for GameRender<'_> {
    fn draw(&self, painter: &mut Painter) {
        self.draw_border(painter);
        self.draw_snake(painter);
        self.draw_food(painter);
    }
}

impl Default for VizData {
    fn default() -> Self {
        Self {
            agent: None,
            stats: GenerationSummary::default(),
            sim_start_ts: Instant::now(),
            scores: Vec::new(),
            gen_times: Vec::new(),
            mutation_magnitude: 0.0,
            mutation_rate: 0.0,
        }
    }
}
