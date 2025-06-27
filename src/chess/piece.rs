use std::f32::consts::PI;

use crate::chess::emulator::Event;

/// An iterable over the distances of a [`DistancesAngle`].
///
/// There was an intentional choice to make this iterable just the distances,
/// and not repeatly return the angle. We are guaranteed to maintain the same
/// angle as the `DistancesAngle` this iterable was generated from, so just get
/// the angle from there.
struct IterableDA {
    curr: f32,
    step: f32,
    inclusive_upper_bound: f32,
}

impl Iterator for IterableDA {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr > self.inclusive_upper_bound {
            return None;
        }
        let ans = self.curr;
        self.curr += self.step;
        Some(ans)
    }
}

/// An iterator of (x, y) distances and an angle with which to advance them.
///
/// ```
/// use rotchess_mq::chess::piece::DistancesAngle;
/// let (start, step, n, angle) = (0., f32::sqrt(2.), 4, 45.0_f32.to_radians());
/// let da = DistancesAngle::repeated(start, step, n, angle);
/// let offset_angle = 0.;
///
/// assert_eq![
///     vec![(0., 0.), (1., 1.), (2., 2.), (3., 3.)],
///     da.get_offsets(offset_angle).collect::<Vec<(f32, f32)>>()
/// ];
/// ```
#[derive(Debug, Clone, Copy)]
struct DistancesAngle {
    start: f32,
    step: f32,
    inclusive_upper_bound: f32,
    /// Angle of travel in radians.
    ///
    /// Direction is fnined in the standard math sense, with 0 at the positive
    /// x-axis, increasing clockwise.
    angle: f32,
}

impl IntoIterator for DistancesAngle {
    type Item = f32;

    type IntoIter = IterableDA;

    fn into_iter(self) -> Self::IntoIter {
        IterableDA {
            curr: self.start,
            step: self.step,
            inclusive_upper_bound: self.inclusive_upper_bound,
        }
    }
}

impl DistancesAngle {
    pub const fn singleton(distance: f32, angle: f32) -> Self {
        Self {
            start: distance,
            step: 1.,
            inclusive_upper_bound: distance,
            angle,
        }
    }

    /// .
    ///
    /// ```
    /// use rotchess_mq::chess::piece::DistancesAngle;
    ///
    /// let n = 5;
    /// let da = DistancesAngle::repeated(1.,  2., n, 5.);
    ///
    /// assert_eq![n as usize, da.into_iter().collect::<Vec<f32>>().len()];
    /// ```
    pub const fn repeated(start: f32, step: f32, n: i32, angle: f32) -> Self {
        Self {
            start,
            step,
            inclusive_upper_bound: start + step * (n - 1) as f32,
            angle,
        }
    }

    pub const fn range(start: f32, step: f32, inclusive_upper_bound: f32, angle: f32) -> Self {
        Self {
            start,
            step,
            inclusive_upper_bound,
            angle,
        }
    }
}

impl DistancesAngle {
    /// Get the point offsets when a piece rotated by angle applies this DistancesAngle.
    pub fn get_offsets(&self, angle: f32) -> impl Iterator<Item = (f32, f32)> {
        self.clone()
            .into_iter()
            .map(move |d| self.get_point(d, self.angle, angle))
    }

    /// .
    ///
    /// Angle in radians.
    fn get_point(&self, distance: f32, base_angle: f32, offset_angle: f32) -> (f32, f32) {
        let angle = base_angle - offset_angle;
        crate::chess::floating_drift::floating_drift_adjust!(
            distance * f32::cos(angle),
            distance * f32::sin(angle),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Black,
    White,
}

impl Side {
    pub fn to_file_desc(&self) -> &str {
        match self {
            Side::Black => "B",
            Side::White => "W",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl PieceKind {
    pub fn to_file_desc(&self) -> &str {
        match self {
            PieceKind::Pawn => "pawn",
            PieceKind::Rook => "rook",
            PieceKind::Knight => "knight",
            PieceKind::Bishop => "bishop",
            PieceKind::Queen => "queen",
            PieceKind::King => "king",
        }
    }
}

// pub fn __init_movement(&self):
//     """initializes DAs"""
//     if self.__piece_name == "pawn":
//         self.__move_DAs.append(
//             DistsAngle(
//                 [50, 100],
//                 angle=math.pi / -2,
//             )
//         )
//         self.__capture_DAs.append(
//             DistsAngle(
//                 [50 * math.sqrt(2)],
//                 angle=3 * math.pi / -4,
//             )
//         )
//         self.__capture_DAs.append(
//             DistsAngle(
//                 [50 * math.sqrt(2)],
//                 angle=math.pi / -4,
//             )
//         )
//     elif self.__piece_name == "rook":
//         self.include_level_DAs()
//     elif self.__piece_name == "knight":
//         for rad in [
//             0.4636476090008061,
//             -0.4636476090008061,
//             -1.1071487177940904,
//             -2.0344439357957027,
//             -2.677945044588987,
//             2.677945044588987,
//             2.0344439357957027,
//             1.1071487177940904,
//         ]:
//             self.__capture_DAs.append(
//                 DistsAngle(
//                     [math.sqrt(50**2 + 100**2)],
//                     angle=rad,
//                 )
//             )
//         self.__move_DAs = self.__capture_DAs
//     elif self.__piece_name == "bishop":
//         self.include_diagonal_DAs()
//     elif self.__piece_name == "queen":
//         self.include_level_DAs()
//         self.include_diagonal_DAs()
//     elif self.__piece_name == "king":
//         for rad in [math.pi / -2, 0, math.pi / 2, math.pi, 3 * math.pi / 2]:
//             self.__capture_DAs.append(
//                 DistsAngle(
//                     [50],
//                     angle=rad,
//                 )
//             )
//         for rad in [math.pi / -2, 0, math.pi / 2, math.pi, 3 * math.pi / 2]:
//             rad = rad + math.pi / 4
//             self.__capture_DAs.append(
//                 DistsAngle(
//                     [math.sqrt(50**2 + 50**2)],
//                     angle=rad,
//                 )
//             )
//         self.__move_DAs = self.__capture_DAs
//     else:
//         raise Exception(
//             f"no distances angle mapping found for piece name: {self.__piece_name}"
//         )

// pub fn include_diagonal_DAs(&self):
//     """appends the base moveset for a bishop to self's DistsAngles"""
//     self.__capture_DAs.append(
//         DistsAngle(
//             itertools.count(start=50 * math.sqrt(2), step=50 * math.sqrt(2)),
//             angle=math.pi / 4,
//         )
//     )
//     self.__capture_DAs.append(
//         DistsAngle(
//             itertools.count(start=-50 * math.sqrt(2), step=-50 * math.sqrt(2)),
//             angle=math.pi / 4,
//         )
//     )
//     self.__capture_DAs.append(
//         DistsAngle(
//             itertools.count(start=50 * math.sqrt(2), step=50 * math.sqrt(2)),
//             angle=math.pi / -4,
//         )
//     )
//     self.__capture_DAs.append(
//         DistsAngle(
//             itertools.count(start=-50 * math.sqrt(2), step=-50 * math.sqrt(2)),
//             angle=math.pi / -4,
//         )
//     )
//     self.__move_DAs = self.__capture_DAs

// pub fn include_level_DAs(&self):
//     """appends the base moveset for a rook to self's DistsAngles"""
//     self.__capture_DAs.append(
//         DistsAngle(itertools.count(start=50, step=50), angle=0)
//     )
//     self.__capture_DAs.append(
//         DistsAngle(itertools.count(start=-50, step=-50), angle=0)
//     )
//     self.__capture_DAs.append(
//         DistsAngle(itertools.count(start=50, step=50), angle=math.pi / 2)
//     )
//     self.__capture_DAs.append(
//         DistsAngle(itertools.count(start=-50, step=-50), angle=math.pi / 2)
//     )
//     self.__move_DAs = self.__capture_DAs

impl PieceKind {
    pub fn can_jump(&self) -> bool {
        match self {
            PieceKind::Pawn => true,
            PieceKind::King => true,
            PieceKind::Knight => true,
            PieceKind::Rook => false,
            PieceKind::Bishop => false,
            PieceKind::Queen => false,
        }
    }

    pub fn get_capture_das(&self) -> Vec<DistancesAngle> {
        todo!()
    }

    pub fn get_move_das(&self) -> Vec<DistancesAngle> {
        todo!()
    }

    pub fn can_promote(&self) -> bool {
        if *self == PieceKind::Pawn {
            true
        } else {
            false
        }
    }
}

/// Radius of a piece in rotchess-units.
///
/// 17/50 is parity from rotchess-python, where a tile was 50 pixels wide and
/// a piece had a radius of 17 pixels.
pub const PIECE_RADIUS: f32 = 17.0 / 50.0;

/// The data about a piece that matters.
///
/// Everything else (i.e. delayable piece data) can be derived from this.
/// That is, this is the minimum of what needs to be serde'd for a game save
/// to understand what defines a piece.
#[derive(Debug)]
struct CorePieceData {
    center: (f32, f32),
    angle: f32,
    side: Side,
    kind: PieceKind,
}

/// Delayable piece data.
///
/// Some piece data will only be created (un-None'd) when the piece is first moved.
/// this speeds up loading a game save LOTS.
/// so, we require as an invariant that self.init() is called sometime before any
/// forbidden methods are called. this should be enforced with assertions.
/// this can (and probably is) done when a piece is clicked in normal game code,
/// but for test code we need to hack it in somewhere else that's intuitive.
#[derive(Debug)]
struct SecondaryPieceData {
    /// set by init_movement
    capture_das: Vec<DistancesAngle>,
    /// set by init_movement
    move_das: Vec<DistancesAngle>,
}

impl From<&CorePieceData> for SecondaryPieceData {
    fn from(value: &CorePieceData) -> Self {
        todo!()
    }
}

struct TertiaryPieceData {
    /// set by init_capture_points
    capture_points: Vec<(f32, f32)>,
    /// set by init_move_points
    move_points: Vec<(f32, f32)>,
}

/// Piece data useful for drawing a game.
///
/// This is any data that is not technically necessary for rotchess, but is
/// helpful for a rotchess drawer to know.
struct GamePieceData {
    selected: bool,
}

impl From<(&CorePieceData, &SecondaryPieceData)> for TertiaryPieceData {
    fn from((core, secondary): (&CorePieceData, &SecondaryPieceData)) -> Self {
        todo!()
    }
}

pub struct Piece {
    core: CorePieceData,
    secondary: Option<SecondaryPieceData>,
    tertiary: Option<TertiaryPieceData>,
    game: GamePieceData,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Piece(x={}, y={}, side={:?}), kind={:?}",
            self.core.center.0, self.core.center.1, self.core.side, self.core.kind
        )
    }
}

/// Basic piece stuff.
impl Piece {
    pub fn new(center: (f32, f32), angle: f32, side: Side, kind: PieceKind) -> Self {
        Self {
            core: CorePieceData {
                center,
                angle,
                side,
                kind,
            },
            secondary: None,
            tertiary: None,
            game: GamePieceData { selected: false },
        }
    }

    /// From tile indices. i.e. tile (0,0) is center (0.5, 0.5).
    pub fn from_tile(tile: (u8, u8), angle: f32, side: Side, kind: PieceKind) -> Self {
        let (x, y) = tile;
        Self {
            core: CorePieceData {
                center: (x as f32 + 0.5, y as f32 + 0.5),
                angle,
                side,
                kind,
            },
            secondary: None,
            tertiary: None,
            game: GamePieceData { selected: false },
        }
    }

    pub fn collidepoint(&self, x: f32, y: f32) -> bool {
        (x - self.core.center.0).powf(2.) + (y - self.core.center.1).powf(2.)
            < PIECE_RADIUS.powf(2.)
    }

    /// Whether a piece with center (x, y) collides with self.
    pub fn collidepiece(&self, x: f32, y: f32) -> bool {
        ((x - self.core.center.0).powf(2.) + (y - self.core.center.1).powf(2.))
            < (PIECE_RADIUS * 2.).powf(2.)
    }

    pub fn center(&self) -> (f32, f32) {
        self.core.center
    }

    pub fn x(&self) -> f32 {
        return self.core.center.0;
    }

    pub fn y(&self) -> f32 {
        return self.core.center.1;
    }

    pub fn angle(&self) -> f32 {
        return self.core.angle;
    }

    pub fn rotate_to(&mut self, angle: f32) {
        self.core.angle = angle;
    }

    pub fn side(&self) -> Side {
        self.core.side
    }

    pub fn kind(&self) -> PieceKind {
        return self.core.kind;
    }
}

/// Nontrivial piece stuff.
impl Piece {
    /// Whether a piece with center (x, y) is on the board.
    ///
    /// TODO: this probably should be in a board struct. we might want to move Pieces and Board into the same struct?
    pub fn on_board(x: f32, y: f32) -> bool {
        const BOARD_SIZE: f32 = 8.;
        const MARGIN: f32 = PIECE_RADIUS;
        !(x < 0. - MARGIN || x > BOARD_SIZE + MARGIN || y < 0. - MARGIN || y > BOARD_SIZE + MARGIN)
    }

    /// Whether a piece with given characteristics should promote.
    ///
    /// TODO: like on_board, this should probably be in a board struct.
    pub fn should_promote(kind: PieceKind, side: Side, y: f32) -> bool {
        if !kind.can_promote() {
            return false;
        }

        match side {
            Side::Black => y + PIECE_RADIUS > (7. / 8.),
            Side::White => y - PIECE_RADIUS < (1. / 8.),
        }
    }

    /// strictly just moves self to x,y and updates self invariants.
    /// doesn't even check for promotion---should be done in Pieces.move().
    fn move_to(&mut self, x: f32, y: f32) {
        println!(
            "moving {:?} xy {}, {} to xy {}, {}",
            self.core.kind,
            self.x(),
            self.y(),
            x,
            y
        );

        self.core.center = (x, y);

        if self.secondary.is_some() && self.tertiary.is_some() {
            self.update_capture_points_unchecked();
            self.update_move_points_unchecked();
        }
    }

    pub fn movable_points_unchecked(&self) -> impl Iterator<Item = &(f32, f32)> {
        let tertiary = self
            .tertiary
            .as_ref()
            .expect("Invariant was that delayed is Some.");

        tertiary
            .capture_points
            .iter()
            .chain(tertiary.move_points.iter())
    }

    pub fn init_auxiliary_data(&mut self) {
        self.secondary = Some(SecondaryPieceData::from(&self.core));
        self.tertiary = Some(TertiaryPieceData::from((
            &self.core,
            self.secondary.as_ref().expect("We just created this."),
        )));
    }

    /// Update self's capture points with the drawable DistancesAngles.
    fn update_capture_points_unchecked(&mut self) {
        let capture_points: &mut Vec<(f32, f32)> =
            &mut self.tertiary.as_mut().expect("Invariant.").capture_points;
        let capture_das: &Vec<DistancesAngle> =
            &self.secondary.as_ref().expect("Invariant.").capture_das;

        capture_points.clear();
        Piece::extend_with_drawable_points(&self.core, capture_points, capture_das.iter());
    }

    /// Update self's move points with the drawable DistancesAngles.
    fn update_move_points_unchecked(&mut self) {
        let move_points: &mut Vec<(f32, f32)> =
            &mut self.tertiary.as_mut().expect("Invariant.").move_points;
        let move_das: &Vec<DistancesAngle> = &self.secondary.as_ref().expect("Invariant.").move_das;

        move_points.clear();
        Piece::extend_with_drawable_points(&self.core, move_points, move_das.iter());
    }

    /// Extend points with the drawable points from each DA in das.
    ///
    /// Necessary metadata like offset angle and piece center is retrieved from self.
    fn extend_with_drawable_points<'a>(
        core: &CorePieceData,
        points: &mut Vec<(f32, f32)>,
        das: impl Iterator<Item = &'a DistancesAngle>,
    ) {
        for da in das {
            for (x, y) in da.get_offsets(core.angle) {
                let point = (x + core.center.0, y + core.center.1);
                if !Piece::on_board(point.0, point.1) {
                    break;
                }
                points.push(point);
            }
        }
    }
}

pub struct Pieces {
    pub inner: Vec<Piece>,
}

impl Pieces {
    /// Get a piece's index within inner, if it exists.
    ///
    /// A maximum of one must exist.
    pub fn get<'a>(&'a self, x: f32, y: f32) -> Option<usize> {
        self.inner.iter().position(|piece| piece.collidepoint(x, y))
    }

    pub fn pieces(&self) -> &[Piece] {
        &self.inner
    }

    pub fn standard_board() -> Self {
        let mut inner = vec![];

        const ORDER: [PieceKind; 8] = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];

        for i in 0..8 {
            inner.push(Piece::from_tile(
                (i, 1),
                -PI / 2.,
                Side::Black,
                PieceKind::Pawn,
            ));
            inner.push(Piece::from_tile(
                (i, 6),
                PI / 2.,
                Side::White,
                PieceKind::Pawn,
            ));
        }

        for (i, kind) in ORDER.iter().enumerate() {
            inner.push(Piece::from_tile((i as u8, 0), -PI / 2., Side::Black, *kind));
            inner.push(Piece::from_tile((i as u8, 7), PI / 2., Side::White, *kind));
        }

        Self { inner }
    }
}
