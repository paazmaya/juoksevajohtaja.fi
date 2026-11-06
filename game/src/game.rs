use crate::level::{Level, ObstacleData, PickupData, QuestionCheckpoint};
use crate::physics::{Aabb, GRAVITY, GROUND_Y, JUMP_VELOCITY, RUNNER_HEIGHT, RUNNER_WIDTH};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioCue {
    None,
    Jump,
    Pickup,
    QuizCorrect,
    QuizWrong,
    ObstacleHit,
    Victory,
    GameOver,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InputState {
    pub right: bool,
    pub left: bool,
    pub up: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Start,
    Running,
    Quiz {
        checkpoint_index: usize,
        feedback: Option<QuizFeedback>,
    },
    Victory {
        score: u32,
        time_seconds: f64,
    },
    GameOver {
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct QuizFeedback {
    pub is_correct: bool,
    pub explanation: String,
    pub timer: f64,
}

#[derive(Debug, Clone)]
pub struct Runner {
    pub x: f64,
    pub y: f64,
    pub vy: f64,
    pub is_grounded: bool,
    pub speed: f64,
    pub stamina: f64,
    pub max_stamina: f64,
    pub score: u32,
    pub animation_frame: usize,
    pub animation_timer: f64,
    pub invulnerability_timer: f64,
}

#[derive(Debug, Clone)]
pub struct HostCompanion {
    pub x_offset: f64, // relative to player x
    pub y: f64,
    pub vy: f64,
    pub is_grounded: bool,
    pub active_dialogue: Option<String>,
    pub dialogue_timer: f64,
    pub is_helping: bool,
    pub is_sabotaging: bool,
    pub animation_frame: usize,
    pub animation_timer: f64,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub level: Level,
    pub state: GameState,
    pub input: InputState,
    pub distance: f64,
    pub elapsed_time: f64,
    pub player: Runner,
    pub host: HostCompanion,
    pub active_checkpoints: Vec<(usize, QuestionCheckpoint)>,
    pub active_obstacles: Vec<ObstacleData>,
    pub active_pickups: Vec<PickupData>,
    pub triggered_events: Vec<usize>,
    pub last_audio_cue: AudioCue,
}

impl Game {
    pub fn new(level: Level) -> Self {
        let player_speed = level.guest.speed;
        let mut active_cps = Vec::new();
        for (i, cp) in level.checkpoints.iter().enumerate() {
            active_cps.push((i, cp.clone()));
        }

        Self {
            active_obstacles: level.obstacles.clone(),
            active_pickups: level.pickups.clone(),
            active_checkpoints: active_cps,
            triggered_events: Vec::new(),
            level,
            state: GameState::Start,
            input: InputState::default(),
            distance: 0.0,
            elapsed_time: 0.0,
            last_audio_cue: AudioCue::None,
            player: Runner {
                x: 120.0,
                y: GROUND_Y - RUNNER_HEIGHT,
                vy: 0.0,
                is_grounded: true,
                speed: player_speed,
                stamina: 100.0,
                max_stamina: 100.0,
                score: 0,
                animation_frame: 0,
                animation_timer: 0.0,
                invulnerability_timer: 0.0,
            },
            host: HostCompanion {
                x_offset: 70.0, // host runs slightly ahead or alongside
                y: GROUND_Y - RUNNER_HEIGHT,
                vy: 0.0,
                is_grounded: true,
                active_dialogue: Some("Moi! Tervetuloa lenkille podcastin vieraana!".to_string()),
                dialogue_timer: 3.5,
                is_helping: false,
                is_sabotaging: false,
                animation_frame: 0,
                animation_timer: 0.0,
            },
        }
    }

    pub fn start(&mut self) {
        if self.state == GameState::Start {
            self.state = GameState::Running;
        }
    }

    pub fn set_input(&mut self, right: bool, left: bool, up: bool) {
        self.input.right = right;
        self.input.left = left;
        self.input.up = up;
        if up && self.state == GameState::Running && self.player.is_grounded {
            self.jump();
        }
    }

    pub fn jump(&mut self) {
        if self.state == GameState::Running && self.player.is_grounded {
            self.player.vy = JUMP_VELOCITY;
            self.player.is_grounded = false;
            self.last_audio_cue = AudioCue::Jump;
        }
    }

    pub fn current_checkpoint(&self) -> Option<&QuestionCheckpoint> {
        match &self.state {
            GameState::Quiz {
                checkpoint_index, ..
            } => self.level.checkpoints.get(*checkpoint_index),
            _ => None,
        }
    }

    pub fn answer_quiz(&mut self, option_id: &str) -> bool {
        let (cp_idx, is_correct, explanation) = match &self.state {
            GameState::Quiz {
                checkpoint_index,
                feedback: None,
            } => {
                if let Some(cp) = self.level.checkpoints.get(*checkpoint_index) {
                    let correct = cp
                        .options
                        .iter()
                        .find(|o| o.id == option_id)
                        .map(|o| o.is_correct)
                        .unwrap_or(false);
                    let expl = cp.explanation.clone().unwrap_or_else(|| {
                        if correct {
                            "Oikein! Kuuntelit jakson tarkasti!".to_string()
                        } else {
                            "Väärin! Oikea vastaus löytyy podcast-jaksosta.".to_string()
                        }
                    });
                    (*checkpoint_index, correct, expl)
                } else {
                    return false;
                }
            }
            _ => return false,
        };

        if is_correct {
            self.player.score += 150;
            self.player.stamina = (self.player.stamina + 25.0).min(self.player.max_stamina);
            self.host.active_dialogue = Some("Loistavaa! Olet kuunnellut jakson!".to_string());
            self.host.dialogue_timer = 3.0;
            self.last_audio_cue = AudioCue::QuizCorrect;
        } else {
            self.player.stamina = (self.player.stamina - 20.0).max(0.0);
            self.host.active_dialogue =
                Some("Hups! Tämä oli väärin, ota vähän takapakkia!".to_string());
            self.host.dialogue_timer = 3.0;
            self.last_audio_cue = AudioCue::QuizWrong;
        }

        self.state = GameState::Quiz {
            checkpoint_index: cp_idx,
            feedback: Some(QuizFeedback {
                is_correct,
                explanation,
                timer: 2.0,
            }),
        };

        is_correct
    }

    pub fn update(&mut self, dt: f64) {
        self.last_audio_cue = AudioCue::None;

        // Dialogue timer decay
        if self.host.dialogue_timer > 0.0 {
            self.host.dialogue_timer -= dt;
            if self.host.dialogue_timer <= 0.0 {
                self.host.active_dialogue = None;
            }
        }

        match &mut self.state {
            GameState::Start => {
                // Just animate idle/waiting
                return;
            }
            GameState::GameOver { .. } | GameState::Victory { .. } => {
                return;
            }
            GameState::Quiz {
                checkpoint_index,
                feedback,
            } => {
                if let Some(fb) = feedback {
                    fb.timer -= dt;
                    if fb.timer <= 0.0 {
                        let is_correct = fb.is_correct;
                        let cp_index = *checkpoint_index;
                        if is_correct {
                            // Resume running past checkpoint
                            self.distance += 40.0;
                            self.state = GameState::Running;
                        } else if self.player.stamina <= 0.0 {
                            self.state = GameState::GameOver {
                                reason: "Kuntosi loppui väärään vastaukseen!".to_string(),
                            };
                            self.last_audio_cue = AudioCue::GameOver;
                        } else {
                            // Retry checkpoint after resetting back slightly
                            self.distance = (self.distance - 60.0).max(0.0);
                            self.state = GameState::Running;
                        }
                        // Remove checkpoint from active list so it doesn't trigger immediately again if passed
                        if is_correct {
                            self.active_checkpoints.retain(|(idx, _)| *idx != cp_index);
                        }
                    }
                }
                return;
            }
            GameState::Running => {}
        }

        self.elapsed_time += dt;

        // Horizontal movement: ONLY via input arrow keys
        let is_moving_forward = self.input.right;
        let is_moving_backward = self.input.left;

        if is_moving_forward {
            self.distance += self.player.speed * dt;
            self.player.score += (dt * 10.0) as u32;
            self.player.stamina = (self.player.stamina - dt * 0.4).max(0.0);

            // Animate runner
            self.player.animation_timer += dt;
            if self.player.animation_timer >= 0.12 {
                self.player.animation_timer = 0.0;
                self.player.animation_frame = (self.player.animation_frame + 1) % 2;
            }

            // Animate host running alongside
            self.host.animation_timer += dt;
            if self.host.animation_timer >= 0.12 {
                self.host.animation_timer = 0.0;
                self.host.animation_frame = (self.host.animation_frame + 1) % 2;
            }
        } else if is_moving_backward {
            self.distance = (self.distance - self.player.speed * 0.6 * dt).max(0.0);
            self.player.stamina = (self.player.stamina - dt * 0.2).max(0.0);

            // Animate runner
            self.player.animation_timer += dt;
            if self.player.animation_timer >= 0.12 {
                self.player.animation_timer = 0.0;
                self.player.animation_frame = (self.player.animation_frame + 1) % 2;
            }

            // Animate host
            self.host.animation_timer += dt;
            if self.host.animation_timer >= 0.12 {
                self.host.animation_timer = 0.0;
                self.host.animation_frame = (self.host.animation_frame + 1) % 2;
            }
        } else {
            // Idle standing
            self.player.animation_frame = 0;
            self.player.animation_timer = 0.0;
            self.host.animation_frame = 0;
            self.host.animation_timer = 0.0;
        }

        // Runner gravity & jump physics
        self.player.vy += GRAVITY * dt;
        self.player.y += self.player.vy * dt;
        let ground_runner_y = GROUND_Y - RUNNER_HEIGHT;
        if self.player.y >= ground_runner_y {
            self.player.y = ground_runner_y;
            self.player.vy = 0.0;
            self.player.is_grounded = true;
        }

        if self.player.stamina <= 0.0 {
            self.state = GameState::GameOver {
                reason: "Voimat ehtyivät lenkillä! Muista juoda kahvia ja vastata oikein!"
                    .to_string(),
            };
            self.last_audio_cue = AudioCue::GameOver;
            return;
        }

        if self.player.invulnerability_timer > 0.0 {
            self.player.invulnerability_timer -= dt;
        }

        // Check host events
        for (idx, event) in self.level.host_events.iter().enumerate() {
            if !self.triggered_events.contains(&idx)
                && self.distance >= event.distance
                && self.distance <= event.distance + 150.0
            {
                self.triggered_events.push(idx);
                self.host.active_dialogue = Some(event.dialogue.clone());
                self.host.dialogue_timer = 4.0;
                if event.action == "help" {
                    self.host.is_helping = true;
                    self.host.is_sabotaging = false;
                    self.host.x_offset = 90.0; // runs in front to break wind
                    self.player.stamina = (self.player.stamina + 10.0).min(self.player.max_stamina);
                } else if event.action == "sabotage" {
                    self.host.is_helping = false;
                    self.host.is_sabotaging = true;
                    self.host.x_offset = -60.0; // falls behind and taunts
                }
            }
        }

        // Check quiz checkpoints
        for (cp_idx, cp) in &self.active_checkpoints {
            if self.distance >= cp.distance && self.distance < cp.distance + 40.0 {
                let found_idx = *cp_idx;
                self.input.right = false;
                self.input.left = false;
                self.state = GameState::Quiz {
                    checkpoint_index: found_idx,
                    feedback: None,
                };
                return;
            }
        }

        // Check player bounding box for collisions
        let player_box = Aabb::new(self.player.x, self.player.y, RUNNER_WIDTH, RUNNER_HEIGHT);

        // Check pickups
        let mut collected_indices = Vec::new();
        for (i, pickup) in self.active_pickups.iter().enumerate() {
            let pickup_screen_x = (pickup.distance - self.distance) + self.player.x;
            if pickup_screen_x > -100.0 && pickup_screen_x < 900.0 {
                let pickup_box = Aabb::new(pickup_screen_x, GROUND_Y - 45.0, 32.0, 32.0);
                if player_box.intersects(&pickup_box) {
                    collected_indices.push(i);
                    self.player.stamina =
                        (self.player.stamina + pickup.value).min(self.player.max_stamina);
                    self.player.score += 50;
                    self.last_audio_cue = AudioCue::Pickup;
                }
            }
        }
        for i in collected_indices.into_iter().rev() {
            self.active_pickups.remove(i);
        }

        // Check obstacle collisions
        if self.player.invulnerability_timer <= 0.0 {
            for obs in &self.active_obstacles {
                let obs_screen_x = (obs.distance - self.distance) + self.player.x;
                if obs_screen_x > -100.0 && obs_screen_x < 900.0 {
                    let obs_box =
                        Aabb::new(obs_screen_x, GROUND_Y - obs.height, obs.width, obs.height);
                    if player_box.intersects(&obs_box) {
                        self.player.stamina = (self.player.stamina - 15.0).max(0.0);
                        self.player.invulnerability_timer = 1.0;
                        self.last_audio_cue = AudioCue::ObstacleHit;
                        self.host.active_dialogue =
                            Some("Auts! Varovasti niiden esteiden kanssa!".to_string());
                        self.host.dialogue_timer = 2.5;
                        break;
                    }
                }
            }
        }

        // Check victory condition
        if self.distance >= self.level.track_length {
            self.state = GameState::Victory {
                score: self.player.score + 500,
                time_seconds: self.elapsed_time,
            };
            self.last_audio_cue = AudioCue::Victory;
        }
    }

    pub fn progress_percent(&self) -> f64 {
        if self.level.track_length <= 0.0 {
            0.0
        } else {
            (self.distance / self.level.track_length * 100.0).clamp(0.0, 100.0)
        }
    }
}
