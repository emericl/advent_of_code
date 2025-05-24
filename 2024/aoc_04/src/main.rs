/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/4
 */
use std::fs::read_to_string;
use std::path::Path;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn create_columns(list: &Vec<String>) -> Vec<String> {
    let mut columns = Vec::new();
    let items_count = list[0].len();

    for idx_c in 0..items_count {
        let mut row = String::with_capacity(items_count);
        for idx_r in 0..items_count {
            if let Some(c) = list[idx_r].chars().nth(idx_c) {
                row.push(c);
            }
        }
        columns.push(row);
    }

    columns
}

fn create_diagonals(list: &Vec<String>, left_right: bool) -> Vec<String> {
    let mut result = Vec::new();
    let char_count = list[0].len();

    if left_right == true {
        /* Create diagonals left to right from each char of first line */
        for char_idx in 0..char_count {
            let mut diag = String::with_capacity(char_count);
            for i in 0..(char_count-char_idx) {
                if let Some(c) = list[i].chars().nth(char_idx + i) {
                    diag.push(c);
                }
            }
            result.push(diag);
        }

        // /* Create diagonals left to right from each first char of each line */
        for row_idx in 1..char_count {
            let mut diag = String::with_capacity(char_count);
            for i in 0..(char_count - row_idx) {
                if let Some(c) = list[row_idx + i].chars().nth(i) {
                    diag.push(c);
                }
            }
            result.push(diag);
        }
    }
    else {
        /* Create diagonals right to left from each char of first line */
        for char_idx in 0..char_count {
            let mut diag = String::with_capacity(char_count);
            for i in 0..(char_count - char_idx) {
                if let Some(c) = list[i].chars().nth((char_count-char_idx-1) - i) {
                    diag.push(c);
                }
            }
            result.push(diag);
        }

        /* Create diagonals right to left from each first char of each line */
        for char_idx in 1..char_count {
            let mut diag = String::with_capacity(char_count);
            for i in 0..(char_count-char_idx) {
                if let Some(c) = list[char_idx + i].chars().nth((char_count-1) - i) {
                    diag.push(c);
                }
            }
            result.push(diag);
        }
    }

    result
}

fn count_occurences(list: &Vec<String>, pattern: &str) -> usize {
    let mut occurences = 0;
    match Regex::new(pattern) {
        Ok(result) => {
            for line in list {
                let count = result.captures_iter(line.as_str()).count();
                occurences += count;
            }
        },
        Err(_) => (),
    }
    
    occurences
}

fn main() {
    //let filename = "test_input_data.txt";
    let filename = "input_data.txt";
    let mut total_occurences = 0;

    /* Verify presence of input file */
    if Path::new(filename).is_file() == false {
        println!("File '{filename}' not found.");
        return ();
    }

    /*****************************************************
     * COUNT THE OCCURENCES IN THE LINES OF THE MATRIX
     */
    let row_list = read_lines(filename);
    total_occurences += count_occurences(&row_list, "XMAS");
    total_occurences += count_occurences(&row_list, "SAMX");

    /*****************************************************
     * COUNT THE OCCURENCES IN THE COLUMNS OF THE MATRIX
     */
    let list = create_columns(&row_list);
    total_occurences += count_occurences(&list, "XMAS");
    total_occurences += count_occurences(&list, "SAMX");

    /*****************************************************
     * COUNT THE OCCURENCES IN THE DIAGONALS (BL to UR and reverse) OF THE MATRIX
     */
    let list = create_diagonals(&row_list, true);
    total_occurences += count_occurences(&list, "XMAS");
    total_occurences += count_occurences(&list, "SAMX");

    /*****************************************************
     * COUNT THE OCCURENCES IN THE DIAGONALS (TL to BR and reverse) OF THE MATRIX
     */
    let list = create_diagonals(&row_list, false);
    total_occurences += count_occurences(&list, "XMAS");
    total_occurences += count_occurences(&list, "SAMX");

    println!("Total occurences of XMAS in the matrix: {total_occurences}");
}
