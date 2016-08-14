// Namespaces
use sdl2::EventPump;

//
pub struct Events {
    pump: EventPump,

    pub quit: bool,
    pub key_escape: bool,
}

impl Events {
    pub fn new(pump: EventPump) -> Events {
        Events {
            pump: pump,

            quit: false,
            key_escape: false,
        }
    }

    // Update the events record.
    pub fn pump(&mut self) {
        // If the SDL context is dropped, then poll_iter() will simply stop yielding.
        for event in self.pump.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;

            match event {
                // User has asked to close the window, so set quit to true
                Quit { .. } => self.quit = true,

                // User pressed a key down
                KeyDown { keycode, .. } => match keycode {
                    // User pressed the Escape key down
                    Some(Escape) => self.key_escape = true,
                    // Default case
                    _ => {}
                },

                // User released a key
                KeyUp { keycode, .. } => match keycode {
                    // User released the Escape key
                    Some(Escape) => self.key_escape = false,
                     // Default case
                     _ => {}
                },

                // Default case
                _ => {}
            }
        }
    }
}
