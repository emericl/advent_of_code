/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/4#part2
 */
use std::fs::read_to_string;
use std::option::Option;
use std::path::Path;

fn read_as_array(filename: &str) -> Option<Vec<Vec<char>>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        let mut row = Vec::new();

        for c in line.chars() {
            row.push(c);
        }
        result.push(row);
    }

    Some(result)
}

fn get_char_from_table(table: &Vec<Vec<char>>, row: usize, col: usize) -> char {
    if let Some(r) = table.get(row) {
        if let Some(&character) = r.get(col) {
            return character.clone();
        };
    }
    0 as char
}

fn search_patterns(table: &Vec<Vec<char>>, start_row: usize, start_col: usize) -> bool {
    let first_char = get_char_from_table(table, start_row, start_col);

    /* Verify that first character is correct and that we won't be out of bounds during search 
     * Note: table is a square table, so we simplify the verification as we know that each line
     *       will be the size of the number of lines.
     */
    if (first_char != 'M' && first_char != 'S') ||
       (start_row + 2) >= table.len() ||
       (start_col + 2) >= table.len() {
        return false;
    }

    /* If first char is M, search for the following patterns:
     *   M.S      M.M
     *   .A.      .A.
     *   M.S      S.S
     *
     */
     if (get_char_from_table(table, start_row    , start_col    ) == 'M' && get_char_from_table(table, start_row    , start_col + 2) == 'S' &&
         get_char_from_table(table, start_row + 1, start_col + 1) == 'A' &&
         get_char_from_table(table, start_row + 2, start_col    ) == 'M' && get_char_from_table(table, start_row + 2, start_col + 2) == 'S') ||
        (get_char_from_table(table, start_row    , start_col    ) == 'M' && get_char_from_table(table, start_row    , start_col + 2) == 'M' &&
         get_char_from_table(table, start_row + 1, start_col + 1) == 'A' &&
         get_char_from_table(table, start_row + 2, start_col    ) == 'S' && get_char_from_table(table, start_row + 2, start_col + 2) == 'S') {
            return true;
     }

    /* If first char is S, search for the following patterns:
     *   S.M      S.S
     *   .A.      .A.
     *   S.M      M.M
     *
     */
     if (get_char_from_table(table, start_row    , start_col    ) == 'S' && get_char_from_table(table, start_row    , start_col + 2) == 'M' &&
         get_char_from_table(table, start_row + 1, start_col + 1) == 'A' &&
         get_char_from_table(table, start_row + 2, start_col    ) == 'S' && get_char_from_table(table, start_row + 2, start_col + 2) == 'M') ||
        (get_char_from_table(table, start_row    , start_col    ) == 'S' && get_char_from_table(table, start_row    , start_col + 2) == 'S' &&
         get_char_from_table(table, start_row + 1, start_col + 1) == 'A' &&
         get_char_from_table(table, start_row + 2, start_col    ) == 'M' && get_char_from_table(table, start_row + 2, start_col + 2) == 'M') {
            return true;
     }

    false
}

fn main() {
    //let filename = "../input_data/aoc_04_test.txt";
    let filename = "../input_data/aoc_04.txt";
    let mut total_occurences = 0;

    /* Verify presence of input file */
    if Path::new(filename).is_file() == false {
        println!("File '{filename}' not found.");
        return ();
    }

    /* Read the file and organize data as Vec<Vec<char>> */
    let row_list = match read_as_array(filename) {
        Some(a) => a,
        None => return (),
    };

    /* Search for M or S character which can be the start of the X-MAS pattern */
    for (idx_r, r) in row_list.iter().enumerate() {
        for (idx_c, c) in r.iter().enumerate() {
            /* If character found, search for the X-MAS pattern */
            if *c == 'M' || *c == 'S' {
                if search_patterns(&row_list, idx_r, idx_c) {
                    total_occurences += 1;
                }
            }
        }
    }

    println!("Total occurences of X-MAS in the matrix: {total_occurences}");
}
