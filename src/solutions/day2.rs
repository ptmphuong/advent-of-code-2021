use crate::utils::save_lines_to_vec_string;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Forward,
    Down,
    Up,
}

impl Direction {
    pub fn new(s: &str) -> Result<Direction> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction:: Up),
            _ => Err(anyhow::anyhow!("invalid direction")),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    value: i32,
}

impl Instruction {
    pub fn new(s: String) -> Result<Instruction> {
        let split : Vec<&str> = s.split_whitespace().collect();
        if split.len() != 2 {
            return Err(anyhow::anyhow!("invalid instruction string: {:?}", split));
        }
        let dir = Direction::new(split[0])?;
        let val = split[1].parse()?;
        let instruction = Instruction {
            direction: dir,
            value: val,
        };
        Ok(instruction)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn new() -> Position {
        Position{ x : 0, y : 0 }
    }
    fn move_submarine(&mut self, instruction: Instruction) {
        match instruction.direction {
            Direction::Up =>  {
                self.y -= instruction.value;
            }
            Direction::Down => {
                self.y += instruction.value;
            }
            _ => {
                self.x += instruction.value;
            }
        }
    }
    pub fn multiply(&self) -> i32 {
        self.x * self.y
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PositionAndAim {
    x: i32,
    y: i32,
    aim: i32,
}

impl PositionAndAim {
    pub fn new() -> PositionAndAim {
        PositionAndAim { x : 0, y : 0, aim: 0 }
    }
    fn move_instr(&mut self, instruction: Instruction) {
        match instruction.direction {
            Direction::Up =>  {
                self.aim -= instruction.value;
            }
            Direction::Down => {
                self.aim += instruction.value;
            }
            _ => {
                self.x += instruction.value;
                let increment = self.aim * instruction.value;
                self.y += increment;
            }
        }
    }
    pub fn multiply(&self) -> i32 {
        self.x * self.y
    }
}

pub fn result_part1<P>(filename: P) -> Result<Position> where P: AsRef<Path>, {
    let mut pos = Position::new();

    let input_vec = save_lines_to_vec_string(filename)?;

    for line in input_vec {
        let instruction = Instruction::new(line)?;
        pos.move_submarine(instruction);
    }

    Ok(pos)
}

pub fn result_part2<P>(filename: P) -> Result<PositionAndAim> where P: AsRef<Path>, {
    let mut p = PositionAndAim::new();

    let input_vec = save_lines_to_vec_string(filename)?;

    for line in input_vec {
        let instruction = Instruction::new(line)?;
        p.move_instr(instruction);
    }

    Ok(p)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ins_string = String::from("forward 5");
        let ins = Instruction::new(ins_string).unwrap();
        let mut pos = Position::new();
        pos.move_submarine(ins);
        let expected = Position { x: 5, y: 0 };
        assert_eq!(expected, pos);
    }
}
