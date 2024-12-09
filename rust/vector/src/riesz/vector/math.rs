use std::{iter::zip, ops};

use crate::Vector;

/// Add two vectors
/// ```
/// # use riesz::Vector;
/// let v1 = Vector::new([1.0, 2.0, 3.0]);
/// let v2 = Vector::<3>::constant(100.0);
/// assert_eq!(v1 + v2, Vector::new([101.0, 102.0, 103.0]));
/// ```
impl<const N: usize> ops::Add for Vector<N> {
    type Output = Vector<N>;
    fn add(self, other: Self) -> Self::Output {
        Self::Output::try_from(
            zip(self.data, other.data)
                .map(|(x, y)| x + y)
                .collect::<Vec<f64>>(),
        )
        .unwrap()
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
        Self::Output::try_from(self.data.iter().map(|x| x * scalar).collect::<Vec<f64>>()).unwrap()
    }
}

impl<const N: usize> ops::Mul<Vector<N>> for f64 {
    type Output = Vector<N>;
    fn mul(self, v: Vector<N>) -> Self::Output {
        v * self
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
    pub fn dot(v1: Self, v2: Self) -> f64 {
        zip(v1.data.iter(), v2.data.iter())
            .map(|(x, y)| x * y)
            .sum()
    }
}
