/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/9/part2
 */
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct DiskEntry {
    idx: usize,
    block_count: usize,
    is_file: bool
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn search_place_for_disk_entry(list: & Vec<DiskEntry>, size: usize) -> Option<usize> {
    for (idx, entry) in list.clone().iter().enumerate() {
        if entry.is_file == false && entry.block_count >= size {
            return Some(idx);
        }
    }

    None
}

fn move_disk_entry(list: &mut Vec<DiskEntry>, from: usize, to: usize) {
    // TODO Add verifications

    /* First, we clone the entry to be moved from the Vec */
    let data_entry = list[from].clone();

    println!("moving: {:?}", data_entry);

    /* Then, we replace the entry to be moved by free-space */
    list[from].is_file = false;

    /* If data is exactly the size of free-space */
    if data_entry.block_count == list[to].block_count {
        /* Replace free-space by data */
        list[to] = data_entry;
    }
    else {
        /* Reduce the size of free-space */
        list[to].block_count = list[to].block_count - data_entry.block_count;
    }

    /* Insert data before free-space */
    list.insert(to, data_entry);
}

fn main() {
    let filename = "../input_data/aoc_09_test.txt";
    //let filename = "../input_data/aoc_09.txt";

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
    let mut files: Vec<DiskEntry> = Vec::new();
    for (idx, disk_entry) in disk_map[0].chars().enumerate() {
        let block_count = disk_entry as i32 - '0' as i32;
        let is_file;
        if idx % 2 == 0 {
            /* Add File blocks */
            is_file = true;
            for _ in 0..block_count as i32 {
                disk_data.push(file_id);
            }

            /* Store disk information */
            files.push(DiskEntry{idx: file_id as usize, block_count: block_count as usize, is_file: is_file});

            /* Increment file identifier */
            file_id += 1;
        }
        else {
            /* Add Free-space blocks to disk map */
            is_file = false;
            for _ in 0..block_count as i32 {
                disk_data.push(-1);
            }

            /* Store disk information */
            files.push(DiskEntry{idx: usize::MAX, block_count: block_count as usize, is_file: is_file});
        }
    }

    /* Store the last file id for future use */
    let last_file_id: i32 = file_id - 1;

    println!("disk_data: {:?}", disk_data);
    println!("files: {:?}", files);
    println!("last_file_id: {:?}", last_file_id);

    /*****************************************************
     * COMPACT ALL THE FILES TO THE BEGINING OF THE DISK
     */
    /* Start from the last file in the disk to the second one (first one is already at the start of the disk) */
    let mut file_id: i32 = last_file_id;
    let mut read_idx: i32;
    while file_id > 1 {
        println!("Searching file_id {file_id}...");
        /* Find the file with current file ID in the disk */
        read_idx = (files.len() - 1) as i32;
        while read_idx > -1 {
            if files[read_idx as usize].is_file == true &&
               files[read_idx as usize].idx == file_id as usize {
                println!("File_id {file_id} found at index {read_idx}.");
                break;
            }
            read_idx = read_idx - 1;
        }

        /* If the file with specified ID has been found */
        if read_idx > -1 {
            /* Look for a place where to store the file */
            match search_place_for_disk_entry(&files, files[read_idx as usize].block_count) {
                Some(idx) => {
                    println!("File #{file_id} will be moved at index {idx}.");
                    /* Move the file at the new location */
                    move_disk_entry(&mut files, read_idx as usize, idx);
                },
                None => {
                    println!("File #{file_id} could not be moved.");
                    file_id = file_id - 1;
                    continue;
                },
            }
        }
        
        /* Going to next file */
        file_id = file_id - 1;
    }

    /*****************************************************
     * COMPUTE THE CHECKSUM OF THE DISK
     */
    let mut checksum: i64 = 0;
    disk_data.clear();
println!("disk_data: {:?}", disk_data);
    for e in files {
        if e.is_file == true {
            for _ in 0..e.block_count {
                disk_data.push(e.idx as i32);
            }
        }
        else {
            disk_data.push(-1);
        }
    }
println!("disk_data: {:?}", disk_data);
    println!("Checkum: {:?}", checksum);
}
