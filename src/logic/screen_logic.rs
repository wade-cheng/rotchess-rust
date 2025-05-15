use macroquad::color::colors::*;
use macroquad::input;
use macroquad::input::KeyCode;
use macroquad::input::MouseButton;
use macroquad::shapes;
use macroquad::text;
use macroquad::window;
use statig::Response;

use crate::logic::screen_state::Event;
use crate::logic::screen_state::GlobalData;
use crate::logic::screen_state::Screen;
use crate::logic::screen_state::State;
pub fn start_update(context: &mut GlobalData) -> Response<State> {
    if input::is_mouse_button_pressed(MouseButton::Left) {
        Screen::make_screen_light(context);
    }

    if input::is_mouse_button_pressed(MouseButton::Right) {
        return Response::Transition(State::Darkness {});
    }

    Response::Handled
}

pub fn start_render(context: &GlobalData) -> Response<State> {
    window::clear_background(context.bg_color);

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

    Response::Handled
}

pub fn darkness_update(context: &mut GlobalData) -> Response<State> {
    if input::is_key_pressed(KeyCode::Space) {
        Screen::make_screen_dark(context);
    }

    if input::is_key_pressed(KeyCode::A) {
        return Response::Transition(State::Start {});
    }

    Response::Handled
}

pub fn darkness_render(context: &GlobalData) -> Response<State> {
    window::clear_background(context.bg_color);

    let (x, y) = input::mouse_position();
    shapes::draw_circle(x, y, 15.0, BLACK);
    text::draw_text("HELLO", 20.0, 20.0, 20.0, WHITE);

    Response::Handled
}
