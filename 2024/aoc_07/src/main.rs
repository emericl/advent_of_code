/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/7
 */
use std::fs::read_to_string;
use std::path::Path;

trait RemoveLast {
    fn remove_last(&self) -> &Self;
}

impl RemoveLast for str {
    fn remove_last(&self) -> &Self {
        self.strip_suffix(|_: char| true).unwrap_or(self)
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn equation_is_valid(operands: &mut Vec<i64>, expected_result: i64) -> bool {
    /* Note: For N operands we need N-1 operators, so there are 2^(N-1) possibilities to test.
     *       We decide to store the number of possibilities in a u32 integer and we decide that
     *       for each possibility value, a 0b value means addition and 1b means multiplication.
     *       Each bit of the possibility value represents the position of each operator in the
     *       equation.
     */
     let mut equation_is_valid = false;
     let mut possibility: u32 = 0;
     let max_possibilities = match operands.len() {
                                0 => 0,
                                _ => 2u32.pow(operands.len() as u32 - 1),
                            };

     while equation_is_valid == false && possibility < max_possibilities as u32 {
        let mut result = operands[0];
        let mut dbg: String = String::new();

        dbg.push_str(&operands[0].to_string());
        for (idx, operand) in operands[1..operands.len()].iter().enumerate() {
            if possibility & (1 << idx) > 0 {
                result *= operand;
            }
            else {
                result += operand;
            }
            dbg.push_str(&operand.to_string());
        }
        if result == expected_result {
            equation_is_valid = true;
        }

        possibility += 1;
     }

     equation_is_valid
}

fn main() {
    //let filename = "../input_data/aoc_07_test.txt";
    let filename = "../input_data/aoc_07.txt";

    /* Verify presence of input file */
    if Path::new(filename).is_file() == false {
        println!("File '{filename}' not found.");
        return ();
    }

    /*****************************************************
     * CREATE LIST OF EQUATIONS
     */
    let equations = read_lines(filename);

    /*****************************************************
     * VERIFY EACH EQUATION
     */
     let mut valid_equations = 0;
     let mut calibration = 0;

    for equation in equations {
        if equation.contains(":") {
            /* Extract the terms of the equation */
            let terms:Vec<_> = equation.split(" ").collect();

            /* Convert terms into integers */
            let mut operands:Vec<i64> = Vec::new();
            let mut result = -1;
            for (idx,term) in &mut terms.into_iter().enumerate() {
                match idx {
                    0 => result = term.remove_last().parse().unwrap_or(-1), /* .pop() to remove last character */
                    _ => operands.push(term.parse().unwrap_or(-1)),
                };
            }


            println!("result: {:?}, operands: {:?}", result, operands);
            /* Test all possible combination of operators for the equation */
            if equation_is_valid(&mut operands, result) {
                println!("Equation is valid !");
                valid_equations += 1;
                calibration += result;

            }
            println!("");
        }

    }

    println!("Number of valid equations: {:?}, calibration value: {:?}", valid_equations, calibration);
}
