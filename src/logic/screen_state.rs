use macroquad::color::Color;
use statig::Response;
use statig::state_machine;

use super::screen_logic;

pub enum Command {
    Update,
    Render,
}

pub struct GlobalData {
    pub tick_command: Option<Command>,
    pub bg_color: Color,
}

impl GlobalData {
    pub fn new() -> Self {
        Self {
            tick_command: None,
            bg_color: macroquad::color::RED,
        }
    }
}

/// Dummy value enum to satisfy type checking. If we leave the start state without an event param,
/// statig miiiight let us use &() in handle_with_context. This is just here because
/// we're probably hand-coding an event queue later anyways.
pub enum Event {
    /// Dummy value. will be changed.
    Dummy1,
    /// Dummy value. will be changed.
    LeftClick {
        x: f32,
        y: f32,
    },
    RightClick {
        x: f32,
        y: f32,
    },
}

#[derive(Default)]
pub struct Screen;

#[state_machine(initial = "State::start()")]
impl Screen {
    #[state(entry_action = "make_screen_light")]
    pub fn start(event: &Event, context: &mut GlobalData) -> Response<State> {
        match context.tick_command {
            Some(Command::Update) => screen_logic::start_update(event, context),
            Some(Command::Render) => screen_logic::start_render(context),
            None => unreachable!(),
        }
    }

    #[action]
    pub fn make_screen_light(context: &mut GlobalData) {
        context.bg_color = Color {
            r: rand::random_range(0.7..1.0),
            g: rand::random_range(0.7..1.0),
            b: rand::random_range(0.7..1.0),
            a: 1.0,
        }
    }

    #[state(entry_action = "make_screen_dark")]
    pub fn darkness(event: &Event, context: &mut GlobalData) -> Response<State> {
        match context.tick_command {
            Some(Command::Update) => screen_logic::darkness_update(event, context),
            Some(Command::Render) => screen_logic::darkness_render(context),
            None => unreachable!(),
        }
    }

    #[action]
    pub fn make_screen_dark(context: &mut GlobalData) {
        context.bg_color = Color {
            r: rand::random_range(0.0..0.3),
            g: rand::random_range(0.0..0.3),
            b: rand::random_range(0.0..0.3),
            a: 1.0,
        }
    }
}
