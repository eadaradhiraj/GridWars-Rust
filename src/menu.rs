use macroquad::prelude::*;
use crate::colors::*;
use crate::highscores::HighScores;
use crate::vector_font::{draw_string, draw_string_centered};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AttractScreen {
    Title,
    Scores,
    Friends,
    Enemies,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ControlMode {
    ArrowKeysAndSpace,
    TwinStickWasdAndArrows,
    MouseAim,
}

pub struct MenuState {
    pub attract_screen: AttractScreen,
    pub screen_timer: f32,
    pub control_mode: ControlMode,
    pub sound_fx: bool,
    pub selected_option: usize,
    pub options_open: bool,
    pub game_over_timer: f32,
}

impl MenuState {
    pub fn new() -> Self {
        Self {
            attract_screen: AttractScreen::Title,
            screen_timer: 0.0,
            control_mode: ControlMode::ArrowKeysAndSpace,
            sound_fx: true,
            selected_option: 0,
            options_open: false,
            game_over_timer: 0.0,
        }
    }

    pub fn update_attract(&mut self) {
        self.screen_timer += 1.0;
        // Cycle every 400 frames (~6.5 seconds)
        if self.screen_timer > 400.0 {
            self.screen_timer = 0.0;
            self.attract_screen = match self.attract_screen {
                AttractScreen::Title => AttractScreen::Scores,
                AttractScreen::Scores => AttractScreen::Friends,
                AttractScreen::Friends => AttractScreen::Enemies,
                AttractScreen::Enemies => AttractScreen::Title,
            };
        }
    }

    pub fn draw_attract(&self, highscores: &HighScores, width: f32, _height: f32) {
        let center_x = width * 0.5;

        match self.attract_screen {
            AttractScreen::Title => {
                // Pulsing title glow
                let pulse = (self.screen_timer * 0.05).sin() * 0.2 + 0.8;
                let title_col = Color::new(COL_PLAYER.r * pulse, COL_PLAYER.g * pulse, COL_PLAYER.b * pulse, 1.0);

                draw_string_centered("G R I D   W A R S", center_x, 140.0, 5.0, title_col, 3.0);
                draw_string_centered("PURE RUST EDITION", center_x, 195.0, 1.8, COL_DIAMOND, 1.5);

                draw_string_centered("PRESS SPACE OR ENTER TO PLAY", center_x, 280.0, 2.5, COL_POWERUP, 2.0);

                // Controls overview box
                draw_rectangle_lines(center_x - 260.0, 340.0, 520.0, 240.0, 1.5, COL_GRID_ACCENT);
                draw_string_centered("- CONTROLS -", center_x, 360.0, 2.0, COL_SCORE, 1.5);

                draw_string("ARROW KEYS / WASD : MOVE SHIP", center_x - 210.0, 400.0, 1.8, WHITE, 1.5);
                draw_string("SPACEBAR          : SHOOT FORWARD", center_x - 210.0, 435.0, 1.8, WHITE, 1.5);
                draw_string("TWIN-STICK ARROWS : 360 DEGREE AIM", center_x - 210.0, 470.0, 1.8, WHITE, 1.5);
                draw_string("B / SHIFT / RMB   : SMART BOMB", center_x - 210.0, 505.0, 1.8, WHITE, 1.5);
                draw_string("ESC / TAB         : OPTIONS MENU", center_x - 210.0, 540.0, 1.8, WHITE, 1.5);

                draw_string_centered("ORIGINAL GAME BY MARK INC - PORTED TO PURE RUST", center_x, 660.0, 1.4, Color::new(0.6, 0.6, 0.8, 1.0), 1.0);
            }

            AttractScreen::Scores => {
                draw_string_centered("TOP 10 PILOTS", center_x, 100.0, 3.5, COL_SCORE, 2.5);

                let start_y = 170.0;
                let step_y = 42.0;

                draw_string("RANK", center_x - 280.0, start_y, 1.8, COL_POWERUP, 1.5);
                draw_string("NAME", center_x - 180.0, start_y, 1.8, COL_POWERUP, 1.5);
                draw_string("SCORE", center_x + 50.0, start_y, 1.8, COL_POWERUP, 1.5);
                draw_string("TIME", center_x + 220.0, start_y, 1.8, COL_POWERUP, 1.5);

                for (i, entry) in highscores.entries.iter().enumerate() {
                    let y = start_y + ((i + 1) as f32) * step_y;
                    let rank_str = format!("{:2}.", i + 1);
                    let score_str = format!("{:>8}", entry.score);

                    let col = if i == 0 {
                        COL_POWERUP
                    } else if i < 3 {
                        COL_DIAMOND
                    } else {
                        WHITE
                    };

                    draw_string(&rank_str, center_x - 280.0, y, 1.7, col, 1.2);
                    draw_string(&entry.name, center_x - 180.0, y, 1.7, col, 1.2);
                    draw_string(&score_str, center_x + 50.0, y, 1.7, col, 1.2);
                    draw_string(&entry.time, center_x + 220.0, y, 1.7, col, 1.2);
                }

                draw_string_centered("PRESS SPACE TO PLAY", center_x, 690.0, 2.0, COL_SCORE, 1.5);
            }

            AttractScreen::Friends => {
                draw_string_centered("ALLIED SHIPS & POWERUPS", center_x, 100.0, 3.2, COL_PLAYER, 2.0);

                let p1_y = 190.0;
                draw_circle_lines(center_x - 180.0, p1_y + 10.0, 22.0, 2.0, COL_PLAYER);
                draw_string("PLAYER SHIP", center_x - 130.0, p1_y, 2.0, WHITE, 1.5);
                draw_string("AGILE VECTOR FIGHTER WITH DUAL ENGINES", center_x - 130.0, p1_y + 25.0, 1.4, Color::new(0.7, 0.7, 0.7, 1.0), 1.0);

                let p2_y = 280.0;
                draw_circle_lines(center_x - 180.0, p2_y + 10.0, 20.0, 2.0, COL_POWERUP);
                draw_string_centered("3-WAY", center_x - 180.0, p2_y + 5.0, 1.2, COL_POWERUP, 1.5);
                draw_string("TRIPLE SPREAD SHOT", center_x - 130.0, p2_y, 2.0, COL_POWERUP, 1.5);
                draw_string("FIRES A 3-WAY SPREAD DESTROYING ENTIRE HORDES", center_x - 130.0, p2_y + 25.0, 1.4, Color::new(0.7, 0.7, 0.7, 1.0), 1.0);

                let p3_y = 370.0;
                draw_circle_lines(center_x - 180.0, p3_y + 10.0, 20.0, 2.0, COL_SHOT_SUPER);
                draw_string_centered("SUPER", center_x - 180.0, p3_y + 5.0, 1.2, COL_SHOT_SUPER, 1.5);
                draw_string("SUPER PENETRATING SHOT", center_x - 130.0, p3_y, 2.0, COL_SHOT_SUPER, 1.5);
                draw_string("LASER BOLTS PENETRATE THROUGH MULTIPLE ENEMIES", center_x - 130.0, p3_y + 25.0, 1.4, Color::new(0.7, 0.7, 0.7, 1.0), 1.0);

                let p4_y = 460.0;
                draw_circle_lines(center_x - 180.0, p4_y + 10.0, 20.0, 2.0, COL_SHOT_BOUNCY);
                draw_string_centered("BOUNCE", center_x - 180.0, p4_y + 5.0, 1.2, COL_SHOT_BOUNCY, 1.5);
                draw_string("BOUNCING PROJECTILES", center_x - 130.0, p4_y, 2.0, COL_SHOT_BOUNCY, 1.5);
                draw_string("DEFLECTS OFF SCREEN BORDERS CREATING A LETHAL MAZE", center_x - 130.0, p4_y + 25.0, 1.4, Color::new(0.7, 0.7, 0.7, 1.0), 1.0);

                let p5_y = 550.0;
                draw_circle_lines(center_x - 180.0, p5_y + 10.0, 20.0, 2.0, COL_BOMB);
                draw_string_centered("BOMB", center_x - 180.0, p5_y + 5.0, 1.2, COL_BOMB, 1.5);
                draw_string("SMART BOMB (B / SHIFT)", center_x - 130.0, p5_y, 2.0, COL_BOMB, 1.5);
                draw_string("SENDS SHOCKWAVES CLEARING ALL ENEMIES ON SCREEN", center_x - 130.0, p5_y + 25.0, 1.4, Color::new(0.7, 0.7, 0.7, 1.0), 1.0);

                draw_string_centered("PRESS SPACE TO PLAY", center_x, 690.0, 2.0, COL_PLAYER, 1.5);
            }

            AttractScreen::Enemies => {
                draw_string_centered("KNOWN HOSTILES", center_x, 90.0, 3.2, COL_SUN, 2.0);

                let enemies_info = [
                    ("PINWHEEL", "25 PTS", "SPINS & BOUNCES OFF WALLS", COL_PIN),
                    ("DIAMOND", "50 PTS", "SEEKER - CHASES YOUR EXACT POSITION", COL_DIAMOND),
                    ("SQUARE", "100 PTS", "WANDERER - DEFLECTS AT DIAGONAL ANGLES", COL_SQUARE),
                    ("SUN / BLACK HOLE", "250 PTS", "GRAVITATIONAL SINGULARITY - WARPS GRID", COL_SUN),
                    ("SLITHERING SNAKE", "250 PTS", "MULTI-NODE BODY WITH ARMORED TAIL", COL_SNAKE),
                    ("SHIELDED TRIANGLE", "150 PTS", "IMMUNE TO DIRECT HEAD-ON FIRE", COL_TRIANGLE),
                    ("BUTTERFLY", "100 PTS", "WEAVER - EVADES INCOMING BULLETS", COL_BUTTER),
                ];

                for (i, (name, pts, desc, col)) in enemies_info.iter().enumerate() {
                    let y = 160.0 + (i as f32) * 68.0;
                    draw_circle_lines(center_x - 220.0, y + 10.0, 16.0, 2.0, *col);
                    draw_string(name, center_x - 180.0, y, 1.8, *col, 1.5);
                    draw_string(pts, center_x + 120.0, y, 1.8, COL_POWERUP, 1.5);
                    draw_string(desc, center_x - 180.0, y + 24.0, 1.3, Color::new(0.7, 0.7, 0.7, 1.0), 1.0);
                }

                draw_string_centered("PRESS SPACE TO PLAY", center_x, 690.0, 2.0, COL_SUN, 1.5);
            }
        }
    }

    pub fn handle_options_input(&mut self) -> bool {
        // returns true if should close options
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Tab) {
            self.options_open = false;
            return true;
        }

        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            if self.selected_option > 0 {
                self.selected_option -= 1;
            } else {
                self.selected_option = 4;
            }
        }

        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            if self.selected_option < 4 {
                self.selected_option += 1;
            } else {
                self.selected_option = 0;
            }
        }

        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::Left) {
            match self.selected_option {
                0 => {
                    self.control_mode = match self.control_mode {
                        ControlMode::ArrowKeysAndSpace => ControlMode::TwinStickWasdAndArrows,
                        ControlMode::TwinStickWasdAndArrows => ControlMode::MouseAim,
                        ControlMode::MouseAim => ControlMode::ArrowKeysAndSpace,
                    };
                }
                1 => {
                    self.sound_fx = !self.sound_fx;
                }
                2 => {
                    // Resume game
                    self.options_open = false;
                    return true;
                }
                3 => {
                    // Back to Title
                    self.options_open = false;
                    return true;
                }
                4 => {
                    // Quit game
                    std::process::exit(0);
                }
                _ => {}
            }
        }

        false
    }

    pub fn draw_options(&self, width: f32, height: f32) {
        let center_x = width * 0.5;
        let center_y = height * 0.5;

        // Dark modal overlay
        draw_rectangle(0.0, 0.0, width, height, Color::new(0.0, 0.0, 0.0, 0.8));
        draw_rectangle_lines(center_x - 280.0, center_y - 200.0, 560.0, 400.0, 2.5, COL_BORDER);

        draw_string_centered("CONFIGURATION OPTIONS", center_x, center_y - 150.0, 2.8, COL_POWERUP, 2.0);

        let control_str = match self.control_mode {
            ControlMode::ArrowKeysAndSpace => "ARROWS + SPACE",
            ControlMode::TwinStickWasdAndArrows => "TWIN-STICK (WASD+ARROWS)",
            ControlMode::MouseAim => "MOUSE AIM & CLICK",
        };

        let sfx_str = if self.sound_fx { "ENABLED" } else { "MUTED" };

        let options = [
            format!("CONTROL SCHEME : {}", control_str),
            format!("SOUND EFFECTS  : {}", sfx_str),
            "RESUME GAME".to_string(),
            "RETURN TO TITLE".to_string(),
            "QUIT GAME".to_string(),
        ];

        for (i, opt) in options.iter().enumerate() {
            let y = center_y - 60.0 + (i as f32) * 50.0;
            let is_sel = self.selected_option == i;
            let col = if is_sel { COL_PLAYER } else { Color::new(0.6, 0.6, 0.6, 1.0) };
            let prefix = if is_sel { "> " } else { "  " };
            let line = format!("{}{}", prefix, opt);
            draw_string(&line, center_x - 220.0, y, 1.8, col, if is_sel { 1.8 } else { 1.2 });
        }

        draw_string_centered("USE ARROW KEYS & ENTER TO ADJUST", center_x, center_y + 160.0, 1.4, COL_DIAMOND, 1.0);
    }

    pub fn draw_hud(&self, player: &crate::player::Player, width: f32, _height: f32) {
        // Score (top left)
        let score_str = format!("SCORE: {:>8}", player.score);
        draw_string(&score_str, 20.0, 20.0, 2.2, COL_SCORE, 1.8);

        // Multiplier
        let mult_str = format!("x{}", player.multiplier);
        let mult_col = if player.multiplier > 1 { COL_POWERUP } else { WHITE };
        draw_string(&mult_str, 320.0, 20.0, 2.2, mult_col, 1.8);

        // Lives icons (top right)
        let lives_x = width - 240.0;
        draw_string("LIVES:", lives_x, 20.0, 1.8, COL_PLAYER, 1.5);
        for i in 0..player.lives.max(0) {
            let lx = lives_x + 90.0 + (i as f32) * 24.0;
            let ly = 28.0;
            // Draw mini vector ship icon
            draw_line(lx, ly - 8.0, lx - 6.0, ly + 6.0, 1.5, COL_PLAYER);
            draw_line(lx - 6.0, ly + 6.0, lx, ly + 2.0, 1.5, COL_PLAYER);
            draw_line(lx, ly + 2.0, lx + 6.0, ly + 6.0, 1.5, COL_PLAYER);
            draw_line(lx + 6.0, ly + 6.0, lx, ly - 8.0, 1.5, COL_PLAYER);
        }

        // Bombs icons
        let bombs_x = width - 110.0;
        draw_string("B:", bombs_x, 20.0, 1.8, COL_BOMB, 1.5);
        for i in 0..player.bombs.max(0) {
            let bx = bombs_x + 35.0 + (i as f32) * 20.0;
            let by = 28.0;
            draw_circle_lines(bx, by, 6.0, 1.5, COL_BOMB);
        }
    }
}
