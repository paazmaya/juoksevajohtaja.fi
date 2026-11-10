//! Core gameplay state machine, runner simulation, quiz interaction, and collision handling.

use crate::level::{Level, ObstacleData, PickupData, QuestionCheckpoint};
use crate::physics::{Aabb, GRAVITY, GROUND_Y, JUMP_VELOCITY, RUNNER_HEIGHT, RUNNER_WIDTH};

/// Default horizontal starting position of the player runner in pixels from canvas left.
pub const DEFAULT_PLAYER_START_X: f64 = 120.0;

/// Initial stamina value assigned when a run starts.
pub const INITIAL_STAMINA: f64 = 100.0;

/// Maximum stamina ceiling for the player runner.
pub const MAX_STAMINA: f64 = 100.0;

/// Default horizontal offset of the host runner relative to the player runner in pixels.
pub const DEFAULT_HOST_OFFSET_X: f64 = 70.0;

/// Initial duration for the welcome dialogue speech bubble in seconds.
pub const INITIAL_DIALOGUE_DURATION: f64 = 3.5;

/// Bonus score points awarded for answering a checkpoint quiz correctly.
pub const QUIZ_CORRECT_SCORE_BONUS: u32 = 150;

/// Stamina points restored when answering a quiz correctly.
pub const QUIZ_CORRECT_STAMINA_RECOVERY: f64 = 25.0;

/// Duration in seconds for host dialogue after a quiz answer.
pub const QUIZ_HOST_DIALOGUE_DURATION: f64 = 3.0;

/// Stamina penalty subtracted when answering a quiz incorrectly.
pub const QUIZ_WRONG_STAMINA_PENALTY: f64 = 20.0;

/// Duration in seconds that quiz answer feedback remains on screen before resuming running.
pub const QUIZ_FEEDBACK_DURATION: f64 = 2.0;

/// Distance in pixels advanced forward when resuming after a correct quiz answer.
pub const CHECKPOINT_PASS_ADVANCE_DISTANCE: f64 = 40.0;

/// Distance in pixels pushed backward when retrying after a wrong quiz answer.
pub const CHECKPOINT_RETRY_BACKTRACK_DISTANCE: f64 = 60.0;

/// Score points accumulated per second of forward running.
pub const RUNNING_SCORE_PER_SECOND: f64 = 10.0;

/// Stamina consumed per second of forward running.
pub const FORWARD_STAMINA_DRAIN_PER_SECOND: f64 = 0.4;

/// Duration per animation frame in seconds while running.
pub const RUN_ANIMATION_FRAME_DURATION: f64 = 0.16;

/// Total number of animation frames in the running cycle.
pub const RUN_ANIMATION_FRAME_COUNT: usize = 2;

/// Speed multiplier applied when running backward relative to base speed.
pub const BACKWARD_SPEED_MULTIPLIER: f64 = 0.6;

/// Stamina consumed per second of backward running.
pub const BACKWARD_STAMINA_DRAIN_PER_SECOND: f64 = 0.2;

/// Distance window in pixels within which a host narrative event triggers.
pub const HOST_EVENT_TRIGGER_RANGE: f64 = 150.0;

/// Duration in seconds that host event speech bubbles stay active.
pub const HOST_EVENT_DIALOGUE_DURATION: f64 = 4.0;

/// Host runner horizontal offset when providing wind-break assistance.
pub const HOST_HELP_OFFSET_X: f64 = 90.0;

/// Stamina boost granted when host provides assistance.
pub const HOST_HELP_STAMINA_BOOST: f64 = 10.0;

/// Host runner horizontal offset when falling behind to taunt/sabotage.
pub const HOST_SABOTAGE_OFFSET_X: f64 = -60.0;

/// Distance window in pixels around a checkpoint that triggers the quiz modal.
pub const CHECKPOINT_TRIGGER_TOLERANCE: f64 = 40.0;

/// Horizontal distance offscreen to the left before items are culled.
pub const DESPAWN_OFFSCREEN_LEFT: f64 = -100.0;

/// Horizontal distance offscreen to the right before items are culled.
pub const DESPAWN_OFFSCREEN_RIGHT: f64 = 900.0;

/// Vertical offset of pickups relative to ground level in pixels.
pub const PICKUP_Y_OFFSET_FROM_GROUND: f64 = 45.0;

/// Width and height of collectible pickup bounding boxes in pixels.
pub const PICKUP_SIZE: f64 = 32.0;

/// Score points awarded for collecting a pickup.
pub const PICKUP_SCORE_BONUS: u32 = 50;

/// Stamina penalty inflicted when colliding with an obstacle.
pub const OBSTACLE_HIT_STAMINA_PENALTY: f64 = 15.0;

/// Invulnerability duration in seconds after an obstacle collision.
pub const INVULNERABILITY_DURATION: f64 = 1.0;

/// Duration in seconds for host dialogue following an obstacle collision.
pub const OBSTACLE_HIT_DIALOGUE_DURATION: f64 = 2.5;

/// Bonus score points awarded for reaching the finish line.
pub const VICTORY_SCORE_BONUS: u32 = 500;

/// Multiplier for computing progress percentages.
pub const PERCENT_MULTIPLIER: f64 = 100.0;

/// Maximum clamp ceiling for progress percentage.
pub const PERCENT_MAX: f64 = 100.0;

/// Discrete sound effect cues triggered by game state changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioCue {
    /// No sound event.
    None,
    /// Player started a jump.
    Jump,
    /// Player collected a coffee or microphone pickup.
    Pickup,
    /// Player answered a checkpoint quiz correctly.
    QuizCorrect,
    /// Player answered a checkpoint quiz incorrectly.
    QuizWrong,
    /// Player collided with an obstacle.
    ObstacleHit,
    /// Player crossed the finish line.
    Victory,
    /// Player ran out of stamina.
    GameOver,
}

/// Instantaneous keyboard and touch input state.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InputState {
    /// Moving forward (right arrow, D).
    pub right: bool,
    /// Moving backward (left arrow, A).
    pub left: bool,
    /// Jumping (up arrow, W, space).
    pub up: bool,
}

/// Top-level lifecycle state of the running game.
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    /// Initial title screen, awaiting player start.
    Start,
    /// Active running and obstacle navigation.
    Running,
    /// Paused at a question checkpoint gate.
    Quiz {
        /// Index of the current checkpoint in `level.checkpoints`.
        checkpoint_index: usize,
        /// Active feedback banner state after choosing an answer.
        feedback: Option<QuizFeedback>,
    },
    /// Crossed the finish line with final score and completion time.
    Victory {
        /// Total score achieved.
        score: u32,
        /// Elapsed time in seconds.
        time_seconds: f64,
    },
    /// Runner ran out of stamina.
    GameOver {
        /// Explanation for why the run ended.
        reason: String,
    },
}

/// Feedback banner state displayed after selecting a quiz answer.
#[derive(Debug, Clone, PartialEq)]
pub struct QuizFeedback {
    /// Whether the selected answer was correct.
    pub is_correct: bool,
    /// Explanation text from the podcast episode.
    pub explanation: String,
    /// Remaining time in seconds before resuming running.
    pub timer: f64,
}

/// State of the player-controlled guest runner.
#[derive(Debug, Clone)]
pub struct Runner {
    /// Canvas X position in pixels.
    pub x: f64,
    /// Canvas Y position in pixels.
    pub y: f64,
    /// Vertical velocity in pixels / second.
    pub vy: f64,
    /// Whether the runner is currently on the ground.
    pub is_grounded: bool,
    /// Horizontal running speed in pixels / second.
    pub speed: f64,
    /// Current stamina points (0..100).
    pub stamina: f64,
    /// Maximum stamina capacity.
    pub max_stamina: f64,
    /// Current accumulated score points.
    pub score: u32,
    /// Current running animation frame index (0 or 1).
    pub animation_frame: usize,
    /// Timer tracking running animation step cycles.
    pub animation_timer: f64,
    /// Remaining post-hit invulnerability time in seconds.
    pub invulnerability_timer: f64,
}

/// State of the host companion (Jukka Paasonen) running alongside the player.
#[derive(Debug, Clone)]
pub struct HostCompanion {
    /// Relative horizontal offset from the player runner in pixels.
    pub x_offset: f64,
    /// Canvas Y position in pixels.
    pub y: f64,
    /// Vertical velocity in pixels / second.
    pub vy: f64,
    /// Whether the host is grounded.
    pub is_grounded: bool,
    /// Currently displayed dialogue text, if any.
    pub active_dialogue: Option<String>,
    /// Remaining duration in seconds for the active dialogue bubble.
    pub dialogue_timer: f64,
    /// Whether the host is currently in a helping (drafting) state.
    pub is_helping: bool,
    /// Whether the host is currently in a sabotaging state.
    pub is_sabotaging: bool,
    /// Running animation frame index.
    pub animation_frame: usize,
    /// Running animation frame timer.
    pub animation_timer: f64,
}

/// Comprehensive game session holding state, level definition, and active entities.
#[derive(Debug, Clone)]
pub struct Game {
    /// Level configuration and layout.
    pub level: Level,
    /// High-level game state.
    pub state: GameState,
    /// Current user inputs.
    pub input: InputState,
    /// Total distance run along the track in pixels.
    pub distance: f64,
    /// Total elapsed run time in seconds.
    pub elapsed_time: f64,
    /// Player runner state.
    pub player: Runner,
    /// Host companion state.
    pub host: HostCompanion,
    /// Unvisited question checkpoints along the track.
    pub active_checkpoints: Vec<(usize, QuestionCheckpoint)>,
    /// Remaining obstacles on the track.
    pub active_obstacles: Vec<ObstacleData>,
    /// Remaining collectible pickups on the track.
    pub active_pickups: Vec<PickupData>,
    /// Indices of triggered host narrative events.
    pub triggered_events: Vec<usize>,
    /// Most recent audio cue emitted during the last tick.
    pub last_audio_cue: AudioCue,
}

impl Game {
    /// Creates and initializes a new game session using the given level configuration.
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
                x: DEFAULT_PLAYER_START_X,
                y: GROUND_Y - RUNNER_HEIGHT,
                vy: 0.0,
                is_grounded: true,
                speed: player_speed,
                stamina: INITIAL_STAMINA,
                max_stamina: MAX_STAMINA,
                score: 0,
                animation_frame: 0,
                animation_timer: 0.0,
                invulnerability_timer: 0.0,
            },
            host: HostCompanion {
                x_offset: DEFAULT_HOST_OFFSET_X,
                y: GROUND_Y - RUNNER_HEIGHT,
                vy: 0.0,
                is_grounded: true,
                active_dialogue: Some("Moi! Tervetuloa lenkille podcastin vieraana!".to_string()),
                dialogue_timer: INITIAL_DIALOGUE_DURATION,
                is_helping: false,
                is_sabotaging: false,
                animation_frame: 0,
                animation_timer: 0.0,
            },
        }
    }

    /// Transitions the game from `GameState::Start` to `GameState::Running`.
    pub fn start(&mut self) {
        if self.state == GameState::Start {
            self.state = GameState::Running;
        }
    }

    /// Updates directional and jump inputs. Triggers a jump if `up` is pressed while grounded.
    pub fn set_input(&mut self, right: bool, left: bool, up: bool) {
        self.input.right = right;
        self.input.left = left;
        self.input.up = up;
        if up && self.state == GameState::Running && self.player.is_grounded {
            self.jump();
        }
    }

    /// Makes the player runner jump upwards if currently grounded.
    pub fn jump(&mut self) {
        if self.state == GameState::Running && self.player.is_grounded {
            self.player.vy = JUMP_VELOCITY;
            self.player.is_grounded = false;
            self.last_audio_cue = AudioCue::Jump;
        }
    }

    /// Returns a reference to the active checkpoint if currently in a quiz state.
    pub fn current_checkpoint(&self) -> Option<&QuestionCheckpoint> {
        match &self.state {
            GameState::Quiz {
                checkpoint_index, ..
            } => self.level.checkpoints.get(*checkpoint_index),
            _ => None,
        }
    }

    /// Submits an answer option ID ("A", "B", etc.) to the current quiz checkpoint.
    /// Returns `true` if the answer was correct, or `false` otherwise.
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
            self.player.score += QUIZ_CORRECT_SCORE_BONUS;
            self.player.stamina =
                (self.player.stamina + QUIZ_CORRECT_STAMINA_RECOVERY).min(self.player.max_stamina);
            self.host.active_dialogue = Some("Loistavaa! Olet kuunnellut jakson!".to_string());
            self.host.dialogue_timer = QUIZ_HOST_DIALOGUE_DURATION;
            self.last_audio_cue = AudioCue::QuizCorrect;
        } else {
            self.player.stamina = (self.player.stamina - QUIZ_WRONG_STAMINA_PENALTY).max(0.0);
            self.host.active_dialogue =
                Some("Hups! Tämä oli väärin, ota vähän takapakkia!".to_string());
            self.host.dialogue_timer = QUIZ_HOST_DIALOGUE_DURATION;
            self.last_audio_cue = AudioCue::QuizWrong;
        }

        self.state = GameState::Quiz {
            checkpoint_index: cp_idx,
            feedback: Some(QuizFeedback {
                is_correct,
                explanation,
                timer: QUIZ_FEEDBACK_DURATION,
            }),
        };

        is_correct
    }

    /// Advances the game simulation by `dt` seconds.
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
                // Idle standing, waiting for start input
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
                            self.distance += CHECKPOINT_PASS_ADVANCE_DISTANCE;
                            self.state = GameState::Running;
                        } else if self.player.stamina <= 0.0 {
                            self.state = GameState::GameOver {
                                reason: "Kuntosi loppui väärään vastaukseen!".to_string(),
                            };
                            self.last_audio_cue = AudioCue::GameOver;
                        } else {
                            // Retry checkpoint after resetting back slightly
                            self.distance =
                                (self.distance - CHECKPOINT_RETRY_BACKTRACK_DISTANCE).max(0.0);
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
            self.player.score += (dt * RUNNING_SCORE_PER_SECOND) as u32;
            self.player.stamina =
                (self.player.stamina - dt * FORWARD_STAMINA_DRAIN_PER_SECOND).max(0.0);

            // Animate runner
            self.player.animation_timer += dt;
            if self.player.animation_timer >= RUN_ANIMATION_FRAME_DURATION {
                self.player.animation_timer = 0.0;
                self.player.animation_frame =
                    (self.player.animation_frame + 1) % RUN_ANIMATION_FRAME_COUNT;
            }

            // Animate host running alongside
            self.host.animation_timer += dt;
            if self.host.animation_timer >= RUN_ANIMATION_FRAME_DURATION {
                self.host.animation_timer = 0.0;
                self.host.animation_frame =
                    (self.host.animation_frame + 1) % RUN_ANIMATION_FRAME_COUNT;
            }
        } else if is_moving_backward {
            self.distance =
                (self.distance - self.player.speed * BACKWARD_SPEED_MULTIPLIER * dt).max(0.0);
            self.player.stamina =
                (self.player.stamina - dt * BACKWARD_STAMINA_DRAIN_PER_SECOND).max(0.0);

            // Animate runner
            self.player.animation_timer += dt;
            if self.player.animation_timer >= RUN_ANIMATION_FRAME_DURATION {
                self.player.animation_timer = 0.0;
                self.player.animation_frame =
                    (self.player.animation_frame + 1) % RUN_ANIMATION_FRAME_COUNT;
            }

            // Animate host
            self.host.animation_timer += dt;
            if self.host.animation_timer >= RUN_ANIMATION_FRAME_DURATION {
                self.host.animation_timer = 0.0;
                self.host.animation_frame =
                    (self.host.animation_frame + 1) % RUN_ANIMATION_FRAME_COUNT;
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
                && self.distance <= event.distance + HOST_EVENT_TRIGGER_RANGE
            {
                self.triggered_events.push(idx);
                self.host.active_dialogue = Some(event.dialogue.clone());
                self.host.dialogue_timer = HOST_EVENT_DIALOGUE_DURATION;
                if event.action == "help" {
                    self.host.is_helping = true;
                    self.host.is_sabotaging = false;
                    self.host.x_offset = HOST_HELP_OFFSET_X;
                    self.player.stamina = (self.player.stamina + HOST_HELP_STAMINA_BOOST)
                        .min(self.player.max_stamina);
                } else if event.action == "sabotage" {
                    self.host.is_helping = false;
                    self.host.is_sabotaging = true;
                    self.host.x_offset = HOST_SABOTAGE_OFFSET_X;
                }
            }
        }

        // Check quiz checkpoints
        for (cp_idx, cp) in &self.active_checkpoints {
            if self.distance >= cp.distance
                && self.distance < cp.distance + CHECKPOINT_TRIGGER_TOLERANCE
            {
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
            if pickup_screen_x > DESPAWN_OFFSCREEN_LEFT && pickup_screen_x < DESPAWN_OFFSCREEN_RIGHT
            {
                let pickup_box = Aabb::new(
                    pickup_screen_x,
                    GROUND_Y - PICKUP_Y_OFFSET_FROM_GROUND,
                    PICKUP_SIZE,
                    PICKUP_SIZE,
                );
                if player_box.intersects(&pickup_box) {
                    collected_indices.push(i);
                    self.player.stamina =
                        (self.player.stamina + pickup.value).min(self.player.max_stamina);
                    self.player.score += PICKUP_SCORE_BONUS;
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
                if obs_screen_x > DESPAWN_OFFSCREEN_LEFT && obs_screen_x < DESPAWN_OFFSCREEN_RIGHT {
                    let obs_box =
                        Aabb::new(obs_screen_x, GROUND_Y - obs.height, obs.width, obs.height);
                    if player_box.intersects(&obs_box) {
                        self.player.stamina =
                            (self.player.stamina - OBSTACLE_HIT_STAMINA_PENALTY).max(0.0);
                        self.player.invulnerability_timer = INVULNERABILITY_DURATION;
                        self.last_audio_cue = AudioCue::ObstacleHit;
                        self.host.active_dialogue =
                            Some("Auts! Varovasti niiden esteiden kanssa!".to_string());
                        self.host.dialogue_timer = OBSTACLE_HIT_DIALOGUE_DURATION;
                        break;
                    }
                }
            }
        }

        // Check victory condition
        if self.distance >= self.level.track_length {
            self.state = GameState::Victory {
                score: self.player.score + VICTORY_SCORE_BONUS,
                time_seconds: self.elapsed_time,
            };
            self.last_audio_cue = AudioCue::Victory;
        }
    }

    /// Computes the player's percentage progress towards the finish line (0.0 to 100.0).
    pub fn progress_percent(&self) -> f64 {
        if self.level.track_length <= 0.0 {
            0.0
        } else {
            (self.distance / self.level.track_length * PERCENT_MULTIPLIER).clamp(0.0, PERCENT_MAX)
        }
    }
}
