use core::fmt;
use nalgebra as na;

/// Polynomial represented by its coefficients.
///
/// Note: `coeff[i]` corresponds to the `x^i` term.
pub struct Polynomial {
    coeff: Vec<f64>,
}

impl Polynomial {
    pub fn new(coeff: Vec<f64>) -> Polynomial {
        return Polynomial { coeff };
    }

    /// Evalulate a polynomial at a point.
    pub fn evaluate(&self, x: f64) -> f64 {
        // Horner's method
        self.coeff.iter().rev().fold(0.0, |acc, c| acc * x + c)
    }

    /// Evaluate a polynomial over a vector.
    pub fn evaluate_vec(&self, x: &Vec<f64>) -> Vec<f64> {
        x.iter().map(|x| self.evaluate(*x)).collect()
    }

    /// Use linear least squares to fit a polynomial to a set of data
    /// points.
    ///
    /// `data_xy` must be a dynamic X x 2 matrix where each row is an
    /// `[x, y]` data point.
    pub fn fit(data_xy: &na::MatrixXx2<f64>, degree: usize) -> Result<Polynomial, String> {
        /// Prepares a vector of `[1, x, x^2, ...]`
        fn make_row_of_x(x: f64, degree: usize) -> impl Iterator<Item = f64> {
            (0..=degree).map(move |deg| x.powi(deg as i32))
        }

        let a = na::DMatrix::from_row_iterator(
            data_xy.nrows(),
            degree + 1,
            data_xy
                .column(0)
                .iter()
                .copied()
                .flat_map(|x| make_row_of_x(x, degree)),
        );

        let atai = (a.transpose() * &a)
            .try_inverse()
            .ok_or("Failed to invert A^T A.")?;

        let y = data_xy.column(1);
        let coeff = atai * a.transpose() * y;
        Ok(Polynomial::new(coeff.data.as_vec().to_owned()))
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut format_term = |coeff: f64, exponent: usize| {
            if coeff == 0. {
                return Ok(()); // no string
            }

            write!(f, "{}", if coeff > 0. { " +" } else { " -" })?;

            let abs_coeff = coeff.abs();
            let precision = f.precision().unwrap_or(3);

            if abs_coeff != 1. {
                write!(f, "{:.*}", precision, abs_coeff)?;
            }

            match exponent {
                0 => Ok(()),
                1 => write!(f, "x"),
                e => write!(f, "x^{e}"),
            }
        };

        if self.coeff.iter().all(|x| *x == 0.) {
            // Ensure something is returned even if all coeffs are 0
            f.write_str("0")?;
        } else {
            for (i, c) in self.coeff.iter().enumerate() {
                format_term(*c, i)?
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluate_constant() {
        let p = Polynomial::new(vec![7.0]);
        assert_eq!(p.evaluate(0.0), 7.0);
        assert_eq!(p.evaluate(10.0), 7.0);
    }

    #[test]
    fn evaluate_quad() {
        let p = Polynomial::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(p.evaluate(0.0), 1.0);
        assert_eq!(p.evaluate(1.0), 6.0);
        assert_eq!(p.evaluate(2.0), 17.0);
    }
}
