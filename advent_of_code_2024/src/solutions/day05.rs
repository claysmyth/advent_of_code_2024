use regex::bytes::Regex;
use std::collections::HashMap;
use itertools::Itertools;

fn part_1_body() -> Result<i64, polars::prelude::PolarsError> {
    let re = Regex::new(r"(\d{1,3})\|(\d{1,3})").unwrap();
    let mut ordering: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut sum: i32 = 0;
    
    for line in include_bytes!("../inputs/day05.txt").split(|&c| c == b'\n') {
        if re.is_match(line) {
            let caps = re.captures(line).unwrap();
            let a = atoi::atoi::<usize>(caps.get(1).unwrap().as_bytes()).unwrap();
            let b = atoi::atoi::<usize>(caps.get(2).unwrap().as_bytes()).unwrap();
            ordering.entry(a as i32).or_insert(vec![]).push(b as i32);
        } else if line.len() > 0 {
            let nums: Vec<_> = line.split(|&c| c == b',')
                .map(|x| atoi::atoi::<usize>(x).unwrap() as i32)
                .collect();

            let mut passing: i32 = 0;
            for i in 0..(nums.len()-1) {
                if nums[i+1..].iter().all(|&x| ordering.get(&nums[i]).unwrap().contains(&x)) {
                    passing += 1;
                }
            } 
            if passing == (nums.len()-1) as i32 {
                sum += nums[(nums.len()-1)/2];
            }
        }
    }
    Ok(sum as i64)
}

pub fn part_1() -> Result<i64, polars::prelude::PolarsError> {
    part_1_body().map_err(|e| polars::prelude::PolarsError::ComputeError(e.to_string().into()))
}

fn check_ordering(nums: &Vec<i32>, ordering: &HashMap<i32, Vec<i32>>) -> bool {
    for i in 0..(nums.len()-1) {
        if !ordering.get(&nums[i]).unwrap().contains(&nums[i+1]) {
            return false;
        }
    }
    true
}

fn part_2_body() -> Result<i64, polars::prelude::PolarsError> {
    let re = Regex::new(r"(\d{1,3})\|(\d{1,3})").unwrap();
    let mut ordering: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut sum: i32 = 0;
    
    for line in include_bytes!("../inputs/day05.txt").split(|&c| c == b'\n') {
        if re.is_match(line) {
            let caps = re.captures(line).unwrap();
            let a = atoi::atoi::<usize>(caps.get(1).unwrap().as_bytes()).unwrap();
            let b = atoi::atoi::<usize>(caps.get(2).unwrap().as_bytes()).unwrap();
            ordering.entry(a as i32).or_insert(vec![]).push(b as i32);
        } else if line.len() > 0 {
            let mut nums: Vec<_> = line.split(|&c| c == b',')
                .map(|x| atoi::atoi::<usize>(x).unwrap() as i32)
                .collect();

            let mut passing: i32 = 0;
            for i in 0..(nums.len()-1) {
                if nums[i+1..].iter().all(|&x| ordering.get(&nums[i]).unwrap().contains(&x)) {
                    passing += 1;
                }
            }
            if passing < (nums.len()-1) as i32 {
                // for i in 0..(nums.len()-1) {
                //     let mut j = i+1;
                //     while !ordering.get(&nums[i]).unwrap().contains(&nums[j]) {
                //         nums.swap(i, j);
                //         j += 1;
                //     }
                // }
                while !check_ordering(&nums, &ordering) {
                    for i in 0..(nums.len()-1) {
                        let mut j = i+1;
                        while !ordering.get(&nums[i]).unwrap().contains(&nums[j]) {
                            nums.swap(i, j);
                            j += 1;
                            if j >= nums.len() {
                                break;
                            }
                        }
                    }
                }
                sum += nums[(nums.len()-1)/2];
            }
        }
    }
    Ok(sum as i64)
}

pub fn part_2() -> Result<i64, polars::prelude::PolarsError> {
    part_2_body().map_err(|e| polars::prelude::PolarsError::ComputeError(e.to_string().into()))
}