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

    // Game loop
    loop {
        // Pump new events
        context.events.pump();

        match current_view.render(&mut context, 0.01) {
            ViewAction::None => context.renderer.present(),
            ViewAction::Quit => break,
        }
    }
}
