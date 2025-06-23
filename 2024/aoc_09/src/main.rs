/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/9
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

fn main() {
    //let filename = "../input_data/aoc_09_test.txt";
    let filename = "../input_data/aoc_09.txt";

    /* Verify presence of input file */
    if Path::new(filename).is_file() == false {
        println!("File '{filename}' not found.");
        return ();
    }

    /*****************************************************
     * READ DISK MAP FROM FILE
     */
    let disk_map = read_lines(filename);
    if disk_map.len() == 0 {
        panic!("disk_map length is zero");
    }

    /*****************************************************
     * CONSTRUCT THE DISK DATA USING THE DISK MAP
     */
     /* Note: I've decided to store the disk file ID as i32 and -1 means free-space */
     let mut file_id: i32 = 0;
     let mut disk_data: Vec<i32> = Vec::new();
     for (idx, disk_entry) in disk_map[0].chars().enumerate() {
        let block_count = disk_entry as i32 - '0' as i32;
        if idx % 2 == 0 {
            /* Add File blocks */
            for _ in 0..block_count as i32 {
                disk_data.push(file_id);
            }
            file_id += 1;
        }
        else {
            /* Add Free space blocks */
            for _ in 0..block_count as i32 {
                disk_data.push(-1);
            }
        }
     }

     println!("disk_data: {:?}", disk_data);

    /*****************************************************
     * COMPACT ALL THE FILES TO THE BEGINING OF THE DISK
     */
    /* Start reading at the end of the disk */
    let mut read_idx = (disk_data.len() - 1) as i32;
    /* Start writting at the begining of the disk */
    let mut write_idx = 0;
    let mut value = -1;

    /* Until all the block data has been moved */
    while read_idx >= write_idx {
        /* If no block data is currently moving */
        if value == -1 {
            /* If a block data (non free-space) is detected */
            if disk_data[read_idx as usize] != -1 {
                /* Temporarely store the block data */
                value = disk_data[read_idx as usize];
                /* Mark the current block data location as free-space */
                disk_data[read_idx as usize] = -1;
            }
            /* Move read index one block to the left */
            read_idx -= 1;
        }
        else {
            /* If the current block for writting is empty space */
            if disk_data[write_idx as usize] == -1 {
                /* Store the block to be moved in the new location */
                disk_data[write_idx as usize] = value;
                /* Clear the temporary storage */ 
                value = -1;
            }
            /* Move read index one block to the right */
            write_idx += 1;
        }
    }

    /* If a value is currently moving, find first empty block and store the data */
    if value != -1 {
        while (write_idx as usize) < disk_data.len() && disk_data[write_idx as usize] != -1 {
            write_idx += 1;
        }
        if (write_idx as usize) < disk_data.len() && disk_data[write_idx as usize] == -1 {
            disk_data[write_idx as usize] = value;
        }
        else {
            println!("Could not store last block of data !");
        }
    }

    /*****************************************************
     * COMPUTE THE CHECKSUM OF THE DISK
     */
    let mut checksum: i64 = 0;
    for (blk_id, blk_data) in disk_data.iter().enumerate() {
        if *blk_data > -1 {
            checksum += blk_id as i64 * *blk_data as i64
        }
    }

    println!("Checkum: {:?}", checksum);
}
