use std::{
    cmp::Ordering,
    convert::TryFrom,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cell {
    Dead = 0u8,
    Alive = 1u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

pub(crate) use Quadrant::*;
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Quadrant {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

// use enum instead with East, West, etc. variants?
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Offset {
    pub dx: i64,
    pub dy: i64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Level(u8);

impl From<(i64, i64)> for Position {
    fn from(t: (i64, i64)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl Position {
    #[allow(dead_code)]
    pub const ORIGIN: Self = Self::new(0, 0);

    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub(crate) fn quadrant(self) -> Quadrant {
        match (self.x < 0, self.y < 0) {
            (true, true) => Quadrant::NorthWest,
            (false, true) => Quadrant::NorthEast,
            (true, false) => Quadrant::SouthWest,
            (false, false) => Quadrant::SouthEast,
        }
    }

    pub(crate) fn relative_to(self, other: Self) -> Self {
        self + Offset::new(-other.x, -other.y)
    }

    pub(crate) fn in_bounds(self, level: Level) -> bool {
        let bounds = level.coord_range();
        bounds.contains(&self.x) && bounds.contains(&self.y)
    }
}

impl From<(i64, i64)> for Offset {
    fn from(t: (i64, i64)) -> Self {
        Self::new(t.0, t.1)
    }
}

impl Add for Offset {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Offset::new(self.dx + other.dy, self.dy + other.dy)
    }
}

impl AddAssign for Offset {
    fn add_assign(&mut self, other: Self) {
        self.dx += other.dx;
        self.dy += other.dy;
    }
}

impl Sub for Offset {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Offset::new(self.dx - other.dy, self.dy - other.dy)
    }
}

impl SubAssign for Offset {
    fn sub_assign(&mut self, other: Self) {
        self.dx -= other.dx;
        self.dy -= other.dy;
    }
}

impl Add<Offset> for Position {
    type Output = Self;
    fn add(self, other: Offset) -> Self::Output {
        Position::new(self.x + other.dx, self.y + other.dy)
    }
}

impl AddAssign<Offset> for Position {
    fn add_assign(&mut self, other: Offset) {
        self.x += other.dx;
        self.y += other.dy;
    }
}

impl Sub<Offset> for Position {
    type Output = Self;
    fn sub(self, other: Offset) -> Self::Output {
        Position::new(self.x - other.dx, self.y - other.dy)
    }
}

impl SubAssign<Offset> for Position {
    fn sub_assign(&mut self, other: Offset) {
        self.x -= other.dx;
        self.y -= other.dy;
    }
}

impl Offset {
    pub const fn new(dx: i64, dy: i64) -> Self {
        Self { dx, dy }
    }
}

impl PartialEq<u8> for Level {
    fn eq(&self, n: &u8) -> bool {
        self.0 == *n
    }
}

impl PartialOrd<u8> for Level {
    fn partial_cmp(&self, n: &u8) -> Option<Ordering> {
        Some(self.0.cmp(n))
    }
}

//impl Ord<u8> for Level {
//fn cmp(&self, n: &u8) -> Ordering {
//self.0.cmp(n)
//}
//}

impl Add for Level {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let l = Level(self.0 + other.0);
        l.check_validity();
        l
    }
}

impl Add<u8> for Level {
    type Output = Self;

    fn add(self, n: u8) -> Self {
        let l = Level(self.0 + n);
        l.check_validity();
        l
    }
}

impl AddAssign for Level {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.check_validity();
    }
}

impl AddAssign<u8> for Level {
    fn add_assign(&mut self, n: u8) {
        self.0 += n;
        self.check_validity();
    }
}

impl Sub for Level {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Level(self.0 - other.0)
    }
}

impl Sub<u8> for Level {
    type Output = Self;

    fn sub(self, n: u8) -> Self {
        Level(self.0 - n)
    }
}

impl SubAssign for Level {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl SubAssign<u8> for Level {
    fn sub_assign(&mut self, n: u8) {
        self.0 -= n;
    }
}

impl Level {
    pub(crate) const MAX_LEVEL: Self = Self(63);
    pub(crate) const LEAF_LEVEL: Self = Self(0);

    pub(crate) fn new(n: u8) -> Self {
        Self(n)
    }

    pub(crate) const fn side_len(self) -> u64 {
        1 << self.0
    }

    pub(crate) fn quadrant_center(self, quadrant: Quadrant) -> Position {
        let delta = i64::try_from(self.side_len() / 4).unwrap();
        match quadrant {
            NorthWest => (-delta, -delta).into(),
            NorthEast => (delta, -delta).into(),
            SouthWest => (-delta, delta).into(),
            SouthEast => (delta, delta).into(),
        }
    }

    pub(crate) const fn min_coord(self) -> i64 {
        -(1 << (self.0 - 1))
    }

    pub(crate) const fn max_coord(self) -> i64 {
        (1 << (self.0 - 1)) - 1
    }

    pub(crate) const fn coord_range(self) -> std::ops::Range<i64> {
        self.min_coord()..self.max_coord()
    }

    #[allow(dead_code)]
    pub(crate) fn min_pos(self) -> Position {
        let min = Self::min_coord(self);
        (min, min).into()
    }

    #[allow(dead_code)]
    pub(crate) fn max_pos(self) -> Position {
        let max = Self::max_coord(self);
        (max, max).into()
    }

    #[allow(dead_code)]
    pub fn max_steps(self) -> u64 {
        debug_assert!(self.0 >= 2, "inode evolution is level 2 or higher");
        1u64 << (self.0 - 2)
    }

    fn check_validity(self) {
        if self > Self::MAX_LEVEL {
            panic!("the maximal level ({}) was exceeded", Self::MAX_LEVEL.0);
        }
    }
}
