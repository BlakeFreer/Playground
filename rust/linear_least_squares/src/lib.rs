use core::fmt;

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

    pub fn evaluate_vec(&self, x: &Vec<f64>) -> Vec<f64> {
        x.iter().map(|x| self.evaluate(*x)).collect()
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
