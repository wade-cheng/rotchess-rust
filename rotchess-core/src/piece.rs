use std::{collections::HashSet, f32::consts::PI, hash::Hash};

use crate::turn::{MAX_CAPTURES, Move, Score, TravelPhase};

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
#[derive(Debug, Clone, Copy)]
struct DistancesAngle {
    start: f32,
    step: f32,
    inclusive_upper_bound: f32,
    /// Used as a piece's angle *offset*.
    ///
    /// An angle of 0 means the piece is facing "forward." This means it has an actual
    /// angle of who knows what.
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
    const fn singleton(distance: f32, angle: f32) -> Self {
        Self {
            start: distance,
            step: 1.,
            inclusive_upper_bound: distance,
            angle,
        }
    }

    const fn repeated(start: f32, step: f32, n: i32, angle: f32) -> Self {
        Self {
            start,
            step,
            inclusive_upper_bound: start + step * (n - 1) as f32,
            angle,
        }
    }

    const fn range(start: f32, step: f32, inclusive_upper_bound: f32, angle: f32) -> Self {
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
    fn get_offsets(&self, angle: f32) -> impl Iterator<Item = (f32, f32)> {
        self.clone()
            .into_iter()
            .map(move |d| self.get_point(d, self.angle, angle))
    }

    /// .
    ///
    /// Angle in radians.
    fn get_point(&self, distance: f32, base_angle: f32, offset_angle: f32) -> (f32, f32) {
        let angle = base_angle - offset_angle;
        crate::floating_drift::floating_drift_adjust!(
            distance * f32::cos(angle),
            distance * f32::sin(angle),
        )
    }
}

#[cfg(test)]
mod da_tests {
    use super::DistancesAngle;

    #[test]
    fn rep() {
        let n = 5;
        let da = DistancesAngle::repeated(1., 2., n, 5.);
        assert_eq!(n as usize, da.into_iter().collect::<Vec<f32>>().len());
    }

    #[test]
    fn rep_45deg() {
        let (start, step, n, angle) = (0., f32::sqrt(2.), 4, 45.0_f32.to_radians());
        let da = DistancesAngle::repeated(start, step, n, angle);
        let offset_angle = 0.;
        assert_eq![
            vec![(0., 0.), (1., 1.), (2., 2.), (3., 3.)],
            da.get_offsets(offset_angle).collect::<Vec<(f32, f32)>>()
        ];
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

    pub fn toggled(&self) -> Self {
        match self {
            Side::Black => Side::White,
            Side::White => Side::Black,
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

    pub fn value(&self) -> Score {
        match self {
            PieceKind::Pawn => 1.0,
            PieceKind::Rook => 5.0,
            PieceKind::Knight => 3.0,
            PieceKind::Bishop => 3.0,
            PieceKind::Queen => 9.0,
            PieceKind::King => 1000.0,
        }
    }

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

    pub fn can_promote(&self) -> bool {
        *self == PieceKind::Pawn
    }

    /// Return the unique rotations of this piece.
    ///
    /// These are the "possible" rotations, removing duplicates. We use a constant precision, so only every so
    /// many real values will be sampled. So, as an example, a queen might have `(0..45).step_by(PRECISION)`
    /// while a pawn has `(0..360).step_by(PRECISION)`.
    pub fn unique_rotations(&self) -> impl Iterator<Item = f32> {
        Self::unique_rotations_generic(self.rotational_symmetry()).map(|val| val as f32)
    }

    fn rotational_symmetry(&self) -> u16 {
        match self {
            PieceKind::Pawn => 360,
            PieceKind::King => 90,
            PieceKind::Knight => 90,
            PieceKind::Rook => 90,
            PieceKind::Bishop => 90,
            PieceKind::Queen => 45,
        }
    }

    /// Return the unique rotations of a piece that has the same distances angles when rotating by the parameter.
    fn unique_rotations_generic(
        rotational_symmetry: u16,
    ) -> std::iter::StepBy<std::ops::Range<u16>> {
        // const PRECISION: usize = 10;
        // const PRECISION: usize = 22;
        const PRECISION: usize = 45;
        // ok turns out this is outright wrong because we do radians, not degrees, whoops.
        (0..rotational_symmetry).step_by(PRECISION)
    }

    /// Add the DAs of a rook to `v`.
    fn add_level_das(v: &mut Vec<DistancesAngle>) {
        for i in 0..4 {
            v.push(DistancesAngle::range(
                1.,
                1.,
                f32::INFINITY,
                i as f32 * PI / 2.,
            ))
        }
    }

    /// From distance formula with 1 and 2.
    const KNIGHT_DISTANCE: f32 = 2.23606797749979;

    const KNIGHT_ANGLES: [f32; 8] = [
        0.4636476090008061,
        -0.4636476090008061,
        -1.1071487177940904,
        -2.0344439357957027,
        -2.677945044588987,
        2.677945044588987,
        2.0344439357957027,
        1.1071487177940904,
    ];

    /// Add the DAs of a bishop to `v`.
    fn add_diag_das(v: &mut Vec<DistancesAngle>) {
        for i in 0..4 {
            v.push(DistancesAngle::range(
                f32::sqrt(2.),
                f32::sqrt(2.),
                f32::INFINITY,
                (i as f32 * PI / 2.) + (PI / 4.),
            ))
        }
    }

    fn capture_das(&self) -> Vec<DistancesAngle> {
        // TODO: these will somehow want to return impl iterator distancesangle...
        let mut ans = vec![];
        match self {
            PieceKind::Pawn => {
                ans.push(DistancesAngle::singleton(f32::sqrt(2.), PI / 4.));
                ans.push(DistancesAngle::singleton(f32::sqrt(2.), -PI / 4.));
            }
            PieceKind::King => {
                for i in 0..8 {
                    ans.push(DistancesAngle::singleton(
                        if i % 2 == 0 { 1. } else { f32::sqrt(2.) },
                        i as f32 * PI / 4.,
                    ))
                }
            }
            PieceKind::Knight => {
                for rad in PieceKind::KNIGHT_ANGLES {
                    ans.push(DistancesAngle::singleton(PieceKind::KNIGHT_DISTANCE, rad));
                }
            }
            PieceKind::Rook => PieceKind::add_level_das(&mut ans),
            PieceKind::Bishop => PieceKind::add_diag_das(&mut ans),
            PieceKind::Queen => {
                PieceKind::add_level_das(&mut ans);
                PieceKind::add_diag_das(&mut ans);
            }
        };
        ans
    }

    fn move_das(&self) -> Vec<DistancesAngle> {
        let mut ans = vec![];
        match self {
            PieceKind::Pawn => ans.push(DistancesAngle::repeated(1., 1., 2, 0.)),
            PieceKind::King => {
                for i in 0..8 {
                    ans.push(DistancesAngle::singleton(
                        if i % 2 == 0 { 1. } else { f32::sqrt(2.) },
                        i as f32 * PI / 4.,
                    ))
                }
            }
            PieceKind::Knight => {
                for rad in PieceKind::KNIGHT_ANGLES {
                    ans.push(DistancesAngle::singleton(PieceKind::KNIGHT_DISTANCE, rad));
                }
            }
            PieceKind::Rook => PieceKind::add_level_das(&mut ans),
            PieceKind::Bishop => PieceKind::add_diag_das(&mut ans),
            PieceKind::Queen => {
                PieceKind::add_level_das(&mut ans);
                PieceKind::add_diag_das(&mut ans);
            }
        };
        ans
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
#[derive(Debug, Clone)]
struct CorePieceData {
    center: (f32, f32),
    angle: f32,
    side: Side,
    kind: PieceKind,
}

impl Hash for CorePieceData {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.center.0.to_be_bytes().hash(state);
        self.center.1.to_be_bytes().hash(state);
    }
}

impl PartialEq for CorePieceData {
    fn eq(&self, other: &Self) -> bool {
        self.center == other.center
            && self.angle == other.angle
            && self.side == other.side
            && self.kind == other.kind
    }
}
impl Eq for CorePieceData {}

#[derive(Clone)]
struct TravelPointData {
    /// set by init_capture_points
    capture_points: Vec<(f32, f32)>,
    /// set by init_move_points
    move_points: Vec<(f32, f32)>,
}

impl From<&CorePieceData> for TravelPointData {
    fn from(core: &CorePieceData) -> Self {
        let mut cap_points = vec![];
        let mut move_points = vec![];
        Piece::extend_with_drawable_points(
            core,
            &mut cap_points,
            core.kind.capture_das().into_iter(),
        );
        Piece::extend_with_drawable_points(
            core,
            &mut move_points,
            core.kind.move_das().into_iter(),
        );

        Self {
            capture_points: cap_points,
            move_points,
        }
    }
}

/// A piece. Like from chess. Like a rook, or such.
///
/// Currently, a piece is represented by core, secondary, and tertiary data.
///
/// - core: the core definitions of a piece, from which the other data can be derived. This would be its position, angle, kind, and such.
/// - secondary: uh hm. looks like we don't need this one actually. we should make this static somehow.
/// - tertiary: yuh. meat of the algo. we are able to cache the points we can move to.
#[derive(Clone)]
pub struct Piece {
    id: PieceId,
    core: CorePieceData,
    tvp_cache: Option<TravelPointData>,
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

impl Hash for Piece {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.core.hash(state);
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        self.core == other.core
    }
}
impl Eq for Piece {}

/// Instantiation.
impl Piece {
    pub fn new(id: PieceId, center: (f32, f32), angle: f32, side: Side, kind: PieceKind) -> Self {
        Self {
            id,
            core: CorePieceData {
                center,
                angle,
                side,
                kind,
            },
            tvp_cache: None,
        }
    }

    /// From tile indices. i.e. tile (0,0) is center (0.5, 0.5).
    pub fn from_tile(id: PieceId, tile: (u8, u8), angle: f32, side: Side, kind: PieceKind) -> Self {
        let (x, y) = tile;
        Self {
            id,
            core: CorePieceData {
                center: (x as f32 + 0.5, y as f32 + 0.5),
                angle,
                side,
                kind,
            },
            tvp_cache: None,
        }
    }
}

/// Trivial getters and setters.
impl Piece {
    pub fn id(&self) -> PieceId {
        self.id
    }

    pub fn center(&self) -> (f32, f32) {
        self.core.center
    }

    pub fn set_center(&mut self, center: (f32, f32)) {
        self.core.center = center;
    }

    pub fn x(&self) -> f32 {
        self.core.center.0
    }

    pub fn set_x(&mut self, x: f32) {
        self.core.center.0 = x;
    }

    pub fn y(&self) -> f32 {
        self.core.center.1
    }

    pub fn set_y(&mut self, y: f32) {
        self.core.center.1 = y;
    }

    pub fn angle(&self) -> f32 {
        self.core.angle
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.core.angle = angle;
    }

    pub fn side(&self) -> Side {
        self.core.side
    }

    pub fn kind(&self) -> PieceKind {
        self.core.kind
    }

    fn set_kind(&mut self, kind: PieceKind) {
        self.core.kind = kind;
    }

    pub fn needs_init(&self) -> bool {
        self.tvp_cache.is_none()
    }
}

/// Nontrivial piece stuff.
impl Piece {
    /// The distance from a piece's side in rotchess units.
    pub fn forward_distance(&self) -> f32 {
        match self.side() {
            Side::Black => self.y(),
            Side::White => 8.0 - self.y(),
        }
    }

    /// If one coord is a piece and the other is a point, do they collide?
    pub fn collidepoint_generic(x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
        (x1 - x2).powi(2) + (y1 - y2).powi(2) < PIECE_RADIUS.powi(2)
    }

    /// Whether the point (x, y) collides with this piece.
    pub fn collidepoint(&self, x: f32, y: f32) -> bool {
        Piece::collidepoint_generic(x, y, self.core.center.0, self.core.center.1)
    }

    /// Whether a piece with center (x, y) collides with this piece.
    pub fn collidepiece(&self, x: f32, y: f32) -> bool {
        ((x - self.core.center.0).powi(2) + (y - self.core.center.1).powi(2))
            < (PIECE_RADIUS * 2.).powi(2)
    }

    /// Whether a piece with center (x, y) is on the board.
    pub fn on_board(x: f32, y: f32) -> bool {
        const BOARD_SIZE: f32 = 8.;
        const MARGIN: f32 = PIECE_RADIUS;
        !(x < 0. - MARGIN || x > BOARD_SIZE + MARGIN || y < 0. - MARGIN || y > BOARD_SIZE + MARGIN)
    }

    /// Whether a piece with given characteristics should promote.
    pub fn should_promote(kind: PieceKind, side: Side, y: f32) -> bool {
        if !kind.can_promote() {
            return false;
        }

        match side {
            Side::Black => y + PIECE_RADIUS > 7.,
            Side::White => y - PIECE_RADIUS < 1.,
        }
    }

    /// Get the points this piece could travel to, not accounting for pathing.
    ///
    /// This function will panic if this piece [`Self::needs_init`]. This function will return
    /// outdated information if the interior data is not updated.
    pub fn travel_points_unchecked(&self) -> impl Iterator<Item = (TravelKind, f32, f32)> {
        self.move_points_unchecked()
            .map(|&(x, y)| (TravelKind::Move, x, y))
            .chain(
                self.capture_points_unchecked()
                    .map(|&(x, y)| (TravelKind::Capture, x, y)),
            )
    }

    fn capture_points_unchecked(&self) -> impl Iterator<Item = &(f32, f32)> {
        let tertiary = self
            .tvp_cache
            .as_ref()
            .expect("Invariant was that delayed is Some.");

        tertiary.capture_points.iter()
    }

    fn move_points_unchecked(&self) -> impl Iterator<Item = &(f32, f32)> {
        let tertiary = self
            .tvp_cache
            .as_ref()
            .expect("Invariant was that delayed is Some.");

        tertiary.move_points.iter()
    }

    pub fn init_auxiliary_data(&mut self) {
        self.tvp_cache = Some(TravelPointData::from(&self.core));
    }

    pub fn update_travel_points_unchecked(&mut self) {
        self.update_capture_points_unchecked();
        self.update_move_points_unchecked();
    }

    /// Update self's capture points with the drawable DistancesAngles.
    fn update_capture_points_unchecked(&mut self) {
        let capture_das = self.kind().capture_das().into_iter();
        let capture_points: &mut Vec<(f32, f32)> =
            &mut self.tvp_cache.as_mut().expect("Invariant.").capture_points;

        capture_points.clear();
        Piece::extend_with_drawable_points(&self.core, capture_points, capture_das);
    }

    /// Update self's move points with the drawable DistancesAngles.
    fn update_move_points_unchecked(&mut self) {
        let move_das = self.kind().move_das().into_iter();
        let move_points: &mut Vec<(f32, f32)> =
            &mut self.tvp_cache.as_mut().expect("Invariant.").move_points;

        move_points.clear();
        Piece::extend_with_drawable_points(&self.core, move_points, move_das);
    }

    /// Extend points with the drawable points from each DA in das.
    ///
    /// Necessary metadata like offset angle and piece center is retrieved from self.
    ///
    /// This function blocks, and will become an infinite loop if some idiot (me) manages to
    /// define a chess piece that moves infinitely without leaving the board. An error
    /// arising from this should become pretty obvious. "Oh, hey, I just added Mr. moves
    /// around in circles, and for some reason my game freezes whenever I try to use him."
    fn extend_with_drawable_points(
        core: &CorePieceData,
        points: &mut Vec<(f32, f32)>,
        das: impl Iterator<Item = DistancesAngle>,
    ) {
        for da in das {
            for (x, y) in da.get_offsets(core.angle + PI / 2.) {
                let point = (x + core.center.0, y + core.center.1);
                if !Piece::on_board(point.0, point.1) {
                    break;
                }
                points.push(point);
            }
        }
    }
}

/// The scalar composition of vectors point in the direction of dir where the vectors have starting point start
fn scalar_comp(
    start_x: f32,
    start_y: f32,
    point_x: f32,
    point_y: f32,
    dir_x: f32,
    dir_y: f32,
) -> f32 {
    // scalar comp of v in the direction of u: we find u dot v / magn(u)
    let u = (dir_x - start_x, dir_y - start_y);
    let v = (point_x - start_x, point_y - start_y);

    (u.0 * v.0 + u.1 * v.1) / f32::sqrt(u.0.powi(2) + u.1.powi(2))
}

/// simple distance formula + hitcirclerad
///
/// might be made into Piece const? this is legacy code.
fn max_hit_distance(start_x: f32, start_y: f32, end_x: f32, end_y: f32) -> f32 {
    f32::sqrt((start_x - end_x).powi(2) + (start_y - end_y).powi(2)) + PIECE_RADIUS
}

/// distance from a point to a line, where the line is given by two points
fn point_to_line_dist(
    start_x: f32,
    start_y: f32,
    end_x: f32,
    end_y: f32,
    point_x: f32,
    point_y: f32,
) -> f32 {
    f32::abs((end_x - start_x) * (point_y - start_y) - (point_x - start_x) * (end_y - start_y))
        / f32::sqrt((end_x - start_x).powi(2) + (end_y - start_y).powi(2))
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TravelKind {
    Capture,
    Move,
}

/// An identifier used to find a piece within a [`Pieces`]
///
/// The type or concept this aliases may change. In particular, do not assume this is an index into the inner piece Vec.
pub type PieceId = usize;

/// A set of pieces.
///
/// # Invariants
///
/// We maintain that
/// - pieces will not overlap
#[derive(Clone)]
pub struct Pieces {
    inner: Vec<(bool, Piece)>,
}

impl Pieces {
    /// Create a board with standard piece positions.
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
                i * 2,
                (i as u8, 1),
                -PI,
                Side::Black,
                PieceKind::Pawn,
            ));
            inner.push(Piece::from_tile(
                i * 2 + 1,
                (i as u8, 6),
                0.,
                Side::White,
                PieceKind::Pawn,
            ));
        }

        for (i, kind) in ORDER.iter().enumerate() {
            inner.push(Piece::from_tile(
                i * 2 + 16,
                (i as u8, 0),
                -PI,
                Side::Black,
                *kind,
            ));
            inner.push(Piece::from_tile(
                i * 2 + 17,
                (i as u8, 7),
                0.,
                Side::White,
                *kind,
            ));
        }

        debug_assert!({
            // piece ids must be their indices
            let all_piece_ids: Vec<PieceId> = inner.iter().map(|piece| piece.id()).collect();
            all_piece_ids == (0..all_piece_ids.len()).collect::<Vec<PieceId>>()
        });

        let inner = inner.into_iter().map(|piece| (true, piece)).collect();

        Self { inner }
    }

    /// Create a board with a shuffled back row.
    ///
    /// This is known as the 960, or Fischer, variant setup.
    ///
    /// `idx_ordering` must be a permutation of `0..8`. It is used as indices for the pieces,
    /// so it should probably be randomly generated.
    pub fn chess960_board(idx_ordering: impl FnOnce() -> [usize; 8]) -> Self {
        let mut inner = vec![];

        let pieces: [PieceKind; 8] = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];

        let ordering = idx_ordering();
        debug_assert!({
            let mut ordering2 = ordering.to_vec();
            ordering2.sort();
            ordering2 == (0..8).collect::<Vec<_>>()
        });
        let order = ordering.map(|i| pieces[i]);

        for i in 0..8 {
            inner.push(Piece::from_tile(
                i * 2,
                (i as u8, 1),
                -PI,
                Side::Black,
                PieceKind::Pawn,
            ));
            inner.push(Piece::from_tile(
                i * 2 + 1,
                (i as u8, 6),
                0.,
                Side::White,
                PieceKind::Pawn,
            ));
        }

        for (i, kind) in order.iter().enumerate() {
            inner.push(Piece::from_tile(
                i * 2 + 16,
                (i as u8, 0),
                -PI,
                Side::Black,
                *kind,
            ));
            inner.push(Piece::from_tile(
                i * 2 + 17,
                (i as u8, 7),
                0.,
                Side::White,
                *kind,
            ));
        }

        debug_assert!({
            // piece ids must be their indices
            let all_piece_ids: Vec<PieceId> = inner.iter().map(|piece| piece.id()).collect();
            all_piece_ids == (0..all_piece_ids.len()).collect::<Vec<PieceId>>()
        });

        let inner = inner.into_iter().map(|piece| (true, piece)).collect();

        Self { inner }
    }

    pub fn board_pieces(&self) -> impl Iterator<Item = &Piece> {
        self.inner
            .iter()
            .filter_map(|&(alive, ref piece)| alive.then(|| piece))
    }

    pub fn board_pieces_mut(&mut self) -> impl Iterator<Item = &mut Piece> {
        self.inner
            .iter_mut()
            .filter_map(|&mut (alive, ref mut piece)| alive.then(|| piece))
    }

    pub fn make_move(&mut self, move_: &Move) {
        let travel_piece = self.get_mut(move_.travel.piece()).expect("exists");

        travel_piece.set_center(move_.travel.dest());
        for pieceid in move_.travel.captures() {
            // not-alive the piece
            self.inner[*pieceid].0 = false;
        }

        let rotate_piece = self.get_mut(move_.rotate.piece).expect("exists");
        rotate_piece.set_angle(move_.rotate.dest);
    }

    pub fn unmake_move(&mut self, move_: &Move) {
        let rotate_piece = self.get_mut(move_.rotate.piece).expect("exists");
        rotate_piece.set_angle(move_.rotate.src);

        let travel_piece = self.get_mut(move_.travel.piece()).expect("exists");

        travel_piece.set_center(move_.travel.src());
        for pieceid in move_.travel.captures() {
            // re-alive the piece
            self.inner[*pieceid].0 = true;
        }
    }

    /// Get the piece that collides with `(x, y)`, if it exists.
    pub fn get_id(&self, x: f32, y: f32) -> Option<PieceId> {
        for piece in self.board_pieces() {
            if piece.collidepoint(x, y) {
                return Some(piece.id());
            }
        }
        None
    }

    /// Get the piece with an id, if the piece exists.
    pub fn get(&self, id: PieceId) -> Option<&Piece> {
        if let Some((alive, piece)) = self.inner.get(id)
            && *alive
        {
            return Some(piece);
        }
        None
    }

    /// Get a mutable piece given an id, if the piece exists.
    pub fn get_mut(&mut self, id: PieceId) -> Option<&mut Piece> {
        if id < self.inner.len() && self.inner[id].0 {
            return Some(&mut self.inner[id].1);
        }
        None
    }

    /// Inits (or reinits) every piece's auxiliary data.
    pub fn init_all_auxiliary_data(&mut self) {
        for piece in self.board_pieces_mut() {
            piece.init_auxiliary_data();
        }
    }

    /// Move the piece at id to x, y.
    ///
    /// # Warnings
    ///
    /// This may shuffle piece identifiers! Returns the piece's new ID..
    pub fn travel(&mut self, id: PieceId, x: f32, y: f32) {
        for &mut (ref mut alive, ref mut piece) in self.inner.iter_mut() {
            if *alive && piece.collidepiece(x, y) {
                *alive = false;
            }
        }

        let piece = &mut self.inner[id].1;
        piece.set_x(x);
        piece.set_y(y);
        if Piece::should_promote(piece.kind(), piece.side(), y) {
            piece.set_kind(PieceKind::Queen);
            piece.init_auxiliary_data();
        }
    }

    pub fn travelable(
        &self,
        piece: &Piece,
        x: f32,
        y: f32,
        kind: TravelKind,
    ) -> Option<TravelPhase> {
        // println!("checking travelable points");
        let mut pieces_overlapping_endpoint: HashSet<&Piece> = HashSet::new();

        /// Get the answer for Some cases of travelable.
        ///
        /// We move things, so this must be used right before returning.
        macro_rules! travelphase_answer {
            () => {{
                let len = pieces_overlapping_endpoint.len();
                let mut capture_ids: [usize; MAX_CAPTURES] = [0; MAX_CAPTURES];
                for (i, piece) in pieces_overlapping_endpoint.into_iter().enumerate() {
                    if i == MAX_CAPTURES {
                        panic!("How in the world did you manage to capture that many pieces?");
                    }
                    capture_ids[i] = piece.id();
                }
                TravelPhase::new(piece.id(), piece.center(), (x, y), (len, capture_ids))
            }};
        }

        // disallow capturing own side. also find which pieces overlap the endpoint
        for other_piece in self.board_pieces() {
            if other_piece == piece {
                debug_assert!(!piece.needs_init());
                continue;
            }

            if other_piece.collidepiece(x, y) {
                pieces_overlapping_endpoint.insert(other_piece);

                if other_piece.side() == piece.side() {
                    return None;
                }
            }
        }

        if piece.core.kind.can_jump() {
            match kind {
                TravelKind::Capture => {
                    if !pieces_overlapping_endpoint.is_empty() {
                        return Some(travelphase_answer!());
                    }
                }
                TravelKind::Move => {
                    return pieces_overlapping_endpoint
                        .is_empty()
                        .then(|| travelphase_answer!());
                }
            };
        }

        let mut in_the_way = 0;
        for other_piece in self.board_pieces() {
            if other_piece == piece {
                continue;
            }

            let comp = scalar_comp(piece.x(), piece.y(), other_piece.x(), other_piece.y(), x, y);
            if 0. < comp && comp < max_hit_distance(piece.x(), piece.y(), x, y) {
                // piece is within correct distance to block. now check:
                if point_to_line_dist(piece.x(), piece.y(), x, y, other_piece.x(), other_piece.y())
                    < 2. * PIECE_RADIUS
                {
                    // piece is within correct point to line distance to block. we may be blocked unless we can capture this piece.
                    // println!("a {:?} can block", other_piece.kind());
                    if !pieces_overlapping_endpoint.contains(&other_piece) {
                        in_the_way += 1;
                    }
                }
            }
        }

        // println!(
        //     "inway: {in_the_way}, overlaps: {}",
        //     pieces_overlapping_endpoint.len()
        // );
        if in_the_way > 0 {
            return None;
        }

        debug_assert!(
            pieces_overlapping_endpoint
                .iter()
                .all(|other_piece| other_piece.side() != piece.side())
        );
        match kind {
            TravelKind::Capture => {
                (!pieces_overlapping_endpoint.is_empty()).then(|| travelphase_answer!())
            }
            TravelKind::Move => pieces_overlapping_endpoint
                .is_empty()
                .then(|| travelphase_answer!()),
        }
    }
}
