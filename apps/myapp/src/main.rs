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

#[macroquad::main("Flappy Bird")]
async fn main() {
    let mut bird = Bird {
        position: vec2(SCREEN_WIDTH * 0.2, SCREEN_HEIGHT / 2.0),
        velocity: 0.0,
    };

    let mut pipes: Vec<Pipe> = Vec::new();

    loop {
        // Bird physics
        bird.velocity += GRAVITY;
        bird.position.y += bird.velocity;

        if is_mouse_button_pressed(MouseButton::Left) {
            bird.velocity = JUMP_STRENGTH;
        }

        // Pipe logic
        if pipes.is_empty() || pipes.last().unwrap().position.x < SCREEN_WIDTH - PIPE_SPACING {
            pipes.push(Pipe::new(SCREEN_WIDTH));
        }

        for pipe in &mut pipes {
            pipe.position.x += PIPE_SPEED;
        }

        pipes.retain(|pipe| pipe.position.x > -60.0); // Remove pipes that are off-screen

        // Collision logic...

        clear_background(SKYBLUE);

        // Draw bird...

        // Draw pipes...
        
        next_frame().await;
    }
}
