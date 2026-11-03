use macroquad::audio::{load_sound, play_sound, PlaySoundParams, Sound};

pub struct AudioManager {
    pub enabled: bool,
    pub volume: f32,
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

        Self {
            enabled: true,
            volume: 0.6,
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
                    volume: (self.volume * vol_scale).clamp(0.0, 1.0),
                    looped: false,
                },
            );
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

    pub fn _play_sun_hit(&self) {
        self.play(&self.sun_hit, 0.7);
    }

    pub fn _play_sun_exp(&self) {
        self.play(&self.sun_exp, 1.0);
    }

    pub fn _play_pop(&self) {
        self.play(&self.pop, 0.7);
    }

    pub fn _play_snake_hit(&self) {
        self.play(&self.snake_hit, 0.8);
    }

    pub fn _play_shield(&self) {
        self.play(&self.shield, 0.7);
    }
}
