/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/6
 */
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn turn_right(direction: &mut Direction) {
    let new_direction;

    match *direction {
        Direction::UP => new_direction = Direction::RIGHT,
        Direction::DOWN => new_direction = Direction::LEFT,
        Direction::LEFT => new_direction = Direction::UP,
        Direction::RIGHT => new_direction = Direction::DOWN,
    }

    *direction = new_direction;
}

fn main() {
    //let filename = "../input_data/aoc_06_test.txt";
    let filename = "../input_data/aoc_06.txt";
    let mut guard = "^";

    /* Verify presence of input file */
    if Path::new(filename).is_file() == false {
        println!("File '{filename}' not found.");
        return ();
    }

    /*****************************************************
     * CREATE WORLD
     */
    let mut world = read_lines(filename);
    let world_x_len = world[0].len() as i32;
    let world_y_len = world.len() as i32;

    /*****************************************************
     * SEARCH THE GUARD IN THE WORLD
     */
    let mut x;
    let mut y;
    let mut new_x;
    let mut new_y;
    let mut direction;

    let idx = world.iter().position(|l| l.contains(guard)).unwrap_or(usize::MAX);
    if idx == usize::MAX {
        println!("Guard not found !");
        return ();
    }
    else {
        y = idx as i32;
        let idx = world[y as usize].chars().position(|c| c == '^').unwrap_or(usize::MAX);
        x = idx as i32;
    }
    println!("Guard found (x:{:?}, y:{:?})", x, y);
    direction = Direction::UP;

    /*****************************************************
     * MOVE THE GUARD UNTIL IT LEAVES THE WORLD
     */
    let mut guard_disappeared = false;
    while guard_disappeared == false {
        /* Compute new position */
        match direction {
            Direction::UP => {
                guard = "^";
                new_x = x;
                new_y = y - 1;
            },
            Direction::DOWN => {
                guard = "v";
                new_x = x;
                new_y = y + 1;
            },
            Direction::LEFT => {
                guard = "<";
                new_x = x - 1;
                new_y = y;
            },
            Direction::RIGHT => {
                guard = ">";
                new_x = x + 1;
                new_y = y;
            },
        }

        /* Is guard outside the world ? */
        if new_x < 0 || new_y < 0 || new_x >= world_x_len || new_y >= world_y_len {
            /* Paint where the guard was before moving */
            world[y as usize].replace_range(x as usize..x as usize +1, "X");
            /* Indicates that the guard disappeared */
            guard_disappeared = true;
        }
        else {
            /* Is there an obstacle ? */
            if world[new_y as usize].chars().nth(new_x as usize).unwrap() == '#' {
                /* Change direction by turning right */
                turn_right(&mut direction);
                /* Do not change position of the guard */
            }
            else {
                /* Paint the area where the guard is */
                world[y as usize].replace_range(x as usize..x as usize+1, "X");
                /* Move the guard */
                x = new_x;
                y = new_y;
                world[y as usize].replace_range(x as usize..x as usize+1, guard);
            }
        }
    }

    /*****************************************************
     * COMPUTE AREA VIEWED BY THE GUARD
     */
    let mut area = 0;
    for line in world {
        area += line.chars().filter(|c| *c == 'X').count();
    }

    println!("Area searched by the guard: {:?}", area);
}
