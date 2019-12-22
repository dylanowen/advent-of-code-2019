use std::fmt;
use std::fmt::{Debug, Display, Formatter};

pub static NEIGHBOR_DELTAS: [Point; 4] = [
    Point { x: 0, y: 1 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: -1 },
    Point { x: -1, y: 0 },
];

pub trait PointLike {
    fn new(x: isize, y: isize) -> Self
    where
        Self: Sized;

    fn x(&self) -> isize;
    fn x_mut(&mut self) -> &mut isize;
    fn y(&self) -> isize;
    fn y_mut(&mut self) -> &mut isize;

    fn neighbors(&self) -> [Self; 4]
    where
        Self: Sized + Copy,
    {
        let mut result = [*self; 4];
        for (i, r) in result.iter_mut().enumerate() {
            r.inc(&NEIGHBOR_DELTAS[i])
        }

        result
    }

    #[inline]
    fn inc(&mut self, other: &dyn PointLike) {
        *self.x_mut() += other.x();
        *self.y_mut() += other.y();
    }

    #[inline]
    fn dec(&mut self, other: &dyn PointLike) {
        *self.x_mut() -= other.x();
        *self.y_mut() -= other.y();
    }

    #[inline]
    fn add(&self, other: &dyn PointLike) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x() + other.x(), self.y() + other.y())
    }

    #[inline]
    fn sub(&self, other: &dyn PointLike) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x() - other.x(), self.y() - other.y())
    }

    #[inline]
    fn distance(&self, other: &dyn PointLike) -> usize {
        ((self.x() - other.x()).abs() + (self.y() - other.y()).abs()) as usize
    }
}

impl PartialEq for dyn PointLike {
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() && self.y() == other.y()
    }
}

impl Display for dyn PointLike {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x(), self.y())
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

pub static ZERO_POINT: Point = Point { x: 0, y: 0 };

impl PointLike for Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    #[inline]
    fn x(&self) -> isize {
        self.x
    }

    #[inline]
    fn x_mut(&mut self) -> &mut isize {
        &mut self.x
    }

    #[inline]
    fn y(&self) -> isize {
        self.y
    }

    #[inline]
    fn y_mut(&mut self) -> &mut isize {
        &mut self.y
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <dyn PointLike as Display>::fmt(self, f)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <dyn PointLike as Display>::fmt(self, f)
    }
}

impl PointLike for (isize, isize) {
    fn new(x: isize, y: isize) -> (isize, isize) {
        (x, y)
    }

    #[inline]
    fn x(&self) -> isize {
        self.0
    }

    #[inline]
    fn x_mut(&mut self) -> &mut isize {
        &mut self.0
    }

    #[inline]
    fn y(&self) -> isize {
        self.1
    }

    #[inline]
    fn y_mut(&mut self) -> &mut isize {
        &mut self.1
    }
}
