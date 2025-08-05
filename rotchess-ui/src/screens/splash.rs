use macroquad::{
    shapes::draw_rectangle,
    text::{TextParams, draw_text},
};

use super::{GlobalData, Screen, ScreenId};

pub struct Splash {}

impl Splash {
    pub fn new() -> Self {
        Self {}
    }
}

impl Screen for Splash {
    fn id(&self) -> ScreenId {
        ScreenId::Splash
    }

    fn enter(&mut self, global_data: &mut GlobalData) {}

    fn exit(&mut self, global_data: &mut GlobalData) {}

    fn update(&mut self, global_data: &mut GlobalData) -> Option<ScreenId> {
        None
    }

    fn draw(&self) {
        macroquad::text::draw_text_ex(
            "hello!AAAAAAAAAAAAAAAA",
            100.,
            300.,
            TextParams {
                font: Some(super::super::font()),
                font_size: 50,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color: macroquad::color::BLACK,
            },
        );
    }
}
