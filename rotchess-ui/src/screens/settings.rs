use super::{GlobalData, Screen, ScreenId};

pub struct Settings {}

impl Settings {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for Settings {
    fn id(&self) -> ScreenId {
        ScreenId::Settings
    }

    fn enter(&mut self, _global_data: &mut GlobalData) {}

    fn exit(&mut self, _global_data: &mut GlobalData) {}

    fn update(&mut self, _global_data: &mut GlobalData) -> Option<ScreenId> {
        None
    }

    fn draw(&self) {}
}
