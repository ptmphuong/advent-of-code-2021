use crate::utils::save_lines_to_vec_string;
use std::{path::Path, collections::{HashMap, HashSet}};
use anyhow::Result;
use std::cmp;


pub fn result_part1<P>(filename: P) -> Result<i32> where P: AsRef<Path>, {
    count_points(filename, false)
}

pub fn result_part2<P>(filename: P) -> Result<i32> where P: AsRef<Path>, {
    count_points(filename, true)
}

pub fn count_points<P>(filename: P, consider_diagonal_lines: bool) -> Result<i32> where P: AsRef<Path>, {
    let input_vec = save_lines_to_vec_string(filename)?;
    let mut position_map: HashMap<Position, i32> = HashMap::new();
    let mut counter = 0;

    for line in input_vec {
        let line_of_vents = LineOfVents::new(line)?;
        let all_pos = line_of_vents.get_all_positions(consider_diagonal_lines)?;
        for pos in all_pos {
            let count = position_map.entry(pos).or_insert(0);
            *count+=1;
            if (*count).eq(&2) {
                counter += 1;
            }
        }
    }
    Ok(counter)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    pub fn new_int(row: i32, col: i32) -> Position {
        Position { row, col }
    }

    pub fn new(s: String) -> Result<Position> {
        let split : Vec<&str> = s.split(",").collect();
        if !split.len().eq(&2) {
            return Err(anyhow::anyhow!("invalid argument"));
        }
        let v = split.iter().map(|&num_str| num_str.trim().parse::<i32>().unwrap()).collect::<Vec<_>>();
        let row = v[0];
        let col = v[1];
        Ok(Position {row, col})
    }

    pub fn eq_row(&self, o: &Position) -> bool {
        self.row.eq(&o.row)
    }

    pub fn eq_col(&self, o: &Position) -> bool {
        self.col.eq(&o.col)
    }

    pub fn get_min<'a>(&'a self, o: &'a Position) -> &'a Position {
        if self.row < o.row {
            self
        } else {
            o
        }
    }

    pub fn get_max<'a>(&'a self, o: &'a Position) -> &'a Position {
        if self.row > o.row {
            self
        } else {
            o
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct LineOfVents {
    from: Position,
    to: Position,
}

impl LineOfVents {
    pub fn new(s: String) -> Result<LineOfVents> {
        let split: Vec<&str> = s.split("->").collect();
        if !split.len().eq(&2) {
            return Err(anyhow::anyhow!("invalid argument"));
        }

        let from = Position::new(split[0].to_string())?;
        let to = Position::new(split[1].to_string())?;
        Ok(LineOfVents {from, to})
    }

    pub fn get_all_positions(&self, consider_diagonal_lines: bool) -> Result<Vec<Position>> {
        let mut all_pos = Vec::new();

        let from = &self.from;
        let to = &self.to;

        if from.eq_row(&to) {
            let row = from.row;
            let col_start = cmp::min(from.col, to.col);
            let col_end = cmp::max(from.col, to.col) + 1;
            for col in col_start..col_end {
                all_pos.push(Position::new_int(row, col));
            }
        } else if from.eq_col(&to) {
            let col = from.col;
            let row_start = cmp::min(from.row, to.row);
            let row_end = cmp::max(from.row, to.row) + 1;
            for row in row_start..row_end {
                all_pos.push(Position::new_int(row, col));
            }
        } else {
            if consider_diagonal_lines {
                let pos_start = from.get_min(to);
                let pos_end = from.get_max(to);
                let row_start = pos_start.row;
                let row_end = pos_end.row + 1;
                let mut col = pos_start.col;
                for row in row_start..row_end {
                    all_pos.push(Position::new_int(row, col));
                    if pos_start.col > pos_end.col {
                        col -= 1;
                    } else {
                        col += 1;
                    }
                }
            }
        }
        Ok(all_pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_position_no_space() {
        let s = String::from("43,243");
        let pos = Position::new(s);
        assert_eq!(pos.unwrap(), Position{row: 43, col: 243});
    }

    #[test]
    fn parse_position_with_space() {
        let s = String::from("43,243 ");
        let pos = Position::new(s);
        assert_eq!(pos.unwrap(), Position{row: 43, col: 243});
    }

    #[test]
    fn parse_line_of_vents() {
        let pos1 = Position { row: 1, col: 1 };
        let pos2 = Position { row: 1, col: 3 };
        let expected = LineOfVents { from: pos1, to: pos2 };
        let actual = LineOfVents::new(String::from("1,1 -> 1,3")).unwrap();
        println!("{:?}", actual.get_all_positions(false));
        assert_eq!(expected, actual);
    }

    #[test]
    fn get_all_positions_in_line_reverse() {
        let pos1 = Position { row: 9, col: 1 };
        let pos2 = Position { row: 7, col: 1 };
        let expected = LineOfVents { from: pos1, to: pos2 };
        let actual = LineOfVents::new(String::from("9,1 -> 7,1")).unwrap();
        println!("{:?}", actual.get_all_positions(false));
        assert_eq!(expected, actual);
    }
}
