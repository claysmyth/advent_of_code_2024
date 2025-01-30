use std::fs::File;
use std::io::{BufReader, BufRead};
use ndarray::{Array3, s};
use std::collections::HashMap;

// Translate, transpose, and reflect the following will complete all possible patterns.
static TRANSLATE_AND_TRANSPOSE_AND_REFLECT: [&str; 4] = 
    [
        "XMAS",
        "....",
        "....",
        "....",
    ];

// Transpose, reflect, and invert the following will complete all possible patterns.
static TRANSPOSE_AND_REFLECT_AND_INVERT: [&str; 4] = 
    [
        "X...",
        ".M..",
        "..A.",
        "...S",
    ];

fn one_hot_encode_block(block: &[&str]) -> Result<Array3<i64>, String> {
    if block.len() != 4 || block.iter().any(|row| row.len() != 4) {
        return Err("Block must be exactly 4x4".to_string());
    }

    // Pre-allocate a flat vector to store all values
    let mut flat_data = Vec::with_capacity(64); // 4x4x4 = 64 elements
    
    for row in block {
        for ch in row.chars() {
            let encoding = match ch {
                'X' => [1, 0, 0, 0],
                'M' => [0, 1, 0, 0],
                'A' => [0, 0, 1, 0],
                'S' => [0, 0, 0, 1],
                _ => [0, 0, 0, 0]
            };
            flat_data.extend_from_slice(&encoding);
        }
    }
    
    // Reshape the flat data into a 4x4x4 array
    Ok(Array3::from_shape_vec((4, 4, 4), flat_data)
        .map_err(|e| format!("Failed to create ndarray: {}", e))?)
}

fn part_1_body() -> Result<i64, Box<dyn std::error::Error>> {
    let file = File::open("src/inputs/day04.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    let mut xmas_count = 0;
    let ttr_base = one_hot_encode_block(&TRANSLATE_AND_TRANSPOSE_AND_REFLECT)?;
    let tri_base = one_hot_encode_block(&TRANSPOSE_AND_REFLECT_AND_INVERT)?;

    let mut i = 0;
    let debug_int = 1811;

    let mut horz_tracker: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut vert_tracker = vec![0; 4];
    
    // Process 4 lines at a time
    for chunk in lines.windows(4) {
        if chunk.len() != 4 {
            continue;
        }
        
        let block: Vec<&str> = chunk.iter().map(|s| s.as_str()).collect();

        let block: Vec<String> = block.iter()
            .map(|s| s.chars().filter(|c| !c.is_whitespace()).collect())
            .collect();

        // Shift all vectors in horz_tracker to the left
        for vec in horz_tracker.values_mut() {
            vec.rotate_left(1);
        }

        // Remove entries from horz_tracker where all values are 0
        horz_tracker.retain(|_, vec| vec.iter().any(|&x| x != 0));
        
        // Process each 4-character window
        for start_col in 0..chunk[0].len().saturating_sub(3) {
            let window: Vec<&str> = block.iter()
                .map(|line| &line[start_col..start_col + 4])
                .collect();

            vert_tracker.rotate_left(1);

            if let Ok(encoded) = one_hot_encode_block(&window) {
                if i % debug_int == 0 {
                    println!("--------------------------------");
                    println!("Processing block {}", i);
                    println!("Current count: {}", xmas_count);
                    println!("Window: {:?}", window);
                }


                // TTR operations (existing code)
                for y_shift in 0..4 {
                    let mut shifted = Array3::zeros(ttr_base.raw_dim());
                    shifted.slice_mut(s![y_shift.., .., ..])
                        .assign(&ttr_base.slice(s![..(4-y_shift), .., ..]));
                    
                    // Original
                    if check_match(&encoded, &shifted) {
                        if let Some(vec) = horz_tracker.get(&(start_col as i32)) {
                            if vec[y_shift as usize] == 0 {
                                xmas_count += 1;
                                horz_tracker.get_mut(&(start_col as i32)).unwrap()[y_shift as usize] = 1;
                            }
                        } else {
                            let mut new_vec = vec![0; 4];
                            new_vec[y_shift as usize] = 1;
                            horz_tracker.insert(start_col as i32, new_vec);
                            xmas_count += 1;
                        }
                        if i % debug_int == 0 {
                            println!("Match found with y_shift {} {}", y_shift, shifted);
                        }
                    }

                    // Reversed
                    let reversed = shifted.slice(s![.., ..;-1, ..])
                        .to_owned();
                    if check_match(&encoded, &reversed) {
                        if let Some(vec) = horz_tracker.get(&(start_col as i32)) {
                            if vec[y_shift as usize] == 0 {
                                xmas_count += 1;
                                horz_tracker.get_mut(&(start_col as i32)).unwrap()[y_shift as usize] = 1;
                            }
                        } else {
                            let mut new_vec = vec![0; 4];
                            new_vec[y_shift as usize] = 1;
                            horz_tracker.insert(start_col as i32, new_vec);
                            xmas_count += 1;
                        }
                        if i % debug_int == 0 {
                            println!("Match found with reversed y_shift {}", reversed);
                        }
                    }

                    
                    // Transposed
                    let transposed = shifted.permuted_axes([1, 0, 2]);
                    if check_match(&encoded, &transposed) {
                        if vert_tracker[y_shift] == 0 {
                            xmas_count += 1;
                            vert_tracker[y_shift] = 1;
                        }
                        if i % debug_int == 0 {
                            println!("Match found with transposed y_shift {}", transposed);
                        }
                    }
                    
                    // Reflected and transposed
                    let reflected_transposed = reversed.permuted_axes([1, 0, 2]);
                    if check_match(&encoded, &reflected_transposed) {
                        if vert_tracker[y_shift] == 0 {
                            xmas_count += 1;
                            vert_tracker[y_shift] = 1;
                        }
                        if i % debug_int == 0 {
                            println!("Match found with reflected and transposed y_shift {}", reflected_transposed);
                        }
                    }
                }

                // TRI operations (new code)
                // Original
                if check_match(&encoded, &tri_base) {
                    xmas_count += 1;
                    if i % debug_int == 0 {
                        println!("Match found with original tribase {}", tri_base);
                    }
                }

                // Reflected across dim 1
                let reflected = tri_base.slice(s![.., .., ..])
                    .to_owned()
                    .slice(s![.., ..;-1, ..])
                    .to_owned();
                if check_match(&encoded, &reflected) {
                    xmas_count += 1;
                    if i % debug_int == 0 {
                        println!("Match found with reflected tribase {}", reflected);
                    }
                }

                // Inverted across dim 0
                let inverted = tri_base.slice(s![.., .., ..])
                    .to_owned()
                    .slice(s![..;-1, .., ..])
                    .to_owned();
                if check_match(&encoded, &inverted) {
                    xmas_count += 1;
                    if i % debug_int == 0 {
                        println!("Match found with inverted tribase {}", inverted);
                    }
                }

                // Reflected and inverted
                let reflected_inverted = reflected.slice(s![.., .., ..])
                    .to_owned()
                    .slice(s![..;-1, .., ..])
                    .to_owned();
                if check_match(&encoded, &reflected_inverted) {
                    xmas_count += 1;
                    if i % debug_int == 0 {
                        println!("Match found with reflected and inverted tribase {}", reflected_inverted);
                    }
                }

                if i % debug_int == 0 {
                    println!("Count after checking matches: {}", xmas_count);
                }
            }
            i += 1;
        }
    }
    println!("Final count: {}", xmas_count);
    println!("Number of windows: {}", i);
    Ok(xmas_count)
}

// Helper function to check if arrays match (element-wise multiply and sum == 4)
fn check_match(a: &Array3<i64>, b: &Array3<i64>) -> bool {
    (a * b).sum() == 4
}

pub fn part_1() -> Result<i64, polars::prelude::PolarsError> {
    // Frustratingly, solves the example but not the actual input.
    part_1_body().map_err(|e| polars::prelude::PolarsError::ComputeError(e.to_string().into()))
}


// The below code is not mine. Taken from github as a comparison.
fn part_1_comp_body() -> Result<i64, polars::prelude::PolarsError> {
    let mut word = [0; 4];
    let map = include_bytes!("../inputs/day04.txt")
        .split(|&c| c == b'\n')
        .collect::<Vec<_>>();

    println!(
        "{}",
        (0..map[0].len() as isize)
            .flat_map(|x| (0..map.len() as isize).map(move |y| (x, y)))
            .flat_map(|(x, y)| {
                [
                    [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)], // NE
                    [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],             // E
                    [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)], // SE
                    [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],             // S
                ]
            })
            .filter(|coords| {
                let mut iter = coords.iter().map(|(x, y)| {
                    map.get(*y as usize)
                        .and_then(|row| row.get(*x as usize).copied())
                        .unwrap_or_default()
                });
                word.fill_with(|| iter.next().unwrap_or_default());
                &word == b"XMAS" || &word == b"SAMX"
            })
            .count(),
    );
    Ok(0)
}

pub fn part_1_comp() -> Result<i64, polars::prelude::PolarsError> {
    // Frustratingly, solves the example but not the actual input.
    part_1_comp_body().map_err(|e| polars::prelude::PolarsError::ComputeError(e.to_string().into()))
}
