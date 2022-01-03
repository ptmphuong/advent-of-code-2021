mod solutions;
mod utils;

fn main() {

    /* 
    let filename1 = "input_files/input1_1.txt";
    let day1_part1 = solutions::day1::result_part1(filename1);
    println!("part1_result: {:?}", day1_part1);
    let day1_part2 = solutions::day1::result_part2(filename1);
    println!("part1_result: {:?}", day1_part2);

    let filename2 = "input_files/input2.txt";
    let part1_pos = solutions::day2::result_part1(filename2);
    println!("part1_result: {:?}", part1_pos);
    println!("part1_result: {:?}", part1_pos.unwrap().multiply());
    let part2_pos = solutions::day2::result_part2(filename2);
    println!("part2_result: {:?}", part2_pos);
    println!("part2_result: {:?}", part2_pos.unwrap().multiply());
    

    let filename3 = "input_files/input3.txt";
    let part1_pos = solutions::day3::result_part1(filename3);
    println!("part1_result: {:?}", part1_pos);
    let part2_pos = solutions::day3::result_part2(filename3);
    println!("part1_result: {:?}", part2_pos);

    */

    let filename4 = "input_files/input4.txt";
    // let filename4 = "input_files/input4 copy.txt";
    let part1_pos = solutions::day4::result_part1(filename4);
    println!("part1_result: {:?}", part1_pos);
    let part2_pos = solutions::day4::result_part2(filename4);
    println!("part2_result: {:?}", part2_pos);
}

