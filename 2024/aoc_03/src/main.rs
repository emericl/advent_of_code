/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/3
 */

 use std::fs::read_to_string;
 use std::path::Path;
 
 fn create_report_list_from_string(reports: &String) -> Option<Vec<i32>> {
     let mut list: Vec<i32> = Vec::new();
 
     for r in reports.split_whitespace() {
         let curr_report = match i32::from_str_radix(r, 10) {
             Ok(v) => v,
             Err(_) => return None,
         };
 
         list.push(curr_report);
     }
 
     Some(list.clone())
 }
  
 fn read_lines(filename: &str) -> Vec<String> {
     let mut result = Vec::new();
 
     for line in read_to_string(filename).unwrap().lines() {
         result.push(line.to_string())
     }
 
     result
 }
  
 fn main() {
     //let filename = "test_input_data.txt";
     //let filename = "input_data.txt";
 
     /* Verify presence of report file */
     if Path::new(filename).is_file() == false {
         println!("File '{filename}' not found.");
         return ();
     }
 
     /* Retrieve all the lines of the report */
     let lines: Vec<String> = read_lines(filename);
 
     /* Iterate over reports (one per line) in the file */
     for line in lines {
         /* Create a report list from string */
         if let Some(report) = create_report_list_from_string(&line) {
             let report_count = report.len();
             let mut report_is_safe = false;
 
             /* Verify the full report list */
             if is_safe_report(&report) == true {
                 /* Indicate that the report is safe */
                 report_is_safe = true;
             }
             else { /* Need to try other report list with one report removed */
                 for rep_idx in 0..report_count {
                     let new_report = new_report_list_without_idx(&report, rep_idx as i32);
 
                     /* Verify the report list */
                     if is_safe_report(&new_report) == true {
                         /* Indicate that the report is safe */
                         report_is_safe = true;
                         /* Stop the for loop */
                         break;
                     }
                 }
             } 
 
             if report_is_safe == true {
                 /* Update number of safe reports */
                 safe_reports += 1;
             }
         }
         else {
             println!("Could not create report list for this line '{line}'.");
             return ();
         }
     }
 
     /* Print number of safe reports */
     println!("Safe reports count: {safe_reports}");
 }
 