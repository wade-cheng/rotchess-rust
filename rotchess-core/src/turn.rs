use crate::{
    emulator::TravelKind,
    piece::{Pieces, Side},
};

pub struct Turns {
    working_board: Pieces,
    curr_turn: usize,
    turns: Vec<Pieces>,
    /// Whose turn it is.
    ///
    /// Update this manually, which is odd. Recall we have the playground style
    /// of board where turn order may not matter.
    to_move: Side,
}

/// Generic turn methods.
impl Turns {
    pub fn with(pieces: Pieces) -> Self {
        Self {
            working_board: pieces.clone(),
            curr_turn: 0,
            turns: vec![pieces],
            to_move: Side::White,
        }
    }

    pub fn set_to_move(&mut self, side: Side) {
        self.to_move = side;
    }

    pub fn curr_turn(&self) -> usize {
        self.curr_turn
    }

    pub fn inner_ref(&self) -> &Vec<Pieces> {
        &self.turns
    }

    pub fn working_board_ref(&self) -> &Pieces {
        &self.working_board
    }

    pub fn working_board_mut(&mut self) -> &mut Pieces {
        &mut self.working_board
    }

    /// Saves the working board as a turn.
    ///
    /// Follows standard saving procedure used throughout applications. When the turn is:
    ///
    /// - not the most recent turn: truncates the turns s.t. the current turn is the most recent, continue below
    /// - the most recent turn: pushes a clone of the working board to the turns, increments curr_turn
    pub fn save_turn(&mut self) {
        if self.turns.get(self.curr_turn + 1).is_some() {
            self.turns
                .resize_with(self.curr_turn + 1, || unreachable!("see if guard"));
        }

        self.turns.push(self.working_board.clone());
        self.curr_turn += 1;
    }

    pub fn first(&mut self) {
        self.load_turn(0);
    }

    pub fn last(&mut self) {
        self.load_turn(self.turns.len() - 1);
    }

    pub fn prev(&mut self) -> Result<(), ()> {
        if self.curr_turn == 0 {
            Err(())
        } else {
            self.curr_turn -= 1;
            self.load_turn(self.curr_turn);
            Ok(())
        }
    }

    pub fn next(&mut self) -> Result<(), ()> {
        if self.curr_turn + 1 >= self.turns.len() {
            Err(())
        } else {
            self.curr_turn += 1;
            self.load_turn(self.curr_turn);
            Ok(())
        }
    }

    fn load_turn(&mut self, turn: usize) {
        self.working_board.clone_from(&self.turns[turn]);
        self.curr_turn = turn;
    }
}

/// A move as represented by our engine.
struct EngineMove {
    travel: (usize, f32, f32),
    rotate: (usize, f32),
}

/// Score for how good a position is.
pub type Score = f32;
/// depth of negamax search in plies
const DEPTH: usize = 3;

/// Engine code.
impl Turns {
    /// Returns the score, statically evaluated at the current position.
    ///
    /// A float with more positive favoring the current player from `self.to_move`, 0 even.
    fn eval(&self) -> Score {
        let mult = match self.to_move {
            Side::Black => -1.,
            Side::White => 1.,
        };
        let mut ans = 0.0;
        for piece in self.working_board.inner_ref() {
            // add score value of each piece
            ans +=
                mult * match piece.side() {
                    Side::Black => -1.,
                    Side::White => 1.,
                } * piece.kind().value()
                    * 100.;

            // make pieces go toward center.
            /// Center of the board in rotchess units.
            const CENTER_X: f32 = 4.0;
            /// Center of the board in rotchess units.
            const CENTER_Y: f32 = 4.0;
            ans +=
                mult * match piece.side() {
                    Side::Black => -1.,
                    Side::White => 1.,
                } * (5.0
                    - Score::sqrt((piece.x() - CENTER_X).powi(2) + (piece.y() - CENTER_Y).powi(2)));
        }
        ans
    }

    /// Return the score we get in `depth` plies when minimizing our maximum loss.
    ///
    /// - "We" should be `self.to_move`.
    /// - alpha is the highest score we already found. (if we see a score lower than it,
    ///   no need to consider it.)
    /// - beta is the best score we are able to get before the opponent is able to deny it
    ///   with a reply we already found.
    fn negamax_ab(&mut self, depth: usize, mut alpha: Score, beta: Score) -> Score {
        // println!("depth is {depth}");
        if depth == 0 {
            return self.eval();
        }

        let mut best_score = Score::NEG_INFINITY;

        for move_ in self.all_moves() {
            self.apply(&move_);
            let score = -self.negamax_ab(depth - 1, -beta, -alpha);
            self.unapply();

            if score > best_score {
                best_score = score;
                if score > alpha {
                    alpha = score;
                }
            }
            if score >= beta {
                break;
            }
        }

        best_score
    }

    /// Make the best move for the current player from `self.to_move`.
    pub fn make_best_move(&mut self) {
        let mut best_score: Score = Score::NEG_INFINITY;
        let mut best_move: Option<EngineMove> = None;

        self.working_board.init_all_auxiliary_data();
        self.turns[self.curr_turn].init_all_auxiliary_data();

        let moves = self.all_moves();
        assert!(!moves.is_empty());
        for move_ in moves {
            self.apply(&move_);
            let score = -self.negamax_ab(DEPTH, Score::NEG_INFINITY, Score::INFINITY);
            self.unapply();

            if score >= best_score {
                best_score = score;
                best_move = Some(move_);
            }
        }

        self.apply(&best_move.expect("should've found a valid move."));

        println!("best move had score {best_score}");
        println!(
            "current board state has score {} according to {:?}",
            self.eval(),
            self.to_move
        );
    }

    /// Reverses effects of [`apply`][`Turns::apply`].
    fn unapply(&mut self) {
        self.prev().expect("There will be a prev move.");
        self.to_move = self.to_move.toggle();
    }

    /// Applies a move to the current board, saves the turn, and toggles the side to_move.
    ///
    /// Since we save, this will remove future turns that were saved!
    /// Also, the entire turn is saved as one turn, not two, which would
    /// happen if a user were to move.
    fn apply(&mut self, move_: &EngineMove) {
        // println!("tomove is {:?}", self.to_move);
        debug_assert_eq!(
            self.working_board
                .get_mut(move_.travel.0)
                .expect("EngineMove supplied wasn't valid")
                .side(),
            self.to_move
        );
        debug_assert_eq!(
            self.working_board
                .get_mut(move_.rotate.0)
                .expect("EngineMove supplied wasn't valid")
                .side(),
            self.to_move
        );

        let (i, x, y) = move_.travel;

        let i = self.working_board.travel(i, x, y);
        self.working_board
            .get_mut(i)
            .expect("EngineMove supplied wasn't valid")
            // .init_auxiliary_data();
            .update_travel_points_unchecked();

        let (_, r) = move_.rotate;
        self.working_board
            .get_mut(i)
            .expect("EngineMove supplied wasn't valid")
            .set_angle(r);

        self.save_turn();
        self.to_move = self.to_move.toggle();
    }

    /// Return all possible moves that the current player can make.
    ///
    /// Current player defined by `self.to_move`.
    fn all_moves(&mut self) -> Vec<EngineMove> {
        self.working_board.init_all_auxiliary_data();

        let mut ans = vec![];
        for (i, piece) in self.working_board.inner_ref().iter().enumerate() {
            if piece.side() != self.to_move {
                continue;
            }
            for (tvk, x, y) in piece
                .move_points_unchecked()
                .map(|&(x, y)| (TravelKind::Move, x, y))
                .chain(
                    piece
                        .capture_points_unchecked()
                        .map(|&(x, y)| (TravelKind::Capture, x, y)),
                )
            {
                if self.working_board_ref().travelable(piece, x, y, tvk) {
                    ans.push(EngineMove {
                        travel: (i, x, y),
                        rotate: (i, piece.angle()),
                    });
                }
            }
        }
        ans
    }
}
