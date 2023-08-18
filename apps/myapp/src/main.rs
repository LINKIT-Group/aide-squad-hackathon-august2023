use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const GRAVITY: f32 = 0.25;
const JUMP_STRENGTH: f32 = -5.0;
const PIPE_SPEED: f32 = -3.0;
const PIPE_SPACING: f32 = 300.0;

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

    fn restart(&mut self) {
        *self = GameState::new();
    }
}

// ... [Bird and Pipe structures as before] ...

#[macroquad::main("Flappy Bird")]
async fn main() {
    let mut game_state = GameState::new();
    let mut restart_game = false;


    loop {
        // Bird physics
        game_state.bird.velocity += GRAVITY;
        game_state.bird.position.y += game_state.bird.velocity;

        if is_mouse_button_pressed(MouseButton::Left) {
            game_state.bird.velocity = JUMP_STRENGTH;
        }

        // Pipe logic
        for pipe in &mut game_state.pipes {
            pipe.position.x += PIPE_SPEED;
        }

        if game_state.pipes[0].position.x + 60.0 < 0.0 {
            game_state.pipes.remove(0);
            game_state.pipes.push(Pipe::new(SCREEN_WIDTH));
        }

        // Collision with ground or ceiling
        if game_state.bird.position.y <= 0.0 || game_state.bird.position.y >= SCREEN_HEIGHT {
            restart_game = true;
        }

        // Collision with pipes
        for pipe in &game_state.pipes {
            if game_state.bird.position.y < pipe.gap_y - 50.0 || game_state.bird.position.y > pipe.gap_y + 50.0 {
                if game_state.bird.position.x > pipe.position.x && game_state.bird.position.x < pipe.position.x + 60.0 {
                    restart_game = true;
                    break; // no need to check further pipes if we're restarting
                }
            }
        }

        if restart_game {
            game_state.restart();
            restart_game = false;
            continue; // skip drawing and go to the next loop iteration
        }

        // ... [drawing code] ...


        clear_background(SKYBLUE);

        // Draw bird
        draw_circle(game_state.bird.position.x, game_state.bird.position.y, 20.0, YELLOW);

        // Draw pipes
        for pipe in &game_state.pipes {
            draw_rectangle(pipe.position.x, 0.0, 60.0, pipe.gap_y - 50.0, DARKGREEN);
            draw_rectangle(pipe.position.x, pipe.gap_y + 50.0, 60.0, SCREEN_HEIGHT - pipe.gap_y - 50.0, DARKGREEN);
        }

        next_frame().await;
    }
}
