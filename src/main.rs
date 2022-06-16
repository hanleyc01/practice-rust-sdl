use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas}; use std::collections::VecDeque;
// type WindowCanvas = Canvas<Window>
use std::time::Duration;

const PLAYER_MOVEMENT_SPEED: i32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Abstract representation of the player as a point on the screen, and a 
/// location within the asset spritesheet
#[derive(Debug)]
struct Player {
    position: Point,
    sprite: Rect,
    speed: i32,
    movm_queue: VecDeque<Direction>,
}

impl Player {
    fn new(position: Point, sprite: Rect, speed: i32) -> Self {
        Self { position, sprite, speed, movm_queue: VecDeque::new() }
    }
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
    // Origin shifted from top left to center of the screen
    let world_origin = Point::new(width as i32 / 2, height as i32 / 2);

    let screen_position = player.position + world_origin;

    let screen_rect = Rect::from_center(
        screen_position,
        player.sprite.width(),
        player.sprite.height(),
    );
    // We could also use copy_ex() for more oprtions :3
    canvas.copy(
        &texture,
        player.sprite, // loc of sprite in spritesheet
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
    use self::Direction::*;
    match player.movm_queue.pop_front() {
        // Left => {
        //     player.position = player.position.offset(-player.speed, 0);
        // },
        // Right => {
        //     player.position = player.position.offset(player.speed, 0);
        // },
        // Up => {
        //     player.position = player.position.offset(0, -player.speed);
        // },
        // Down => {
        //     player.position = player.position.offset(0, player.speed);
        // },
        Some(movm) => todo!(),
        None => todo!(),
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
    let texture = texture_creator.load_texture("assets/bardo.png")?;

    let mut player = Player::new(
        Point::new(0, 0), 
        Rect::new(0, 0, 26, 36),
        0i32,
    );

    // if player is currently moving to stop directional inputs affecting current movement
    let mut is_moving: bool = false;

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
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, ..} => {
                    move_queue.push_back(Direction::Left);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, ..} => {
                    move_queue.push_back(Direction::Right);
                    if !is_moving {
                        player.speed= PLAYER_MOVEMENT_SPEED;
                        player.direction = Direction::Right;
                        is_moving = true;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, ..} => {
                    move_queue.push_back(Direction::Up);
                    if !is_moving {
                        player.speed = PLAYER_MOVEMENT_SPEED;
                        player.direction = Direction::Up;
                        is_moving = true;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, ..} => {
                    move_queue.push_back(Direction::Down);
                    if !is_moving {
                        player.speed= PLAYER_MOVEMENT_SPEED;
                        player.direction = Direction::Down;
                        is_moving = true;
                    }
                },
                
                // When the arrow key is released
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    match move_queue.pop_front() {
                        Some(mov) => {
                            if mov == Direction::Left {

                            }
                        },
                        None => (),
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    if is_moving == true && player.direction == Direction::Right {
                        player.speed = 0;
                        is_moving = false;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    if is_moving == true && player.direction == Direction::Up {
                        player.speed = 0;
                        is_moving = false;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    if is_moving == true && player.direction == Direction::Down {
                        player.speed = 0;
                        is_moving = false;
                    }
                },
                _ => {},
            }
        }

        // Update
        i = (i + 1) % 255;
        update_player(&mut player);

        // Render
        let color = Color::RGB(i, 64, 255 - i);
        render(&mut canvas, color, &texture, &player)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
