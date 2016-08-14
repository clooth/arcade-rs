// Internal dependencies
#[macro_use]
mod events;

// Namespaces
use sdl2::render::Renderer;

// Event bindings
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

/// Bundles the Phi abstractions in a single structure which can be passed easily between functions.
pub struct Phi<'window> {
    pub events: Events,
    pub renderer: Renderer<'window>, // Renderer holds a reference to Window
}


/// A `ViewAction` is a way for the currently executed view to communicate with the game loop.
/// It specifies which action should be executed before the next rendering.
pub enum ViewAction {
    None,
    Quit,
}

pub trait View {
    /// Called on every frame to take care of both the logic and the rendering of the current view.
    ///
    /// `elapsed` is expressed in seconds.
    fn render(&mut self, context: &mut Phi, elapsed: f64) -> ViewAction;
}
