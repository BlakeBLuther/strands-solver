use std::collections::{HashMap, HashSet};

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
                            Self::recurse_find_words(
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
        println!("Found {:?} potential words.", candidates.keys().len());
        let mut word_list = Vec::new();
        for word in candidates.keys() {
            word_list.push(word.clone());
        }
        word_list.sort_by(|x, y| y.len().cmp(&x.len()));
        println!("Words: ");
        for word in word_list {
            println!("{:?}", word);
        }

        let mut potential_solutions = vec![];
        let depth = 1;
        Self::recurse_find_solution(&self.puzzle, &mut potential_solutions, &candidates, self.num_answers, depth);
        let mut result: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        for word in potential_solutions {
            result.insert(word.clone(), candidates.get(&word).unwrap().to_vec());
        }
        return Some(result);
    }

    fn recurse_find_words(puzzle: &Vec<Vec<char>>, trie: &Trie, start: (isize, isize), visited: &mut Vec<Vec<bool>>, guess_word: &mut String, guess_coords: &mut Vec<(isize, isize)>, result: &mut HashMap<String, Vec<(isize, isize)>>) {
        // Given a coordinate `start`, will find all possible words starting at that point by recursively checking adjacent letters

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
                Self::recurse_find_words(
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

    fn recurse_find_solution(puzzle: &Vec<Vec<char>>, solution: &mut Vec<String>, candidates: &HashMap<String, Vec<(isize, isize)>>, num_answers: usize, mut depth: usize) {
        // Given a list of candidates, will find the one that uses all possible words with no overlap (the solution).
        if depth == num_answers {
            //check to guarantee full coverage of the puzzle
            let mut used_coords = vec![
                vec![
                    false; puzzle[0].len() as usize
                ]; puzzle.len() as usize
            ];
            for word in &mut *solution { 
                for coord in candidates.get(word).unwrap() {
                    used_coords[coord.0 as usize][coord.1 as usize] = true;
                }
            }
            let mut all_used = true;
            for row in used_coords {
                for col in row {
                    all_used &= col;
                }
            }
            if all_used {
                //answer found, it's stored in `solution`
                return;
            } 
        }
        //haven't hit maximum depth yet. still potential solutions.
        let mut potential_next = candidates.clone();
        for word in &mut *solution {
            potential_next.remove(word);
        } //`potential_next` and `solution` now have no equal elements
        for word1 in potential_next.keys() {
            let mut overlap_found = false;
            for word2 in &mut *solution {
                if Self::has_overlap(candidates, word1, word2) {
                    overlap_found = true;
                }
            }
            if !overlap_found {
                solution.push(word1.to_string());
                depth += 1;
                Self::recurse_find_solution(puzzle, solution, candidates, num_answers, depth);
                solution.pop();
                depth -= 1;
            }
        }
    }

    fn has_overlap(candidates: &HashMap<String, Vec<(isize, isize)>>, a: &String, b: &String) -> bool {
        let a_coords: HashSet<_> = candidates.get(a).unwrap().iter().collect();
        let b_coords: HashSet<_> = candidates.get(b).unwrap().iter().collect();
        return !a_coords.is_disjoint(&b_coords);
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
    fn test_recurse_find_words_1() {
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
        Strands::recurse_find_words(&strands.puzzle, &trie, (0, 0), &mut visited, &mut guess_word, guess_coords, &mut result);
        assert_eq!(result, good)
    }

    #[test]
    fn test_recurse_find_words_2() {
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
        Strands::recurse_find_words(&strands.puzzle, &trie, (0, 0), &mut visited, &mut guess_words, guess_coords, &mut result);

        assert_eq!(result, good) 
    }

    #[test]
    fn test_recurse_find_words_3() {
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
        Strands::recurse_find_words(&strands.puzzle, &trie, (0, 0), &mut visited, &mut guess_words, guess_coords, &mut result);
        assert_eq!(result, good)
    }

    #[test]
    fn test_recurse_find_words_4() {
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
        Strands::recurse_find_words(&strands.puzzle, &trie, (0, 0), &mut visited, &mut guess_words, guess_coords, &mut result);
        assert_eq!(result, good)
    }

    #[test]
    fn test_find_overlap_1() {
        let mut candidates: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        candidates.insert("A".to_string(), vec![(0,0)]);
        let a = "A".to_string();
        let b = "A".to_string();
        assert!(Strands::has_overlap(&candidates, &a, &b))
    }

    #[test]
    fn test_find_overlap_2() {
        let mut candidates: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        candidates.insert("ABC".to_string(), vec![(0,0),(0,1),(1,0)]);
        candidates.insert("BCD".to_string(), vec![(0,1),(1,0),(1,1)]);
        let a = "ABC".to_string();
        let b = "BCD".to_string();
        assert!(Strands::has_overlap(&candidates, &a, &b))
    }

    #[test]
    fn test_find_overlap_3() {
        let mut candidates: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        candidates.insert("ABCDEFG".to_string(), vec![(0,0),(0,1),(0,2),(1,0),(1,1),(1,2),(2,0),(2,1),(2,2)]);
        candidates.insert("D".to_string(), vec![(1,1)]);
        let a = "ABCDEFG".to_string();
        let b = "D".to_string();
        assert!(Strands::has_overlap(&candidates, &a, &b))
    }

    #[test]
    fn test_recurse_find_solution_1() {
        let good = vec!["ABCD".to_string()];
        
        let strands = Strands::new("AB\nCD".to_string(),1);
        let mut solution = Vec::new();
        let mut candidates: HashMap<String, Vec<(isize, isize)>> = HashMap::new();
        candidates.insert("ABCD".to_string(), vec![(0,0),(0,1),(1,0),(1,1)]);
        let depth = 1;
        Strands::recurse_find_solution(&strands.puzzle, &mut solution, &candidates, strands.num_answers, depth);
        
        assert_eq!(solution, good);
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