/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/5/part2
 */
use std::fs::read_to_string;
use std::path::Path;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}


fn create_rules(configs: &Vec<String>) -> Vec<(i32, i32)> {
    let mut rules: Vec<(i32, i32)> = Vec::new();

    for line in configs {
        if line.contains("|") {
            let r:Vec<_> = line.split("|").collect();

            if r.len() >= 2 {
                let val_1:i32 = r[0].trim().parse().unwrap_or(-1);
                let val_2:i32 = r[1].trim().parse().unwrap_or(-1);

                if val_1 > 0 && val_2 > 0 {
                    rules.push((val_1, val_2));
                }
            }
        }
    }

    rules
}

fn create_updates(configs: &Vec<String>) -> Vec<Vec<i32>> {
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in configs {
        if line.contains(",") {
            let mut update: Vec<i32> = Vec::new();

            let pages:Vec<_> = line.split(",").collect();
            for page in pages {
                let val:i32 = page.trim().parse().unwrap_or(-1);
                if val > 0 {
                    update.push(val);
                }
            }

            updates.push(update);
        }
    }

    updates
}

fn main() {
    //let filename = "../input_data/aoc_05_test.txt";
    let filename = "../input_data/aoc_05.txt";

    /* Verify presence of input file */
    if Path::new(filename).is_file() == false {
        println!("File '{filename}' not found.");
        return ();
    }

    /*****************************************************
     * CREATE LIST OF RULES AND UPDATES
     */
    let configs = read_lines(filename);
    let rules = create_rules(&configs);
    let mut updates = create_updates(&configs);

    /*****************************************************
     * VERIFY EACH UPDATE WITH THE SET OF RULES
     */
    let mut total = 0;
    let mut needs_reorder;
    for update in &mut updates {
        needs_reorder = false;
        for (first_page, other_page) in &rules {
            let p1_idx = update.iter().position(|n| n == first_page).unwrap_or(usize::MAX);
            let p2_idx = update.iter().position(|n| n == other_page).unwrap_or(usize::MAX);

            if p1_idx != usize::MAX && p2_idx != usize::MAX && p1_idx > p2_idx {
                needs_reorder = true;
                break;
            }
        }

        if needs_reorder {
            update.sort_by(|a, b| {
                for &(x, y) in &rules {
                    if *a == x && *b == y {
                        return std::cmp::Ordering::Less;
                    } else if *a == y && *b == x {
                        return std::cmp::Ordering::Greater;
                    }
                }
                std::cmp::Ordering::Equal
            });

            /* Find the middle page number */
            let middle = update.len() / 2;
            println!("update: {:?}, middle: {:?}", update, middle);

            /* Add it to total */
            total += update[middle];
        }
    }

    println!("Total: {:?}", total);
}