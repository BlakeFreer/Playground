use std::fs;

use clap::{Parser, Subcommand};
use nalgebra as na;

use linear_least_squares::{linspace, make_data, polynomial::Polynomial, rms, save_matrix};

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

fn main() {
    let cli = Cli::parse();

    println!("{:#?}", cli);

    let noise = 0.1;
    let p = Polynomial::new(vec![0., 0., 1.]);

    let noisy_data = make_data(&p, 400, noise);
    let poly = Polynomial::fit(&noisy_data, 2).unwrap();

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
