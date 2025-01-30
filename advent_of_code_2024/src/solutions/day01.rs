use polars::prelude::*;
use polars::lazy::dsl::col;  // Add this line
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

const INPUT_PATH: &str = "src/inputs/day01.txt";
const OUTPUT_PATH: &str = "src/inputs/day01.csv";


fn convert_to_csv(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut output = File::create(output_path)?;

    for line in reader.lines() {
        let line = line?;
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() == 2 {
            writeln!(output, "{},{}", numbers[0], numbers[1])?;
        }
    }

    Ok(())
}


pub fn part_1() -> Result<i64, PolarsError> {
    // Check if CSV file exists and create it if it doesn't
    if !std::path::Path::new(OUTPUT_PATH).exists() {
        convert_to_csv(INPUT_PATH, OUTPUT_PATH).expect("Failed to convert file to CSV");
    }

    // read from path
    let file = std::fs::File::open(OUTPUT_PATH)?;
    let df_raw = CsvReader::new(file).finish()?;

    println!("{:?}", df_raw);

    let df_result = df_raw
        .lazy()
        .select([
            col("col1").sort(SortOptions::default()).alias("sorted_col1"),
            col("col2").sort(SortOptions::default()).alias("sorted_col2")
        ])
        .select([
            (col("sorted_col1") - col("sorted_col2")).abs().sum().alias("result")
        ])
        .collect()?;

    // Get the actual value from the DataFrame
    let result = df_result.get_columns()[0].get(0).unwrap().try_extract::<i64>().unwrap();
    
    Ok(result)
}


pub fn part_2() -> Result<i64, PolarsError> {
    let file = std::fs::File::open(OUTPUT_PATH)?;
    let df_raw = CsvReader::new(file).finish()?;

    let col_1_counts = df_raw
    .clone()
    .lazy()
    .select([
        col("col1").value_counts(true, true, "counts_1", false),
    ])
    .unnest(["col1"])
    .collect()?;

    println!("{:?}", col_1_counts);

    let col_2_counts = df_raw
    .clone()
    .lazy()
    .select([
        col("col2").value_counts(true, true, "counts_2", false),
    ])
    .unnest(["col2"])
    .collect()?;

    println!("{:?}", col_2_counts);

    let df_tmp = col_1_counts.inner_join(&col_2_counts, ["col1"], ["col2"])?;

    println!("{:?}", df_tmp);

    let df_result = df_tmp
    .lazy()
    .select([
        (col("col1") * col("counts_1") * col("counts_2")).sum().alias("result")
    ])
    .collect()?;

    println!("{:?}", df_result);

    // Convert AnyValue to i64
    let result = df_result.get_columns()[0].get(0).unwrap().try_extract::<i64>().unwrap();
    
    Ok(result)

}
