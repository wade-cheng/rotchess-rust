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
pub enum MouseButton {
    LEFT,
    RIGHT,
}

/// User events a chess board can respond to.
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
    /// If `Some(sel_i, tp_i)`, then `turns[curr_turn].inner[sel_i]` is the selected piece.
    /// Additionally, `travelpoints_buffer[tp_i]` is the travel points that that piece
    /// has access to.
    selected: Option<(usize, usize)>,
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
    pub fn handle_event(e: Event) {
        match e {
            Event::Drag { x, y, button } => {
                todo!()
            }
            Event::ButtonDown { x, y, button } => {
                todo!()
            }
            Event::ButtonUp { x, y, button } => {
                todo!()
            }
        }
    }
}

/// Helpful functions for the draw portion of a game loop implementing rotchess.
impl RotchessEmulator {
    pub fn pieces(&self) -> &[&Piece] {
        todo!()
    }
}
