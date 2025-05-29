/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/2
 */

use std::fs::read_to_string;
use std::path::Path;

fn is_safe_report(report: &String) -> bool {
    let mut prev_report = 0;
    let mut first_report = true;
    let mut increasing = false;
    let mut decreasing = false;

    for state in report.split_whitespace() {
        let curr_report = match i32::from_str_radix(state, 10) {
            Ok(v) => v,
            Err(_) => return false,
        };

        //println!("Curr report:{curr_report}, prev:{prev_report}");

        if first_report == true {
            first_report = false;
            prev_report = curr_report;
        }
        else {
            let variation = (prev_report - curr_report).abs();

            if variation < 1 || variation > 3 {
                /* Reports are stationnary => NOT SAFE */
                println!("\nReport line: {report}");
                println!("Reports are stationnary or varying too much => NOT SAFE");
                return false;
            }

            if increasing == false && decreasing == false {
                if prev_report < curr_report {
                    increasing = true;
                }
                else if prev_report > curr_report {
                    decreasing = true;
                }
                else {
                    /* Impossible to reach as variation == 0 is tested earlier */
                }
            }
            else {
                if increasing == true && prev_report > curr_report {
                    /* Decrease after increase => NOT SAFE */
                    println!("\nReport line: {report}");
                    println!("Decrease after increase => NOT SAFE");
                    return false;
                }
                
                if decreasing == true  && prev_report < curr_report {
                    /* Increase after decrease => NOT SAFE */
                    println!("\nReport line: {report}");
                    println!("Increase after decrease => NOT SAFE");
                    return false;
                }
            }

            /* Store current report as prev report */
            prev_report = curr_report;
        }
    }

    /* If we arrive here, the report is SAFE */
    true
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn main() {
    let mut safe_reports = 0;
    //let filename = "../input_data/aoc_02_test.txt";
    let filename = "../input_data/aoc_02.txt";

    /* Verify presence of report file */
    if Path::new(filename).is_file() == false {
        println!("File '{filename}' not found.");
        return ();
    }

    /* Retrieve all the lines of the report */
    let reports = read_lines(filename);

    /* Iterate over reports (one per line) in the file */
    for report in reports {
        /* Verify report */
        let report_result = is_safe_report(&report);

        /* Update number of safe reports */
        if report_result == true {
            safe_reports += 1;
        }
    }

    /* Print number of safe reports */
    println!("Safe reports count: {safe_reports}");
}
