
mod components;
mod physics;
mod animator;

use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use std::collections::VecDeque;
// type WindowCanvas = Canvas<Window>

use specs::prelude::*;

use std::time::Duration;

use crate::components::*;

const PLAYER_MOVEMENT_SPEED: i32 = 5;

/// Returns the row in the spritesheet for animation
fn direction_spritesheet(direction: Direction) -> i32 {
    use self::Direction::*;
    match direction {
        Up => 3,
        Down => 0,
        Left => 1,
        Right => 2,
    }
}



/// Create character animations using sprite-sheet information
fn character_animation_frames(spritesheet: usize, top_left_frame: Rect, direct: Direction) -> Vec<Sprite> 
{
    // General system for encapsulating animations 
    let (frame_width, frame_height) = top_left_frame.size();
    let y_offset = top_left_frame.y() + frame_height as i32 * direction_spritesheet(direct);

    let mut frames = Vec::new();
    for i in 0..3 
    {
        frames.push(Sprite {
            spritesheet,
            region: Rect::new(
                top_left_frame.x + frame_height as i32 * i,
                y_offset,
                frame_width,
                frame_height,
                ),
        })
    }

    frames
}

/// This is our traditional render function for our game loop:
/// it first sets to a drawing color (of color specified), clears the canvas
/// of any previous items, and then finally `present()`'s the rendering to the wider
/// game loop to let it know that the action is a-okay and complete.
fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    // Defining the loop by which the animation cycles through
    let (frame_width, frame_height) = player.sprite.size();
    let current_frame = Rect::new(
        player.sprite.x() + frame_width as i32 * player.current_frame,
        player.sprite.y() + frame_height as i32 * direction_spritesheet(player.direction),
        frame_width,
        frame_height,
    );
    // + + + +

    // Origin shifted from top left to center of the screen
    let world_origin = Point::new(width as i32 / 2, height as i32 / 2);
    let screen_position = player.position + world_origin;

    let screen_rect = Rect::from_center(screen_position, frame_width, frame_height);
    // We could also use copy_ex() for more oprtions :3
    canvas.copy(
        texture,
        current_frame, // loc of sprite in spritesheet
        screen_rect,   // dest of sprite in window
    )?;

    canvas.present();

    Ok(())
}

/// Instead of having the player's movement be tied to the event-handling section of the loop,
/// instead we've tied it to the updating section of the loop. This allows for a smoother movement
/// and doesn't rely player input entirely to be updated!!
/// WARNING!: The book says that we ought not call this a bunch because the variation in the speed
/// will cause the player's movement to be unpredictable, which is not great.
fn update_player(player: &mut Player) {
    
    // Animates player iff they are moving
    if player.speed != 0 {
        player.current_frame = (player.current_frame + 1) % 3;
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not initialize the canvas");

    let texture_creator = canvas.texture_creator();
    let textures = [
        texture_creator.load_texture("assets/bardo.png")?,
    ];
    
    let mut dispatcher = DispatcherBuilder::new()
        .with(physics::Physics, "Physics", &[])
        .with(animator::Animator, "Animator", &[])
        .build();
    
    let mut world = World::new();
    dispatcher.setup(&mut world);

    // First texture in the textures array
    let player_spritesheet = 0;
    let player_top_left_frame = Rect::new(0,0,26,36);

    let player_animation = MovementAnimation {
        current_frame: 0,
        up_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Up),
        down_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Down),
        left_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Left),
        right_frames: character_animation_frames(player_spritesheet, player_top_left_frame, Direction::Right)
    };

 
    world.create_entity()
        .with(Position(Point::new(0,0)))
        .with(Velocity { speed: 0, direction: Direction::Right })
        .with(player_animation.right_frames[0].clone())
        .with(player_animation)
        .build();

    // if player is currently moving to stop directional inputs affecting current movement
    let mut is_moving: bool = false;
    // tracking "queued" direction; so that if the player inputs a move after
    let mut movm_queue: VecDeque<Direction> = VecDeque::new();

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;
    'running: loop {
        // event handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                // When the arrow key is pressed
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => {
                    movm_queue.push_back(Direction::Left);
                    if !is_moving {
                        player.speed = PLAYER_MOVEMENT_SPEED;
                        player.direction = Direction::Left;
                        is_moving = true;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => {
                    movm_queue.push_back(Direction::Right);
                    if !is_moving {
                        player.speed = PLAYER_MOVEMENT_SPEED;
                        player.direction = Direction::Right;
                        is_moving = true;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => {
                    movm_queue.push_back(Direction::Up);
                    if !is_moving {
                        player.speed = PLAYER_MOVEMENT_SPEED;
                        player.direction = Direction::Up;
                        is_moving = true;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => {
                    movm_queue.push_back(Direction::Down);
                    if !is_moving {
                        player.speed = PLAYER_MOVEMENT_SPEED;
                        player.direction = Direction::Down;
                        is_moving = true;
                    }
                }

                // When the arrow key is released
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    repeat: false,
                    ..
                } => match movm_queue.pop_back() {
                    Some(movm) => {
                        if is_moving && movm == Direction::Left {
                            movm_queue.clear();
                            player.speed = 0;
                            is_moving = false;
                        } else {
                            movm_queue.pop_front();
                            player.direction = movm;
                        }
                    }
                    None => {
                        player.speed = 0;
                        is_moving = false;
                    }
                },
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    repeat: false,
                    ..
                } => match movm_queue.pop_back() {
                    Some(movm) => {
                        if is_moving && movm == Direction::Right {
                            movm_queue.clear();
                            player.speed = 0;
                            is_moving = false;
                        } else {
                            movm_queue.pop_front();
                            player.direction = movm;
                        }
                    }
                    None => {
                        player.speed = 0;
                        is_moving = false;
                    }
                },
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    repeat: false,
                    ..
                } => match movm_queue.pop_front() {
                    Some(movm) => {
                        if is_moving && movm == Direction::Up {
                            movm_queue.clear();
                            player.speed = 0;
                            is_moving = false;
                        } else {
                            movm_queue.pop_front();
                            player.direction = movm;
                        }
                    }
                    None => {
                        player.speed = 0;
                        is_moving = false;
                    }
                },
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    repeat: false,
                    ..
                } => match movm_queue.pop_back() {
                    Some(movm) => {
                        if is_moving && movm == Direction::Down {
                            movm_queue.clear();
                            player.speed = 0;
                            is_moving = false;
                        } else {
                            movm_queue.pop_front();
                            player.direction = movm;
                        }
                    }
                    None => {
                        player.speed = 0;
                        is_moving = false;
                    } // if is_moving == true && player.direction == Direction::Down {
                      //     player.speed = 0;
                      //     is_moving = false;
                      // }
                },
                _ => {}
            }
        }

        // Update
        i = (i + 1) % 255;
        dispatcher.dispatch(&mut world);
        world.maintain();

        // Render
        let color = Color::RGB(i, 64, 255 - i);
        render(&mut canvas, color, &texture, &player)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
