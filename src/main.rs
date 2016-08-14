// External dependencies
extern crate sdl2;
extern crate sdl2_image;

// Internal dependencies
mod phi;
mod views;

// Namespaces
use sdl2::pixels::Color;
use phi::{Events, Phi, View, ViewAction};

// Main program
fn main() {
    ::phi::spawn("Shooter", |phi| Box::new(::views::ShipView::new(phi)));
}
