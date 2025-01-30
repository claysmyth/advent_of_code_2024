use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn part_1_body() -> Result<i64, std::io::Error> {
    let file = File::open("src/inputs/day03.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;

    let re = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)$").unwrap();
    let max_window_size = 12;
    let min_window_size = 8;

    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        
        // Look at each position and check window
        for i in 0..chars.len() {
            let mut curr_window_size = max_window_size;
            while i + curr_window_size > chars.len() {
                curr_window_size -= 1;
            }

            while curr_window_size >= min_window_size {
                let window: String = chars[i..i+curr_window_size].iter().collect();
                // Check if matches pattern mul(X,Y)
                if re.is_match(&window) {
                    println!("Window: {}", window);
                    let nums: Vec<&str> = re.captures(&window)
                        .unwrap()
                        .iter()
                        .skip(1)
                        .map(|m| m.unwrap().as_str())
                    .collect();
                
                    if nums.len() == 2 {
                        if let Some(x_str) = nums[0].trim().parse::<i64>().ok() {
                            if let Some(y_str) = nums[1].trim_end_matches(')').trim().parse::<i64>().ok() {
                                    sum += x_str * y_str;
                            }
                        }
                    }
                }
                
                curr_window_size -= 1;
            }
        }
    }

    Ok(sum)
}

pub fn part_1() -> Result<i64, polars::prelude::PolarsError> {
    part_1_body().map_err(|e| polars::prelude::PolarsError::ComputeError(e.to_string().into()))
}


fn part_2_body() -> Result<i64, std::io::Error> {
    let file = File::open("src/inputs/day03.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;

    let re = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)$").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let max_window_size = 12;
    let min_window_size = 8;
    let mut dont_count = 0;
    let mut sum_bool = true;

    for line in reader.lines() {
        let line = line?;
        let chars: Vec<char> = line.chars().collect();
        
        // Look at each position and check window
        for i in 0..chars.len() {

            if i + 7 < chars.len() {
                let window_dont: String = chars[i..i+7].iter().collect();
                let window_do: String = chars[i..i+4].iter().collect();
                if re_dont.is_match(&window_dont) {
                    println!("Window: {}", window_dont);
                    sum_bool = false;
                    dont_count += 1;
                    println!("Dont count: {}", dont_count);
                } else if re_do.is_match(&window_do) {
                    println!("Window: {}", window_do);
                    sum_bool = true;
                }
            }

            if sum_bool {
                let mut curr_window_size = max_window_size;
                while i + curr_window_size > chars.len() {
                    curr_window_size -= 1;
                }

                while curr_window_size >= min_window_size {
                    let window: String = chars[i..i+curr_window_size].iter().collect();
                    // Check if matches pattern mul(X,Y)
                    if re.is_match(&window) {
                        println!("Window: {}", window);
                        let nums: Vec<&str> = re.captures(&window)
                            .unwrap()
                            .iter()
                            .skip(1)
                            .map(|m| m.unwrap().as_str())
                        .collect();
                    
                        if nums.len() == 2 {
                            if let Some(x_str) = nums[0].trim().parse::<i64>().ok() {
                                if let Some(y_str) = nums[1].trim_end_matches(')').trim().parse::<i64>().ok() {
                                        sum += x_str * y_str;
                                }
                            }
                        }
                    }
                    
                    curr_window_size -= 1;
                }
            }
        }
    }

    Ok(sum)
}

pub fn part_2() -> Result<i64, polars::prelude::PolarsError> {
    part_2_body().map_err(|e| polars::prelude::PolarsError::ComputeError(e.to_string().into()))
}