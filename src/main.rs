// Include SDL2 crate
extern crate sdl2;

// Import namespaces for convenience
use sdl2::pixels::Color;
use std::thread;
use std::time::Duration;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Create window
    let window = video.window("Shooter", 800, 600)
        .position_centered().opengl()
        .build().unwrap();

    let mut renderer = window.renderer()
        .accelerated()
        .build().unwrap();

    // Render a fully black window
    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

    // Sleep for a bit before closing
    thread::sleep(Duration::from_millis(3000));
}
