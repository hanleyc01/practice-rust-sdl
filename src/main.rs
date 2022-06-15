use sdl2::event::Event;
use sdl2::image::{self, InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas}; // type WindowCanvas = Canvas<Window>
use std::time::Duration;

const PLAYER_MOVEMENT_SPEED: i32 = 20;

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
    direction: Direction,
}

impl Player {
    fn new(position: Point, sprite: Rect, speed: i32) -> Self {
        Self { position, sprite, speed, direction: Direction::Right }
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

fn update_player(player: &mut Player) {
    use self::Direction::*;
    match player.direction {
        Left => {
        },
        Right => {
        },
        Up => {
        },
        Down => {
        },
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
        5i32,
    );

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
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    player.position = player.position.offset(-player.speed, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    player.position = player.position.offset(player.speed, 0);
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    player.position = player.position.offset(0, -player.speed);
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    player.position = player.position.offset(0, player.speed);
                },
                _ => {}
            }
        }

        // Update
        i = (i + 1) % 255;

        // Render
        let color = Color::RGB(i, 64, 255 - i);
        render(&mut canvas, color, &texture, &player)?;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
