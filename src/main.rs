use std::fs::File;
use std::io::{stdout, BufWriter, Write};
use std::f64::consts::PI;
use std::f64::consts::FRAC_PI_2;

#[allow(unused_variables, unused_mut)]
fn main() -> Result<(), Box<dyn std::error::Error>>{

    let lambda = 1.03;
    let lattice_constant = 0.7;
    let radius = 1.0e6;

    let num_grids = 8;
    let theta_resolution = 0.001;
    let time_resolution = 0.001;
    let grid_resolution = 0.001;
    let grid_half_width = 100;
    let period = 1_00;

    let num_phis = 20;

    for i in 0..num_phis {
        let outputname = &format!("./output{}.dat", i);
        let mut f = BufWriter::new(File::create(outputname)?);

        let phi = FRAC_PI_2 * (i as f64 / num_phis as f64);
        println!("processing phi = {:01.4}", phi);

        let theta_min = -i as f64 / num_phis as f64 - 1.0;
        let theta_max = -i as f64 / num_phis as f64 + 1.0;

        let mut theta_ = theta_min;

        let out = stdout();
        let mut out = BufWriter::new(out.lock());

        while theta_ < theta_max {
            // write!(out, "{:>03.2}% done \r", 100.0 * (theta_ - theta_min) / (theta_max - theta_min))?;
            // out.flush()?;
            let theta = theta_ * FRAC_PI_2;
            let screen = (radius * theta.cos(), radius * theta.sin());

            let intensity = (0..period).map(|j| {
                (j as f64 / period as f64) * time_resolution
            }).map(|t| {
                (-num_grids..=num_grids).flat_map(|k| {
                    (-grid_half_width..=grid_half_width).map(|l| {
                        (
                            (- k as f64 * lattice_constant + l as f64 * grid_resolution) * phi.sin(),
                            (- k as f64 * lattice_constant + l as f64 * grid_resolution) * phi.cos(),
                        )
                    }).collect::<Vec<(f64, f64)>>()
                }).map(|grid| {
                    t + (grid.0 + distance(screen, grid)) / lambda
                }).map(|p| (p * 2.0 * PI).sin()).sum::<f64>()
            }).map(|e| e * e).sum::<f64>() / period as f64 / ((2 * num_grids + 1) * (2 * num_grids + 1) * (2 * grid_half_width + 1) * (2 * grid_half_width + 1)) as f64;

            writeln!(f, "{} {}", theta, intensity)?;

            theta_ += theta_resolution;
        }
    }

    Ok(())
}

fn distance(p1:(f64, f64), p2:(f64, f64)) -> f64 {
    ((p1.0 - p2.0) * (p1.0 - p2.0) + (p1.1 - p2.1) * (p1.1 - p2.1)).sqrt()
}