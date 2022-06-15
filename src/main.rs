use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas; // type WindowCanvas = Canvas<Window>
use std::time::Duration;

/// This is our traditional render function for our game loop:
/// it first sets to a drawing color (of color specified), clears the canvas
/// of any previous items, and then finally `present()`'s the rendering to the wider
/// game loop to let it know that the action is a-okay and complete.
fn render(canvas: &mut WindowCanvas, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.present();
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not initialize the canvas");

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
                _ => {}
            }
        }
        
        // Update
        i = (i + 1) % 255;
        
        // Render
        let color = Color::RGB(i, 64, 255 - i);
        render(&mut canvas, color);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
