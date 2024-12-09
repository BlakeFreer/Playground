use std::{iter::zip, ops};

use crate::Vector;

/// Add two vectors.
/// ```
/// # use riesz::Vector;
/// let v1 = Vector::new([1.0, 2.0, 3.0]);
/// let v2 = Vector::<3>::constant(100.0);
/// assert_eq!(v1 + v2, Vector::new([101.0, 102.0, 103.0]));
/// ```
impl<const N: usize> ops::Add for Vector<N> {
    type Output = Vector<N>;
    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            data: std::array::from_fn(|i| self[i] + other[i]),
        }
    }
}

/// Add onto a vector.
/// ```
/// # use riesz::Vector;
/// let mut v1 = Vector::new([1.0, 2.0, 3.0]);
/// v1 += Vector::<3>::constant(100.0);
/// assert_eq!(v1, Vector::new([101.0, 102.0, 103.0]));
impl<const N: usize> ops::AddAssign for Vector<N> {
    fn add_assign(&mut self, rhs: Self) {
        for (x, y) in self.data.iter_mut().zip(rhs.data.iter()) {
            *x += y;
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
impl<const N: usize> ops::Mul<f64> for Vector<N> {
    type Output = Vector<N>;
    fn mul(self, scalar: f64) -> Self::Output {
        Self::Output {
            data: self.data.map(|x| x * scalar),
        }
    }
}

impl<const N: usize> ops::Mul<Vector<N>> for f64 {
    type Output = Vector<N>;
    fn mul(self, v: Vector<N>) -> Self::Output {
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
impl<const N: usize> ops::MulAssign<f64> for Vector<N> {
    fn mul_assign(&mut self, scalar: f64) {
        for x in self.data.iter_mut() {
            *x *= scalar;
        }
    }
}

/// Divide a vector by a scalar.
/// ```
/// # use riesz::Vector;
/// let v = Vector::new([1.0, 2.0, 3.0, 4.0, 5.0]);
/// assert_eq!(v / 2.0, Vector::new([0.5, 1.0, 1.5, 2.0, 2.5]));
/// ```
impl<const N: usize> ops::Div<f64> for Vector<N> {
    type Output = Vector<N>;
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
impl<const N: usize> ops::DivAssign<f64> for Vector<N> {
    fn div_assign(&mut self, scalar: f64) {
        *self *= 1.0 / scalar
    }
}

/// Invert a vector.
/// ```
/// # use riesz::Vector;
/// assert_eq!(-Vector::new([1.0, 2.0]), Vector::new([-1.0, -2.0]));
/// ```
impl<const N: usize> ops::Neg for Vector<N> {
    type Output = Vector<N>;
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
impl<const N: usize> ops::Sub for Vector<N> {
    type Output = Vector<N>;
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
impl<const N: usize> ops::SubAssign for Vector<N> {
    fn sub_assign(&mut self, rhs: Self) {
        for (x, y) in self.data.iter_mut().zip(rhs.data.iter()) {
            *x -= y;
        }
    }
}

impl<const N: usize> Vector<N> {
    /// Calculate the Euclidean magnitude of a vector.
    /// ```
    /// # use riesz::Vector;
    /// let v = Vector::<2>::new([3.0, 4.0]);
    /// assert_eq!(v.mag(), 5.0);
    /// ```
    pub fn mag(&self) -> f64 {
        self.data.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
    }

    // ------ Methods ------
    /// Take the product of two vectors.
    /// ```
    /// # use riesz::Vector;
    /// let v1 = Vector::new([3.0, 4.5]);
    /// let v2 = Vector::new([-8.0, 2.0]);
    /// assert_eq!(v1.dot(v2), -15.0);
    /// ```
    pub fn dot(&self, v2: Self) -> f64 {
        zip(self.data, v2.data).map(|(x, y)| x * y).sum()
    }
}
