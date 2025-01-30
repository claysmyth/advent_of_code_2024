use ndarray::Array2;
use ndarray::s;

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
                        1
                    }
                })
                .collect()
        );
    }
    let rows = input_mat.len();
    let cols = input_mat[0].len();
    let flat_vec: Vec<i32> = input_mat.into_iter().flatten().collect();
    let mut input_mat = Array2::from_shape_vec((rows, cols), flat_vec).unwrap();
    println!("Input matrix:");
    println!("{:?}", input_mat);

    let mut keep_going = true;
    let mut curr_position: Vec<(usize, usize)> = input_mat
        .indexed_iter()
        .filter(|((row, col), value)| **value == 1)
        .map(|((row, col), _)| (row, col))
        .collect();
    let mut front_pos = (curr_position[0].0 - (1 as usize), curr_position[0].1);

    while keep_going {
        while input_mat[front_pos] >= 0 {
            input_mat[(curr_position[0].0, curr_position[0].1)] = 1;
            if front_pos.0 == 0 {
                println!("Sum: {:?}", input_mat.indexed_iter()
                .filter(|((row, col), value)| **value == 1)
                .map(|(_, value)| value)
                .sum::<i32>() + 1);
                keep_going = false;
                break;
            }
            curr_position = vec![(curr_position[0].0 - (1 as usize), curr_position[0].1)];
            front_pos = (front_pos.0 - (1 as usize), front_pos.1);

        }
        // Temporarily set curr_position to something identifiable
        input_mat[(curr_position[0].0, curr_position[0].1)] = 3;
        // Rotate entire matrix to the left
        input_mat = input_mat.slice(s![.., ..;-1]).permuted_axes([1, 0]).to_owned();

        curr_position = input_mat
            .indexed_iter()
            .filter(|((row, col), value)| **value == 3)
            .map(|((row, col), _)| (row, col))
            .collect();
        front_pos = (curr_position[0].0 - (1 as usize), curr_position[0].1);
        // Reset curr_position to its original value
        input_mat[(curr_position[0].0, curr_position[0].1)] = 1;

    }
}
