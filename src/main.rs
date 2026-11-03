#![allow(dead_code)]
mod audio;
mod colors;
mod enemies;
mod grid;
mod highscores;
mod menu;
mod particles;
mod player;
mod shots;
mod vector_font;

use macroquad::prelude::*;

use audio::AudioManager;
use colors::*;
use enemies::{EnemyKind, EnemyManager, EnemyTextures};
use grid::Grid;
use highscores::HighScores;
use menu::{MenuState, AttractScreen};
use particles::ParticleSystem;
use player::Player;
use shots::ShotManager;
use vector_font::draw_string_centered;

fn window_conf() -> Conf {
    Conf {
        window_title: "GridWars".to_string(),
        window_width: 1024,
        window_height: 768,
        window_resizable: false,
        ..Default::default()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameState {
    Attract,
    Playing,
    GameOver,
}

#[macroquad::main(window_conf)]
async fn main() {
    let playfield_w = 1024.0f32;
    let playfield_h = 768.0f32;

    // Load authentic sound effects & textures
    let mut audio = AudioManager::load().await;
    let enemy_textures = EnemyTextures::load().await;
    let player_tex = load_texture("gfx/high/whiteplayer.png").await.ok();
    let shot_tex = load_texture("gfx/high/yellowshot.png").await.ok();

    let mut highscores = HighScores::load_or_default();
    let mut grid = Grid::new(playfield_w, playfield_h, 24.0);
    let mut particles = ParticleSystem::new(300, playfield_w, playfield_h);
    let mut player = Player::new(playfield_w * 0.5, playfield_h * 0.5);
    let mut shots = ShotManager::new();
    let mut enemies = EnemyManager::new();
    let mut menu = MenuState::new();

    let mut state = GameState::Attract;
    let mut wave_timer = 0u64;
    let mut game_over_timer = 0.0f32;
    let mut ready_timer = 0.0f32;
    let mut elapsed_play_time = 0.0f32;

    loop {
        if is_key_pressed(KeyCode::Q) {
            std::process::exit(0);
        }

        clear_background(BLACK);

        match state {
            GameState::Attract => {
                audio.play_music(0);
                // Background ambient grid motion & starfield
                grid.update();
                particles.update(playfield_w, playfield_h);
                particles.draw_stars();
                grid.draw();
                particles.draw_particles();

                // Gentle ambient grid wave
                let t = get_time() as f32;
                grid.pull(
                    playfield_w * 0.5 + (t * 1.5).cos() * 200.0,
                    playfield_h * 0.5 + (t * 1.5).sin() * 150.0,
                    80.0,
                    0.6,
                );

                if menu.options_open {
                    if menu.handle_options_input() {
                        // Options closed
                    }
                    menu.draw_options(playfield_w, playfield_h);
                } else {
                    menu.update_attract();
                    menu.draw_attract(&highscores, playfield_w, playfield_h);

                    if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Tab) {
                        menu.options_open = true;
                    }

                    // Start Game on Space or Enter
                    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                        state = GameState::Playing;
                        player.reset(playfield_w * 0.5, playfield_h * 0.5);
                        shots.clear();
                        enemies.clear();
                        grid = Grid::new(playfield_w, playfield_h, 24.0);
                        wave_timer = 0;
                        ready_timer = 120.0; // 2 seconds "GET READY"
                        elapsed_play_time = 0.0;
                        audio.play_startup();
                    }
                }
            }

            GameState::Playing => {
                audio.play_music(1);
                elapsed_play_time += get_frame_time();

                if menu.options_open {
                    // Paused - only draw scene and options modal
                    particles.draw_stars();
                    grid.draw();
                    shots.draw(shot_tex.as_ref());
                    enemies.draw(&enemy_textures);
                    player.draw(player_tex.as_ref());
                    particles.draw_particles();
                    menu.draw_hud(&player, playfield_w, playfield_h);

                    if menu.handle_options_input() {
                        if menu.selected_option == 3 {
                            // Return to title
                            state = GameState::Attract;
                        }
                    }
                    menu.draw_options(playfield_w, playfield_h);
                } else {
                    // Check Pause / Options trigger
                    if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Tab) {
                        menu.options_open = true;
                    }

                    // Ready Countdown
                    if ready_timer > 0.0 {
                        ready_timer -= 1.0;
                    } else {
                        wave_timer += 1;

                        // Spawn logic based on original GridWars rhythms
                        let speed_mult = 1.0 + (wave_timer as f32 / 3600.0) * 0.4;

                        // Continuous single enemy spawns
                        if wave_timer % 32 == 0 {
                            let kind = match (wave_timer / 300) % 9 {
                                0 => EnemyKind::Pinwheel,
                                1 => EnemyKind::SeekerDiamond,
                                2 => EnemyKind::WandererSquare,
                                3 => EnemyKind::Quark,
                                4 => EnemyKind::Butterfly,
                                5 => EnemyKind::SplitterCube,
                                6 => EnemyKind::ShieldedTriangle,
                                7 => EnemyKind::Clone,
                                _ => EnemyKind::Pinwheel,
                            };
                            let corner = rand::gen_range(0, 4);
                            let (sx, sy) = match corner {
                                0 => (40.0, 40.0),
                                1 => (playfield_w - 40.0, 40.0),
                                2 => (playfield_w - 40.0, playfield_h - 40.0),
                                _ => (40.0, playfield_h - 40.0),
                            };
                            enemies.spawn_enemy(kind, sx, sy, 20);
                        }

                        // Generator spawns periodically
                        if wave_timer % 1500 == 0 {
                            let gx = rand::gen_range(150.0, playfield_w - 150.0);
                            let gy = rand::gen_range(150.0, playfield_h - 150.0);
                            enemies.spawn_enemy(EnemyKind::Generator, gx, gy, 40);
                        }

                        // Black Hole spawns periodically
                        if wave_timer % 750 == 0 {
                            let bx = rand::gen_range(100.0, playfield_w - 100.0);
                            let by = rand::gen_range(100.0, playfield_h - 100.0);
                            enemies.spawn_enemy(EnemyKind::BlackHole, bx, by, 30);
                            audio.play_sun_born();
                        }

                        // Snake spawns
                        if wave_timer % 1100 == 0 {
                            let sx = rand::gen_range(50.0, playfield_w - 50.0);
                            let sy = 50.0;
                            enemies.spawn_enemy(EnemyKind::Snake, sx, sy, 25);
                        }

                        // Corner Swarms
                        if wave_timer % 600 == 0 {
                            let corner_x = if rand::gen_range(0, 2) == 0 { 50.0 } else { playfield_w - 50.0 };
                            let corner_y = if rand::gen_range(0, 2) == 0 { 50.0 } else { playfield_h - 50.0 };
                            for _ in 0..6 {
                                enemies.spawn_enemy(EnemyKind::Pinwheel, corner_x, corner_y, 25);
                            }
                        }

                        // Handle player inputs & shooting
                        let (bomb_active, shot_fired) = player.handle_input(
                            &mut shots,
                            &mut grid,
                            &mut particles,
                            playfield_w,
                            playfield_h,
                        );

                        if shot_fired {
                            audio.play_shot();
                        }

                        if bomb_active {
                            audio.play_bomb();
                            let (bomb_score, bomb_kills) = enemies.destroy_all(&mut particles, &mut grid);
                            if bomb_kills > 0 {
                                player.add_kills(bomb_kills, bomb_score);
                                enemies.add_floating_text(
                                    format!("+{}", bomb_score * player.multiplier),
                                    player.x,
                                    player.y - 30.0,
                                    COL_POWERUP,
                                );
                            }
                        }

                        // Check powerup collection
                        let mut collected_texts = Vec::new();
                        for pu in &mut enemies.powerups {
                            if !pu.alive {
                                continue;
                            }
                            let dx = player.x - pu.x;
                            let dy = player.y - pu.y;
                            if (dx * dx + dy * dy) < 28.0 * 28.0 {
                                pu.alive = false;
                                audio.play_bonus();
                                match pu.kind {
                                    0 => {
                                        player.lives = (player.lives + 1).min(6);
                                        collected_texts.push(("1-UP!".to_string(), pu.x, pu.y, COL_POWERUP));
                                    }
                                    1 => {
                                        player.bombs = (player.bombs + 1).min(6);
                                        collected_texts.push(("BOMB!".to_string(), pu.x, pu.y, COL_BOMB));
                                    }
                                    2 => {
                                        player.triple_shot = 600.0;
                                        collected_texts.push(("3-WAY SHOT!".to_string(), pu.x, pu.y, COL_POWERUP));
                                    }
                                    3 => {
                                        player.side_shot = 600.0;
                                        collected_texts.push(("SIDE SHOT!".to_string(), pu.x, pu.y, COL_POWERUP));
                                    }
                                    4 => {
                                        player.super_shot = 500.0;
                                        collected_texts.push(("SUPER SHOT!".to_string(), pu.x, pu.y, COL_SHOT_SUPER));
                                    }
                                    5 => {
                                        player.bouncy_shot = 600.0;
                                        collected_texts.push(("BOUNCE SHOT!".to_string(), pu.x, pu.y, COL_SHOT_BOUNCY));
                                    }
                                    _ => {
                                        player.shield_timer = 300.0;
                                        collected_texts.push(("SHIELD ACTIVE!".to_string(), pu.x, pu.y, Color::new(0.3, 0.9, 1.0, 1.0)));
                                    }
                                }
                            }
                        }
                        for (text, tx, ty, col) in collected_texts {
                            enemies.add_floating_text(text, tx, ty, col);
                        }

                        // Update shots & collisions
                        shots.update(playfield_w, playfield_h, &mut particles);

                        let (earned_pts, kills) = enemies.update(
                            player.x,
                            player.y,
                            playfield_w,
                            playfield_h,
                            &mut grid,
                            &mut shots,
                            &mut particles,
                            &audio,
                            speed_mult,
                        );

                        if kills > 0 {
                            audio.play_explo();
                            let old_mult = player.multiplier;
                            let mult_up = player.add_kills(kills, earned_pts);
                            if mult_up {
                                audio.play_multiplier();
                                enemies.add_floating_text(
                                    format!("x{} MULTIPLIER!", player.multiplier),
                                    player.x,
                                    player.y - 45.0,
                                    COL_POWERUP,
                                );
                            }
                            enemies.add_floating_text(
                                format!("+{}", earned_pts * old_mult),
                                player.x,
                                player.y - 20.0,
                                COL_SCORE,
                            );
                        }

                        // Check player death from enemies
                        if !player.dead && player.shield_timer <= 0.0 {
                            for e in &enemies.enemies {
                                let dx = player.x - e.x;
                                let dy = player.y - e.y;
                                let hit_dist = e.radius + 10.0;
                                if (dx * dx + dy * dy) < hit_dist * hit_dist {
                                    // Player killed!
                                    player.dead = true;
                                    player.lives -= 1;
                                    player.reset_multiplier();
                                    audio.play_die();
                                    particles.burst(player.x, player.y, 40, COL_PLAYER, 8.0, 40.0);
                                    grid.push(player.x, player.y, 120.0, 15.0);

                                    if player.lives < 0 {
                                        state = GameState::GameOver;
                                        game_over_timer = 0.0;
                                        audio.play_game_over();
                                    } else {
                                        player.respawn_timer = 90.0;
                                    }
                                    break;
                                }
                            }
                        }

                        // Player respawn countdown
                        if player.dead && player.lives >= 0 {
                            player.respawn_timer -= 1.0;
                            if player.respawn_timer <= 0.0 {
                                player.respawn(playfield_w * 0.5, playfield_h * 0.5);
                            }
                        }
                    }

                    // Update physics & graphics
                    grid.update();
                    particles.update(playfield_w, playfield_h);

                    // Render game layers
                    particles.draw_stars();
                    grid.draw();
                    shots.draw(shot_tex.as_ref());
                    enemies.draw(&enemy_textures);
                    player.draw(player_tex.as_ref());
                    particles.draw_particles();
                    menu.draw_hud(&player, playfield_w, playfield_h);

                    // "GET READY" banner at start
                    if ready_timer > 0.0 {
                        let scale = (ready_timer / 120.0) * 0.5 + 2.5;
                        draw_string_centered("GET READY", playfield_w * 0.5, playfield_h * 0.5 - 30.0, scale, COL_POWERUP, 2.5);
                    }
                }
            }

            GameState::GameOver => {
                audio.play_music(2);
                grid.update();
                particles.update(playfield_w, playfield_h);

                particles.draw_stars();
                grid.draw();
                enemies.draw(&enemy_textures);
                particles.draw_particles();
                menu.draw_hud(&player, playfield_w, playfield_h);

                game_over_timer += 1.0;
                let scale = (game_over_timer * 0.05).sin().abs() * 2.0 + 3.5;
                draw_string_centered("GAME OVER", playfield_w * 0.5, playfield_h * 0.5 - 40.0, scale, COL_SUN, 3.0);

                let final_score_str = format!("FINAL SCORE: {:>8}", player.score);
                draw_string_centered(&final_score_str, playfield_w * 0.5, playfield_h * 0.5 + 40.0, 2.2, COL_POWERUP, 1.8);

                if game_over_timer > 180.0 {
                    draw_string_centered("PRESS SPACE TO CONTINUE", playfield_w * 0.5, playfield_h * 0.5 + 90.0, 1.8, WHITE, 1.5);

                    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) {
                        // Record high score
                        if highscores.is_high_score(player.score) {
                            let mins = (elapsed_play_time / 60.0).floor() as i32;
                            let secs = (elapsed_play_time % 60.0).floor() as i32;
                            let time_str = format!("{} mns, {} secs", mins, secs);
                            highscores.add(player.score, "Pilot".to_string(), time_str);
                        }
                        state = GameState::Attract;
                        menu.attract_screen = AttractScreen::Scores;
                        menu.screen_timer = 0.0;
                    }
                }
            }
        }

        next_frame().await;
    }
}
