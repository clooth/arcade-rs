// External dependencies
extern crate sdl2;

// Internal dependencies
mod phi;
mod views;

// Namespaces
use sdl2::pixels::Color;
use phi::{Events, Phi, View, ViewAction};

/// The main program
fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();

    // Create window
    let window = video.window("Shooter", 800, 600)
        .position_centered()
        .opengl()
        .allow_highdpi()
        .build()
        .unwrap();

    // Create the Phi context
    let mut context = Phi {
        events: Events::new(sdl_context.event_pump().unwrap()),
        renderer: window.renderer()
            .accelerated()
            .build()
            .unwrap(),
    };

    // Create the default view
    let mut current_view: Box<View> = Box::new(::views::DefaultView);

    // Frame timing
    let interval = 1_000 / 60;
    let mut before = timer.ticks();
    let mut last_second = timer.ticks();
    let mut fps = 0u16;

    // Game loop
    loop {
        // Frame timing (bis)
        let now = timer.ticks();
        let delta = now - before;
        let elapsed = delta as f64 / 1_000.0;

        // If the time elapsed since the last frame is too small, wait out the
        // difference and try again.
        if delta < interval {
            timer.delay(interval - delta);
            continue;
        }

        // Update tick time and
        before = now;
        fps += 1;

        // Calculate average fps
        if now - last_second > 1_000 {
            println!("FPS: {}", fps);
            last_second = now;
            fps = 0;
        }

        // Pump new events
        context.events.pump();

        match current_view.render(&mut context, 0.01) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break,
        }
    }
}
