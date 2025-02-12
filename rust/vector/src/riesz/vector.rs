use std::{cmp::PartialEq, fmt, ops};

// pub mod dim3;
pub mod math;

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const R: usize, const C: usize> {
    data: [[f64; C]; R],
}

impl<const R: usize, const C: usize> Matrix<R, C> {
    // ------ Constructors ------
    pub fn zero() -> Self {
        Self {
            data: [[0.0; C]; R],
        }
    }
    pub fn constant(value: f64) -> Self {
        Self {
            data: [[value; C]; R],
        }
    }
    pub fn new(data: [[f64; C]; R]) -> Self {
        Self { data }
    }

    // ------ Properties ------
    pub fn size(&self) -> usize {
        R * C
    }

    pub fn shape(&self) -> (usize, usize) {
        (R, C)
    }
}

/// Element-wise equality.
/// ```
/// # use riesz::Matrix;
/// let m1 = Matrix::new([[3.0, 3.0, 3.0], [3.0, 3.0, 3.0]]);
/// let m2 = Matrix::<2, 3>::constant(3.0);
/// assert_eq!(m1, m2);
/// ```
impl<const R: usize, const C: usize> PartialEq for Matrix<R, C> {
    fn eq(&self, other: &Self) -> bool {
        self.data
            .iter()
            .zip(other.data.iter())
            .all(|(row_a, row_b)| row_a.iter().zip(row_b.iter()).all(|(x, y)| x == y))
    }
}

/// 2D Indexing (row, column)
impl<const R: usize, const C: usize> ops::Index<(usize, usize)> for Matrix<R, C> {
    type Output = f64;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}
impl<const R: usize, const C: usize> ops::IndexMut<(usize, usize)> for Matrix<R, C> {
    fn index_mut<'a>(&'a mut self, index: (usize, usize)) -> &'a mut f64 {
        &mut self.data[index.0][index.1]
    }
}

// 1D Indexing.
/// ```
/// # use riesz::Matrix;
/// let m = Matrix::<2, 2>::new([[0.0, 1.0],[2.0, 3.0]]);
/// assert_eq!(m[0], 0.0);
/// assert_eq!(m[1], 1.0);
/// assert_eq!(m[2], 2.0);
/// assert_eq!(m[3], 3.0);
/// ```
impl<const R: usize, const C: usize> ops::Index<usize> for Matrix<R, C> {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        &self[(index / C, index % C)]
    }
}
impl<const R: usize, const C: usize> ops::IndexMut<usize> for Matrix<R, C> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f64 {
        &mut self[(index / C, index % C)]
    }
}

impl<const R: usize, const C: usize> fmt::Display for Matrix<R, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let decimals = f.precision().unwrap_or(2);
        for r in 0..R {
            write!(f, "{}[", if r == 0 { "[" } else { " " })?;
            for c in 0..C {
                write!(
                    f,
                    "{:.decimals$}{}",
                    self[(r, c)],
                    if c < C - 1 { ", " } else { "]" }
                )?;
            }
            if r < R - 1 {
                write!(f, "\n")?;
            }
        }
        write!(f, "]")
    }
}
