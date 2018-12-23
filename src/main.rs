extern crate pest;
#[macro_use]
extern crate pest_derive;

use image::{DynamicImage, GenericImage, Pixel};
use ndarray::{Array, Array2};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

mod claim;
use crate::claim::Claim;

fn main() -> Result<(), String> {
    let claims = read_claims_from_stdin()?;
    let width = claims.iter().map(Claim::max_x).max().unwrap_or(0) + 1;
    let height = claims.iter().map(Claim::max_y).max().unwrap_or(0) + 1;

    if claims.is_empty() {
        println!("Found no claims in stdin.");
        return Ok(());
    }

    println!(
        "Found {} claims that requires a sheet of size {}×{} inches",
        claims.len(),
        width,
        height
    );
    // NOTE: Array2 uses (y, x), not (x, y)
    let mut sheet = Array::from_elem((height, width), 0u32);
    let mut overlapping_sqr_inches = 0;

    for claim in claims {
        overlapping_sqr_inches += process_claim(&claim, &mut sheet)?;
    }

    println!(
        "Result: {} square inch(es) are overlapping other claims.",
        overlapping_sqr_inches
    );
    println!(
        "Bonus: {} square inch(es) are left unclaimed",
        sheet.iter().filter(|x| **x == 0).count(),
    );

    generate_heatmap(sheet, "heatmap.png")?;
    println!("Heatmap generated and saved to heatmap.png");

    Ok(())
}

fn read_claims_from_stdin() -> Result<Vec<Claim>, String> {
    // Read plans from stdin (streaming)
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);

    reader
        .lines()
        .map(|line| match line {
            Ok(line) => Claim::parse(&line),
            Err(error) => Err(format!("Line is not valid UTF-8. {}", error)),
        })
        .collect()
}

fn process_claim(claim: &Claim, sheet: &mut Array2<u32>) -> Result<usize, String> {
    let mut overlapping = 0;

    debug_assert!(
        claim.max_x() < sheet.cols(),
        "Max x is {}, but max is {}",
        claim.max_x(),
        sheet.cols() - 1
    );
    assert!(claim.max_y() < sheet.rows());

    for y in claim.y_range() {
        for x in claim.x_range() {
            // NOTE: Array2 uses (y, x), not (x, y)
            let index = [y, x];

            if sheet[index] > 0 {
                overlapping += 1;
            }
            sheet[index] += 1;
        }
    }

    Ok(overlapping)
}

/// Generate a heatmap from a sheet.
fn generate_heatmap(sheet: Array2<u32>, filename: impl AsRef<Path>) -> Result<(), String> {
    // Normalize the sheet by calculating the max number of overlaps. Then the max overlaps will
    // have the strongest value in the heatmap.
    let max_overlaps = sheet.iter().map(|v| *v).max().unwrap_or(0) as f64;

    let mut image = DynamicImage::new_rgb8(sheet.cols() as u32, sheet.rows() as u32);

    for (y, row) in sheet.outer_iter().enumerate() {
        for (x, overlaps) in row.iter().enumerate() {
            image.put_pixel(
                x as u32,
                y as u32,
                heatmap_color(*overlaps as f64, max_overlaps).to_rgba(),
            );
        }
    }

    image
        .save(filename)
        .map_err(|err| format!("Failed to write heatmap: {}", err))
}

fn heatmap_color(current: f64, max: f64) -> image::Rgb<u8> {
    // Unused squares should be white, and then get redder and redder the more overlaps it had.
    let scale = ((current / max) * 255.0).round() as u8;
    image::Rgb([255, 255 - scale, 255 - scale])
}
