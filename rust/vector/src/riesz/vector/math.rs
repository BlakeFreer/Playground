use std::{iter::zip, ops};

use crate::Matrix;

/// Add two vectors.
/// ```
/// # use riesz::Vector;
/// let v1 = Vector::new([1.0, 2.0, 3.0]);
/// let v2 = Vector::<3>::constant(100.0);
/// assert_eq!(v1 + v2, Vector::new([101.0, 102.0, 103.0]));
/// ```
impl<const R: usize, const C: usize> ops::Add for Matrix<R, C> {
    type Output = Matrix<R, C>;
    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            data: std::array::from_fn(|r| std::array::from_fn(|c| self[(r, c)] + other[(r, c)])),
        }
    }
}

/// Add onto a vector.
/// ```
/// # use riesz::Vector;
/// let mut v1 = Vector::new([1.0, 2.0, 3.0]);
/// v1 += Vector::<3>::constant(100.0);
/// assert_eq!(v1, Vector::new([101.0, 102.0, 103.0]));
/// ```
impl<const R: usize, const C: usize> ops::AddAssign for Matrix<R, C> {
    fn add_assign(&mut self, rhs: Self) {
        for (r, c) in self.data.iter_mut().zip(rhs.data.iter()) {
            for (x, y) in r.iter_mut().zip(c.iter()) {
                *x += y;
            }
        }
    }
}

/// Scale a vector.
/// ```
/// # use riesz::Vector;
/// let v = Vector::new([1.0, 2.0, 3.0]);
/// assert_eq!(v * 5.0, Vector::new([5.0, 10.0, 15.0]));
/// assert_eq!(5.0 * v, Vector::new([5.0, 10.0, 15.0]));
/// ```
impl<const R: usize, const C: usize> ops::Mul<f64> for Matrix<R, C> {
    type Output = Matrix<R, C>;
    fn mul(self, scalar: f64) -> Self::Output {
        Self::Output {
            data: self.data.map(|r| r.map(|x| x * scalar)),
        }
    }
}

impl<const R: usize, const C: usize> ops::Mul<Matrix<R, C>> for f64 {
    type Output = Matrix<R, C>;
    fn mul(self, v: Matrix<R, C>) -> Self::Output {
        v * self
    }
}

/// Scale a vector in place.
/// ```
/// # use riesz::Vector;
/// let mut v = Vector::new([1.0, 2.0, 3.0]);
/// v *= 5.0;
/// assert_eq!(v, Vector::new([5.0, 10.0, 15.0]));
/// ```
impl<const R: usize, const C: usize> ops::MulAssign<f64> for Matrix<R, C> {
    fn mul_assign(&mut self, scalar: f64) {
        for r in self.data.iter_mut() {
            for x in r.iter_mut() {
                *x *= scalar;
            }
        }
    }
}

/// Divide a vector by a scalar.
/// ```
/// # use riesz::Vector;
/// let v = Vector::new([1.0, 2.0, 3.0, 4.0, 5.0]);
/// assert_eq!(v / 2.0, Vector::new([0.5, 1.0, 1.5, 2.0, 2.5]));
/// ```
impl<const R: usize, const C: usize> ops::Div<f64> for Matrix<R, C> {
    type Output = Matrix<R, C>;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

/// Divide a vector in place.
/// ```
/// # use riesz::Vector;
/// let mut v = Vector::new([1.0, 2.0, 3.0, 4.0, 5.0]);
/// v /= 2.0;
/// assert_eq!(v, Vector::new([0.5, 1.0, 1.5, 2.0, 2.5]));
/// ```
impl<const R: usize, const C: usize> ops::DivAssign<f64> for Matrix<R, C> {
    fn div_assign(&mut self, scalar: f64) {
        *self *= 1.0 / scalar
    }
}

/// Invert a vector.
/// ```
/// # use riesz::Vector;
/// assert_eq!(-Vector::new([1.0, 2.0]), Vector::new([-1.0, -2.0]));
/// ```
impl<const R: usize, const C: usize> ops::Neg for Matrix<R, C> {
    type Output = Matrix<R, C>;
    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

/// Subtract two vectors.
/// ```
/// # use riesz::Vector;
/// let v1 = Vector::new([10.0, 5.0, 0.0]);
/// let v2 = Vector::new([20.0, -5.0, 3.0]);
/// assert_eq!(v1 - v2, Vector::new([-10.0, 10.0, -3.0]));
/// ```
impl<const R: usize, const C: usize> ops::Sub for Matrix<R, C> {
    type Output = Matrix<R, C>;
    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

/// Subtract a vector from another.
/// ```
/// # use riesz::Vector;
/// let mut v = Vector::new([10.0, 5.0, 0.0]);
/// v -= Vector::new([20.0, -5.0, 3.0]);
/// assert_eq!(v, Vector::new([-10.0, 10.0, -3.0]));
/// ```
impl<const R: usize, const C: usize> ops::SubAssign for Matrix<R, C> {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs
    }
}
