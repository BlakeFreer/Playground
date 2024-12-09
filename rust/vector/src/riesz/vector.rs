use std::{cmp::PartialEq, convert::TryFrom, fmt, iter::zip, ops};

pub mod dim3;
pub mod math;

#[derive(Debug, Clone, Copy)]
pub struct Vector<const N: usize> {
    data: [f64; N],
}

impl<const N: usize> Vector<N> {
    // ------ Constructors ------
    pub fn zero() -> Self {
        Self { data: [0.0; N] }
    }
    pub fn constant(value: f64) -> Self {
        Self { data: [value; N] }
    }
    pub fn new(data: [f64; N]) -> Self {
        Self { data }
    }

    // ------ Properties ------
    pub fn size(&self) -> usize {
        N
    }
}

impl<const N: usize> PartialEq for Vector<N> {
    fn eq(&self, other: &Self) -> bool {
        zip(self.data, other.data).all(|(x, y)| x == y)
    }
}

impl<const N: usize> ops::Index<usize> for Vector<N> {
    type Output = f64;
    fn index(&self, index: usize) -> &f64 {
        &self.data[index]
    }
}
impl<const N: usize> ops::IndexMut<usize> for Vector<N> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f64 {
        &mut self.data[index]
    }
}

impl<const N: usize> fmt::Display for Vector<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let decimals = f.precision().unwrap_or(2);
        write!(f, "(")?;
        for (idx, &scalar) in self.data.iter().enumerate() {
            write!(
                f,
                "{scalar:.decimals$}{}",
                if idx < N - 1 { ", " } else { "" }
            )?;
        }
        write!(f, ")")
    }
}

impl<const N: usize> TryFrom<Vec<f64>> for Vector<N> {
    type Error = &'static str;

    fn try_from(vec: Vec<f64>) -> Result<Self, Self::Error> {
        if vec.len() != N {
            return Err("Wrong length");
        }

        Ok(Self {
            data: vec.try_into().unwrap(),
        })
    }
}
