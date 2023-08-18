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

    let mut pipes = vec![Pipe::new(SCREEN_WIDTH)];

    loop {
        // Bird physics
        bird.velocity += GRAVITY;
        bird.position.y += bird.velocity;

        if is_mouse_button_pressed(MouseButton::Left) {
            bird.velocity = JUMP_STRENGTH;
        }

        // Pipe logic
        for pipe in &mut pipes {
            pipe.position.x += PIPE_SPEED;
        }

        if pipes[0].position.x + 60.0 < 0.0 {
            pipes.remove(0);
            pipes.push(Pipe::new(SCREEN_WIDTH));
        }

        // Collision with ground or ceiling
        if bird.position.y <= 0.0 || bird.position.y >= SCREEN_HEIGHT {
            bird.position.y = SCREEN_HEIGHT / 2.0;
            bird.velocity = 0.0;
        }

        // Collision with pipes
        for pipe in &pipes {
            if bird.position.y < pipe.gap_y - 50.0 || bird.position.y > pipe.gap_y + 50.0 {
                if bird.position.x > pipe.position.x && bird.position.x < pipe.position.x + 60.0 {
                    bird.position.y = SCREEN_HEIGHT / 2.0;
                    bird.velocity = 0.0;
                }
            }
        }

        clear_background(SKYBLUE);

        // Draw bird
        draw_circle(bird.position.x, bird.position.y, 20.0, YELLOW);

        // Draw pipes
        for pipe in &pipes {
            draw_rectangle(pipe.position.x, 0.0, 60.0, pipe.gap_y - 50.0, DARKGREEN);
            draw_rectangle(pipe.position.x, pipe.gap_y + 50.0, 60.0, SCREEN_HEIGHT - pipe.gap_y - 50.0, DARKGREEN);
        }

        next_frame().await;
    }
}
