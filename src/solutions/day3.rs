use crate::utils::save_lines_to_vec_string;
use std::path::Path;
use anyhow::Result;
use std::collections::HashMap;

fn flip_bin_string(s: String) -> String {
    let mut res = String::from("");
    let mut split = s.chars();
    for _ in 0..s.len() {
        let c = split.nth(0);
        if c.eq(&Some('1')) {
            res.push('0');
        } else {
            res.push('1');
        }
    }
    res
}

fn create_hashmap(len: usize) -> HashMap<usize, i32> {
    // key = index * 2 + val
    // val is 0 or 1
    let mut map = HashMap::new();
    for i in 0..len {
        let k1 = i * 2;
        let k2 = i * 2 + 1;
        map.insert(k1, 0);
        map.insert(k2, 0);
    }
    map
}

fn construct_bin_string(map: HashMap<usize, i32>, len: usize) -> String {
    let mut s = String::from("");
    for i in 0..len {
        let key0 = i * 2;
        let key1 = i * 2 + 1;
        let total0 = *map.get(&key0).unwrap();
        let total1 = *map.get(&key1).unwrap();
        if total1 >= total0 {
            s.push('1');
        } else {
            s.push('0');
        }
    }
    s
}

// 00000, 09100, 01100
fn get_gamma_bin(input_vec: &Vec<String>) -> Result<String> {
    let len = input_vec[0].len();
    let mut map = create_hashmap(len);
    for line in input_vec {
        let mut chars = line.chars();
        for i in 0..len {
            let num_char = chars.nth(0).unwrap();
            let mut key = i * 2;
            if num_char.eq(&'1') {
                key += 1;
            }
            map.insert(key, map.get(&key).unwrap() + 1);
        }
    }
    Ok(construct_bin_string(map, len))
}

fn get_epsilon_bin(gamma_string: String) -> String {
    flip_bin_string(gamma_string)
}

fn bin_string_to_int(binary_string: &str) -> Result<i32> {
    let num = isize::from_str_radix(binary_string.as_ref(), 2)?;
    Ok(num as i32)
}

pub fn result_part1<P>(filename: P) -> Result<i32> where P: AsRef<Path>, {
    let input_vec = save_lines_to_vec_string(filename)?;
    let gamma_binary = get_gamma_bin(&input_vec)?;
    let epsilon_binary = get_epsilon_bin(gamma_binary.clone());
    let gamma = bin_string_to_int(&gamma_binary)?;
    let epsilon = bin_string_to_int(&epsilon_binary)?;
    Ok(gamma*epsilon)
}

pub fn find_cut_index(input_vec: &Vec<String>) -> usize {
    for i in 0..input_vec.len() {
        if input_vec[i].chars().nth(0).unwrap().eq(&'1') {
            return i;
        }
    }
    input_vec.len()
}

fn find_oxygen_generator_rating(input_vec_ptr: &Vec<String>) -> String {
    let mut input_vec = input_vec_ptr.clone();
    let mut res_string1 = String::from("");

    while input_vec.len() > 1 {
        input_vec.sort();
        let cut = find_cut_index(&input_vec);
        if cut > input_vec.len()/2 || cut == input_vec.len() {
            input_vec = Vec::from_iter(input_vec[0..cut].iter().cloned());
            res_string1.push('0');
        } else {
            input_vec = Vec::from_iter(input_vec[cut..].iter().cloned());
            res_string1.push('1');
        }

        for i in 0..input_vec.len() {
            let w = input_vec[i].clone();
            let trimmed = &w[1..];
            input_vec[i] = trimmed.to_string();
        }
    }

    if input_vec.len() == 1 {
        let tail = input_vec[0].clone();
        res_string1 = res_string1 + &tail;
    }

    res_string1
}

fn find_co2_scrubber_rating (input_vec_ptr: &Vec<String>) -> String {
    let mut input_vec = input_vec_ptr.clone();
    let mut res_string1 = String::from("");

    while input_vec.len() > 1 {
        input_vec.sort();
//        println!("input_vec: {:?}", &input_vec);
        let cut = find_cut_index(&input_vec);
        if cut > input_vec.len()/2 { // more 0
            input_vec = Vec::from_iter(input_vec[cut..].iter().cloned());
            res_string1.push('1');
        } else {
            input_vec = Vec::from_iter(input_vec[0..cut].iter().cloned());
            res_string1.push('0');
        }

        for i in 0..input_vec.len() {
            let w = input_vec[i].clone();
            let trimmed = &w[1..];
            input_vec[i] = trimmed.to_string();
        }
    }

    if input_vec.len() == 1 {
        let tail = input_vec[0].clone();
        res_string1 = res_string1 + &tail;
    }

    res_string1
}
pub fn result_part2<P>(filename: P) -> Result<i32> where P: AsRef<Path>, {
    let input_vec = save_lines_to_vec_string(filename)?;
    let string1 = find_oxygen_generator_rating(&input_vec);
    let string2 = find_co2_scrubber_rating(&input_vec);
    let num1 = bin_string_to_int(&string1)?;
    let num2 = bin_string_to_int(&string2)?;
    println!("String 1: {} = {}", string1, num1);
    println!("String 2: {} = {}", string2, num2);
    Ok(num1 * num2)
    /*
    let gamma_binary = get_gamma_bin(&input_vec)?;
    let epsilon_binary = get_epsilon_bin(gamma_binary.clone());
    let first_num = match_til_one(&input_vec, gamma_binary);
    let second_num = match_til_one(&input_vec, epsilon_binary);
    Ok(bin_string_to_int(first_num)? * bin_string_to_int(second_num)?)
*/
    // if index_vec = 1 break
    // find most repeated vals in the index of input_vec/processing_vec
}


