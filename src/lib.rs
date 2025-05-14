use macroquad::color::Color;
use macroquad::color::colors::*;
use macroquad::input;
use macroquad::input::MouseButton;
use macroquad::shapes;
use macroquad::text;
use macroquad::window;

mod screen;
use screen::{Event, GlobalData, Screen};

use statig::blocking::{IntoStateMachineExt, StateMachine};

pub struct App {
    context: GlobalData,
    screen: StateMachine<Screen>,
}

impl App {
    pub fn new() -> Self {
        App {
            context: GlobalData { bg_color: RED },
            screen: Screen::default().state_machine(),
        }
    }

    pub fn update(&mut self) {
        self.screen
            .handle_with_context(&Event::Dummy1, &mut self.context);
    }

    pub fn render(&self) {
        window::clear_background(self.context.bg_color);

        shapes::draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        shapes::draw_rectangle(
            window::screen_width() / 2.0 - 60.0,
            100.0,
            120.0,
            60.0,
            GREEN,
        );
        let (x, y) = input::mouse_position();
        shapes::draw_circle(x, y, 15.0, YELLOW);
        text::draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);
    }
}
