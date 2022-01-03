use crate::utils::save_lines_to_vec_string;
use std::{path::Path, collections::{HashMap, HashSet}};
use anyhow::Result;

const BOARD_SIZE : usize = 25;
const BOARD_DIMENSION : i32 = 5;
const ROW_BASE : i32 = 100;
const COL_BASE : i32 = ROW_BASE + BOARD_DIMENSION;

/// Each board has a hashmap to store all values and drawn values
/// All values are 2 digit numbers => K: number, V: its index
///                                                 num_row = index/5
///                                                 num_col = index%5
/// Store the number of drawn values in each num and col => 
///     K: row i = 100 + i  , V: count of nums drawn in that row
///     K: col i = 100 + i*2, V: count of nums drawn in that col
pub fn result_part1<P>(filename: P) -> Result<i32> where P: AsRef<Path>, {
    // part 1: play until the FIRST board wins.
    play(filename, true)
}

pub fn result_part2<P>(filename: P) -> Result<i32> where P: AsRef<Path>, {
    // part 2: play until the LAST board wins.
    play(filename, false)
}

pub fn play<P>(filename: P, get_win_first: bool) -> Result<i32> where P: AsRef<Path>, {
    let input_vec = save_lines_to_vec_string(filename)?;
    let mut it = input_vec.iter();

    let num_str = it.next().unwrap();
    let drawing_vec = get_num_vec(num_str)?;

    it.next();
    let boards = get_board_chunks(it)?;

    let mut board_map_vec = make_board_maps(&boards);

    let (winning_map_i, winning_draw_i) = draw(&drawing_vec, &mut board_map_vec, get_win_first)?;

    let winning_board = &boards[winning_map_i];
    let winning_map = &board_map_vec[winning_map_i];

    let drawn_sum = get_drawn_sum(&winning_map, &drawing_vec, winning_draw_i)?;
    let map_sum: i32 = winning_board.iter().sum();
    let undrawn_sum = map_sum - drawn_sum;

    let winning_num = drawing_vec[winning_draw_i];

    println!("winning_map: {:?} -- {:?}", winning_map_i, winning_board);
    println!("winning num: {:?}", winning_num);

    Ok(undrawn_sum * winning_num)
}

fn get_num_vec(num_str: &String) -> Result<Vec<i32>> {
    let str_vec = num_str.split(",");
    let mut num_vec = Vec::new();
    for s in str_vec {
        let num = s.parse::<i32>()?;
        num_vec.push(num);
    }
    Ok(num_vec)
}

fn get_board_chunks<'a>(it: impl Iterator<Item = &'a String>) -> Result<Vec<Vec<i32>>> {
    let mut board = Vec::new();

    for line in it {
        // println!("\nline: {:?}", &line);

        if !"".eq(line) {
            let split : Vec<&str> = line.split(" ").collect();
            for s in split {
                if !"".eq(s) {
                    let num = s.trim().parse::<i32>()?;
                    board.push(num)
                }
            }
        }
    }
    let dst: Vec<Vec<i32>> = board.chunks(BOARD_SIZE).map(|s| s.into()).collect();
    Ok(dst)
}

fn make_board_maps(boards: &Vec<Vec<i32>>) -> Vec<HashMap<i32, i32>> {
    let mut map_vec = Vec::new();

    let row_col_key_start = ROW_BASE;
    let row_col_key_end = ROW_BASE + BOARD_DIMENSION*2;

    for board in boards {
        let mut map = HashMap::new();
        for i in 0..board.len() {
            map.insert(board[i], i as i32);
        }
        for i in row_col_key_start..row_col_key_end  {
            map.insert(i, 0);
        }
        map_vec.push(map);
    }

    map_vec
}

fn draw(drawing_vec: &Vec<i32>, board_map: &mut Vec<HashMap<i32, i32>>, til_win_first: bool) -> Result<(usize, usize)> {
    let mut won_set : HashSet<usize> = HashSet::new();
    let num_board = board_map.len();

    for num_i in 0..drawing_vec.len() {
        let num = drawing_vec[num_i];
        for map_i in 0..num_board {
            let map = &mut board_map[map_i];
            if let Some(index) = map.get(&num) {
                let row_key = index/BOARD_DIMENSION + ROW_BASE;
                let col_key = index%BOARD_DIMENSION + COL_BASE;
                map.insert(row_key, map.get(&row_key).unwrap() + 1);
                map.insert(col_key, map.get(&col_key).unwrap() + 1);

                let won = map.get(&row_key).unwrap().eq(&5) || map.get(&col_key).unwrap().eq(&5);
                if won && til_win_first {
                    return Ok((map_i, num_i));
                } else if won && !til_win_first {
                    won_set.insert(map_i);
                    if won_set.len().eq(&num_board) {
                        return Ok((map_i, num_i));
                    }
                }
            };
        }
    }
    Err(anyhow::anyhow!("no board ever won"))
}

fn get_drawn_sum(winning_map: &HashMap<i32, i32>, drawing_vec: &Vec<i32>, winning_draw_i: usize) -> Result<i32> {
    let mut drawn_sum = 0;
    for i in 0..winning_draw_i+1 {
        let num = drawing_vec[i];
        if winning_map.contains_key(&num) {
            drawn_sum += num;
        }
    }
    Ok(drawn_sum)
}
