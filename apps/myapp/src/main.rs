use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 1800.0;
const SCREEN_HEIGHT: f32 = 900.0;
const GRAVITY: f32 = 0.25;
const JUMP_STRENGTH: f32 = -5.0;
const PIPE_SPEED: f32 = -3.0;
const PIPE_SPACING: f32 = 300.0;
const FRAME_THICKNESS: f32 = 5.0;

fn draw_game_frame() {
    // Top line
    draw_line(0.0, 0.0, SCREEN_WIDTH, 0.0, FRAME_THICKNESS, BLACK);
    
    // Bottom line
    draw_line(0.0, SCREEN_HEIGHT, SCREEN_WIDTH, SCREEN_HEIGHT, FRAME_THICKNESS, BLACK);
    
    // Left line
    draw_line(0.0, 0.0, 0.0, SCREEN_HEIGHT, FRAME_THICKNESS, BLACK);
    
    // Right line
    draw_line(SCREEN_WIDTH, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT, FRAME_THICKNESS, BLACK);
}


struct Bird {
    position: Vec2,
    velocity: f32,
    // texture: Texture2D, // This is the new field for the image texture
}

// impl Bird {
//     fn new() -> Self {
//         // let texture = load_texture("assets/pocket.png").unwrap(); // Load the texture
//         let texture = futures::executor::block_on(async {
//             load_texture("assets/rocker.png").await.unwrap()
//         });
//         Self {
//             position: Vec2::new(SCREEN_WIDTH / 4.0, SCREEN_HEIGHT / 2.0),
//             velocity: 0.0,
//             texture, // Set the loaded texture
//         }
//     }
// }

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
        // let bird = Bird::new();
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
    let player_texture = load_texture("assets/my_image.png").await.unwrap();



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
                for pipe in &mut game_state.pipes {
                    pipe.position.x += PIPE_SPEED;
                }

                if game_state.pipes[0].position.x + 60.0 < 0.0 {
                    game_state.pipes.remove(0);
                    game_state.pipes.push(Pipe::new(SCREEN_WIDTH));
                }

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
                draw_game_frame();


                // Draw bird
                draw_circle(game_state.bird.position.x, game_state.bird.position.y, 20.0, YELLOW);
                // draw_texture(&player_texture, game_state.bird.position.x, game_state.bird.position.y, YELLOW);


                // Draw pipes
                for pipe in &game_state.pipes {
                    draw_rectangle(pipe.position.x, 0.0, 60.0, pipe.gap_y - 50.0, DARKGREEN);
                    draw_rectangle(pipe.position.x, pipe.gap_y + 50.0, 60.0, SCREEN_HEIGHT - pipe.gap_y - 50.0, DARKGREEN);
                }
            }

            GameMode::GameOver => {
                clear_background(SKYBLUE);
                draw_game_frame();

                
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
