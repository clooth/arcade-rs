// Namespaces
use phi::{Phi, View, ViewAction};
use sdl2::pixels::Color;


pub struct MenuView;

impl View for MenuView {
    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let renderer = &mut context.renderer;
        let events = &context.events;

        if events.now.quit || events.now.key_escape == Some(true) {
            return ViewAction::Quit;
        }

        if events.now.key_space == Some(true) {
            return ViewAction::ChangeView(Box::new(GameView));
        }

        renderer.set_draw_color(Color::RGB(236, 240, 241));
        renderer.clear();

        ViewAction::None
    }
}


pub struct GameView;

impl View for GameView {
    fn render(&mut self, context: &mut Phi, _: f64) -> ViewAction {
        let renderer = &mut context.renderer;
        let events = &context.events;

        if events.now.quit {
            return ViewAction::Quit;
        }

        if events.now.key_escape == Some(true) {
            return ViewAction::ChangeView(Box::new(MenuView));
        }

        renderer.set_draw_color(Color::RGB(44, 62, 80));
        renderer.clear();

        ViewAction::None
    }
}
