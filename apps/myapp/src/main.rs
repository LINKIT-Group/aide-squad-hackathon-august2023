use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 800.0;
const GRAVITY: f32 = 0.25;
const JUMP_STRENGTH: f32 = -5.0;
const PIPE_SPEED: f32 = -3.0;
const PIPE_SPACING: f32 = 550.0;

struct Bird {
    position: Vec2,
    velocity: f32,
}

struct Pipe {
    position: Vec2,
    gap_y: f32,
}

impl Pipe {
    fn new(x: f32) -> Self {
        let gap_y = rand::gen_range(SCREEN_HEIGHT * 0.3, SCREEN_HEIGHT * 0.7);
        Pipe {
            position: vec2(x, 0.0),
            gap_y,
        }
    }
}


enum GameMode {
    Playing,
    GameOver,
}


struct GameState {
    bird: Bird,
    pipes: Vec<Pipe>,
}

impl GameState {
    fn new() -> Self {
        let bird = Bird {
            position: vec2(SCREEN_WIDTH * 0.2, SCREEN_HEIGHT / 2.0),
            velocity: 0.0,
        };
        let pipes = vec![Pipe::new(SCREEN_WIDTH)];
        GameState { bird, pipes }
    }

    // fn restart(&mut self) {
    //     *self = GameState::new();
    // }
}

// ... [Bird and Pipe structures as before] ...

#[macroquad::main("Flappy Bird")]
async fn main() {
    let mut game_state = GameState::new();
    let mut mode = GameMode::Playing;


    loop {
        match mode {
            GameMode::Playing => {
                // Bird physics
                game_state.bird.velocity += GRAVITY;
                game_state.bird.position.y += game_state.bird.velocity;

                if is_mouse_button_pressed(MouseButton::Left) {
                    game_state.bird.velocity = JUMP_STRENGTH;
                }

                // Pipe logic
                if game_state.pipes.is_empty() || game_state.pipes.last().unwrap().position.x < SCREEN_WIDTH - PIPE_SPACING {
                    let num_new_pipes = ((SCREEN_WIDTH - game_state.pipes.last().unwrap().position.x) / PIPE_SPACING) as usize;
                    for _ in 0..num_new_pipes {
                        let new_pipe_x = if game_state.pipes.is_empty() {
                            SCREEN_WIDTH
                        } else {
                            game_state.pipes.last().unwrap().position.x + PIPE_SPACING
                        };
                        game_state.pipes.push(Pipe::new(new_pipe_x));
                    }
                }

                for pipe in &mut game_state.pipes {
                    pipe.position.x += PIPE_SPEED;
                }

                game_state.pipes.retain(|pipe| pipe.position.x > -60.0);

                // Collision with ground or ceiling
                if game_state.bird.position.y <= 0.0 || game_state.bird.position.y >= SCREEN_HEIGHT {
                    mode = GameMode::GameOver;
                }

                // Collision with pipes
                for pipe in &game_state.pipes {
                    if game_state.bird.position.y < pipe.gap_y - 50.0 || game_state.bird.position.y > pipe.gap_y + 50.0 {
                        if game_state.bird.position.x > pipe.position.x && game_state.bird.position.x < pipe.position.x + 60.0 {
                            mode = GameMode::GameOver;
                            break; // no need to check further pipes if we're restarting
                        }
                    }
                }
                 // ... [drawing code] ...


                clear_background(SKYBLUE);

                // Draw bird
                draw_circle(game_state.bird.position.x, game_state.bird.position.y, 20.0, YELLOW);

                // Draw pipes
                for pipe in &game_state.pipes {
                    draw_rectangle(pipe.position.x, 0.0, 60.0, pipe.gap_y - 90.0, DARKGREEN);
                    draw_rectangle(pipe.position.x, pipe.gap_y + 90.0, 60.0, SCREEN_HEIGHT - pipe.gap_y - 90.0, DARKGREEN);
                }
            }

            GameMode::GameOver => {
                clear_background(SKYBLUE);
                
                draw_text("Game Over", SCREEN_WIDTH / 2.0 - 100.0, SCREEN_HEIGHT / 2.0 - 20.0, 40.0, WHITE);
                draw_text("Click to restart!", SCREEN_WIDTH / 2.0 - 150.0, SCREEN_HEIGHT / 2.0 + 30.0, 30.0, WHITE);

                if is_mouse_button_pressed(MouseButton::Left) {
                    game_state = GameState::new();
                    mode = GameMode::Playing;
                }
            }
        }
       
        next_frame().await;
    }
}
