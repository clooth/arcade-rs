// External dependencies
extern crate sdl2;

// Internal dependencies
#[macro_use]
mod events;

// Namespaces
use sdl2::pixels::Color;

// Macros
struct_events! {
    keyboard: {
        key_escape: Escape,
        key_up: Up,
        key_down: Down,
        key_left: Left,
        key_right: Right
    },
    else: {
        quit: Quit { .. }
    }
}

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create window
    let window = video.window("Shooter", 800, 600)
        .position_centered().opengl().allow_highdpi()
        .build().unwrap();

    // Create renderer
    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    // Initialize the events handler
    let mut events = Events::new(sdl_context.event_pump().unwrap());

    // Game loop
    loop {
        // Pump new events
        events.pump();

        // Always quit
        if events.now.quit || events.now.key_escape == Some(true) {
            break;
        }

        // Re-draw the scene
        renderer.set_draw_color(Color::RGB(255, 255, 255));
        renderer.clear();
        renderer.present();
    }
}
