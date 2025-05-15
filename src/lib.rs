pub mod icon;
pub mod logic;

use logic::screen_state::{Command, Event, GlobalData, Screen};
use statig::blocking::{IntoStateMachineExt, StateMachine};

pub struct App {
    context: GlobalData,
    screen: StateMachine<Screen>,
}

impl App {
    pub fn new() -> Self {
        App {
            context: GlobalData::new(),
            screen: Screen::default().state_machine(),
        }
    }

    pub fn update(&mut self, events: Vec<Event>) {
        self.context.tick_command = Some(Command::Update);
        self.screen
            .handle_with_context(&Event::Dummy1, &mut self.context);
    }

    pub fn render(&mut self) {
        self.context.tick_command = Some(Command::Render);
        self.screen
            .handle_with_context(&Event::Dummy1, &mut self.context);
    }
}
