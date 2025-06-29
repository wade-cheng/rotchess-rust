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

use crate::chess::piece::{Piece, Pieces};

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
    Drag { x: f32, y: f32, button: MouseButton },
}

#[derive(PartialEq)]
pub enum TravelKind {
    Capture,
    Move,
}

pub struct TravelPoint {
    pub x: f32,
    pub y: f32,
    pub travelable: bool,
    pub kind: TravelKind,
}

pub struct Guide {
    a: (f32, f32),
    b: (f32, f32),
}

pub enum AuxiliaryDrawable {
    TravelPoint(TravelPoint),
    Guides(Vec<Guide>),
}

pub struct RotchessEmulator {
    curr_turn: usize,
    /// A valid representation of travelpoints a user needs to draw iff we
    ///  update this every time a piece.core changes and `self.selected.is_some()`.
    travelpoints_buffer: Vec<TravelPoint>,
    /// Whether a piece is selected.
    ///
    /// If `Some(sel_i)`, then `self.turns[curr_turn].inner[sel_i]` is the
    /// selected piece. Additionally, `travelpoints_buffer` is the travel
    /// points that that piece has access to.
    selected: Option<usize>,
    turns: Vec<Pieces>,
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
            curr_turn: 0,
            travelpoints_buffer: vec![],
            selected: None,
            turns: vec![pieces],
        }
    }
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
        let pieces = &mut self.turns[self.curr_turn];
        let piece = &mut pieces.inner[self.selected.expect("Invariant")];
        if piece.needs_init() {
            piece.init_auxiliary_data();
        } else {
            piece.update_capture_points_unchecked();
            piece.update_move_points_unchecked();
        }

        let pieces = &self.turns[self.curr_turn];
        let piece = &pieces.inner[self.selected.expect("Invariant")];
        self.travelpoints_buffer.clear();
        for &(x, y) in piece.capture_points_unchecked() {
            self.travelpoints_buffer.push(TravelPoint {
                x,
                y,
                travelable: pieces.travelable(piece, x, y, TravelKind::Capture),
                kind: TravelKind::Capture,
            });
        }
        for &(x, y) in piece.move_points_unchecked() {
            self.travelpoints_buffer.push(TravelPoint {
                x,
                y,
                travelable: pieces.travelable(piece, x, y, TravelKind::Move),
                kind: TravelKind::Move,
            });
        }
    }
    /// Handle an event.
    ///
    /// Priority order (high to low) for clicks:
    ///
    /// 1. captures
    /// 1. piece selection
    /// 1. moves
    pub fn handle_event(&mut self, e: Event) {
        match e {
            Event::Drag { x, y, button } => {
                // println!("dragged: {} {}", x, y);
            }
            Event::ButtonDown { x, y, button } => {
                let pieces = &self.turns[self.curr_turn];
                let idx_of_piece_at_xy = pieces.get(x, y);
                // println!("{}", idx_of_piece_at_xy.is_some());

                // handle captures
                let pieces = &mut self.turns[self.curr_turn];
                if let Some(idx) = self.selected {
                    for tp in &self.travelpoints_buffer {
                        if tp.kind == TravelKind::Capture
                            && tp.travelable
                            && Piece::collidepoint_generic(x, y, tp.x, tp.y)
                        {
                            pieces.travel(idx, tp.x, tp.y);
                            self.selected =
                                pieces.inner.iter().position(|p| p.center() == (tp.x, tp.y));
                            debug_assert!(self.selected.is_some());
                            self.update_travelpoints_unchecked();
                            self.selected = None;
                            return;
                        }
                    }
                }

                // handle piece selection
                match (idx_of_piece_at_xy, self.selected) {
                    (Some(new_i), Some(curr_sel_i)) => {
                        // we clicked on a piece, and a piece is already selected.
                        if new_i == curr_sel_i {
                            // we clicked on the already-selected piece, deselect it.
                            self.selected = None;
                        } else {
                            // we clicked on a different piece, select that instead.
                            self.selected = Some(new_i);
                            self.update_travelpoints_unchecked();
                        }
                        return;
                    }
                    (Some(new_i), None) => {
                        // we clicked on a piece, and None pieces were selected.
                        self.selected = Some(new_i);
                        self.update_travelpoints_unchecked();
                        return;
                    }
                    _ => {}
                }

                // handle moves
                let pieces = &mut self.turns[self.curr_turn];
                if let Some(idx) = self.selected {
                    for tp in &self.travelpoints_buffer {
                        if tp.kind == TravelKind::Move
                            && tp.travelable
                            && Piece::collidepoint_generic(x, y, tp.x, tp.y)
                        {
                            pieces.travel(idx, tp.x, tp.y);
                            self.update_travelpoints_unchecked();
                            self.selected = None;
                            return;
                        }
                    }
                }
            }
            Event::ButtonUp { x, y, button } => {
                // println!("up: {} {}", x, y);
            }
            _ => {}
        }
    }
}

/// Helpful functions for the draw portion of a game loop implementing rotchess.
impl RotchessEmulator {
    pub fn pieces(&self) -> &[Piece] {
        self.turns[self.curr_turn].pieces()
    }

    /// Whether there is a selected piece.
    ///
    /// If Some, it contains the piece and its possible travelpoints.
    pub fn selected(&self) -> Option<(&Piece, &[TravelPoint])> {
        self.selected.map(|sel_i| {
            (
                &self.turns[self.curr_turn].inner[sel_i],
                self.travelpoints_buffer.as_slice(),
            )
        })
    }
}
