pub trait PointLike {
    fn new(x: isize, y: isize) -> Self
    where
        Self: Sized;

    fn x(&self) -> isize;
    fn x_mut(&mut self) -> &mut isize;
    fn y(&self) -> isize;
    fn y_mut(&mut self) -> &mut isize;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
