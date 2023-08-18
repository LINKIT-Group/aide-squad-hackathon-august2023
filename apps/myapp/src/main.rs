use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 1800.0;
const SCREEN_HEIGHT: f32 = 900.0;
const GRAVITY: f32 = 0.25;
const JUMP_STRENGTH: f32 = -5.0;
const PIPE_SPEED: f32 = -3.0;
const PIPE_SPACING: f32 = 550.0;
const FRAME_THICKNESS: f32 = 5.0;
const COIN_SPAWN_RATE: f32 = 0.02;  // for instance, 2% chance every frame

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

fn is_circle_colliding_with_bird(bird: &Bird, circle_pos: Vec2, circle_radius: f32) -> bool {
    bird.position.distance(circle_pos) < bird.radius + circle_radius
}

struct Coin {
    position: Vec2,
    radius: f32,
}

struct Bird {
    position: Vec2,
    velocity: f32,
    radius: f32,
    // texture: Texture2D, // This is the new field for the image texture
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
    coins: Vec<Coin>,
    score: i32,   // tracks the score, incremented when a coin is collected

}

impl GameState {
    fn new() -> Self {
        // let bird = Bird::new();
        let bird = Bird {
            position: vec2(SCREEN_WIDTH * 0.2, SCREEN_HEIGHT / 2.0),
            velocity: 0.0,
            radius: 20.0,

        };
        let pipes = vec![Pipe::new(SCREEN_WIDTH)];
        let coins = Vec::new();
        // GameState { bird, pipes, coins }
        Self {
            bird: bird,
            pipes: pipes,
            coins: coins,
            score: 0,
            // game_over: false,
            // show_new_game_screen: true,
        }
    }

    // fn restart(&mut self) {
    //     *self = GameState::new();
    // }
}

// ... [Bird and Pipe structures as before] ...

#[macroquad::main("Flappy Bird")]
async fn main() {
    let background_texture = load_texture("background.png").await.unwrap();
    let mut game_state = GameState::new();
    let mut mode = GameMode::Playing;
    let player_texture = load_texture("paper.png").await.unwrap();
    let player_scale_factor = 0.2;


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

                // periodically add coins:
                if rand::gen_range(0.0, 1.0) < COIN_SPAWN_RATE {
                    game_state.coins.push(Coin {
                        position: Vec2::new(SCREEN_WIDTH, rand::gen_range(100.0, SCREEN_HEIGHT - 100.0)),
                        radius: 15.0,
                    });
                }

                for pipe in &mut game_state.pipes {
                    pipe.position.x += PIPE_SPEED;
                }

                for coin in &mut game_state.coins {
                    coin.position.x += PIPE_SPEED;
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


                // collision with coins
                game_state.coins.retain(|coin| {
                    let collided = is_circle_colliding_with_bird(&game_state.bird, coin.position, coin.radius);
                    if collided {
                        game_state.score += 1;  // Assuming you have a score field in GameState
                    }
                    !collided && coin.position.x + coin.radius > 0.0
                });
                 // ... [drawing code] ...


                draw_texture_ex(
                    &background_texture,
                    0.0,
                    0.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(SCREEN_WIDTH, SCREEN_HEIGHT)),
                        ..Default::default()
                    },
                );
                draw_game_frame();

                // Draw bird
                // draw_circle(game_state.bird.position.x, game_state.bird.position.y, 20.0, YELLOW);
                draw_texture_ex(&player_texture, game_state.bird.position.x, game_state.bird.position.y, WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(player_texture.width() * player_scale_factor, player_texture.height() * player_scale_factor)),
                        ..Default::default()  // use default values for other parameters
                    },);

                
                // Draw Coins
                for coin in &game_state.coins {
                    draw_circle(coin.position.x, coin.position.y, coin.radius, GOLD);
                }
                // Draw pipes
                for pipe in &game_state.pipes {
                    draw_rectangle(pipe.position.x, 0.0, 60.0, pipe.gap_y - 90.0, DARKGREEN);
                    draw_rectangle(pipe.position.x, pipe.gap_y + 90.0, 60.0, SCREEN_HEIGHT - pipe.gap_y - 90.0, DARKGREEN);
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
