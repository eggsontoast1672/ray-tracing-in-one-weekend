/// A mathematical interval.
///
/// This type represents an interval of real numbers. A particular number `x` is contained in the
/// interval if and only if `min <= x <= max`, where `min` and `max` are the members as defined
/// below. To use this type like an open interval, see [`Interval::surrounds`]. To use it like a
/// closed interval, see [`Interval::contains`].
#[derive(Clone, Copy)]
pub struct Interval {
    /// The lower bound of the interval.
    pub min: f64,

    /// The upper bound of the interval.
    pub max: f64,
}

impl Interval {
    /// The empty interval.
    ///
    /// This interval contains no number. The bounds are not defined, so no part of the program
    /// should depend on them. However, one can depend on the fact that the lower bound of this
    /// interval is greater than its upper bound.
    pub const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);

    /// The universal interval.
    ///
    /// This interval contains every number. Since the lower bound must in particular be less than
    /// or equal to every number, the only reasonable value is [`f64::INFINITY`]. Similarly, the
    /// only sensical upper bound is [`f64::INFINITY`].
    pub const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);

    /// Create a new interval with specified bounds.
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    /// Get the size of the interval.
    ///
    /// This method retrieves the size of this interval. The size is positive if `self.min <
    /// self.max` and is negative if `self.min > self.max`.
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// Determine if this interval contains the given number.
    ///
    /// This method will return `true` if the given number is contained in this interval and
    /// `false` otherwise. Note that if `self.size()` is negative, this method will always return
    /// `false`.
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// Determine if this interval surrounds the given number.
    ///
    /// This method will return `true` if the given number is an interior point of the interval.
    /// More precisely, if there exists an interval centered at `x` which is entirely contained in
    /// this interval, this method will return `true`. If not (i.e. `x` is one of the boundary
    /// points, `false` is returned.
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    /// Clamp a number on this interval.
    ///
    /// If `x` is contained by this interval, then the method returns `x`. If it lies outside the
    /// interval below, the lower bound is returned. Otherwise, if it lies outside the interval
    /// above, the upper bound is returned. If the interval has negative size, the output of this
    /// method is not specified.
    pub fn clamp(&self, x: f64) -> f64 {
        match (self.min <= x, x <= self.max) {
            (false, _) => self.min,
            (true, false) => self.max,
            (true, true) => x,
        }
    }
}
