/* ADVENT OF CODE
 * See: https://adventofcode.com/2024/day/3
 */
use std::fs::read_to_string;
use std::path::Path;
use regex::Regex;

/* TODO:
 *    - Lire le fichier pour créer un vecteur de ligne de texte
 *    - Rechercher dans chaque ligne les occurences de XMAS  (horizontal)
 *    - Rechercher dans chaque ligne les occurences de SAMX  (horizontal inverse)
 *    - Lire le fichier pour créer un vecteur de colonnes de texte
 *    - Rechercher dans chaque ligne les occurences de XMAS  (vertical)
 *    - Rechercher dans chaque ligne les occurences de SAMX  (vertical inverse)
 *    - Voir comment tester les diagonales
 *         * je peux p-e voir pour créer une fonction qui créée un vecteur de lignes diagonales
 *         * et je réutilise les fonctions pour compter les occurences de XMAS et SAMX
 */

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
/*
fn read_columns(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    // To be implemented

    result
}
*/

fn count_occurences(list: &Vec<String>, pattern: &str) -> usize {
    let mut occurences = 0;
    match Regex::new(pattern) {
        Ok(result) => {
            for line in list {
                let count = result.captures_iter(line.as_str()).count();
                occurences += count;
            }
        },
        Err(_) => (),
    }
    
    occurences
}

fn main() {
    let filename = "test_input_data.txt";
    //let filename = "input_data.txt";
    let mut total_occurences = 0;

    /* Verify presence of input file */
    if Path::new(filename).is_file() == false {
        println!("File '{filename}' not found.");
        return ();
    }

    /*****************************************************
     * COUNT THE OCCURENCES IN THE LINES OF THE MATRIX
     */
    let list = read_lines(filename);
    total_occurences += count_occurences(&list, "XMAS");
    total_occurences += count_occurences(&list, "SAMX");

    /*****************************************************
     * COUNT THE OCCURENCES IN THE COLUMNS OF THE MATRIX
     */

    /*****************************************************
     * COUNT THE OCCURENCES IN THE DIAGONALS (BL to UR and reverse) OF THE MATRIX
     */

    /*****************************************************
     * COUNT THE OCCURENCES IN THE DIAGONALS (TL to BR and reverse) OF THE MATRIX
     */

    println!("Total occurences: {:?}", total_occurences);
}
