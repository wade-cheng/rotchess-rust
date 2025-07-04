use crate::chess::piece::Pieces;

pub struct Turns {
    working_board: Pieces,
    curr_turn: usize,
    turns: Vec<Pieces>,
}

impl Turns {
    pub fn with(pieces: Pieces) -> Self {
        Self {
            working_board: pieces.clone(),
            curr_turn: 0,
            turns: vec![pieces],
        }
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
