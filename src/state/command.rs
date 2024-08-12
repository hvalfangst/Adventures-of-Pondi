use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

use minifb::Key;

use crate::state::{ACCELERATION, jump_obstacles, JUMP_VELOCITY, MAX_VELOCITY, Obstacle};
use crate::state::player::Player;

pub trait Command {
    fn execute(&self, player: &mut Player, obstacles: &Vec<Obstacle>);
}

pub struct MoveLeft;

impl Command for MoveLeft {
    fn execute(&self, player: &mut Player, obstacles: &Vec<Obstacle>) {

        // Check if the player has any obstacles to the left by checking if its x coordinate violates any of the thresholds set by obstacles
        let obstacle_left: bool = obstacles.iter().any(|obs| {
            player.obstacle_left = true;
            player.x < obs.x_right && player.x > obs.x_right -10.0 && player.y == obs.y_transition_pos
        });

        if !obstacle_left {

            player.obstacle_left = false;

            // Update velocity if no collision is detected
            player.vx += ACCELERATION;
            if player.vx > MAX_VELOCITY {
                player.vx = MAX_VELOCITY;
            }

            player.last_key = Some(Key::A);
            player.direction = "LEFT".parse().unwrap();

            // Initialize a new field to track the frame count
            player.left_increment_frame_count += 1;

            if player.left_increment_frame_count >= 3 {
                player.left_increment_frame_count = 0; // Reset the frame count


                match player.left_increment {
                    7 => {
                        player.left_increment = 4;
                    }
                    _ => {
                        player.left_increment += 1;
                    }
                };
            }

            // Move player based on current velocity
            player.x -= player.vx;

            jump_obstacles(player);

        }  else {
            // Stop the player from moving left if colliding
            player.vx = 0.0;
        }
    }
}

pub struct MoveRight;

impl Command for MoveRight {
    fn execute(&self, player: &mut Player, obstacles: &Vec<Obstacle>) {

        // Check if the player has any obstacles to the right by checking if its x coordinate violates any of the thresholds set by obstacles
        let obstacle_right: bool = obstacles.iter().any(|obs| {
            player.obstacle_right = true;
            player.x > obs.x_left && player.x < obs.x_left + 10.0 && player.y >= obs.y_transition_pos
        });

        if !obstacle_right {
            player.obstacle_right = false;

            // Update velocity if no collision is detected
            player.vx += ACCELERATION;
            if player.vx > MAX_VELOCITY {
                player.vx = MAX_VELOCITY;
            }

            player.last_key = Some(Key::D);
            player.direction = "RIGHT".parse().unwrap();

            // Initialize a new field to track the frame count
            player.right_increment_frame_count += 1;

            if player.right_increment_frame_count >= 3 {
                player.right_increment_frame_count = 0; // Reset the frame count

                match player.right_increment {
                    3 => {
                        player.right_increment = 0;
                    }
                    _ => {
                        player.right_increment += 1;
                    }
                }
            }

            // Move player based on current velocity
            player.x += player.vx;

            jump_obstacles(player);

        }   else {
            // Stop the player from moving right if colliding
            player.vx = 0.0;
        }
    }
}

pub struct Jump;

impl Command for Jump {
    fn execute(&self, player: &mut Player, _obstacles: &Vec<Obstacle>) {

        if !player.is_jumping {
            player.vy = JUMP_VELOCITY;
            player.on_ground = false;
            player.last_key = Some(Key::Space);
            player.is_jumping = true;
        }
    }
}

pub struct Kick;

impl Command for Kick {
    fn execute(&self, player: &mut Player, _obstacles: &Vec<Obstacle>) {

        player.last_key = Some(Key::X);

        match player.direction.as_str() {
            "RIGHT" => player.right_increment = 2,
            "LEFT"  => player.left_increment = 5,
            _ => {}
        };
    }
}

pub type CommandMap = HashMap<Key, Arc<dyn Command>>;

pub fn initialize_command_map() -> CommandMap {
    let mut commands: CommandMap = HashMap::new();

    commands.insert(Key::A, Arc::new(MoveLeft));
    commands.insert(Key::D, Arc::new(MoveRight));
    commands.insert(Key::Space, Arc::new(Jump));
    commands.insert(Key::X, Arc::new(Kick));

    commands
}

