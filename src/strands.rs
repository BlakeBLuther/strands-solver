use std::collections::HashMap;
use combinatorial::Combinations;
// use rayon::prelude::*;

// Datastructure for the actual Strands puzzle
use crate::trie::Trie;

#[derive(PartialEq, Debug, Clone)]
pub struct Strands {
    pub puzzle: Vec<Vec<char>>,
    pub num_answers: usize,
}

impl Strands {
    pub fn new(input: String, answers: usize) -> Self {
        let mut strands = Strands {
            puzzle: vec![],
            num_answers: answers,
        };
        let mut row = 0;
        for line in input.lines() {
            strands.puzzle.push(vec![]);
            for c in line.chars() {
                strands.puzzle[row].push(c);
            }
            row += 1;
        }
        strands
    }

    pub fn print(&self) {
        for row in &self.puzzle {
            for c in row {
                print!(" {} ", *c);
            }
            print!("\n")
        }
    }

    pub fn solve(&self, trie: &Trie) -> Option<HashMap<String, Vec<(isize, isize)>>> {
        let rows = self.puzzle.len();
        let cols = self.puzzle[0].len();
        
        let mut candidates: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        for row in 0..rows {
            for col in 0..cols {
                let mut visited_tracker: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
                for row2 in row..rows {
                    for col2 in col..cols {
                        let starting_point = (row2 as isize, col2 as isize);
                        let mut guess_word = String::new();
                        let mut guess_coords = vec![];
                        if !visited_tracker[row2][col2] {
                            Self::solve_helper(
                                &self.puzzle,
                                trie,
                                starting_point,
                                &mut visited_tracker,
                                &mut guess_word,
                                &mut guess_coords,
                                &mut candidates,
                            );
                        }
                    }
                }
            }
        }
        //`candidates` is now all possible dict words formed from the puzzle.
        // next step is to identify the combination that meets our requirement (all nodes visited, no overlap)
        for candidate_words_and_coords in Combinations::of_size(candidates, self.num_answers) {
            let mut answer_found = true;
            let mut used_coords = vec![vec![false; cols]; rows];
            for word_and_coords in &candidate_words_and_coords {
                let candidate_coords = &word_and_coords.1;
                for coord in candidate_coords {
                    if used_coords[coord.0 as usize][coord.1 as usize] {
                        answer_found = false; //overlap identified, invalid solution
                        break
                    }
                    else {
                        used_coords[coord.0 as usize][coord.1 as usize] = true;
                    }
                }
            }
            for used_row in 0..used_coords.len() {
                for used_col in 0..used_coords[used_row].len() {
                    answer_found &= used_coords[used_row][used_col]; //bitwise AND with each entry in the used tracker matrix. if all true, valid answer. if answer_found was already false, fail.
                }
            }
            if answer_found {
                //reconstruct answer into appropriate return value
                let mut answer = HashMap::new();
                for (key, value) in candidate_words_and_coords {
                    answer.insert(key, value);
                }
                return Some(answer);
            }
        }
        return None
    }

    fn solve_helper(puzzle: &Vec<Vec<char>>, trie: &Trie, start: (isize, isize), visited: &mut Vec<Vec<bool>>, guess_word: &mut String, guess_coords: &mut Vec<(isize, isize)>, result: &mut HashMap<String, Vec<(isize, isize)>>) {
        // Given a coordinate `start`, will find a possible word starting at that point by recursively checking adjacent letters

        //Grid boundary checks
        let (row, col) = start;
        if row < 0 || col < 0 || row >= puzzle.len() as isize || col >= puzzle[0].len() as isize {
            // the comparisons to 0 are partly why the starting coords need to be an isize
            return;
        }
        if visited[row as usize][col as usize] {
            return;
        }

        //Append current letter to the guess
        guess_word.push(puzzle[row as usize][col as usize]);
        guess_coords.push((row, col));
        visited[row as usize][col as usize] = true;


        //If guess is valid (in dict and is word end), add to result
        if let Some(word) = trie.search(guess_word){
            if word.1 {
                result.insert(guess_word.clone(), guess_coords.clone());
            }
            let directions: [(isize, isize); 8] = [
                (-1, 0), (-1, 1), (0, 1), (1, 1),
                (1, 0), (1, -1), (0, -1), (-1, -1)
            ];
            for &(dir_row, dir_col) in directions.iter() {
                Self::solve_helper(
                    puzzle,
                    trie,
                    (start.0 + dir_row, start.1 + dir_col),
                    visited,
                    guess_word,
                    guess_coords,
                    result
                );
            }
        }
        visited[row as usize][col as usize] = false;
        guess_word.pop();
        guess_coords.pop();

    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::trie::Trie;
    use super::Strands;

    #[test]
    fn test_strands_new() {
        let inputstr = "AB\nCD";
        let strands = Strands::new(inputstr.to_string(), 1);
        let good = Strands {
            puzzle: vec![
                vec!['A', 'B'],
                vec!['C', 'D']
            ],
            num_answers: 1,
        };
        assert_eq!(strands, good);
    }

    #[test]
    fn test_solve_helper_recurse_1() {
        let strands = Strands::new("C".to_string(), 1);
        let mut trie = Trie::new();
        trie.insert("C".to_string());

        let good: HashMap<String, Vec<(isize, isize)>> = HashMap::from([
            ("C".to_string(), vec![(0 as isize, 0 as isize)])
        ]);
        let mut visited = vec![vec![false]];
        let mut guess_word = String::new();
        let guess_coords = &mut vec![];
        let mut result: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        Strands::solve_helper(&strands.puzzle, &trie, (0, 0), &mut visited, &mut guess_word, guess_coords, &mut result);
        assert_eq!(result, good)
    }

    #[test]
    fn test_solve_helper_recurse_2() {
        let strands = Strands::new("CA".to_string(), 1);
        let mut trie = Trie::new();
        trie.insert("CA".to_string());
        let good: HashMap<String, Vec<(isize, isize)>> = HashMap::from([
            ("CA".to_string(), vec![(0 as isize, 0 as isize), (0 as isize, 1 as isize)])
        ]);
        let mut visited = vec![vec![false; 2]];
        let mut guess_words = String::new();
        let guess_coords = &mut vec![];
        let mut result: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        Strands::solve_helper(&strands.puzzle, &trie, (0, 0), &mut visited, &mut guess_words, guess_coords, &mut result);

        assert_eq!(result, good) 
    }

    #[test]
    fn test_solve_helper_recurse_3() {
        let strands = Strands::new("CA\nTS".to_string(), 1);
        let mut trie = Trie::new();
        trie.insert("CAT".to_string());
        trie.insert("CATS".to_string());
        let mut good: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        good.insert("CAT".to_string(), vec![
            (0 as isize, 0 as isize), (0 as isize, 1 as isize),
            (1 as isize, 0 as isize)]);
        good.insert("CATS".to_string(), vec![
            (0 as isize, 0 as isize), (0 as isize, 1 as isize),
            (1 as isize, 0 as isize), (1 as isize, 1 as isize)]);
        let mut visited = vec![vec![false; 2]; 2];
        let mut guess_words = String::new();
        let guess_coords = &mut vec![];
        let mut result: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        Strands::solve_helper(&strands.puzzle, &trie, (0, 0), &mut visited, &mut guess_words, guess_coords, &mut result);
        assert_eq!(result, good)
    }

    #[test]
    fn test_solve_helper_recurse_4() {
        let strands = Strands::new("CA\nTD".to_string(), 1);
        let mut trie = Trie::new();
        trie.insert("CAT".to_string());
        let mut good: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        good.insert("CAT".to_string(), vec![
            (0 as isize, 0 as isize), (0 as isize, 1 as isize),
            (1 as isize, 0 as isize)]);
        let mut visited = vec![vec![false; 2]; 2];
        let mut guess_words = String::new();
        let guess_coords = &mut vec![];
        let mut result: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        Strands::solve_helper(&strands.puzzle, &trie, (0, 0), &mut visited, &mut guess_words, guess_coords, &mut result);
        assert_eq!(result, good)
    }

    #[test]
    fn test_solve_1() {
        let strands = Strands::new("CAT\nDOG\nBEE".to_string(), 3);
        // CAT
        // DOG
        // BEE
        let mut trie = Trie::new();
        trie.insert("CAT".to_string());
        trie.insert("DOG".to_string());
        trie.insert("BEE".to_string());
        let mut good: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        good.insert("CAT".to_string(), vec![
            (0 as isize, 0 as isize), (0 as isize, 1 as isize),
            (0 as isize, 2 as isize)]);
        good.insert("DOG".to_string(), vec![
            (1 as isize, 0 as isize), (1 as isize, 1 as isize),
            (1 as isize, 2 as isize)]);
        good.insert("BEE".to_string(), vec![
            (2 as isize, 0 as isize), (2 as isize, 1 as isize),
            (2 as isize, 2 as isize)]);
        let result = strands.solve(&trie).unwrap();
        assert_eq!(result, good)
    }

    #[test]
    fn test_solve_2() {
        let strands = Strands::new("GFE\nHAD\nIBC".to_string(), 1);
        // GFE
        // HAD
        // IBC

        let mut trie = Trie::new();
        trie.insert("ABCDEFGHI".to_string());

        let mut good: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        good.insert("ABCDEFGHI".to_string(), vec![
            (1 as isize, 1 as isize), (2 as isize, 1 as isize), (2 as isize, 2 as isize), 
            (1 as isize, 2 as isize), (0 as isize, 2 as isize), (0 as isize, 1 as isize),
            (0 as isize, 0 as isize), (1 as isize, 0 as isize), (2 as isize, 0 as isize)]);
        assert_eq!(strands.solve(&trie), Some(good));
    }

    #[test]
    fn test_solve_3() {
        //CAT
        //DOG
        //EES
        let strands = Strands::new("CAT\nDOG\nEES".to_string(), 3);
        let mut trie = Trie::new();
        trie.insert("CAT".to_string());
        trie.insert("DOG".to_string());
        trie.insert("SEE".to_string());
        trie.insert("DOGS".to_string());
        let mut good: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        good.insert("CAT".to_string(), vec![
            (0 as isize, 0 as isize), (0 as isize, 1 as isize),
            (0 as isize, 2 as isize)]);
        good.insert("DOG".to_string(), vec![
            (1 as isize, 0 as isize), (1 as isize, 1 as isize),
            (1 as isize, 2 as isize)]);
        good.insert("SEE".to_string(), vec![
            (2 as isize, 2 as isize), (2 as isize, 1 as isize),
            (2 as isize, 0 as isize)]);
        let result = strands.solve(&trie).unwrap();
        assert_eq!(result, good)
    }
}