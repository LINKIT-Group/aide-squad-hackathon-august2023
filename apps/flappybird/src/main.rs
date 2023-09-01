use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 1700.0;
const SCREEN_HEIGHT: f32 = 900.0;
const GRAVITY: f32 = 0.25;
const JUMP_STRENGTH: f32 = -5.0;
const PIPE_SPEED: f32 = -3.0;
const PIPE_SPACING: f32 = 550.0;
const FRAME_THICKNESS: f32 = 5.0;
const COIN_SPAWN_RATE: f32 = 0.02;  // for instance, 2% chance every frame

const PIPE_WIDTH: f32 = 60.0;
const PIPE_GAP_HEIGHT: f32 = 200.0;
const PIPE_HEIGHT: f32 = SCREEN_HEIGHT;  // Pipes span from top to bottom of screen

// struct Pipe {
//     position: Vec2,  // This represents the x position and the y position of the bottom pipe's top edge
//     gap_start: f32,  // This denotes the y position where the gap starts
// }
struct Pipe {
    position: Vec2,  // This is the top-left corner of the rectangle
    size: Vec2,      // Width and Height of the rectangle
}

fn create_random_pipe(screen_width: f32) -> Pipe {
    let width = rand::gen_range(30.0, 100.0);   // Random width between 30 and 100 units
    let height = rand::gen_range(100.0, 300.0);  // Random height between 50 and 300 units
    let y_position = rand::gen_range(0.0, SCREEN_HEIGHT - height); // Ensuring the rectangle fits on the screen

    Pipe {
        position: Vec2::new(screen_width, y_position),
        size: Vec2::new(width, height),
    }
}

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

// fn is_bird_colliding_with_pipe(bird: &Bird, pipe: &Pipe) -> bool {
//     // Check collision with bottom pipe
//     if bird.position.x + bird.radius > pipe.position.x &&
//        bird.position.x - bird.radius < pipe.position.x + PIPE_WIDTH &&
//        bird.position.y + bird.radius > pipe.gap_start + PIPE_GAP_HEIGHT {
//            return true;
//     }

//     // Check collision with top pipe
//     if bird.position.x + bird.radius > pipe.position.x &&
//        bird.position.x - bird.radius < pipe.position.x + PIPE_WIDTH &&
//        bird.position.y - bird.radius < pipe.gap_start {
//            return true;
//     }

//     return false;
// }


fn is_bird_colliding_with_pipe(bird: &Bird, pipe: &Pipe) -> bool {
    // Using a helper function to check if the circle collides with the rectangle
    is_circle_colliding_with_rect(bird.position, bird.radius, pipe.position, pipe.size)
}

// Helper function to check if a circle is colliding with a rectangle
fn is_circle_colliding_with_rect(circle_pos: Vec2, circle_radius: f32, rect_pos: Vec2, rect_size: Vec2) -> bool {
    // Find the closest point in the rectangle to the circle
    let closest_x = circle_pos.x.clamp(rect_pos.x, rect_pos.x + rect_size.x);
    let closest_y = circle_pos.y.clamp(rect_pos.y, rect_pos.y + rect_size.y);

    // Calculate the distance between the circle's center and the closest point
    let distance_x = circle_pos.x - closest_x;
    let distance_y = circle_pos.y - closest_y;

    // If the distance is less than the circle's radius, an intersection occurs
    (distance_x * distance_x + distance_y * distance_y) < (circle_radius * circle_radius)
}


fn update_pipes(pipes: &mut Vec<Pipe>) {
    for pipe in pipes.iter_mut() {
        pipe.position.x += PIPE_SPEED;
    }

    pipes.retain(|pipe| pipe.position.x + pipe.size.x > 0.0);
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


// impl Pipe {
//     fn new(x: f32) -> Self {
//         let gap_y = rand::gen_range(SCREEN_HEIGHT * 0.3, SCREEN_HEIGHT * 0.7);
//         Pipe {
//             position: vec2(x, 0.0),
//             gap_y,
//         }
//     }
// }


enum GameMode {
    Playing,
    GameOver,
}

struct GameState {
    bird: Bird,
    pipes: Vec<Pipe>,
    coins: Vec<Coin>,
    score: i32,   // tracks the score, incremented when a coin is collected
    speed_multiplier: f32,
}

impl GameState {
    fn new() -> Self {
        // let bird = Bird::new();
        let bird = Bird {
            position: vec2(SCREEN_WIDTH * 0.2, SCREEN_HEIGHT / 2.0),
            velocity: 0.0,
            radius: 40.0,

        };
        // let pipes = vec![Pipe::new(SCREEN_WIDTH)];
        let pipes = Vec::new();
        let coins = Vec::new();
        // GameState { bird, pipes, coins }
        Self {
            bird: bird,
            pipes: pipes,
            coins: coins,
            score: 0,
            speed_multiplier: 1.0
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


                // Spawn pipes at intervals (for this example, we'll spawn a pipe every few frames for simplicity)
                if game_state.pipes.is_empty() || game_state.pipes.last().unwrap().position.x <= SCREEN_WIDTH - PIPE_SPACING {
                    game_state.pipes.push(create_random_pipe(SCREEN_WIDTH));
                }
                // Pipe logic
                // if game_state.pipes.is_empty() || game_state.pipes.last().unwrap().position.x < SCREEN_WIDTH - PIPE_SPACING {
                //     let num_new_pipes = ((SCREEN_WIDTH - game_state.pipes.last().unwrap().position.x) / PIPE_SPACING) as usize;
                //     for _ in 0..num_new_pipes {
                //         let new_pipe_x = if game_state.pipes.is_empty() {
                //             SCREEN_WIDTH
                //         } else {
                //             game_state.pipes.last().unwrap().position.x + PIPE_SPACING
                //         };
                //         game_state.pipes.push(Pipe::new(new_pipe_x));
                //     }
                // }

                // periodically add coins:
                if rand::gen_range(0.0, 1.0) < COIN_SPAWN_RATE {
                    game_state.coins.push(Coin {
                        position: Vec2::new(SCREEN_WIDTH, rand::gen_range(100.0, SCREEN_HEIGHT - 100.0)),
                        radius: 15.0,
                    });
                }

                // for pipe in &mut game_state.pipes {
                //     pipe.position.x += PIPE_SPEED;
                // }

                update_pipes(&mut game_state.pipes);


                for coin in &mut game_state.coins {
                    coin.position.x += PIPE_SPEED;
                }

                game_state.pipes.retain(|pipe| pipe.position.x > -60.0);

                // Collision with ground or ceiling
                if game_state.bird.position.y <= 0.0 || game_state.bird.position.y + game_state.bird.radius >= SCREEN_HEIGHT {
                    mode = GameMode::GameOver;
                }

                // Collision with pipes
                for pipe in &game_state.pipes {
                    let bird_colliding = is_bird_colliding_with_pipe(&game_state.bird, &pipe);
                    if bird_colliding {
                            mode = GameMode::GameOver;
                            break; // no need to check further pipes if we're restarting
                    }
                    // if game_state.bird.position.y < pipe.gap_y - 50.0 || game_state.bird.position.y > pipe.gap_y + 50.0 {
                    //     if game_state.bird.position.x > pipe.position.x && game_state.bird.position.x < pipe.position.x + 60.0 {
                    //         mode = GameMode::GameOver;
                    //         break; // no need to check further pipes if we're restarting
                    //     }
                    // }
                }

                // collision with coins
                game_state.coins.retain(|coin| {
                    let collided = is_circle_colliding_with_bird(&game_state.bird, coin.position, coin.radius);
                    if collided {
                        game_state.score += 1;  // Assuming you have a score field in GameState
                        game_state.speed_multiplier = 1.0 + 0.1 * game_state.score as f32;
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
                    let color = Color::from_rgba(42, 37, 56, 255);

                    draw_rectangle(pipe.position.x, pipe.position.y, pipe.size.x, pipe.size.y, color);

                    // draw_rectangle(pipe.position.x, 0.0, 60.0, pipe.gap_y - 90.0, DARKGREEN);
                    // draw_rectangle(pipe.position.x, pipe.gap_y + 90.0, 60.0, SCREEN_HEIGHT - pipe.gap_y - 90.0, DARKGREEN);
                }

                // draw score
                let score_text = format!("Score: {}", game_state.score);
                draw_text(&score_text, 10.0, 30.0, 40.0, WHITE);

            }

            GameMode::GameOver => {
                clear_background(Color::new(63.0 / 255.0, 145.0 / 255.0, 195.0 / 255.0, 1.0));
                draw_game_frame();

                
                draw_text("Game Over", SCREEN_WIDTH / 2.0 - 100.0, SCREEN_HEIGHT / 2.0 - 20.0, 40.0, Color::new(42.0 / 255.0, 37.0 / 255.0, 56.0 / 255.0, 1.0));
                draw_text("Click to restart!", SCREEN_WIDTH / 2.0 - 150.0, SCREEN_HEIGHT / 2.0 + 30.0, 30.0, Color::new(42.0 / 255.0, 37.0 / 255.0, 56.0 / 255.0, 1.0));
            

                if is_mouse_button_pressed(MouseButton::Left) {
                    game_state = GameState::new();
                    mode = GameMode::Playing;
                }
            }
        }
       
        next_frame().await;
    }
}
