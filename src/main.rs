mod algorithms;
mod utils;

use std::{collections::HashMap, time::Instant};

use algorithms::bubble_sort::sort as bubble_sort;
use utils::test_algorithm;

const OUTPUT_PATH: &str = "output.csv";

fn main() {
    let mut output = csv::Writer::from_path(OUTPUT_PATH).expect("Failed to create file");

    output
        .write_record(&["n", "Time Elapsed", "Sorting Algorithm"])
        .expect("Failed to write to file");

    let mut algorithms: HashMap<&str, fn(array: &mut Vec<i32>)> = HashMap::new();

    algorithms.insert("Bubble_Sort", bubble_sort);

    let mut n = 10;
    while n <= 100_000_000 {
        for (name, algorithm) in &algorithms {
            let start = Instant::now();
            test_algorithm(*algorithm, n);
            let time_elaped = start.elapsed();

            output
                .write_record(&[
                    n.to_string(),
                    time_elaped.as_millis().to_string(),
                    name.to_string(),
                ])
                .expect("Failed to write to file");
        }

        println!("{n}");
        n *= 10;
    }
}
