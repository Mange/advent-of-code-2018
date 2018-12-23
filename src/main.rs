extern crate pest;
#[macro_use]
extern crate pest_derive;

use ndarray::{Array, Array2};
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

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
    let mut sheet = Array::from_elem((height, width), false);
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
        sheet.iter().filter(|x| !*x).count(),
    );

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

fn process_claim(claim: &Claim, sheet: &mut Array2<bool>) -> Result<usize, String> {
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

            if sheet[index] {
                overlapping += 1;
            }
            sheet[index] = true;
        }
    }

    Ok(overlapping)
}
