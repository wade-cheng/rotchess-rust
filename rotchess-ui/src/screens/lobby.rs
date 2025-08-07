use macroquad::{
    math::vec2,
    text::TextParams,
    ui::{hash, root_ui, widgets},
    window::clear_background,
};
use rotchess_core::piece;

use super::{GlobalData, Screen, ScreenId};

/// Settings for the authority of the player, i.e. whether they're host or client.
enum AuthoritySettings {
    Unselected,
    /// Host may choose which side they start on.
    Host(piece::Side),
    /// Client must supply a lobby key from the host.
    ///
    /// See [`Lobby::lobby_key_buf`]. We cannot keep it here because macroquad ui issue.
    Client,
}

struct OnlineSettings {
    authority: AuthoritySettings,
}

/// Whether this local player should be a human or AI.
enum PlayerKind {
    Human,
    Ai,
}

/// We hold the settings that the user can change.
enum LobbySettings {
    Online(OnlineSettings),
    /// Settings for the player kinds for P1 and P2.
    Local(PlayerKind, PlayerKind),
    Unselected,
}

pub struct Lobby {
    settings: LobbySettings,
    lobby_key_buf: String,
}

impl Lobby {
    pub fn new() -> Self {
        Self {
            settings: LobbySettings::Unselected,
            lobby_key_buf: String::new(),
        }
    }
}

impl Screen for Lobby {
    fn id(&self) -> ScreenId {
        ScreenId::Lobby
    }

    fn enter(&mut self, _global_data: &mut GlobalData) {
        self.settings = LobbySettings::Unselected;
    }

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
        macroquad::text::draw_text_ex(
            "(NEW LOBBY TIME YEAHHHH)",
            10.,
            90.,
            TextParams {
                font: Some(crate::common::font()),
                font_size: 15,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color: macroquad::color::BLACK,
            },
        );

        if root_ui().button(vec2(10., 10.), "back") {
            return Some(ScreenId::Splash);
        }

        root_ui().window(hash!(), vec2(10., 100.), vec2(300., 200.), |ui| {
            ui.label(vec2(10., 10.), "play local");
            widgets::Group::new(hash!(), vec2(280., 65.))
                .position(vec2(10., 10. + 20.))
                .ui(ui, |ui| {
                    ui.button(vec2(200., 0.), "start game");

                    ui.combo_box(hash!(), "   P1", &["Human", "AI"], None);
                    ui.separator();
                    ui.combo_box(hash!(), "   P2", &["Human", "AI"], None);
                });
            ui.label(vec2(10., 10. + 10. + 85.), "play online");
            widgets::Group::new(hash!(), vec2(280., 65.))
                .position(vec2(10., 10. + 10. + 20. + 85.))
                .ui(ui, |ui| {
                    ui.combo_box(hash!(), "            ", &["as black", "as white"], None);
                    ui.button(vec2(140., 2.), "host game");
                    ui.separator();
                    widgets::InputText::new(hash!())
                        .size(vec2(122., 20.))
                        .ui(ui, &mut self.lobby_key_buf);
                    ui.button(vec2(140., 27.), "join game");
                });
        });
        None
    }

    fn draw(&self) {}
}
