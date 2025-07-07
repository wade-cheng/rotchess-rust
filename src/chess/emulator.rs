//! TODO
//!
//! design doc:
//! - on hovering over move/cap points, highlight if it's a possible move
//! - only draw guides for non-jumpers (might be drawer's responsibility)
//! - piece selection on mousedown
//!   - but if piece was alr selected, or no action could be taken, deselect the only selected piece.
//!   - hmm. wondering now, we could probably move only selected to a Option<usize> in each Pieces.
//!   - but that raises the question, can we move everything GamePieceData related to Board?
//! - drag move/cap point to rotate, or mouseup without having dragged to move (if was possible)

use crate::chess::{
    piece::{Piece, Pieces},
    turn::Turns,
};

/// Mouse buttons a chess board can respond to.
///
/// This enum may add new variants.
#[derive(Clone, Copy)]
pub enum MouseButton {
    LEFT,
    RIGHT,
}

/// User events a chess board can respond to.
#[derive(Clone, Copy)]
pub enum Event {
    ButtonDown { x: f32, y: f32, button: MouseButton },
    ButtonUp { x: f32, y: f32, button: MouseButton },
    MouseMotion { x: f32, y: f32 },
    FirstTurn,
    PrevTurn,
    NextTurn,
    LastTurn,
}

#[derive(PartialEq, Debug)]
pub enum TravelKind {
    Capture,
    Move,
}

#[derive(Debug)]
pub struct TravelPoint {
    pub x: f32,
    pub y: f32,
    pub travelable: bool,
    pub kind: TravelKind,
}

pub struct RotchessEmulator {
    /// A valid representation of travelpoints a user needs to draw iff we
    ///  update this every time a piece.core changes and `self.selected.is_some()`.
    travelpoints_buffer: Vec<TravelPoint>,
    /// Whether a piece is selected.
    ///
    /// If `Some(sel_i)`, then `self.turns[curr_turn].inner[sel_i]` is the
    /// selected piece. Additionally, `travelpoints_buffer` is the travel
    /// points that that piece has access to.
    selected_piece: Option<usize>,
    /// (idx of travelpoint within buffer, angle offset of drag, whether we have dragged yet)
    selected_travelpoint: Option<(usize, f32, bool)>,

    turns: Turns,
    // Uhhhh. theses should probably be abstracted in yet another struct for turn management, skull.
    // don't feel like doing it rn.
}

/// Misc.
impl RotchessEmulator {
    /// Create an emulator with an empty board.
    pub fn new() -> Self {
        todo!()
    }

    /// Create an enmulator with pieces.
    pub fn with(pieces: Pieces) -> Self {
        Self {
            travelpoints_buffer: vec![],
            selected_piece: None,
            selected_travelpoint: None,
            turns: Turns::with(pieces),
        }
    }
}

/// Angle between from and to, given a pivot.
fn calc_angle_offset(pivot: (f32, f32), from: (f32, f32), to: (f32, f32)) -> f32 {
    let from = (from.0 - pivot.0, from.1 - pivot.1);
    let to = (to.0 - pivot.0, to.1 - pivot.1);

    let from_angle = f32::atan2(from.1, from.0);
    let to_angle = f32::atan2(to.1, to.0);

    to_angle - from_angle
}

/// Helpful functions for the update portion of a game loop implementing rotchess.
///
/// TODO Future plans: add another impl block for a headless updater? ie, "can i move this piece here,"
/// "where can i move this piece". useful for ml?
impl RotchessEmulator {
    /// Log the current selected piece's travelpoints in the internal buffer.
    ///
    /// Such a piece index must exist.
    /// Will initialize the piece's internal auxiliary data if required.
    /// Will update internal auxiliary data always.
    pub fn update_travelpoints_unchecked(&mut self) {
        let piece =
            &mut self.turns.working_board_mut().inner[self.selected_piece.expect("Invariant")];
        if piece.needs_init() {
            piece.init_auxiliary_data();
        } else {
            piece.update_capture_points_unchecked();
            piece.update_move_points_unchecked();
        }

        let piece = &self.turns.working_board_ref().inner[self.selected_piece.expect("Invariant")];
        self.travelpoints_buffer.clear();
        for &(x, y) in piece.move_points_unchecked() {
            self.travelpoints_buffer.push(TravelPoint {
                x,
                y,
                travelable: self.turns.working_board_ref().travelable(
                    piece,
                    x,
                    y,
                    TravelKind::Move,
                ),
                kind: TravelKind::Move,
            });
        }
        for &(x, y) in piece.capture_points_unchecked() {
            self.travelpoints_buffer.push(TravelPoint {
                x,
                y,
                travelable: self.turns.working_board_ref().travelable(
                    piece,
                    x,
                    y,
                    TravelKind::Capture,
                ),
                kind: TravelKind::Capture,
            });
        }
    }
    /// Handle an event.
    ///
    /// Priority order (high to low) for clicks:
    ///
    /// 1. rotation dragging
    /// 1. captures
    /// 1. piece selection
    /// 1. moves
    pub fn handle_event(&mut self, e: Event) {
        match e {
            Event::MouseMotion { x, y } => {
                // println!("dragged: {} {}", x, y);
                if let Some((tvp_idx, angle_offset, _)) = self.selected_travelpoint {
                    let piece_idx = self
                        .selected_piece
                        .expect("A piece is sel by invariant of tvp.is_some().");
                    let piece = &mut self.turns.working_board_mut().inner[piece_idx];
                    let piece_center = piece.center();

                    // mouse_angle is the angle with piece as pivot, with 0rad being up. because for
                    // some godforsaken reason I made the 0 angle up.
                    let mouse_angle = -calc_angle_offset(
                        piece_center,
                        (piece_center.0, piece_center.1 - 10.), // and also, up is the negative y axis because macroquad.
                        (x, y),
                    );
                    piece.set_angle(mouse_angle + angle_offset);
                    self.update_travelpoints_unchecked();

                    self.selected_travelpoint = Some((tvp_idx, angle_offset, true));
                }
            }
            Event::ButtonDown {
                x,
                y,
                button: MouseButton::RIGHT,
            } => {
                debug_assert!(
                    self.selected_travelpoint.is_none(),
                    "Should not be possible to buttondown without having the
                    travel point be deselected already."
                );

                let idx_of_piece_at_xy = self.turns.working_board_ref().get(x, y);

                // handle piece selection
                match (idx_of_piece_at_xy, self.selected_piece) {
                    (Some(new_i), Some(curr_sel_i)) => {
                        // we clicked on a piece, and a piece is already selected.
                        if new_i == curr_sel_i {
                            // we clicked on the already-selected piece, deselect it.
                            self.selected_piece = None;
                        } else {
                            // we clicked on a different piece, select that instead.
                            self.selected_piece = Some(new_i);
                            self.update_travelpoints_unchecked();
                        }
                        return;
                    }
                    (Some(new_i), None) => {
                        // we clicked on a piece, and None pieces were selected.
                        self.selected_piece = Some(new_i);
                        self.update_travelpoints_unchecked();
                        return;
                    }
                    (None, _) => {
                        self.selected_piece = None;
                    }
                }
            }
            Event::ButtonDown {
                x,
                y,
                button: MouseButton::LEFT,
            } => {
                debug_assert!(
                    self.selected_travelpoint.is_none(),
                    "Should not be possible to buttondown without having the
                    travel point be deselected already."
                );

                let idx_of_piece_at_xy = self.turns.working_board_ref().get(x, y);
                // println!("{}", idx_of_piece_at_xy.is_some());

                // handle clicking a travelpoint
                //
                // if we click a travelpoint, store in emulator data that we've sel'd a tvp
                // with such an angle offset from our mousepos to the tvp center
                let pieces = &mut self.turns.working_board_ref();
                if let Some(sel_idx) = self.selected_piece {
                    for (tvp_idx, tp) in self.travelpoints_buffer.iter().enumerate() {
                        if Piece::collidepoint_generic(x, y, tp.x, tp.y) {
                            self.selected_travelpoint = Some((
                                tvp_idx,
                                calc_angle_offset(
                                    pieces.inner[sel_idx].center(),
                                    (pieces.inner[sel_idx].x(), pieces.inner[sel_idx].y() - 10.),
                                    (x, y),
                                ) + pieces.inner[sel_idx].angle(),
                                false,
                            ));
                            if tp.travelable {
                                return;
                            }
                        }
                    }
                    if self.selected_travelpoint.is_some() {
                        return;
                    }
                }

                // handle piece selection
                match (idx_of_piece_at_xy, self.selected_piece) {
                    (Some(new_i), Some(curr_sel_i)) => {
                        // we clicked on a piece, and a piece is already selected.
                        if new_i == curr_sel_i {
                            // we clicked on the already-selected piece, deselect it.
                            self.selected_piece = None;
                        } else {
                            // we clicked on a different piece, select that instead.
                            self.selected_piece = Some(new_i);
                            self.update_travelpoints_unchecked();
                        }
                        return;
                    }
                    (Some(new_i), None) => {
                        // we clicked on a piece, and None pieces were selected.
                        self.selected_piece = Some(new_i);
                        self.update_travelpoints_unchecked();
                        return;
                    }
                    (None, _) => {
                        if self.selected_travelpoint.is_none() {
                            self.selected_piece = None;
                        }
                    }
                }
            }
            Event::ButtonUp {
                x,
                y,
                button: MouseButton::LEFT,
            } => {
                // println!("up: {} {}", x, y);

                if let Some((trav_idx, _, false)) = self.selected_travelpoint {
                    // if we selected a travelpoint and it hasn't been moved yet, we want to try
                    // to initiate the travel.
                    let tp = &self.travelpoints_buffer[trav_idx];
                    debug_assert!(Piece::collidepoint_generic(x, y, tp.x, tp.y));

                    if tp.travelable {
                        // if it is indeed travelable, travel.
                        let pieces = &mut self.turns.working_board_mut();
                        pieces.travel(
                            self.selected_piece
                                .expect("Invariant of selected_travelpoint.issome"),
                            tp.x,
                            tp.y,
                        );
                        // if tp.kind == TravelKind::Capture {
                        self.selected_piece =
                            pieces.inner.iter().position(|p| p.center() == (tp.x, tp.y));
                        // }
                        debug_assert!(self.selected_piece.is_some());
                        self.update_travelpoints_unchecked();
                        self.selected_piece = None;
                        self.selected_travelpoint = None;
                        self.turns.save_turn();
                        return;
                    }
                    self.selected_travelpoint = None;
                }

                if let Some((_, _, true)) = self.selected_travelpoint {
                    self.selected_travelpoint = None;
                    self.turns.save_turn();
                }
            }
            Event::FirstTurn => {
                self.turns.first();
                self.selected_piece = None;
                self.selected_travelpoint = None;
            }
            Event::PrevTurn => {
                _ = self.turns.prev();
                self.selected_piece = None;
                self.selected_travelpoint = None;
            }
            Event::NextTurn => {
                _ = self.turns.next();
                self.selected_piece = None;
                self.selected_travelpoint = None;
            }
            Event::LastTurn => {
                self.turns.last();
                self.selected_piece = None;
                self.selected_travelpoint = None;
            }
            _ => {}
        }
    }
}

/// Helpful functions for the draw portion of a game loop implementing rotchess.
impl RotchessEmulator {
    pub fn pieces(&self) -> &[Piece] {
        self.turns.working_board_ref().inner_ref()
    }

    /// Whether there is a selected piece.
    ///
    /// If Some, it contains the piece and its possible travelpoints.
    pub fn selected(&self) -> Option<(&Piece, &[TravelPoint])> {
        self.selected_piece.map(|sel_i| {
            (
                &self.turns.working_board_ref().inner[sel_i],
                self.travelpoints_buffer.as_slice(),
            )
        })
    }
}
