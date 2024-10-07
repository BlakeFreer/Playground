pub mod polynomial;

use crate::polynomial::Polynomial;
use nalgebra as na;
use rand;
use rand_distr::Normal;
use std::fs::{self, File};
use std::io::BufReader;
use std::io::{self, prelude::*};
use std::path::PathBuf;

pub fn save_matrix<T1, T2, R>(matrix: &na::Matrix<f64, T1, T2, R>, filename: &PathBuf)
where
    T1: na::Dim,
    T2: na::Dim,
    R: na::Storage<f64, T1, T2>,
{
    // Create directory, if needed
    match filename.parent() {
        Some(parent) => fs::create_dir_all(parent).expect("Failed to create directory for output."),
        None => {}
    }

    let matrix_str = matrix
        .row_iter()
        .map(|row| {
            row.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\t")
        })
        .collect::<Vec<String>>()
        .join("\n");

    std::fs::write(filename, matrix_str.as_bytes()).expect("Failed to write to file");
}

pub fn load_matrix(filename: &PathBuf, delimiter: char) -> Result<na::MatrixXx2<f64>, io::Error> {
    // let f = match File::open(filename) {
    //     Ok(file) => file,
    //     Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Err("missing"),
    //     Err(_) => return Err("other problem"),
    // };
    let f = File::open(filename)?;
    let f = BufReader::new(f);

    let data: Vec<Vec<f64>> = f
        .lines()
        .map(|line| {
            line.unwrap()
                .split(delimiter)
                .map(|x| x.parse::<f64>().expect("Invalid number."))
                .collect::<Vec<f64>>()
        })
        .collect();

    Ok(na::MatrixXx2::from_row_iterator(
        data.len(),
        data.iter().flatten().cloned(),
    ))
}

pub fn linspace(start: f64, stop: f64, count: usize) -> Vec<f64> {
    assert!(start <= stop);
    assert!(count > 0);
    let step = (stop - start) / ((count - 1) as f64);
    (0..count).map(|i| start + i as f64 * step).collect()
}

/// Calculate the RMS of a set of values.
pub fn rms(x: &Vec<f64>) -> f64 {
    (x.iter().map(|x| x * x).sum::<f64>() / x.len() as f64).sqrt()
}

/// Generate noisy "sensor" data.
///
/// Points are distributed uniformly in `x` from -5 to +5
///
/// The `y` values come from the polynomial at `x` with added Gaussian
/// noise.
pub fn make_data(p: &Polynomial, pt_count: usize, noise: f64) -> na::MatrixXx2<f64> {
    let dist_x = rand::distributions::Uniform::new(-5.0, 5.0);
    let dist_y = Normal::new(0.0, noise).unwrap();

    let mut rng = rand::thread_rng();
    let pt_x = na::DVector::from_distribution(pt_count, &dist_x, &mut rng);
    let pt_y = na::DVector::from_vec(p.evaluate_vec(&pt_x.data.as_vec()));
    let noise_y = na::DVector::from_distribution(pt_count, &dist_y, &mut rng);
    na::MatrixXx2::from_columns(&[pt_x, pt_y + noise_y])
}
