use ndarray::Array2;
use ndarray::s;

fn check_loop(mut matrix: &mut Array2<i32>) -> i32 {
    let mut keep_going = true;
    let mut curr_position: Vec<(usize, usize)> = matrix
        .indexed_iter()
        .filter(|((row, col), value)| **value == 3)
        .map(|((row, col), _)| (row, col))
        .collect();
    let mut front_pos = (curr_position[0].0 - (1 as usize), curr_position[0].1);
    let mut count = 0;
    while keep_going {
        while matrix[front_pos] >= 0 {
            matrix[(curr_position[0].0, curr_position[0].1)] = 1;
            if front_pos.0 == 0 {
                return 0;
            }
            curr_position = vec![(curr_position[0].0 - (1 as usize), curr_position[0].1)];
            front_pos = (front_pos.0 - (1 as usize), front_pos.1);
            count += 1;
            if count > 16900 { // Max number for loop to be considered found
                return 1;
            }
        }
        // Temporarily set curr_position to something identifiable
        matrix[(curr_position[0].0, curr_position[0].1)] = -3;
        // Rotate entire matrix to the left
        *matrix = matrix.slice(s![.., ..;-1]).permuted_axes([1, 0]).to_owned();

        curr_position = matrix
            .indexed_iter()
            .filter(|((row, col), value)| **value == -3)
            .map(|((row, col), _)| (row, col))
            .collect();
        front_pos = (curr_position[0].0 - (1 as usize), curr_position[0].1);
        // Reset curr_position to its original value
        matrix[(curr_position[0].0, curr_position[0].1)] = 1;

    }
    0
}

fn main() {
    let mut input_mat: Vec<Vec<i32>> = Vec::new();
    for line in include_bytes!("../input.txt").split(|&c| c == b'\n') {
        input_mat.push(
            line.iter()
                .map(|&x| {
                    if x == b'.' {
                        0
                    } else if x == b'#' {
                        -1
                    } else {
                        3
                    }
                })
                .collect()
        );
    }

    let rows = input_mat.len();
    let cols = input_mat[0].len();
    let flat_vec: Vec<i32> = input_mat.into_iter().flatten().collect();
    let mut input_mat = Array2::from_shape_vec((rows, cols), flat_vec).unwrap();

    let mut loop_total = 0;
    for i in 0..rows {
        for j in 0..cols {
            if input_mat[(i,j)] == 0 {
                let mut matrix = input_mat.clone();
                matrix[(i,j)] = -1;
                loop_total += check_loop(&mut matrix);
            }
        }
    }
    println!("Loop Total: {}", loop_total);
}
