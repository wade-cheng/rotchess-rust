use super::{GlobalData, Screen, ScreenId};

pub struct Singleplayer {}

impl Singleplayer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for Singleplayer {
    fn id(&self) -> ScreenId {
        ScreenId::Singleplayer
    }

    fn enter(&mut self, _global_data: &mut GlobalData) {}

    fn exit(&mut self, _global_data: &mut GlobalData) {}

    fn update(&mut self, _global_data: &mut GlobalData) -> Option<ScreenId> {
        None
    }

    fn draw(&self) {}
}
