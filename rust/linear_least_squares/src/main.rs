use std::fs;

use clap::{Parser, Subcommand};
use nalgebra as na;
use rand;
use rand_distr::Normal;

use linear_least_squares::Polynomial;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate new data
    Generate {
        /// Number of "measured" points
        #[arg(short, long, default_value_t = 100)]
        num_points: i32,

        /// Amount of noise
        #[arg(short, long)]
        stdev: f64,

        /// File to save output
        #[arg(short, long, value_name = "FILE")]
        output_file: String,
    },
    Fit {
        /// Datapoints to fit to (output of `generate`)
        #[arg(short, long)]
        input_file: String,

        /// Degree of polynomial fit
        #[arg(short, long, default_value_t = 2)]
        degree: usize,

        /// Output file for polyfit points
        #[arg(short, long)]
        output_file: String,
    },
}

/// Generate noisy "sensor" data.
///
/// Points are distributed uniformly in `x` from -5 to +5
///
/// The `y` values come from the polynomial at `x` with added Gaussian
/// noise.
fn make_data(p: &Polynomial, pt_count: usize, y_noise_std: f64) -> na::MatrixXx2<f64> {
    let mut rng = rand::thread_rng();
    let dist_x = rand::distributions::Uniform::new(-5.0, 5.0);
    let dist_y = Normal::new(0.0, y_noise_std).unwrap();

    let pt_x = na::DVector::from_distribution(pt_count, &dist_x, &mut rng);
    let pt_y = na::DVector::from_vec(p.evaluate_vec(&pt_x.data.as_vec()))
        + na::DVector::from_distribution(pt_count, &dist_y, &mut rng);
    na::MatrixXx2::from_columns(&[pt_x, pt_y])
}

fn save_matrix<T1, T2, R>(matrix: &na::Matrix<f64, T1, T2, R>, filename: &str)
where
    T1: na::Dim,
    T2: na::Dim,
    R: na::Storage<f64, T1, T2>,
{
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

    std::fs::write(filename, matrix_str.as_bytes()).expect("Failed to write.");
}

fn fit_polynomial(data_xy: &na::MatrixXx2<f64>, degree: usize) -> Result<Polynomial, String> {
    fn make_row(x: f64, degree: usize) -> impl Iterator<Item = f64> {
        (0..=degree).map(move |deg| x.powi(deg as i32))
    }

    let a = na::DMatrix::from_row_iterator(
        data_xy.nrows(),
        degree + 1,
        data_xy
            .column(0)
            .iter()
            .copied()
            .flat_map(|x| make_row(x, degree)),
    );
    let y = data_xy.column(1);

    let ataia = (a.transpose() * &a)
        .try_inverse()
        .ok_or("Failed to invert matrix")?
        * a.transpose();

    let coeff = ataia * y;
    Ok(Polynomial::new(coeff.data.as_vec().to_owned()))
}

fn linspace(start: f64, stop: f64, count: usize) -> Vec<f64> {
    assert!(start <= stop);
    assert!(count > 0);
    let step = (stop - start) / ((count - 1) as f64);
    (0..count).map(|i| start + i as f64 * step).collect()
}

/// Calculate the RMS of a set of values.
fn rms(x: &Vec<f64>) -> f64 {
    (x.iter().map(|x| x * x).sum::<f64>() / x.len() as f64).sqrt()
}

fn main() {
    let cli = Cli::parse();

    println!("{:#?}", cli);

    let noise = 0.1;
    let p = Polynomial::new(vec![0., 0., 1.]);

    let noisy_data = make_data(&p, 400, noise);
    let poly = fit_polynomial(&noisy_data, 2).unwrap();

    let err = rms(&noisy_data
        .row_iter()
        .map(|x| x[1] - poly.evaluate(x[0]))
        .collect());

    println!(
        "Actual Polynomial: {p}\n\
        Injected Noise: {noise}\n\
        Polynomial fit: {poly:.4}\n\
        RMS Error: {err:.4} (should equal noise for a good model)"
    );

    let sample_count = 101;
    let x_samples = na::DVector::from_vec(linspace(-5., 5., sample_count));
    let y_pts = na::DVector::from_vec(poly.evaluate_vec(&x_samples.data.as_vec()));
    let fit = na::MatrixXx2::from_columns(&[x_samples, y_pts]);

    // Save output
    fs::create_dir_all("output").unwrap();
    save_matrix(&noisy_data, "output/NOISY.csv");
    save_matrix(&fit, "output/POLY_FIT.csv");

    println!("Plot the results with\n\tgnuplot -sc plot.gnuplot");
}
