use std::collections::HashMap;
/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/6/part2
 */
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;

#[derive(Copy, Clone, Debug, PartialEq)]
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

fn paint_world(world: &Vec<String>) {
    for line in world {
        println!("{}", line);
    }
    println!("");
}

fn get_unique_cell_id(world: &Vec<String>, x: &i32, y: &i32) -> i32 {
    let world_x_len = world[0].len() as i32;
    let world_y_len = world.len() as i32;

    if *x < world_x_len && *y < world_y_len {
        *y * world_x_len + *x
    }
    else {
        -1 as i32
    }
}

fn change_world_cell_value(world: &mut Vec<String>, x: &i32, y: &i32, value: &str) {
        world[*y as usize].replace_range(*x as usize..*x as usize+1, value);
}

fn put_obstruction_in_world(world: &mut Vec<String>, obstruction_x: &mut i32, obstruction_y: &mut i32) -> bool {
    let world_x_len = world[0].len() as i32;
    let world_y_len = world.len() as i32;
    let mut obstruction_placed = false;

    if *obstruction_x == world_x_len || *obstruction_y == world_y_len {
        return false;
    }

    while obstruction_placed == false {
        let current_cell = world[*obstruction_y as usize].chars().nth(*obstruction_x as usize).unwrap();
        if current_cell == '#' || current_cell == '^' || current_cell == 'v' || current_cell == '<' || current_cell == '>' {
            *obstruction_x += 1;
            if *obstruction_x == world_x_len {
                *obstruction_x = 0;
                *obstruction_y += 1;

                if *obstruction_y == world_y_len {
                    println!("Could not place the first obstruction !");
                    exit(1);
                }
            }
        }
        else {
            obstruction_placed = true;
            change_world_cell_value(world, obstruction_x, obstruction_y, "O");
        }
    }    

    obstruction_placed
}

fn reset_world<'a>(world: &mut Vec<String>, guard: &mut &'a str, direction: &mut Direction, guard_position: &(i32, i32, Direction, &'a str)) {
    let world_x_len = world[0].len() as i32;
    let world_y_len = world.len() as i32;
    let guard_c = guard.chars().next().unwrap_or('^');
    
    change_world_cell_value(world, &guard_position.0, &guard_position.1, guard_position.3);
    *guard = guard_position.3;
    *direction = guard_position.2;

    for x in 0..world_x_len {
        for y in 0..world_y_len {
            let current_cell = world[y as usize].chars().nth(x as usize).unwrap();
            if current_cell != guard_c && current_cell != '#' {
                change_world_cell_value(world, &x, &y, ".");
            }
        }
    }
}

fn main() {
    let filename = "../input_data/aoc_06_test.txt";
    //let filename = "../input_data/aoc_06.txt";
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
    let mut prev_direction;
    let mut obstruction_x = 0;
    let mut obstruction_y = 0;

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
    prev_direction = direction;
    let guard_position = (x, y, direction, guard);

    /*****************************************************
     * MOVE THE GUARD UNTIL IT LEAVES THE WORLD
     */
    let mut loop_count = 0;
    let mut guard_disappeared = false;
    let mut all_mapped_tested = false;
        let mut obstacles_hit: HashMap<i32, Direction> = HashMap::new();

    /* Put the first obstruction on the world */
    obstruction_x = 3; obstruction_y = 6; /* TODO Ligne a supprimer */
    if put_obstruction_in_world(&mut world, &mut obstruction_x, &mut obstruction_y) == false {
        println!("Could not put any obstruction in the world !");
        exit(1);
    }
    paint_world(&world);

    println!("Starting search of loops...\n");

    while guard_disappeared == false && all_mapped_tested == false {
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
            /* Indicates that the guard disappeared */
            guard_disappeared = true;

            paint_world(&world);
        }
        else {
            /* Is there an obstacle ? */
            let current_cell = world[new_y as usize].chars().nth(new_x as usize).unwrap();
            if current_cell == '#' || current_cell == 'O' {
                let cell_id = get_unique_cell_id(&world, &new_x, &new_y);
                /* If the obstacle is "hit" from the same direction twice, then you're in a loop */
                if obstacles_hit.contains_key(&cell_id) && *obstacles_hit.get(&cell_id).unwrap() == direction {
                    println!("Loop detected !!");
                    paint_world(&world);

                    /* Increment the number of loops */
                    loop_count += 1;

                    /* Reset the guard's position */
                    reset_world(&mut world, &mut guard, &mut direction, &guard_position);

                    /* Remove the current obstruction and set the new obstruction */
                    change_world_cell_value(&mut world, &obstruction_x, &obstruction_y, ".");

                    /* Set the new obstruction */
                    if put_obstruction_in_world(&mut world, &mut obstruction_x, &mut obstruction_y) == false {
                        if obstruction_x == world_x_len && obstruction_y == world_y_len {
                            all_mapped_tested = true;
                        }
                        else {
                            println!("Unknown error !");
                            exit(1);
                        }
                    }
                    else {
                        change_world_cell_value(&mut world, &obstruction_x, &obstruction_y, "O");
                    }
                }
                /* Else */
                else {
                    /* Store the current cell position and the direction */
                    obstacles_hit.insert(cell_id,direction);

                    /* Change direction by turning right */
                    turn_right(&mut direction);
                    /* Do not change position of the guard */
                    println!("Hashmap: {:?}", obstacles_hit);
                    paint_world(&world);
                    
                }
            }
            else {
                /* Paint the area where the guard is */
                match direction {
                    Direction::UP => {
                        if prev_direction == direction {
                            change_world_cell_value(&mut world, &x, &y, "|");
                        }
                        else {
                            change_world_cell_value(&mut world, &x, &y, "+");
                        }
                    },
                    Direction::DOWN => {
                        if prev_direction == direction {
                            change_world_cell_value(&mut world, &x, &y, "|");
                        }
                        else {
                            change_world_cell_value(&mut world, &x, &y, "+");
                        }
                    },
                    Direction::LEFT => {
                        if prev_direction == direction {
                            change_world_cell_value(&mut world, &x, &y, "-");
                        }
                        else {
                            change_world_cell_value(&mut world, &x, &y, "+");
                        }
                    },
                    Direction::RIGHT => {
                        if prev_direction == direction {
                            change_world_cell_value(&mut world, &x, &y, "-");
                        }
                        else {
                            change_world_cell_value(&mut world, &x, &y, "+");
                        }
                    },
                }
                
                /* Move the guard */
                x = new_x;
                y = new_y;
                change_world_cell_value(&mut world, &x, &y, guard);

                /* Store current direction */
                prev_direction = direction;
            }
        }
    }

}
