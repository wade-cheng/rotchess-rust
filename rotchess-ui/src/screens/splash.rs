use macroquad::{math::vec2, text::TextParams, ui::root_ui, window::clear_background};

use super::{GlobalData, Screen, ScreenId};

pub struct Splash {}

impl Splash {
    pub fn new() -> Self {
        // let style = root_ui()
        //     .style_builder()
        //     .with_font(crate::common::font())
        //     .unwrap()
        //     .font_size(30)
        //     .build();

        // let old_default = root_ui().default_skin().clone();
        // root_ui().push_skin(&Skin {
        //     label_style: style.clone(),
        //     ..old_default
        // });
        Self {}
    }
}

impl Screen for Splash {
    fn id(&self) -> ScreenId {
        ScreenId::Splash
    }

    fn enter(&mut self, _global_data: &mut GlobalData) {}

    fn exit(&mut self, _global_data: &mut GlobalData) {}

    fn update(&mut self, _global_data: &mut GlobalData) -> Option<ScreenId> {
        clear_background(macroquad::color::WHITE);
        macroquad::text::draw_text_ex(
            "ROTATING CHESS",
            10.,
            70.,
            TextParams {
                font: Some(crate::common::font()),
                font_size: 30,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color: macroquad::color::BLACK,
            },
        );

        if root_ui().button(vec2(20., 100.), "playground") {
            return Some(ScreenId::Game);
        }
        if root_ui().button(vec2(20., 130.), "new lobby") {
            return Some(ScreenId::Lobby);
        }
        if root_ui().button(vec2(20., 160.), "load game") {
            println!("TODO");
        }
        if root_ui().button(vec2(20., 190.), "settings") {
            println!("TODO");
        }
        // if is_mouse_button_down(macroquad::input::MouseButton::Left) {
        //     Some(ScreenId::Game)
        // } else {
        //     None
        // }
        None
    }

    fn draw(&self) {}
}
