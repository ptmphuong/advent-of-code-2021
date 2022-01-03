use std::fs::File;
use std::io::{self, BufRead};
use anyhow::Result;
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn save_lines_to_vec_i32<P>(filename: P) -> Result<Vec<i32>> 
where P: AsRef<Path>, {
    let mut res = vec![];
    let lines = read_lines(filename)?;
    for line in lines {
        let num = line?.parse::<i32>().unwrap();
        res.push(num);
    }
    Ok(res)
}

pub fn save_lines_to_vec_string<P>(filename: P) -> Result<Vec<String>> 
where P: AsRef<Path>, {
    let mut res = vec![];
    let lines = read_lines(filename)?;
    for line in lines {
        res.push(line?);
    }
    Ok(res)
}
