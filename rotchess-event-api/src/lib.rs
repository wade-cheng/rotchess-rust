//! An api wrapper around `rotchess-core` to add an event-based interface for playing a chess game.
//!
//! If you're adding rotchess to a new medium, this api is probably the fastest way
//! to do it, as opposed to hand coding your own wrapper around `rotchess-core`.

use rotchess_core::{
    piece::{Piece, PieceId, Pieces, TravelKind, TravelPoint},
    turn::Turns,
};

pub use rotchess_core::piece;

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
    ButtonDown {
        x: f32,
        y: f32,
        button: MouseButton,
    },
    ButtonUp {
        x: f32,
        y: f32,
        button: MouseButton,
    },
    MouseMotion {
        x: f32,
        y: f32,
    },
    FirstTurn,
    PrevTurn,
    NextTurn,
    LastTurn,
    /// We've been told to rotate the piece to r.
    RotateUnchecked(PieceId, f32),
    /// We've been told to move the piece to x, y.
    MoveUnchecked(PieceId, f32, f32),
}

/// The main entrypoint for any rotchess user.
///
/// design doc:
/// - on hovering over move/cap points, highlight if it's a possible move
/// - only draw guides for non-jumpers (might be drawer's responsibility)
/// - piece selection on mousedown
///   - but if piece was alr selected, or no action could be taken, deselect the only selected piece.
///   - hmm. wondering now, we could probably move only selected to a `Option<usize>` in each Pieces.
///   - but that raises the question, can we move everything GamePieceData related to Board?
/// - drag move/cap point to rotate, or mouseup without having dragged to move (if was possible)
pub struct RotchessEmulator {
    /// A valid representation of travelpoints a user needs to draw iff we
    ///  update this every time a piece.core changes and `self.selected.is_some()`.
    travelpoints_buffer: Vec<TravelPoint>,
    /// Whether a piece is selected.
    ///
    /// If `Some(sel_i)`, then `self.turns[curr_turn].inner[sel_i]` is the
    /// selected piece. Additionally, `travelpoints_buffer` is the travel
    /// points that that piece has access to.
    selected_piece: Option<PieceId>,
    /// (idx of travelpoint within buffer, angle offset of drag, whether we have dragged yet)
    ///
    /// Set when we mbd to hold a travel point, updated when we drag it around.
    selected_travelpoint: Option<(usize, f32, bool)>,

    turns: Turns,
    // Uhhhh. theses should probably be abstracted in yet another struct for turn management, skull.
    // don't feel like doing it rn.
}

/// Misc.
impl RotchessEmulator {
    // /// Create an emulator with an empty board.
    // pub fn new() -> Self {
    //     todo!()
    // }

    /// Create an emulator with pieces.
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

pub enum ThingHappened {
    FirstTurn,
    PrevTurn,
    NextTurn,
    LastTurn,
    /// We rotated the piece at usize to r
    Rotate(PieceId, f32),
    /// We moved the piece at usize to x, y
    Move(PieceId, f32, f32),
}

/// Helpful functions for the update portion of a game loop implementing rotchess.
impl RotchessEmulator {
    /// Log the current selected piece's travelpoints in the internal buffer.
    ///
    /// Such a piece index must exist.
    /// Will initialize the piece's internal auxiliary data if required.
    /// Will update internal auxiliary data always.
    pub fn update_travelpoints_unchecked(&mut self) {
        let piece = &mut self
            .turns
            .working_board_mut()
            .get_mut(self.selected_piece.expect("Invariant"))
            .unwrap();
        if piece.needs_init() {
            piece.init_auxiliary_data();
        } else {
            piece.update_travel_points_unchecked();
        }

        let piece = &self
            .turns
            .working_board_ref()
            .get(self.selected_piece.expect("Invariant"))
            .unwrap();
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

    pub fn make_best_move(&mut self) {
        self.turns.make_best_move();
    }

    /// Handle an event.
    ///
    /// Priority order (high to low) for clicks:
    ///
    /// 1. rotation dragging
    /// 1. captures
    /// 1. piece selection
    /// 1. moves
    pub fn handle_event(&mut self, e: Event) -> Option<ThingHappened> {
        match e {
            Event::MouseMotion { x, y } => {
                // println!("dragged: {} {}", x, y);
                if let Some((tvp_idx, angle_offset, _)) = self.selected_travelpoint {
                    let piece_id = self
                        .selected_piece
                        .expect("A piece is sel by invariant of tvp.is_some().");
                    let piece = &mut self.turns.working_board_mut().get_mut(piece_id).unwrap();
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
                None
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

                let idx_of_piece_at_xy = self.turns.working_board_ref().get_id(x, y);

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
                        return None;
                    }
                    (Some(new_i), None) => {
                        // we clicked on a piece, and None pieces were selected.
                        self.selected_piece = Some(new_i);
                        self.update_travelpoints_unchecked();
                        return None;
                    }
                    (None, _) => {
                        self.selected_piece = None;
                    }
                }
                None
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

                let idx_of_piece_at_xy = self.turns.working_board_ref().get_id(x, y);
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
                                    pieces.get(sel_idx).unwrap().center(),
                                    (
                                        pieces.get(sel_idx).unwrap().x(),
                                        pieces.get(sel_idx).unwrap().y() - 10.,
                                    ),
                                    (x, y),
                                ) + pieces.get(sel_idx).unwrap().angle(),
                                false,
                            ));
                            if tp.travelable {
                                return None;
                            }
                        }
                    }
                    if self.selected_travelpoint.is_some() {
                        return None;
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
                        return None;
                    }
                    (Some(new_i), None) => {
                        // we clicked on a piece, and None pieces were selected.
                        self.selected_piece = Some(new_i);
                        self.update_travelpoints_unchecked();
                        return None;
                    }
                    (None, _) => {
                        if self.selected_travelpoint.is_none() {
                            self.selected_piece = None;
                        }
                    }
                }
                None
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
                    let (tp_x, tp_y) = (tp.x, tp.y);
                    debug_assert!(Piece::collidepoint_generic(x, y, tp_x, tp_y));

                    if tp.travelable {
                        // if it is indeed travelable, travel.
                        let pieces = &mut self.turns.working_board_mut();
                        let piece_id = self // the idx of the piece that moves
                            .selected_piece
                            .expect("Invariant of selected_travelpoint.issome");
                        let new_piece_idx = pieces.travel(piece_id, tp_x, tp_y);
                        self.selected_piece = Some(new_piece_idx);
                        self.update_travelpoints_unchecked();
                        self.selected_piece = None;
                        self.selected_travelpoint = None;
                        self.turns.save_turn();
                        self.turns
                            .set_to_move(self.pieces()[new_piece_idx].side().toggle());
                        return Some(ThingHappened::Move(piece_id, tp_x, tp_y));
                    }
                    self.selected_travelpoint = None;
                }

                if let Some((_, _, true)) = self.selected_travelpoint {
                    self.selected_travelpoint = None;
                    self.turns.save_turn();

                    let piece_id = self
                        .selected_piece
                        .expect("Invariant of sel travelpt.is_some");

                    self.turns.set_to_move(self.pieces()[piece_id].side());
                    let r = self.pieces()[piece_id].angle();
                    self.turns
                        .set_to_move(self.pieces()[piece_id].side().toggle());
                    return Some(ThingHappened::Rotate(piece_id, r));
                }

                None
            }
            Event::FirstTurn => {
                self.turns.first();
                self.selected_piece = None;
                self.selected_travelpoint = None;
                Some(ThingHappened::FirstTurn)
            }
            Event::PrevTurn => {
                _ = self.turns.prev();
                self.selected_piece = None;
                self.selected_travelpoint = None;
                Some(ThingHappened::PrevTurn)
            }
            Event::NextTurn => {
                _ = self.turns.next();
                self.selected_piece = None;
                self.selected_travelpoint = None;
                Some(ThingHappened::NextTurn)
            }
            Event::LastTurn => {
                self.turns.last();
                self.selected_piece = None;
                self.selected_travelpoint = None;
                Some(ThingHappened::LastTurn)
            }
            Event::RotateUnchecked(piece_id, r) => {
                // to elaborate, it would suck to be playing around and then a piece
                // rotates for "no reason"
                assert!(
                    self.selected_travelpoint.is_none() && self.selected_piece.is_none(),
                    "It would probably be a good idea for this check
                     to never fail, but who knows. Go find the dev if
                     you see this error message."
                );

                self.turns
                    .working_board_mut()
                    .get_mut(piece_id)
                    .expect("Invariant of unchecked")
                    .set_angle(r);
                self.turns.save_turn();
                self.turns
                    .set_to_move(self.pieces()[piece_id].side().toggle());
                None
            }
            Event::MoveUnchecked(piece_id, x, y) => {
                // to elaborate, it would suck to be playing around and then a piece
                // move for "no reason"
                assert!(
                    self.selected_travelpoint.is_none() && self.selected_piece.is_none(),
                    "It would probably be a good idea for this check
                     to never fail, but who knows. Go find the dev if
                     you see this error message."
                );

                let pieces = &mut self.turns.working_board_mut();
                let new_piece_idx = pieces.travel(piece_id, x, y);
                self.selected_piece = Some(new_piece_idx);
                self.update_travelpoints_unchecked();
                self.selected_piece = None;
                self.turns.save_turn();
                self.turns
                    .set_to_move(self.pieces()[piece_id].side().toggle());
                None
            }
            _ => None,
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
                self.turns
                    .working_board_ref()
                    .get(sel_i)
                    .expect("exists because selected_piece.is_some()."),
                self.travelpoints_buffer.as_slice(),
            )
        })
    }
}
