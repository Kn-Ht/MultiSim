use crate::gol_cell::Cell;
use crate::gol_consts::*;
use crate::gol_theme::Theme;
use crate::gol_universe::Universe;
use crate::selector::SelectedGame;
use raylib::{
    math::Vector2 as Vec2,
    texture::Texture2D
};

/*****************************************************************/

#[derive(Clone, Debug)]
enum State {
    DesignMode,
    SimulationMode,
    HelpMode,
}

impl State {
    pub fn toggle(&mut self) {
        *self = match self {
            State::DesignMode => Self::SimulationMode,
            State::SimulationMode => Self::DesignMode,
            State::HelpMode => Self::HelpMode,
        };
    }
}

/// game struct
pub struct GameOfLife {
    // game state
    universe: Universe,
    state: State,
    // delta time
    update_frame_cap: f32,
    passed_time: f32,
    passed_resize_time: f32,
    resize_queued: bool,
    // meta information
    mouse_pos: Vec2,
    fullscreen: bool,
    window_width: u32,
    window_height: u32,
    // themes
    theme: Theme,
    // secret!
    bolus: Texture2D,
}

impl crate::game::Game for GameOfLife {
    fn new() -> Self {
        Self {
            universe: Universe::new(GRID_W, GRID_H),
            state: State::HelpMode,
            update_frame_cap: DEFAULT_UPDATE_CAP,
            passed_time: 0.0,
            passed_resize_time: 0.0,
            resize_queued: false,
            mouse_pos: (0.0, 0.0).into(),
            window_width: 0,
            window_height: 0,
            theme: Theme::Default,
            bolus: Texture2D::from_file_with_format(BOLUS, None),
            fullscreen: false,
        }
    }
    fn update(&mut self) -> SelectedGame {
        if self.passed_time >= self.update_frame_cap {
            self.passed_time = 0.0;
        }
        let dt = get_frame_time();
        self.passed_time += dt;
        self.passed_resize_time += dt;

        let should_quit = self.handle_keys();

        let size_changed = self.screen_size_changed();
        if size_changed || self.resize_queued {
            if size_changed {
                //  self.enforce_minimun_screen_size();
            }
            if self.passed_resize_time >= RESIZE_TIME_LIMIT {
                let new_w = self.window_width / SCALE_U32;
                let new_h = self.window_height / SCALE_U32;

                self.universe.resize(new_w as usize, new_h as usize);
                self.resize_queued = false;
                self.passed_resize_time = 0.0;
            } else {
                self.resize_queued = true;
            }
        }

        // get mouse position inside grid
        self.mouse_pos = Vec2::from(mouse_position()).floor().clamp(
            vec2(0.0, 0.0),
            Vec2::new(
                self.window_width as f32 - 1.0,
                self.window_height as f32 - 1.0,
            ),
        ) / SCALE as f32;

        // store theme
        let (bg, fg, accent) = self.theme.to_style().into();

        match self.state {
            State::SimulationMode => {
                if self.passed_time >= self.update_frame_cap {
                    self.update_cells();
                    self.passed_time = 0.0;
                }
            }
            State::DesignMode => {
                if is_mouse_button_down(MouseButton::Left) {
                    self.universe.set(
                        self.mouse_pos.x as usize,
                        self.mouse_pos.y as usize,
                        Cell::Alive,
                    );
                } else if is_mouse_button_down(MouseButton::Right) {
                    self.universe.set(
                        self.mouse_pos.x as usize,
                        self.mouse_pos.y as usize,
                        Cell::Dead,
                    );
                }
            }
            _ => {}
        }

        clear_background(bg);

        // if help mode
        if let State::HelpMode = self.state {
            const FONT_S: f32 = 19.0;
            const FONT_M: f32 = 30.0;
            const FONT_L: f32 = 35.0;
            const FONT_XL: f32 = 60.0;

            let mut spacing = FONT_L;

            let controls = [
                "Left mouse button  - make cell alive",
                "Right mouse button - make cell dead",
                "Space              - pause/unpause game",
                "H                  - help menu",
                "C                  - clear the board",
                "A                  - fill the board with live cells",
                "I                  - invert cells",
                "R                  - generate a random pattern",
                &format!(
                    "T                  - switch themes (currently: {:?})",
                    self.theme
                ),
                &format!(
                    "+                  - add {UPDATE_TIME_STEP:.2}s to update time ({:.2}s)",
                    self.update_frame_cap
                ),
                &format!(
                    "-                  - subtract {UPDATE_TIME_STEP:.2}s to update time ({:.2}s)",
                    self.update_frame_cap
                ),
                &format!("F11                - toggle fullscreen (now: {})", self.fullscreen),
            ];

            draw_text("CONTROLS", 0.0, spacing, FONT_XL, accent);
            spacing += FONT_L;

            for control in controls {
                draw_text(control, 0.0, spacing, FONT_M, fg);
                spacing += FONT_M;
            }

            draw_text(
                "Press Enter or H to start the game",
                0.0,
                self.window_height as f32 - FONT_L * 2.,
                FONT_L,
                fg,
            );
            draw_text(
                "Press Escape to exit to the menu",
                0.0,
                self.window_height as f32 - FONT_L,
                FONT_L,
                fg,
            );
            draw_text(
                "Made by Kian (Kvoid)",
                self.window_width as f32 - 190.0,
                self.window_height as f32 - FONT_S,
                FONT_S,
                accent,
            );
            return if is_key_down(KeyCode::Escape) {
                SelectedGame::None
            } else {
                SelectedGame::GameOfLife
            }
        }

        // draw the cells
        const SCALE_F: f32 = SCALE as f32;

        match self.theme {
            Theme::Midnight => {
                for y in 0..self.universe.height {
                    for x in 0..self.universe.width {
                        draw_rectangle(
                            (x * SCALE) as f32,
                            (y * SCALE) as f32,
                            SCALE_F,
                            SCALE_F,
                            if let Cell::Alive = self.universe.get(x, y) {
                                color_u8!(x, y, 100, 255)
                            } else {
                                bg
                            },
                        );
                    }
                }
            }
            Theme::Bolus => {
                for y in 0..self.universe.height {
                    for x in 0..self.universe.width {
                        if let Cell::Alive = self.universe.get(x, y) {
                            draw_texture_ex(
                                &self.bolus,
                                (x * SCALE) as f32,
                                (y * SCALE) as f32,
                                WHITE, BOLUS_PARAMS
                            )
                        } else {
                            draw_rectangle(
                                (x * SCALE) as f32,
                                (y * SCALE) as f32,
                                SCALE_F,
                                SCALE_F,
                                BLACK
                            );
                        }
                    }
                }
            },
            _ => {
                for y in 0..self.universe.height {
                    for x in 0..self.universe.width {
                        draw_rectangle(
                            (x * SCALE) as f32,
                            (y * SCALE) as f32,
                            SCALE_F,
                            SCALE_F,
                            if let Cell::Alive = self.universe.get(x, y) {
                                fg
                            } else {
                                bg
                            },
                        );
                    }
                }
            }
        }

        // draw mouse hover
        if let State::DesignMode = self.state {
            draw_rectangle_lines(
                self.mouse_pos.x.floor() * SCALE_F,
                self.mouse_pos.y.floor() * SCALE_F,
                SCALE_F,
                SCALE_F,
                2.0,
                accent,
            );

            draw_text(
                "[DESIGN MODE]",
                0.0,
                self.window_height as f32 - 10.0,
                34.0,
                accent,
            );

            draw_text(
                &format!("UPDATE: {:.2}s", self.update_frame_cap),
                0.0,
                25.0,
                30.0,
                accent,
            );
            if let Theme::Bolus = self.theme {
                draw_text("THEME: Bolus mode!!! (cred Fernando)", 0.0, 50.0, 30.0, accent);
            } else {
                draw_text(&format!("THEME: {:?}", self.theme), 0.0, 50.0, 30.0, accent);
            }
        }

        if should_quit {
            SelectedGame::None
        } else {
            SelectedGame::GameOfLife
        }
    }
}

impl GameOfLife {
    fn screen_size_changed(&mut self) -> bool {
        let new_width = screen_width().floor() as u32;
        let new_height = screen_height().floor() as u32;

        let changed = new_width != self.window_width || new_height != self.window_height;

        self.window_width = new_width;
        self.window_height = new_height;

        changed
    }
    /// returns true if Esc pressed, else false
    fn handle_keys(&mut self) -> bool {
        if let Some(key) = get_last_key_pressed() {
            match key {
                KeyCode::H => {
                    if let State::HelpMode = self.state {
                        self.state = State::DesignMode;
                    } else {
                        self.state = State::HelpMode;
                    }
                }
                KeyCode::C => {
                    self.universe.fill(Cell::Dead);
                }
                KeyCode::A => {
                    self.universe.fill(Cell::Alive);
                }
                KeyCode::T => {
                    self.theme.cycle();
                }
                KeyCode::I => {
                    self.universe.invert();
                }
                KeyCode::R => {
                    self.universe.fill_random();
                }
                KeyCode::B => {
                    self.theme.toggle_bolus();
                }
                KeyCode::Equal | KeyCode::KpEqual => {
                    self.update_frame_cap += UPDATE_TIME_STEP;
                }
                KeyCode::KpSubtract | KeyCode::Minus => {
                    self.update_frame_cap =
                        (self.update_frame_cap - UPDATE_TIME_STEP).clamp(0.0, f32::MAX);
                }
                KeyCode::Space => {
                    self.state.toggle();
                }
                KeyCode::Enter => {
                    if let State::HelpMode = self.state {
                        self.state = State::DesignMode;
                    }
                }
                KeyCode::Escape => return true,
                KeyCode::F11 => {
                    self.fullscreen = !self.fullscreen;
                    set_fullscreen(self.fullscreen);
                }
                _ => {}
            }
        }
        false
    }
    fn update_cells(&mut self) {
        let mut next_generation = self.universe.clone();

        for y in 0..self.universe.height {
            for x in 0..self.universe.width {
                let cell_state = self.universe.get(x, y);
                let mut neighbours = 0;

                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;

                        if nx >= 0
                            && nx < self.universe.width as i32
                            && ny >= 0
                            && ny < self.universe.height as i32
                            && matches!(self.universe.get(nx as usize, ny as usize), Cell::Alive)
                        {
                            neighbours += 1;
                        }
                    }
                }

                let new_state = cell_state.calculate_next_iteration(neighbours);
                next_generation.set(x, y, new_state);
            }
        }

        self.universe = next_generation;
    }
}
