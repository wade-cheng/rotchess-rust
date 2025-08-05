use super::{GlobalData, Screen, ScreenId};

pub struct Load {}

impl Load {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for Load {
    fn id(&self) -> ScreenId {
        ScreenId::Load
    }

    fn enter(&mut self, global_data: &mut GlobalData) {}

    fn exit(&mut self, global_data: &mut GlobalData) {}

    fn update(&mut self, global_data: &mut GlobalData) -> Option<ScreenId> {
        None
    }

    fn draw(&self) {}
}
