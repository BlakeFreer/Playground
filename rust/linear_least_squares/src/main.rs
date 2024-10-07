use std::{io, path::PathBuf};

use clap::{Parser, Subcommand};
use nalgebra as na;

use std::process;

use linear_least_squares::{
    linspace, load_matrix, make_data, polynomial::Polynomial, rms, save_matrix,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate new data
    Generate(GenerateArgs),
    /// Fit a polynomial to data points
    Fit(FitArgs),
}
#[derive(Parser, Debug)]
struct GenerateArgs {
    /// Number of "measured" points
    #[arg(short, long, default_value_t = 100)]
    num_points: usize,

    /// Amount of noise
    #[arg(short, long)]
    stdev: f64,

    /// File to save output
    #[arg(short, long, value_name = "FILE")]
    output_file: PathBuf,
}

#[derive(Parser, Debug)]
struct FitArgs {
    /// Datapoints to fit to (output of `generate`)
    #[arg(short, long)]
    input_file: PathBuf,

    /// Degree of polynomial fit
    #[arg(short, long, default_value_t = 2)]
    degree: usize,
}

fn handle_generate(args: GenerateArgs) -> Result<(), io::Error> {
    let p = Polynomial::new(vec![0., 0., 1.]);
    let noisy_data = make_data(&p, args.num_points, args.stdev);

    save_matrix(&noisy_data, &args.output_file);

    println!(
        "Polynomial: {p:.3}\n\
        Noise stdev: {}\n\
        Number of points: {}",
        args.stdev, args.num_points
    );
    println!(
        "Generated noisy data at \"{}\"",
        args.output_file.to_str().unwrap()
    );
    Ok(())
}

fn handle_fit(args: FitArgs) -> Result<(), io::Error> {
    let noisy_data = match load_matrix(&args.input_file, '\t') {
        Ok(data) => data,
        Err(e) => match e.kind() {
            k @ io::ErrorKind::NotFound => {
                return Err(io::Error::new(
                    k,
                    "Input file does not exist. Did you create data with `generate`?",
                ))
            }
            _ => return Err(e),
        },
    };

    println!("Fitting a degree {} polynomial.", args.degree);
    let poly = Polynomial::fit(&noisy_data, args.degree).unwrap();

    let err = rms(&noisy_data
        .row_iter()
        .map(|x| x[1] - poly.evaluate(x[0]))
        .collect());
    println!(
        "Polynomial fit: {poly:.4}\n\
        Number of points: {}\n\
        RMS Error: {err:.4} (should equal noise for a good model)",
        noisy_data.nrows()
    );

    let sample_count = 101;
    let x_samples = na::DVector::from_vec(linspace(-5., 5., sample_count));
    let y_pts = na::DVector::from_vec(poly.evaluate_vec(&x_samples.data.as_vec()));
    let fit = na::MatrixXx2::from_columns(&[x_samples, y_pts]);

    let fit_filename = args.input_file.with_extension("fit");
    save_matrix(&fit, &fit_filename);

    plot(
        &args.input_file.to_str().unwrap(),
        &fit_filename.to_str().unwrap(),
    );

    Ok(())
}

fn plot(raw_filename: &str, fit_filename: &str) {
    let gplot = vec![
        "gnuplot -s",
        &format!(
            "-e 'plot \"{}\" using 1:2 with points, \"{}\" using 1:2 with lines'",
            raw_filename, fit_filename
        ),
        "-e 'pause -1 \"Press [q] to quit gnuplot.\n\"'",
    ]
    .join(" ");

    let mut output = process::Command::new("sh")
        .arg("-c")
        .arg(gplot)
        .spawn()
        .expect("Failed to execute gnuplot.");

    output.wait().expect("Failed to wait on gnuplot");
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Generate(args) => handle_generate(args),
        Commands::Fit(args) => handle_fit(args),
    };

    match result {
        Ok(()) => {}
        Err(e) => println!("Error: {}", e.to_string()),
    }
}
