/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/3/part2
 */
use regex::Regex;
use std::fs::read_to_string;
use std::path::Path;
  
fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    //let filename = "../input_data/aoc_03_test.txt";
    let filename = "../input_data/aoc_03.txt";

    /* Verify presence of input file */
    if Path::new(filename).is_file() == false {
        println!("File '{filename}' not found.");
        return ();
    }

    /* Retrieve all the lines of the file */
    let lines: Vec<String> = read_lines(filename);
    
    /* Iterate over reports (one per line) in the file */
    let mut mult_enabled = true;
    let mut total = 0;
    let re = Regex::new(r#"do\(\)|mul\((\d+),(\d+)\)|don't\(\)"#).unwrap();
    for line in lines {
        for command in re.captures_iter(line.as_str()) {
            // println!("command: {:?}", command);

            if command[0].contains("don't()") == true {
                mult_enabled = false;
            }
            else if command[0].contains("do()") == true {
                mult_enabled = true;
            }
            else if command[0].contains("mul(") == true && mult_enabled == true {
                let val_a = match i32::from_str_radix(&command[1], 10) {
                    Ok(v) => v,
                    Err(_) => return (),
                };
                let val_b = match i32::from_str_radix(&command[2], 10) {
                    Ok(v) => v,
                    Err(_) => return (),
                };

                total += val_a * val_b;
            }
        }
    }

    println!("Total: {total}");
}
