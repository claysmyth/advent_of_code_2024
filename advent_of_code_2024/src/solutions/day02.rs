use polars::prelude::*;
use polars::lazy::dsl::col;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

const INPUT_PATH: &str = "src/inputs/day02.txt";
const OUTPUT_PATH: &str = "src/inputs/day02.csv";

fn check_criteria(nums: Vec<i32>) -> bool {
    let diffs: Vec<i32> = nums.windows(2)
        .map(|w| w[1] - w[0])
        .collect();
    
    // Check if all differences are same sign (all positive or all negative)
    let all_positive = diffs.iter().all(|&x| x > 0);
    let all_negative = diffs.iter().all(|&x| x < 0);
            
    // Get min and max absolute differences
    let abs_diffs: Vec<i32> = diffs.iter().map(|x| x.abs()).collect();
    let max_abs_diff = abs_diffs.iter().max().unwrap();
    let min_abs_diff = abs_diffs.iter().min().unwrap();
    
    if (all_positive || all_negative) && 
        *max_abs_diff <= 3 && 
        *min_abs_diff >= 1 {
        true
    } else {
        false
    }
}


fn convert_to_csv(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let mut output = File::create(output_path)?;

    // Get all lines at once
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    
    // Get maximum number of columns from all lines
    let num_cols = lines.iter()
        .map(|line| line.split_whitespace().count())
        .max()
        .unwrap_or(0);
    
    // Write header row
    let header: Vec<String> = (1..=num_cols).map(|i| format!("col{}", i)).collect();
    writeln!(output, "{}", header.join(","))?;

    let mut i = 0;
    let mut index = 0;
    let mut index_vec: Vec<i32> = Vec::new();
    // Process all lines
    for line in lines {
        let mut numbers: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        // Convert strings to numbers and get differences
        let nums: Vec<i32> = numbers.iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        
        if check_criteria(nums.clone()) {
            i += 1;
            index_vec.push(index);
        } else {
            // Check each possible sequence with one number removed
            for skip_idx in 0..nums.len() {
                let modified_nums: Vec<i32> = nums.iter()
                    .enumerate()
                    .filter(|&(i, _)| i != skip_idx)
                    .map(|(_, &x)| x)
                    .collect();
            
                if check_criteria(modified_nums) {
                    i += 1;
                    index_vec.push(index);
                    break; // Break after finding first valid sequence
                }
            }
        }
        
        // Pad with empty strings if needed
        while numbers.len() < num_cols {
            numbers.push("".to_string());
        }
        writeln!(output, "{}", numbers.join(","))?;
        index += 1;
    }

    println!("{}", i);
    println!("{:?}", index_vec);

    Ok(())
}

pub fn part_1() -> Result<i64, PolarsError> {
    // Check if CSV file exists and create it if it doesn't
    convert_to_csv(INPUT_PATH, OUTPUT_PATH).expect("Failed to convert file to CSV");

    // Below is unnecessary but solves part 1. Csv convert solves part 2.

    // read from path
    let file = std::fs::File::open(OUTPUT_PATH)?;
    let df_raw = CsvReader::new(file).finish()?;

    println!("{:?}", df_raw);

    let df_diff = df_raw
    .lazy()
    .with_row_index("index", Some(0))
    .select([
        col("index"),
        (col("col2") - col("col1")).alias("diff1"),
        (col("col3") - col("col2")).alias("diff2"), 
        (col("col4") - col("col3")).alias("diff3"),
        (col("col5") - col("col4")).alias("diff4"),
        (col("col6") - col("col5")).alias("diff5"),
        (col("col7") - col("col6")).alias("diff6"),
        (col("col8") - col("col7")).alias("diff7"),
    ]).collect()?;

    println!("{:?}", df_diff);

    let df_condition_1 = df_diff
        .clone()
        .lazy()
        .filter(
            (col("diff1").gt(0)
                .and(col("diff2").gt(0))
                .and(col("diff3").gt(0))
                .and(col("diff4").gt(0))
                .and(col("diff5").gt(0).or(col("diff5").is_null()))
                .and(col("diff6").gt(0).or(col("diff6").is_null()))
                .and(col("diff7").gt(0).or(col("diff7").is_null()))
            ).or(
                col("diff1").lt(0)
                    .and(col("diff2").lt(0))
                    .and(col("diff3").lt(0))
                    .and(col("diff4").lt(0))
                    .and(col("diff5").lt(0).or(col("diff5").is_null()))
                    .and(col("diff6").lt(0).or(col("diff6").is_null()))
                    .and(col("diff7").lt(0).or(col("diff7").is_null()))
            )
        );

    let df_condition_2 = df_diff
        .clone()
        .lazy()
        .filter(
            col("diff1").abs().gt_eq(1)
                .and(col("diff1").abs().lt_eq(3))
                .and(col("diff2").abs().gt_eq(1))
                .and(col("diff2").abs().lt_eq(3))
                .and(col("diff3").abs().gt_eq(1))
                .and(col("diff3").abs().lt_eq(3))
                .and(col("diff4").abs().gt_eq(1))
                .and(col("diff4").abs().lt_eq(3))
                .and(col("diff5").abs().gt_eq(1).or(col("diff5").is_null()))
                .and(col("diff5").abs().lt_eq(3).or(col("diff5").is_null()))
                .and(col("diff6").abs().gt_eq(1).or(col("diff6").is_null()))
                .and(col("diff6").abs().lt_eq(3).or(col("diff6").is_null()))
                .and(col("diff7").abs().gt_eq(1).or(col("diff7").is_null()))
                .and(col("diff7").abs().lt_eq(3).or(col("diff7").is_null()))
        );

    let df_result = df_condition_1.inner_join(df_condition_2, "index", "index").collect()?;

    println!("{:?}", df_result);

    let result = df_result.height();

    println!("{}", result);

    Ok(result.try_into().unwrap())
}