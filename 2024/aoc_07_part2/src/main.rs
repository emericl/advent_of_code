/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/7/part2
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
    /* Note: For N operands we need N-1 operators, so there are X^(N-1) possibilities to test,
     *       WHERE X is the number of possible operators, here 3: +, *, || to concatenate 2 operands.
     *       We decide to store the number of possibilities in a Vec<u8> integer and we decide that
     *       for each possibility value, a 0 value means addition and 1 means multiplication and 2
     *       means concatenation.
     *       Each bit of the possibility value represents the position of each operator in the
     *       equation.
     */
     let mut equation_is_valid = false;
     let mut possibility: Vec<u8> = Vec::new();
     let mut possibility_count = 0;
     let max_possibilities = match operands.len() {
                                0 => 0,
                                _ => 3u32.pow(operands.len() as u32 - 1),
                            };
    for _ in 0..operands.len() {
        possibility.push(0u8);
    }

     while equation_is_valid == false && possibility_count < max_possibilities as u32 {
        /* Create the equation as a string vector */
        let mut equation: Vec<String> = Vec::new();
        equation.push(operands[0].to_string());
        for (idx, operand) in operands[1..operands.len()].iter().enumerate() {
            match possibility[idx] {
                0 => equation.push("+".to_string()),
                1 => equation.push("*".to_string()),
                2 => equation.push("||".to_string()),
                _ => panic!("Error in possibility value"),
            };
            equation.push(operand.to_string());
        }

        /* Compute the equation */
        let mut result:i64 = equation[0].parse().unwrap_or(-1);
        let mut idx = 1;
        while idx < equation.len() {
            match equation[idx].as_str() {
                "+" => {
                    result += equation[idx+1].parse().unwrap_or(-1);
                    idx += 1;
                },
                "*" => {
                    result *= equation[idx+1].parse().unwrap_or(-1);
                    idx += 1;
                },
                "||" => {
                    let mut number: String = result.to_string();
                    number.push_str(&equation[idx+1]);
                    result = number.parse().unwrap_or(-1);
                    idx += 1;
                },
                _ => {},
            };
            idx += 1;
        }

        /* Verify the validity of the equation */
        if result == expected_result {
            equation_is_valid = true;
        }

        /* Update operands possibility */
        possibility_count += 1;
        possibility[0] += 1;
        for idx in 0..possibility.len() {
            if possibility[idx] == 3 {
                possibility[idx] = 0;
                possibility[idx+1] += 1;
            }
        }
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

    for (eq_idx, equation) in equations.iter().enumerate() {
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

            /* Test all possible combination of operators for the equation */
            if equation_is_valid(&mut operands, result) {
                println!("Equation {:?}: VALID", eq_idx);
                valid_equations += 1;
                calibration += result;
            }
            else {
                println!("Equation {:?}: VALID", eq_idx);                
            }
        }
    }

    println!("Number of valid equations: {:?}, calibration value: {:?}", valid_equations, calibration);
}
