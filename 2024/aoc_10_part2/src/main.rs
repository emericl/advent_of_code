/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/10/part2
 */
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize
}

#[derive(Clone, Debug, PartialEq)]
struct PathState {
    position: Coordinate,
    path: Vec<Coordinate>
}

#[derive(Clone, Debug, PartialEq)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
    size: usize
}

impl<T> Grid<T> where T: Copy + Clone {
    fn new() -> Self {
        Self { data: Vec::new(), width: 0, height: 0, size: 0 }
    }

    fn set_size(&mut self, width: usize, height: usize, value: &T) {
        let new_size = width * height;

        if new_size > self.size {
            for _ in 0..(new_size - self.size) {
                self.data.push(value.clone());
            }

            self.size   = new_size;
            self.width  = width;
            self.height = height; 
        }
        else if new_size < self.size {
            for _ in 0..(self.size - new_size) {
                self.data.pop();
            }
            
            self.size   = new_size;
            self.width  = width;
            self.height = height; 
        }
    }

    fn set_value(&mut self, c: &Coordinate, value: T) {
        if c.x < self.width && c.y < self.height {
            self.data[c.y*self.width + c.x] = value;
        }
    }

    fn get_value(&self, c: &Coordinate) -> T {
        self.data[c.y*self.width + c.x]
    }

    fn get_x_size(&self) -> usize {
        self.width
    }

    fn get_y_size(&self) -> usize {
        self.height
    }

    fn get_possible_directions(&self, c: &Coordinate) -> Vec<Direction> {
        let mut possible_dirs: Vec<Direction> = Vec::new();

        if c.x > 0 {
            possible_dirs.push(Direction::LEFT);
        }

        if c.x < (self.width - 1) {
            possible_dirs.push(Direction::RIGHT);
        }

        if c.y > 0 {
            possible_dirs.push(Direction::UP);
        }

        if c.y < (self.height - 1) {
            possible_dirs.push(Direction::DOWN);
        }

        possible_dirs
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn create_world(filename: &str) -> Grid<usize> {
    let world = read_lines(filename);
    let mut grid: Grid<usize> = Grid::new();
    let world_x_len = world[0].len() as i32;
    let world_y_len = world.len() as i32;

    grid.set_size(world_x_len as usize, world_y_len as usize, &0);

    for y in 0..grid.get_y_size() {
        for x in 0..grid.get_x_size() {
            let current_cell = world[y].chars().nth(x).unwrap();
            let c = Coordinate{x: x, y: y};
            grid.set_value(&c, current_cell as usize - '0' as usize);
        }
    }

    grid
}

fn change_position(grid: &Grid<usize>, c: &Coordinate, direction: &Direction) -> Option<Coordinate> {
    let step: usize = 1;
    match *direction {
        Direction::UP => {
            if c.y > 0 {
                Some(Coordinate{ x: c.x, y: c.y - step })
            } else {
                None
            }
        },
        Direction::DOWN => {
            if c.y < grid.get_y_size() - 1 {
                Some(Coordinate{ x: c.x, y: c.y + step })
            } else {
                None
            }
        },
        Direction::LEFT => {
            if c.x > 0 {
                Some(Coordinate{ x: c.x - step, y: c.y })
            } else {
                None
            }
        },
        Direction::RIGHT => {
            if c.x < grid.get_x_size() - 1 {
                Some(Coordinate{ x: c.x + step, y: c.y })
            } else {
                None
            }
        }
    }
}

// fn print_grid<T>(grid: &Grid<T>) where T: Clone + std::fmt::Debug, T: Copy {
//     for y in 0..grid.get_y_size() {
//         let mut line = String::new();
//         for x in 0..grid.get_x_size() {
//             let c = Coordinate{x: x, y: y};
//             line.push_str(&format!("{:?} ", grid.get_value(&c)));   
//         }
//         println!("{}", line);
//     }
//}

fn search_paths(grid: &Grid<usize>, start: &Coordinate) -> Vec<Vec<Coordinate>> {
    let mut path_list: Vec<Vec<Coordinate>> = Vec::new();
    let mut coord_to_visit: VecDeque<PathState> = VecDeque::new();

    /* Add starting point to current path */
    coord_to_visit.push_back(PathState{position: *start, path: vec![*start]});

    /* While there is coordinates to visit */
    while coord_to_visit.len() > 0 {
        let state = match coord_to_visit.pop_front() {
            Some(c) => c,
            None => {
                continue
            },
        };
        let path = state.path;
        let coord    = state.position;

        /* If the current coordinate is the end of the path */
        if grid.get_value(&coord) == 9 {
            /* We add the current path to the list of paths */
            path_list.push(path.clone());
        }
        /* Else */
        else {
            /* Verify all directions */
            let possible_dirs = grid.get_possible_directions(&coord);

            /* If there are possible directions to go */
            /* Note: this is not strictly necessary, but it avoids unnecessary iterations */
            /* If there are no possible directions, we just continue */
            if possible_dirs.len() > 0 {
                for direction in possible_dirs {
                    let new_coord =  match change_position(&grid, &coord, &direction) {
                        Some(c) => c,
                        None => {
                            continue
                        },
                    };

                    /* If the next coordinate is the next cell value (+1) */
                    if grid.get_value(&new_coord) == grid.get_value(&coord) + 1 {
                        /* Add the new position to the path and add it to coordinates to visit */
                        if !path.contains(&new_coord) {
                            let mut new_path = path.clone();
                            new_path.push(new_coord);
                            coord_to_visit.push_back(PathState{position: new_coord, path: new_path.clone()});
                        }
                    }
                }
            }
        }
    }

    path_list
}

fn main() {
    let filename = if cfg!(debug_assertions) {
        "../input_data/aoc_10_test.txt"
    }
    else {
        "../input_data/aoc_10.txt"
    };

    /* Verify presence of input file */
    if Path::new(filename).is_file() == false {
        println!("File '{filename}' not found.");
        return ();
    }

    /*****************************************************
     * CREATE WORLD
     */
    let grid = create_world(filename);

    /*****************************************************
     * SEARCH THE FIRST STARTING POINT IN THE WORLD
     */
    /* Note: all other starting points will be added as we discover them during the search of the paths */
    let mut starting_points: Vec<Coordinate> = Vec::new();

    for y in 0..grid.get_y_size() {
        for x in 0..grid.get_x_size() {
            let c = Coordinate{x: x, y: y};
            if grid.get_value(&c) == 0 {
                starting_points.push(c);
            }
        }
    }

    /*****************************************************
     * SEARCH ALL THE POSSIBLE PATHS
     */
    let mut final_score = 0;

    println!("Starting search of paths...");

    for start in starting_points {        
        /* Search paths from the current starting point */
        let paths = search_paths(&grid, &start);

        /* Count the number of paths (trailheads) */
        let path_count = paths.len();

        /* Add to the final score */
        final_score = final_score + path_count;
    }

    println!("Final score: {:?}", final_score);


}
