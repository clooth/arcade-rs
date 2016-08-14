// External dependencies
extern crate sdl2;

// Internal dependencies
mod phi;
mod views;

// Namespaces
use sdl2::pixels::Color;
use phi::{Events, Phi, View, ViewAction};

// Main program
fn main() {
    ::phi::spawn("Shooter", |_| Box::new(::views::MenuView));
}
