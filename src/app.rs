//! An app that lets users play and see (update/draw) chess, computed with help from [`crate::chess`] and macroquad.

use std::{
    collections::HashMap,
    f32::consts::{PI, TAU},
};

use egui::{Pos2, Rect};
use macroquad::prelude::*;

use crate::chess::{
    RotchessEmulator,
    emulator::{self, Event, TravelKind},
    piece::{PIECE_RADIUS, Piece, Pieces},
};

const DARK_TILE_COLOR: Color = Color::from_rgba(181, 136, 99, 255);
const LIGHT_TILE_COLOR: Color = Color::from_rgba(240, 217, 181, 255);
const BACKGROUND_COLOR: Color = Color::from_rgba(230, 230, 230, 255);

/// yellowish
const SELECTED_PIECE_COLOR: Color = Color::from_rgba(255, 255, 153, 200);
/// cyanish
const MOVE_OUTLINE_COLOR: Color = Color::from_rgba(173, 255, 244, 255);
const MOVE_HIGHLIGHT_COLOR: Color = Color::from_rgba(173, 255, 244, 200);
/// red
const CAPTURE_OUTLINE_COLOR: Color = Color::from_rgba(255, 0, 0, 255);
const CAPTURE_HIGHLIGHT_COLOR: Color = Color::from_rgba(255, 0, 0, 200);
/// springgreen
const HITCIRCLE_COLOR: Color = Color::from_rgba(0, 255, 127, 255);

pub struct App {
    chess: RotchessEmulator,
    runit_to_world_multiplier: f32,
    images: HashMap<String, Texture2D>,
}

impl App {
    fn generate_images() -> HashMap<String, Texture2D> {
        let mut images = HashMap::new();
        images.insert(
            "piece_bishopB1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_bishopB1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_bishopW1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_bishopW1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_kingB1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_kingB1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_kingW1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_kingW1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_knightB1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_knightB1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_knightW1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_knightW1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_pawnB1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_pawnB1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_pawnW1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_pawnW1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_queenB1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_queenB1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_queenW1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_queenW1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_rookB1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_rookB1.png"),
                Some(ImageFormat::Png),
            ),
        );
        images.insert(
            "piece_rookW1".to_string(),
            Texture2D::from_file_with_format(
                include_bytes!("assets/pieces_png/piece_rookW1.png"),
                Some(ImageFormat::Png),
            ),
        );

        images
    }

    pub fn new() -> Self {
        Self {
            chess: RotchessEmulator::with(Pieces::standard_board()),
            runit_to_world_multiplier: 0.,
            images: App::generate_images(),
        }
    }

    fn update_runit_to_world_multiplier(&mut self) {
        self.runit_to_world_multiplier = f32::min(screen_width(), screen_height()) / 8.;
    }

    /// Converts from a rotchess unit to world unit (pixel).
    ///
    /// Must be run after we update the ratio after any screen resize, lest the value be outdated.
    fn cnv_r(&self, a: f32) -> f32 {
        a * self.runit_to_world_multiplier
    }

    /// Converts from a world unit (pixel) to rotchess unit.
    ///
    /// Must be run after we update the ratio after any screen resize, lest the value be outdated.
    fn cnv_w(&self, a: f32) -> f32 {
        a / self.runit_to_world_multiplier
    }

    pub fn update(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui")
                .resizable(false)
                // .fixed_pos((10., 10.))
                // .min_size((2000., 500.))
                .fixed_rect(
                    Rect {
                        min: Pos2 {
                            x: self.cnv_r(8.) * screen_dpi_scale(),
                            y: 0.,
                        },
                        max: Pos2 {
                            x: screen_width(),
                            y: screen_height(),
                        },
                    }
                    .shrink(10.),
                )
                .title_bar(false)
                .show(egui_ctx, |ui| {
                    ui.label("Test");
                });
        });
        self.update_runit_to_world_multiplier();
        let (pixel_mouse_x, pixel_mouse_y) = mouse_position();
        let (mouse_x, mouse_y) = (self.cnv_w(pixel_mouse_x), self.cnv_w(pixel_mouse_y));

        if is_key_pressed(KeyCode::Left) {
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                self.chess.handle_event(Event::FirstTurn);
            } else {
                self.chess.handle_event(Event::PrevTurn);
            }
        }

        if is_key_pressed(KeyCode::Right) {
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                self.chess.handle_event(Event::LastTurn);
            } else {
                self.chess.handle_event(Event::NextTurn);
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            self.chess.handle_event(Event::ButtonDown {
                x: mouse_x,
                y: mouse_y,
                button: emulator::MouseButton::LEFT,
            });
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.chess.handle_event(Event::ButtonUp {
                x: mouse_x,
                y: mouse_y,
                button: emulator::MouseButton::LEFT,
            });
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            self.chess.handle_event(Event::ButtonDown {
                x: mouse_x,
                y: mouse_y,
                button: emulator::MouseButton::RIGHT,
            });
        }

        if is_mouse_button_released(MouseButton::Right) {
            self.chess.handle_event(Event::ButtonUp {
                x: mouse_x,
                y: mouse_y,
                button: emulator::MouseButton::RIGHT,
            });
        }

        if mouse_delta_position() != Vec2::ZERO {
            self.chess.handle_event(Event::MouseMotion {
                x: mouse_x,
                y: mouse_y,
            });
        }
    }

    fn draw_board(&self) {
        draw_rectangle(0., 0., self.cnv_r(8.), self.cnv_r(8.), LIGHT_TILE_COLOR);

        let mut top = 0;
        let mut left = 1;
        let mut next_row_immediate_dark = true;

        const NUM_TILES: u8 = 8 * 8;
        const NUM_DARK_TILES: u8 = NUM_TILES / 2;

        for _ in 0..NUM_DARK_TILES {
            draw_rectangle(
                self.cnv_r(left as f32),
                self.cnv_r(top as f32),
                self.cnv_r(1.),
                self.cnv_r(1.),
                DARK_TILE_COLOR,
            );

            left += 2;
            if left >= 8 {
                left = if next_row_immediate_dark { 0 } else { 1 };
                next_row_immediate_dark = !next_row_immediate_dark;
                top += 1;
            }
        }
    }

    fn draw_piece_outline(&self, x: f32, y: f32, color: Color) {
        draw_circle_lines(
            self.cnv_r(x),
            self.cnv_r(y),
            self.cnv_r(PIECE_RADIUS),
            1.,
            color,
        );
    }

    fn draw_piece_highlight(&self, x: f32, y: f32, color: Color) {
        /// Extra addition to the radius of the drawn circle.
        ///
        /// When highlighting a piece, there will be an outline over it. Without
        /// extra tolerance, there will be background poking in between the highlight
        /// and outline.
        const TOLERANCE: f32 = 0.5;
        draw_circle(
            self.cnv_r(x),
            self.cnv_r(y),
            self.cnv_r(PIECE_RADIUS) + TOLERANCE,
            color,
        );
    }

    fn draw_movablepoint_indicator(&self, x: f32, y: f32) {
        draw_circle(
            self.cnv_r(x),
            self.cnv_r(y),
            self.cnv_r(0.12),
            MOVE_HIGHLIGHT_COLOR,
        );
    }

    fn draw_capturablepoint_indicator(&self, x: f32, y: f32) {
        let x = self.cnv_r(x);
        let y = self.cnv_r(y);
        let dist = self.cnv_r(0.12);
        // draw_circle(x, y, 5., MOVE_HIGHLIGHT_COLOR);

        draw_triangle(
            // Vec2 { x, y },
            // Vec2 { x: x + DIST, y },
            // Vec2 { x, y: y - DIST },
            Vec2 { x, y: y - dist },
            Vec2 {
                x: x - dist / 2. * f32::sqrt(3.),
                y: y + dist / 2.,
            },
            Vec2 {
                x: x + dist / 2. * f32::sqrt(3.),
                y: y + dist / 2.,
            },
            CAPTURE_HIGHLIGHT_COLOR,
        );
    }

    fn draw_pieces(&self, show_hitcircles: bool) {
        /// Size as fraction of 1.
        const PIECE_SIZE: f32 = 0.9;
        for piece in self.chess.pieces() {
            draw_texture_ex(
                &self
                    .images
                    .get(&format!(
                        "piece_{}{}1",
                        piece.kind().to_file_desc(),
                        piece.side().to_file_desc()
                    ))
                    .expect("Pieces should correctly map to the file descrs."),
                self.cnv_r(piece.x() - PIECE_SIZE / 2.),
                self.cnv_r(piece.y() - PIECE_SIZE / 2.),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2 {
                        x: self.cnv_r(PIECE_SIZE),
                        y: self.cnv_r(PIECE_SIZE),
                    }),
                    rotation: TAU - piece.angle(),
                    ..Default::default()
                },
            );

            if show_hitcircles {
                self.draw_piece_outline(piece.x(), piece.y(), HITCIRCLE_COLOR);
            }
        }
    }

    pub fn draw(&self) {
        clear_background(BACKGROUND_COLOR);
        self.draw_board();

        let selected = self.chess.selected();

        if let Some((piece, _)) = selected {
            self.draw_piece_highlight(piece.x(), piece.y(), SELECTED_PIECE_COLOR);
        }

        // egui_macroquad::draw();
        self.draw_pieces(selected.is_some());

        if let Some((_, travelpoints)) = selected {
            for tp in travelpoints {
                if tp.travelable {
                    let (xpix, ypix) = mouse_position();
                    if Piece::collidepoint_generic(self.cnv_w(xpix), self.cnv_w(ypix), tp.x, tp.y) {
                        self.draw_piece_highlight(
                            tp.x,
                            tp.y,
                            match tp.kind {
                                TravelKind::Capture => CAPTURE_HIGHLIGHT_COLOR,
                                TravelKind::Move => MOVE_HIGHLIGHT_COLOR,
                            },
                        );
                    } else {
                        match tp.kind {
                            TravelKind::Capture => self.draw_capturablepoint_indicator(tp.x, tp.y),
                            TravelKind::Move => self.draw_movablepoint_indicator(tp.x, tp.y),
                        }
                    }
                }
                self.draw_piece_outline(
                    tp.x,
                    tp.y,
                    match tp.kind {
                        TravelKind::Capture => CAPTURE_OUTLINE_COLOR,
                        TravelKind::Move => MOVE_OUTLINE_COLOR,
                    },
                );
            }
        }

        egui_macroquad::draw();
    }
}
