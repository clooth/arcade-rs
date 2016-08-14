// Namespaces
macro_rules! struct_events {
    (
        // keyboard alias -> sdl code mapping
        keyboard: { $( $k_alias:ident : $k_sdl:ident ),* },

        // Match against a pattern
        else: { $( $e_alias:ident : $e_sdl:pat ),* }
    ) => {
        use sdl2::EventPump;

        pub struct ImmediateEvents {
            // For every keyboard event,we have an Option<bool>
            // Some(true)  => Was just pressed
            // Some(false) => Was just released
            // None        => Nothing happening _now_
            $( pub $k_alias: Option<bool> , )*

            // Other events
            $( pub $e_alias: bool ),*
        }

        impl ImmediateEvents {
            pub fn new() -> ImmediateEvents {
                ImmediateEvents {
                    // Set all aliases' status to None on reinit
                    $( $k_alias: None, )*
                    $( $e_alias: false ),*
                }
            }
        }

        pub struct Events {
            pump: EventPump,
            pub now: ImmediateEvents,

            // true  => pressed
            // false => not pressed
            $( pub $k_alias: bool ),*
        }

        impl Events {
            pub fn new(pump: EventPump) -> Events {
                Events {
                    pump: pump,
                    now: ImmediateEvents::new(),

                    // By default, none of the keys are pressed
                    $( $k_alias: false ),*
                }
            }

            pub fn pump(&mut self) {
                self.now = ImmediateEvents::new();

                for event in self.pump.poll_iter() {
                    use sdl2::event::Event::*;
                    use sdl2::keyboard::Keycode::*;

                    match event {
                        // When a key is pressed
                        KeyDown { keycode, .. } => match keycode {
                            // $( ... ),* containing $k_sdl and $k_alias means:
                            //   "for every element ($k_alias : $k_sdl) pair,
                            //    check whether the keycode is Some($k_sdl). If it is,
                            //    then set the $k_alias field to true.""
                            $(
                                Some($k_sdl) => {
                                    // Prevent multiple presses when user keeps key down
                                    if !self.$k_alias {
                                        self.now.$k_alias = Some(true);
                                    }

                                    self.$k_alias = true;
                                }
                            ),* // Add comma after every option
                            _ => {}
                        },

                        // Key released
                        KeyUp { keycode, .. } => match keycode {
                            $(
                                Some($k_sdl) => {
                                    self.now.$k_alias = Some(false);
                                    self.$k_alias = false;
                                }
                            ),*
                            _ => {}
                        },

                        // Listen for other events
                        $(
                            $e_sdl => {
                                self.now.$e_alias = true;
                            }
                        )*,

                        _ => {}
                    }
                }
            }
        }
    }
}
