use macroquad::prelude::*;
use macroquad::rand::gen_range;

fn create_example_map() -> Vec<Vec<char>> {
      let map_data = vec![
        "###############################",
        "#         x                  ##",
        "#  #####   #####   ########## #",
        "#       #               #     #",
        "#  #####   #####   #####     ##",
        "#      ##                     #",
        "#  #########   ################",
        "#                             #",
        "#########   #########   #######",
        "#              #              #",
        "#   ###################   #####",
        "#          #                  #",
        "###############################",
    ];

    let map: Vec<Vec<char>> = map_data
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();

    map
}

struct Point {
  x: usize,
  y: usize
}

fn generate_random_location(map: &mut Vec<Vec<char>>) -> Point {
  // Randomly position the texture
  let mut random_x;
  let mut random_y;
  loop {
      random_x = gen_range(0, map[0].len() as i32) as usize; // Use macroquad's gen_range
      random_y = gen_range(0, map.len() as i32) as usize;    // Use macroquad's gen_range
      if map[random_y][random_x] == ' ' {
          break;
      }
  }

  Point{x: random_x, y: random_y}
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut map = create_example_map();
    let tile_size = 56.0;
    let mut score = 0;
    let scoreboard_height = tile_size;
    let start_time = get_time();
    let game_length_seconds = 60.0;

    // Initialize the protagonist's position
    let mut protagonist_x = 10 as usize;
    let mut protagonist_y = 1 as usize;

    // Load the images as textures
    let wall_texture = load_texture("freetileset/png/Tiles/2.png").await.unwrap();
    let floor_texture = load_texture("freetileset/png/Tiles/5.png").await.unwrap();
    let protagenist = load_texture("protagenist/Run_1.png").await.unwrap();
    let logo_texture = load_texture("linkit_huisstijl_to_send/Logos/Alternative_versions/logo-square.png").await.unwrap();
    let random_texture = load_texture("linkit_huisstijl_to_send/Icons/Iconen1.png").await.unwrap();

    let mut random_point = generate_random_location(&mut map);

    loop {

        let elapsed_time = get_time() - start_time;
        if elapsed_time > game_length_seconds {
          break ;
        }

        if random_point.x == protagonist_x && random_point.y == protagonist_y {
          random_point = generate_random_location(&mut map);
          score += 1;
        }

        // Handle input
        if is_key_pressed(KeyCode::Right) && protagonist_x + 1 < map[0].len() && map[protagonist_y][protagonist_x + 1] == ' ' {
            map[protagonist_y][protagonist_x] = ' '; // Clear the old position
            protagonist_x += 1;
        }
        if is_key_pressed(KeyCode::Left) && protagonist_x > 0 && map[protagonist_y][protagonist_x - 1] == ' ' {
            map[protagonist_y][protagonist_x] = ' '; // Clear the old position
            protagonist_x -= 1;
        }
        if is_key_pressed(KeyCode::Up) && protagonist_y > 0 && map[protagonist_y - 1][protagonist_x] == ' ' {
            map[protagonist_y][protagonist_x] = ' '; // Clear the old position
            protagonist_y -= 1;
        }
        if is_key_pressed(KeyCode::Down) && protagonist_y + 1 < map.len() && map[protagonist_y + 1][protagonist_x] == ' ' {
            map[protagonist_y][protagonist_x] = ' '; // Clear the old position
            protagonist_y += 1;
        }
        map[protagonist_y][protagonist_x] = 'x'; // Set the new position

         // Draw the map
         for (y, row) in map.iter().enumerate() {
          for (x, &tile) in row.iter().enumerate() {
              let pos_x = x as f32 * tile_size;
              let pos_y = y as f32 * tile_size + scoreboard_height; // Offset the map by the height of the scoreboard

              match tile {
                  'x' => {
                      draw_texture_ex(
                          &floor_texture,
                          pos_x,
                          pos_y,
                          WHITE,
                          DrawTextureParams {
                              dest_size: Some(vec2(tile_size, tile_size)),
                              ..Default::default()
                          },
                      );
                      draw_texture_ex(
                          &protagenist,
                          pos_x,
                          pos_y,
                          WHITE,
                          DrawTextureParams {
                              dest_size: Some(vec2(tile_size, tile_size)),
                              ..Default::default()
                          },
                      );
                  }
                  '#' => {
                      draw_texture_ex(
                          &wall_texture,
                          pos_x,
                          pos_y,
                          WHITE,
                          DrawTextureParams {
                              dest_size: Some(vec2(tile_size, tile_size)),
                              ..Default::default()
                          },
                      );
                  }
                  ' ' => {
                      draw_texture_ex(
                          &floor_texture,
                          pos_x,
                          pos_y,
                          WHITE,
                          DrawTextureParams {
                              dest_size: Some(vec2(tile_size, tile_size)),
                              ..Default::default()
                          },
                      );
                  }
                  _ => {}
              }

              // Draw the random texture at the random position
              if x == random_point.x as usize && y == random_point.y as usize {
                draw_texture_ex(
                    &random_texture,
                    pos_x,
                    pos_y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(tile_size, tile_size)),
                        ..Default::default()
                    },
                );
            }
          }
      }

      // Draw the scoreboard background
      draw_rectangle(0.0, 0.0, map.get(1).unwrap().len() as f32 * tile_size, scoreboard_height, WHITE);

      // Draw the score
      draw_text(
          &format!("Score: {}", score),
          map.get(1).unwrap().len() as f32 * (tile_size - 10.0), // x position
          30.0, // y position
          40.0, // font size
          BLACK,
      );

      // Draw the logo
      draw_texture_ex(
          &logo_texture,
          10.0,
          0.0,
          WHITE,
          DrawTextureParams {
              dest_size: Some(vec2(tile_size, tile_size)),
              ..Default::default()
          },
      );

      draw_text(
          &format!("Time: {:.0}", game_length_seconds - elapsed_time),
          map.get(1).unwrap().len() as f32 * (tile_size - 20.0), // x position,
          30.0,
          40.0,
          BLACK,
      );

      next_frame().await;
    }

    loop {
      // Clear the screen
      clear_background(WHITE);
  
      // Draw the "Finish" text in the center of the screen
      let finish_text = format!("Finished with {} points!", score);
      let screen_width = screen_width();
      let screen_height = screen_height();
      let text_width = measure_text(&finish_text, None, 80, 1.0).width;
      let text_height = 80.0;
      draw_text(
          &finish_text,
          (screen_width - text_width) / 2.0,
          (screen_height - text_height) / 2.0,
          80.0,
          BLACK,
      );
  
      next_frame().await; // Update the screen
    }
}