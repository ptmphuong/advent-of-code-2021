use crate::utils::save_lines_to_vec_i32;
use std::path::Path;
use anyhow::Result;

pub fn result_part1<P>(filename: P) -> Result<i32> where P: AsRef<Path>, {
    let mut num = 0;
    let input_vec = save_lines_to_vec_i32(filename)?;
    for i in 1..input_vec.len() {
        if input_vec.get(i-1) < input_vec.get(i) {
            num+=1;
        }
    }
    return Ok(num);
}

pub fn result_part2<P>(filename: P) -> Result<i32> where P: AsRef<Path>, {
    let mut num = 0;
    let input_vec = save_lines_to_vec_i32(filename)?;
    for i in 0..input_vec.len() - 3 {
        let sum1 = input_vec[i] + input_vec[i+1] + input_vec[i+2];
        let sum2 = input_vec[i+1] + input_vec[i+2] + input_vec[i+3];
        if sum1 < sum2 {
            num+=1;
        }
    }
    return Ok(num);
}
