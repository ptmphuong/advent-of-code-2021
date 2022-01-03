use crate::utils::{save_lines_to_vec_string, string_to_int_vec};
use std::{path::Path, collections::HashMap};
use anyhow::Result;

const CYCLE1 : i32 = 80;
// const CYCLE2 : usize = 256;
const ADULT_NEW_AGE : i32 = 6;
const NEW_BORN_AGE : i32 = 8;


/// hashmap for the cycle: age => how many new fish in the cycle
/// age from 0 -> 8

pub fn result_part1<P>(filename: P) -> Result<i32> where P: AsRef<Path>, {
    let input_vec = save_lines_to_vec_string(filename)?;
    let mut num_vec = string_to_int_vec(input_vec[0].to_string());
    let mut new_fish = 0;

    for _ in 0..CYCLE1 {
        for i in 0..num_vec.len() {
            if num_vec[i] >= 1 {
                num_vec[i] -= 1;
            } else {
                // case num_vec[i] = 0
                num_vec[i] = ADULT_NEW_AGE;
                new_fish += 1;
            }
        }

        while new_fish != 0 {
            num_vec.push(NEW_BORN_AGE);
            new_fish -= 1;
        }
    }

    Ok(num_vec.len() as i32)
}

pub fn count_dp<P>(filename: P) -> Result<i32> where P: AsRef<Path>, {
    let input_vec = save_lines_to_vec_string(filename)?;
    let num_vec = string_to_int_vec(input_vec[0].to_string());
    let map = make_reproduction_hashmap(NEW_BORN_AGE, CYCLE1);
    let mut res = 0;
    for age in num_vec {
        res += map.get(&age).unwrap();
    }
    // let res = how_many_more_fish(3, 5);
    Ok(res)
}

fn hash_key(age: i32, cycle: i32) -> i32 {
    cycle*10 + age
}

fn how_many_more_fish(age: i32, cycle: i32) -> i32 {
    let can_reproduce = age < cycle;
    if can_reproduce {
        let remaining = cycle - age - 1; // including current state
        return how_many_more_fish(6, remaining) + how_many_more_fish(8, remaining);
    } else {
        return 1;
    }
}

fn how_many_more_fish_dp(age: i32, cycle: i32, map: &mut HashMap<i32, i32>) -> i32 {
    let can_reproduce = age < cycle;

    if !can_reproduce {
        return 1;
    } else {
        let key = hash_key(age, cycle);
        let res = match map.get(&key) {
            Some(v) => *v,
            None => {
                let remaining_cycle = cycle - age - 1;
                how_many_more_fish_dp(ADULT_NEW_AGE, remaining_cycle, map) + how_many_more_fish_dp(NEW_BORN_AGE, remaining_cycle, map)
            }
        };

        map.insert(key, res);
        return res;
    }
}

// key: age from 0..max age
// value: how many more fish
fn make_reproduction_hashmap(max_age: i32, cycle: i32) -> HashMap<i32, i32> {
    let mut reproduction_map = HashMap::new();
    let mut dp_map = HashMap::new();
    for age in 0..max_age+1 {
        reproduction_map.insert(age, how_many_more_fish_dp(age, cycle, &mut dp_map));
    }
    reproduction_map
}
