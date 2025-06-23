/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/8
 */
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

#[derive(Debug)]
struct Antenna {
    pos_x: i32,
    pos_y: i32,
}

fn change_world_cell_value(world: &mut Vec<String>, x: &i32, y: &i32, value: &str) {
        world[*y as usize].replace_range(*x as usize..*x as usize+1, value);
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

fn main() {
    //let filename = "../input_data/aoc_08_test.txt";
    let filename = "../input_data/aoc_08.txt";

    /* Verify presence of input file */
    if Path::new(filename).is_file() == false {
        println!("File '{filename}' not found.");
        return ();
    }

    /*****************************************************
     * CREATE MAP OF ANTENNAS
     */
    let mut antenna_map = read_lines(filename);

    /*****************************************************
     * SEARCH AND LIST ANTENNAS IN THE MAP
     */
    let mut antenna_list: HashMap<char, Vec<Antenna>> = HashMap::new();
    for y in 0..antenna_map.len() {
        for x in 0..antenna_map[0].len() {
            let current_cell = antenna_map[y as usize].chars().nth(x as usize).unwrap();
            if current_cell != '.' {
                let antenna = Antenna{pos_x: x as i32, pos_y: y as i32};

                match antenna_list.get_mut(&current_cell) { 
                    None => {
                        let mut vect = Vec::new();
                        vect.push(antenna);
                        antenna_list.insert(current_cell.to_owned(), vect);
                    },
                    Some(elt) => {
                        elt.push(antenna);
                    },
                };
            }
        }
    }

    /*****************************************************
     * COMPUTE ANTINODES FOR EACH TYPE OF ANTENNA
     */
     let mut antinodes_list: Vec<i32> = Vec::new();
     for (freq, antennas) in antenna_list {
        println!("Computing antinodes for frequency {:?}...", freq);

        /* If less than 2 antennas, not antinode possible */
        if antennas.len() < 2 {
            continue;
        }

        /* Compute antinode(s) for each antenna against other antennas */
        for (idx_ant, antenna) in antennas.iter().enumerate() {
            for (idx_oant, oantenna) in antennas.iter().enumerate() {
                /* It's the same antenna, skipping */
                if idx_ant == idx_oant {
                    continue;
                }

                /* Compute distance vector between the two antennas */
                let dist_x = antenna.pos_x - oantenna.pos_x;
                let dist_y = antenna.pos_y - oantenna.pos_y;

                /* Compute antinode position with the distance between the two antennas */
                let anti_node_x = antenna.pos_x + dist_x;
                let anti_node_y = antenna.pos_y + dist_y;

                /* Count the antinode if it's inside the world and the antinode is at a same place of an antenna */
                if anti_node_x > -1 && anti_node_x < antenna_map[0].len() as i32 &&
                   anti_node_y > -1 && anti_node_y < antenna_map.len() as i32 {
                    
                    /* If the antinode has not already be pinned, pin it */
                    let unique_id = get_unique_cell_id(&antenna_map, &anti_node_x, &anti_node_y);
                    if unique_id != -1 && antinodes_list.contains(&unique_id) == false {
                        antinodes_list.push(unique_id);
                        let current_cell = antenna_map[anti_node_y as usize].chars().nth(anti_node_x as usize).unwrap();
                        if current_cell == '.' {
                            change_world_cell_value(&mut antenna_map, &anti_node_x, &anti_node_y, "#");
                        }
                    }


                }
            }
        }
    }

    for line in antenna_map {
        println!("{:?}", line);
    }

    println!("Number of antinodes detected: {:?}", antinodes_list.len());
}
