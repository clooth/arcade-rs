// Namespaces
use phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;
use sdl2::rect::Rect as SdlRect;


/// Pixels traveled by the player's hip every second when it's moving.
const PLAYER_SPEED: f64 = 180.0;


/// Simple Rectangle struct that uses real coordinates instead of integral ones.
/// Allows us to add some utility methods (for example collision checking).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    /// Generates an SDL-compatible Rect equivalent to `self`
    /// Panics if it could not be created, for example if a
    /// coordinate of a corner overflows an `i32`.
    pub fn to_sdl(self) -> SdlRect {
        // Reject negative width and height
        assert!(self.w >= 0.0 && self.h >= 0.0);

        // SdlRect::new : `(i32, i32, u32, u32) -> Result<Option<SdlRect>>`
        SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }

    /// Return a (perhaps moved) rectangle which is contained by a `parent`
    /// rectangle. If it can indeed be moved to fit, Return `Some(result)`;
    /// otherwise, return `None`.
    pub fn move_inside(self, parent: Rectangle) -> Option<Rectangle> {
        // It must be smaller than the parent to fit
        if self.w > parent.w || self.h > parent.h {
            return None;
        }

        Some(Rectangle {
            w: self.w,
            h: self.h,

            x: if self.x < parent.x {
                parent.x
            } else if self.x + self.w >= parent.x + parent.w {
                parent.x + parent.w - self.w
            } else {
                self.x
            },

            y: if self.y < parent.y {
                parent.y
            } else if self.y + self.h >= parent.y + parent.h {
                parent.y + parent.h - self.h
            } else {
                self.y
            },
        })
    }
}


struct Ship {
    rect: Rectangle,
}


pub struct ShipView {
    player: Ship,
}

impl ShipView {
    pub fn new(phi: &mut Phi) -> ShipView {
        ShipView {
            player: Ship {
                rect: Rectangle {
                    x: 64.0,
                    y: 64.0,
                    w: 32.0,
                    h: 32.0,
                },
            },
        }
    }
}

impl View for ShipView {
    fn render(&mut self, phi: &mut Phi, elapsed: f64) -> ViewAction {
        if phi.events.now.quit || phi.events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        // Whether we're moving diagonally or not
        // NOTE: XOR = if only one of given is true, not both
        let diagonal = (phi.events.key_up ^ phi.events.key_down) &&
                       (phi.events.key_left ^ phi.events.key_right);

        // Calculate speed based on whether we're moving diagonally or linearily
        let moved = if diagonal { 1.0 / 2.0f64.sqrt() } else { 1.0 } * PLAYER_SPEED * elapsed;

        // Horizontal movement
        let deltaX = match (phi.events.key_left, phi.events.key_right) {
            // Both pressed or neither pressed
            (true, true) | (false, false) => 0.0,
            // Left pressed
            (true, false) => -moved,
            // Right pressed
            (false, true) => moved,
        };

        // Vertical movement
        let deltaY = match (phi.events.key_up, phi.events.key_down) {
            // Both pressed or neither pressed
            (true, true) | (false, false) => 0.0,
            // Up pressed
            (true, false) => -moved,
            // Down pressed
            (false, true) => moved,
        };

        self.player.rect.x += deltaX;
        self.player.rect.y += deltaY;

        // The movable region spans the entire height of the window and 70% of its width.
        // This way, the player cannot get to the far right of the screen, where we will spawn
        // the asteroids, and get immediately eliminated.
        let movable_region = Rectangle {
            x: 0.0,
            y: 0.0,
            w: phi.output_size().0 * 0.70,
            h: phi.output_size().1,
        };

        // Move player
        self.player.rect = self.player.rect.move_inside(movable_region).unwrap();

        // Clear the screen
        phi.renderer.set_draw_color(Color::RGB(44, 64, 80));
        phi.renderer.clear();

        // Render the scene
        phi.renderer.set_draw_color(Color::RGB(231, 76, 60));
        phi.renderer.fill_rect(self.player.rect.to_sdl());

        ViewAction::None
    }
}



// pub struct ShipView;
//
// impl ShipView {
// pub fn new(phi: &mut Phi) -> ShipView {
// ShipView
// }
// }
//
// impl View for ShipView {
// fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
// if phi.events.now.quit {
// return ViewAction::Quit;
// }
//
// View logic here
//
// phi.renderer.set_draw_color(Color::RGB(236, 240, 241));
// phi.renderer.clear();
//
// View rendering here
//
// ViewAction::None
// }
// }
//
// pub struct MenuView;
//
// impl View for MenuView {
// fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
// let renderer = &mut context.renderer;
// let events = &context.events;
//
// if events.now.quit || events.now.key_escape == Some(true) {
// return ViewAction::Quit;
// }
//
// if events.now.key_space == Some(true) {
// return ViewAction::ChangeView(Box::new(GameView));
// }
//
// renderer.set_draw_color(Color::RGB(236, 240, 241));
// renderer.clear();
//
// ViewAction::None
// }
// }
//
//
// pub struct GameView;
//
// impl View for GameView {
// fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
// let renderer = &mut context.renderer;
// let events = &context.events;
//
// if events.now.quit {
// return ViewAction::Quit;
// }
//
// if events.now.key_escape == Some(true) {
// return ViewAction::ChangeView(Box::new(MenuView));
// }
//
// renderer.set_draw_color(Color::RGB(44, 62, 80));
// renderer.clear();
//
// ViewAction::None
// }
// }
//
