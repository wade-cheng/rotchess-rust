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

pub enum TravelPoint {
    Capture { x: f32, y: f32, travelable: bool },
    Move { x: f32, y: f32, travelable: bool },
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
    travelpoints_buffer: Vec<TravelPoint>,
    /// Whether a piece is selected.
    ///
    /// If `Some(sel_i)`, then `turns[curr_turn].inner[sel_i]` is the selected piece.
    /// Additionally, `travelpoints_buffer` is the travel points that that piece
    /// has access to.
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
    /// Handle an event.
    ///
    /// Priority order (high to low) for clicks:
    ///
    /// 1. captures
    /// 1. piece selection
    /// 1. moves
    pub fn handle_event(&mut self, e: Event) {
        self.turns[self.curr_turn].handle_event(e);
        match e {
            Event::Drag { x, y, button } => {
                // println!("dragged: {} {}", x, y);
            }
            Event::ButtonDown {
                x,
                y,
                button: MouseButton::LEFT,
            } => {
                let pieces = &self.turns[self.curr_turn];
                let p = pieces.get(x, y);
                if let (Some((p, p_i)), Some(curr_sel_i)) = (p, self.selected) {
                    if p_i == curr_sel_i {
                        // we clicked on the already-selected piece.
                        self.selected = None;
                    } else {
                        self.selected = Some(p_i)
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
