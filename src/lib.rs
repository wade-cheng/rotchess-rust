use macroquad::color::Color;
use macroquad::color::colors::*;
use macroquad::input;
use macroquad::input::MouseButton;
use macroquad::shapes;
use macroquad::text;
use macroquad::window;

pub struct App {
    bg_color: Color,
}

impl App {
    pub fn new() -> Self {
        App { bg_color: RED }
    }

    pub fn update(&mut self) {
        if input::is_mouse_button_pressed(MouseButton::Left) {
            self.bg_color = Color {
                r: rand::random_range(0.0..1.0),
                g: rand::random_range(0.0..1.0),
                b: rand::random_range(0.0..1.0),
                a: 1.0,
            }
        }
    }

    pub fn render(&self) {
        window::clear_background(self.bg_color);

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
