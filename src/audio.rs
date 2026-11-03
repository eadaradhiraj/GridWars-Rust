use macroquad::audio::{load_sound, play_sound, stop_sound, PlaySoundParams, Sound};

pub struct AudioManager {
    pub enabled: bool,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub shot: Option<Sound>,
    pub player_die: Option<Sound>,
    pub explo: Option<Sound>,
    pub bomb: Option<Sound>,
    pub bonus: Option<Sound>,
    pub multiplier: Option<Sound>,
    pub startup: Option<Sound>,
    pub game_over: Option<Sound>,
    pub sun_born: Option<Sound>,
    pub sun_hit: Option<Sound>,
    pub sun_exp: Option<Sound>,
    pub pop: Option<Sound>,
    pub snake_hit: Option<Sound>,
    pub shield: Option<Sound>,
    pub quark_hit: Option<Sound>,

    // Background Themes
    pub theme_title: Option<Sound>,
    pub theme_battle: Option<Sound>,
    pub theme_scores: Option<Sound>,
    pub active_theme: Option<usize>,
}

async fn load_safe(path: &str) -> Option<Sound> {
    load_sound(path).await.ok()
}

impl AudioManager {
    pub async fn load() -> Self {
        let shot = load_safe("sounds/shotborn.wav").await;
        let player_die = load_safe("sounds/die1.wav").await;
        let explo = load_safe("sounds/Explo1.wav").await;
        let bomb = load_safe("sounds/explo1.wav").await;
        let bonus = load_safe("sounds/bonus1.wav").await;
        let multiplier = load_safe("sounds/bonus2.wav").await;
        let startup = load_safe("sounds/startup.wav").await;
        let game_over = load_safe("sounds/gameover.wav").await;
        let sun_born = load_safe("sounds/sun1.wav").await;
        let sun_hit = load_safe("sounds/sunhit1.wav").await;
        let sun_exp = load_safe("sounds/sunexp.wav").await;
        let pop = load_safe("sounds/pop2.wav").await;
        let snake_hit = load_safe("sounds/snakehit.wav").await;
        let shield = load_safe("sounds/shield1.wav").await;
        let quark_hit = load_safe("sounds/quarkhit.wav").await;

        let theme_title = load_safe("music/Theme0.ogg").await;
        let theme_battle = load_safe("music/Theme1.ogg").await;
        let theme_scores = load_safe("music/Theme2.ogg").await;

        Self {
            enabled: true,
            sfx_volume: 0.6,
            music_volume: 0.5,
            shot,
            player_die,
            explo,
            bomb,
            bonus,
            multiplier,
            startup,
            game_over,
            sun_born,
            sun_hit,
            sun_exp,
            pop,
            snake_hit,
            shield,
            quark_hit,
            theme_title,
            theme_battle,
            theme_scores,
            active_theme: None,
        }
    }

    fn play(&self, sound: &Option<Sound>, vol_scale: f32) {
        if !self.enabled {
            return;
        }
        if let Some(snd) = sound {
            play_sound(
                snd,
                PlaySoundParams {
                    volume: (self.sfx_volume * vol_scale).clamp(0.0, 1.0),
                    looped: false,
                },
            );
        }
    }

    pub fn play_music(&mut self, track: usize) {
        if !self.enabled {
            return;
        }
        if self.active_theme == Some(track) {
            return;
        }
        self.stop_music();

        let track_snd = match track {
            0 => &self.theme_title,
            1 => &self.theme_battle,
            2 => &self.theme_scores,
            _ => &None,
        };

        if let Some(snd) = track_snd {
            play_sound(
                snd,
                PlaySoundParams {
                    volume: self.music_volume.clamp(0.0, 1.0),
                    looped: true,
                },
            );
            self.active_theme = Some(track);
        }
    }

    pub fn stop_music(&mut self) {
        if let Some(curr) = self.active_theme {
            let track_snd = match curr {
                0 => &self.theme_title,
                1 => &self.theme_battle,
                2 => &self.theme_scores,
                _ => &None,
            };
            if let Some(snd) = track_snd {
                stop_sound(snd);
            }
            self.active_theme = None;
        }
    }

    pub fn play_shot(&self) {
        self.play(&self.shot, 0.4);
    }

    pub fn play_die(&self) {
        self.play(&self.player_die, 1.0);
    }

    pub fn play_explo(&self) {
        self.play(&self.explo, 0.8);
    }

    pub fn play_bomb(&self) {
        self.play(&self.bomb, 1.0);
    }

    pub fn play_bonus(&self) {
        self.play(&self.bonus, 0.9);
    }

    pub fn play_multiplier(&self) {
        self.play(&self.multiplier, 0.9);
    }

    pub fn play_startup(&self) {
        self.play(&self.startup, 0.9);
    }

    pub fn play_game_over(&self) {
        self.play(&self.game_over, 1.0);
    }

    pub fn play_sun_born(&self) {
        self.play(&self.sun_born, 0.8);
    }

    pub fn play_sun_hit(&self) {
        self.play(&self.sun_hit, 0.7);
    }

    pub fn play_sun_exp(&self) {
        self.play(&self.sun_exp, 1.0);
    }

    pub fn play_pop(&self) {
        self.play(&self.pop, 0.7);
    }

    pub fn play_snake_hit(&self) {
        self.play(&self.snake_hit, 0.8);
    }

    pub fn play_shield(&self) {
        self.play(&self.shield, 0.7);
    }

    pub fn play_quark_hit(&self) {
        self.play(&self.quark_hit, 0.8);
    }
}
