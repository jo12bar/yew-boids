use std::{
    f64::consts::{FRAC_PI_3, PI, TAU},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// τ/3
pub const FRAC_TAU_3: f64 = 2.0 * FRAC_PI_3;

/// Get the smallest signed angle from `source` to `target`.
/// The result is in the range `[-PI, PI)`.
pub fn smallest_angle_between(source: f64, target: f64) -> f64 {
    let d = target - source;
    (d + PI).rem_euclid(TAU) - PI
}

/// A two-dimensional vector.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    /// Create a new two-dimensional vector.
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Create a vector by converting polar coordinates into cartesian coordinates.
    pub fn from_polar(angle: f64, radius: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::new(radius * cos, radius * sin)
    }

    /// Calculate the square of the magnitude of this vector. Use this instead of
    /// [`Self::magnitude()`] whenever possible to avoid incurring the cost of a square root.
    pub fn magnitude_squared(self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    /// Calculate the magnitude of this vector.
    ///
    /// If you need the *square* of the magnitude instead of just the magnitude, use
    /// [`Self::magnitude_squared()`] instead. This avoids calculating a square root.
    pub fn magnitude(self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    /// Scale this vector such that its magnitude lies in the range `[0, max]`.
    pub fn clamp_magnitude(self, max: f64) -> Self {
        let mag = self.magnitude();
        if mag > max {
            self / mag * max
        } else {
            self
        }
    }

    /// Calculate the angle this vector makes with the x-axis. Positive angles are measured
    /// counter-clockwise from the positive x-axis.
    pub fn angle(self) -> f64 {
        self.y.atan2(self.x)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Div<f64> for Vec2 {
    type Output = Self;

    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}

impl DivAssign<f64> for Vec2 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Sum for Vec2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), |sum, val| sum + val)
    }
}

/// Compute the weighted mean (or "average") of multiple items.
///
/// **See also**: [`Mean`]
pub trait WeightedMean<T = Self>: Sized {
    /// Consume an iterator and compute the weighted mean of its items. The iterator must yield
    /// pairs of `(value, weight)`.
    ///
    /// Returns `None` if the total sum of all weighted means is zero, infinite,
    /// [subnormal](https://en.wikipedia.org/wiki/Subnormal_number), or `NaN` (i.e. if
    /// [`f64::is_normal()`] returns false).
    fn weighted_mean(it: impl Iterator<Item = (T, f64)>) -> Option<Self>;
}

/// Compute the weighted mean (or "average") of multiple items.
///
/// This is a blanket implementation for anything that implements addition (returning the same type),
/// multiplication (with `f64`), and division (with `f64`), is copyable, and has a default value.
/// This includes most standard numeric types, for example, as well as [`Vec2`].
impl<T> WeightedMean for T
where
    T: Add<Output = T> + Mul<f64, Output = T> + Div<f64, Output = T> + Copy + Default,
{
    fn weighted_mean(it: impl Iterator<Item = (T, f64)>) -> Option<T> {
        let (sum, total_weight) = it.fold(
            (T::default(), 0.0),
            |(sum, total_weight), (value, weight)| (sum + value * weight, total_weight + weight),
        );

        if total_weight.is_normal() {
            Some(sum / total_weight)
        } else {
            None
        }
    }
}

/// Compute the mean (or "average") of multiple items.
///
/// **See also**: [`WeightedMean`]
pub trait Mean<T = Self>: Sized {
    /// Consume an iterator and compute the mean of its items.
    ///
    /// Returns `None` if the total count of items is zero, infinite,
    /// [subnormal](https://en.wikipedia.org/wiki/Subnormal_number), or `NaN` (i.e. if
    /// [`f64::is_normal()`] returns false).
    fn mean(it: impl Iterator<Item = T>) -> Option<Self>;
}

/// Compute the mean (or "average") of multiple items.
///
/// This is a blanket implementation for anything that implements addition (returning the same type),
/// multiplication (with `f64`), and division (with `f64`), is copyable, and has a default value.
/// This includes most standard numeric types, for example, as well as [`Vec2`].
impl<T> Mean for T
where
    T: AddAssign + Sub<Output = T> + Div<f64, Output = T> + Copy + Default,
{
    fn mean(it: impl Iterator<Item = Self>) -> Option<Self> {
        let (avg, count) = it.fold((T::default(), 0.0), |(mut avg, mut count), value| {
            count += 1.0;
            avg += (value - avg) / count;
            (avg, count)
        });

        if count.is_normal() {
            Some(avg)
        } else {
            None
        }
    }
}
