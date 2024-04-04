// Datastructure for the actual Strands puzzle

use std::thread::current;

use crate::trie::Trie;

#[derive(PartialEq, Debug, Clone)]
pub struct Strands {
    pub puzzle: Vec<Vec<Cell>>
}

#[derive(PartialEq, Debug, Clone)]
pub struct Cell {
    letter: char,
    col: usize,
    row: usize,
}

pub struct UsedMatrix {
    pub matrix: Vec<Vec<bool>>
}

impl Strands {
    pub fn new(input: String) -> Self {
        let mut strands = Strands {
            puzzle: vec![],
        };
        let mut i = 0;
        for line in input.lines() {
            let mut j = 0;
            strands.puzzle.push(vec![]);
            for char in line.chars() {
                strands.puzzle[i].push(
                    Cell {
                        letter: char,
                        col: j,
                        row: i,
                    }
                );
                j += 1;
            }
            i += 1;
        }
        strands
    }

    pub fn print(&self) {
        for row in &self.puzzle {
            for cell in row {
                print!(" {} ", cell.letter);
            }
            print!("\n")
        }
    }

    
    pub fn solve(&self, num_words: usize, trie: &Trie) -> Option<Vec<Vec<&Cell>>> {
        let row_max = self.puzzle.len();
        let col_max = self.puzzle[0].len();
        for row in 0..row_max {
            for col in 0..col_max {
                if let Some(solution) = self.solve_helper((row, col), trie) {
                    if solution.len() == num_words {
                        return Some(solution);
                    }
                }
            }
        }
        None
    }

    fn solve_helper(&self, start: (usize, usize), trie: &Trie) -> Option<Vec<Vec<&Cell>>> {
        let mut used_matrix = UsedMatrix::new(self);
        let row_start = start.0;        
        let col_start = start.1;
        let row_max = self.puzzle.len();
        let col_max = self.puzzle[0].len();
        let mut found_words: Vec<Vec<&Cell>> = vec![];
        for row in row_start..(row_start+row_max) {
            for col in col_start..(col_start+col_max) {
                let current_guess = vec![
                    &self.puzzle[row % row_max][col % col_max]];
                if let Some(potential_words) = self.solve_helper_recurse(
                    current_guess, &used_matrix, &trie) {
                        //todo: given a list of potential words radiating out from a single letter,
                        // figure out how to select one of those words
                }
            }
        }
        if found_words.len() == 0 {
            None
        }
        else {
            Some(found_words)
        }
    }

    fn solve_helper_recurse<'a>(&'a self, current_guess: Vec<&'a Cell>, um: &UsedMatrix, trie: &Trie) -> Option<Vec<Vec<&Cell>>> {
        // from a starting cell, get all potential words radiating out from that cell.
        // ignores letters already marked used in the UsedMatrix
        match trie.search(&cells_to_string(&current_guess)) {
            None => { return None }
            Some((_, end_of_word)) => {
                let mut valid_words: Vec<Vec<&Cell>> = Vec::new();
                let adjacents: Vec<&Cell> = self.get_adjacent(
                    (current_guess.last().unwrap().row, current_guess.last().unwrap().col), 
                    &um);
                for next_cell in adjacents {
                    // check if next potential cell is not a doubleback
                    let mut already_used = false;
                    for used in &current_guess {
                        if **used == *next_cell {
                            already_used = true;
                        }
                    }
    
                    if !already_used {
                        let mut next_guess: Vec<&Cell> = current_guess.clone();
                        next_guess.push(next_cell);
                        if let Some(mut found_words) = self.solve_helper_recurse(next_guess, um, trie) {
                            valid_words.append(&mut found_words);
                        }
                    }
                }
                if valid_words.len() == 0 {
                    // none of the adjacent cells return a valid word
                    if end_of_word {
                        Some(vec![current_guess])
                    } else { None }
                } else { None }
             }
        }
    }

    fn get_adjacent(&self, coords: (usize, usize), um: &UsedMatrix) -> Vec<&Cell> {
        let row_max: isize = self.puzzle.len().try_into().unwrap();
        let col_max: isize = self.puzzle.last().unwrap().len().try_into().unwrap();
        let mut neighbors = vec![];

        for row_offset in -1..=1 {
            for col_offset in -1..=1 {
                let row_target = coords.0 as isize + row_offset;
                let col_target = coords.1 as isize + col_offset;

                if row_offset == 0 && col_offset == 0 {
                    //pass - this is the current cell
                }
                else if row_target < 0 || col_target < 0 || row_target >= row_max || col_target >= col_max {
                    //pass
                }
                else {
                    if !um.matrix[row_target as usize][col_target as usize] {
                        let target = &self.puzzle[row_target as usize][col_target as usize];
                        neighbors.push(target);
                    }
                }
            }
        }
        neighbors
    }
}

impl Cell {
    fn new(letter: char) -> Self {
        Cell {
            letter,
            col: 0,
            row: 0,
        }
    }
}

impl UsedMatrix {
    fn new(strands: &Strands) -> Self {
        let mut matrix: Vec<Vec<bool>> = vec![];
        for row in 0..strands.puzzle.len() {
            matrix.push(vec![false; strands.puzzle[row].len()])
        }
        UsedMatrix {
            matrix
        }
    }
}

pub fn cells_to_string(cells: &Vec<&Cell>) -> String {
    let mut s: String = "".to_string();
    for cell in cells {
        s.push(cell.letter)
    }
    s
}

#[cfg(test)]
mod tests {
    use crate::strands::UsedMatrix;
    use crate::trie::Trie;

    use super::Strands;
    use super::Cell;

    #[test]
    fn test_strands_new() {
        let inputstr = "AB\nCD";
        let strands = Strands::new(inputstr.to_string());
        let good = Strands {
            puzzle: vec![vec![
                Cell {letter: 'A', row: 0, col: 0},
                Cell {letter: 'B', row: 0, col: 1}],
            vec![
                Cell {letter: 'C', row: 1, col: 0},
                Cell {letter: 'D', row: 1, col: 1}]],
        };
        assert_eq!(strands, good);
    }

    #[test]
    fn test_strands_neighbors_1() {
        let strands = Strands {
            puzzle: vec![vec![
                Cell {letter: 'A', row: 0, col: 0},
                Cell {letter: 'A', row: 0, col: 1}],
            vec![
                Cell {letter: 'A', row: 1, col: 0},
                Cell {letter: 'A', row: 1, col: 1}]]};
        let um = UsedMatrix {
            matrix: vec![
                vec![false, false],
                vec![false, false]
            ],
        };
        let neighbors = strands.get_adjacent((0,0), &um);
        
        let good = vec![
            &Cell {letter: 'A', row: 0, col: 1},
            &Cell {letter: 'A', row: 1, col: 0},
            &Cell {letter: 'A', row: 1, col: 1},
        ];

        assert_eq!(neighbors, good);
    }

    #[test]
    fn test_strands_neighbors_2() {
        let strands = Strands {
            puzzle: vec![vec![
                Cell {letter: 'A', row: 0, col: 0},
                Cell {letter: 'B', row: 0, col: 1}],
            vec![
                Cell {letter: 'C', row: 1, col: 0},
                Cell {letter: 'D', row: 1, col: 1}]],
        };
        let um = UsedMatrix {
            matrix: vec![
                vec![false, false],
                vec![false, false]
            ],
        };
        let neighbors = strands.get_adjacent((0,0), &um);

        let good = vec![
            &Cell {letter: 'B', row: 0, col: 1},
            &Cell {letter: 'C', row: 1, col: 0},
            &Cell {letter: 'D', row: 1, col: 1},
        ];
        assert_eq!(neighbors, good);
    }

    #[test]
    fn test_strands_neighbors_3() {
        let strands = Strands {
            puzzle: vec![vec![
                Cell {letter: 'A', row: 0, col: 0},
                Cell {letter: 'B', row: 0, col: 1}],
            vec![
                Cell {letter: 'C', row: 1, col: 0},
                Cell {letter: 'D', row: 1, col: 1}]],
        };
        let um = UsedMatrix {
            matrix: vec![
                vec![false, false],
                vec![false, false]
            ],
        };
        let neighbors = strands.get_adjacent((1,1), &um);

        let good = vec![
            &Cell {letter: 'A', row: 0, col: 0},
            &Cell {letter: 'B', row: 0, col: 1},
            &Cell {letter: 'C', row: 1, col: 0},
        ];
        assert_eq!(neighbors, good);
    }

    #[test]
    fn test_strands_neighbors_4() {
        let inputstr = "ABCDE\nFGHIJ\nKLMNO\nPQRST\nUVWXY";
        // A B C D E
        // F(G)H I J
        // K L M N O 
        // P Q R S T
        // U V W X Y
        let strands = Strands::new(inputstr.to_string());
        let um = UsedMatrix::new(&strands);
        let neighbors = strands.get_adjacent((1,1), &um);
        let good = vec![
            &Cell {letter: 'A', row: 0, col: 0},
            &Cell {letter: 'B', row: 0, col: 1},
            &Cell {letter: 'C', row: 0, col: 2},
            &Cell {letter: 'F', row: 1, col: 0},
            &Cell {letter: 'H', row: 1, col: 2},
            &Cell {letter: 'K', row: 2, col: 0},
            &Cell {letter: 'L', row: 2, col: 1},
            &Cell {letter: 'M', row: 2, col: 2},
        ];
        assert_eq!(neighbors, good);
    }

    #[test]
    fn test_strands_neighbors_5() {
        let inputstr = "ABCDE\nFGHIJ\nKLMNO\nPQRST\nUVWXY";
        let strands = Strands::new(inputstr.to_string());
        let mut um = UsedMatrix::new(&strands);
        um.matrix[0][0] = true;
        um.matrix[0][1] = true;
        um.matrix[0][2] = true;
        let neighbors = strands.get_adjacent((1,1), &um);
        let good = vec![
            &Cell {letter: 'F', row: 1, col: 0},
            &Cell {letter: 'H', row: 1, col: 2},
            &Cell {letter: 'K', row: 2, col: 0},
            &Cell {letter: 'L', row: 2, col: 1},
            &Cell {letter: 'M', row: 2, col: 2},
        ];
        assert_eq!(neighbors, good);
    }

    #[test]
    fn test_strands_neighbors_6() {
        let strands = Strands::new("GFE\nHAD\nIBC".to_string());
        // G F E
        // H A D
        // I B C
        let um = UsedMatrix::new(&strands);
        let neighbors = strands.get_adjacent((1,2), &um);
        let good = vec![
            &Cell {letter: 'F', row: 0, col: 1},
            &Cell {letter: 'E', row: 0, col: 2},
            &Cell {letter: 'A', row: 1, col: 1},
            &Cell {letter: 'B', row: 2, col: 1},
            &Cell {letter: 'C', row: 2, col: 2},
        ];
        assert_eq!(neighbors, good);

    }

    #[test]
    fn test_solve_helper_recurse_1() {
        let strands = Strands::new("C".to_string());
        let mut trie = Trie::new();
        trie.insert("C".to_string());
        let input_guess = vec![&Cell {
            letter: 'C',
            col: 0,
            row: 0,
        }];

        let good = vec![ &Cell {
            letter: 'C',
            row: 0,
            col: 0,
        }];
        let um = UsedMatrix {
            matrix: vec![
                vec![false, false],
                vec![false, false]
            ],
        };
        assert_eq!(strands.solve_helper_recurse(input_guess, &um, &trie), Some(good)) 
    }

    #[test]
    fn test_solve_helper_recurse_2() {
        let strands = Strands::new("CA".to_string());
        let mut trie = Trie::new();
        trie.insert("CA".to_string());
        let input_guess = vec![&Cell {
            letter: 'C',
            col: 0,
            row: 0,
        }];

        let good = vec![ &Cell {
            letter: 'C',
            row: 0,
            col: 0,
        }, &Cell {
            letter: 'A',
            row: 0,
            col: 1,
        }];
        let um = UsedMatrix {
            matrix: vec![
                vec![false, false],
                vec![false, false]
            ],
        };

        assert_eq!(strands.solve_helper_recurse(input_guess, &um, &trie), Some(good)) 
    }


    #[test]
    fn test_solve_helper_recurse_3() {
        let strands = Strands::new("CA\nTS".to_string());
        let mut trie = Trie::new();
        trie.insert("CAT".to_string());
        trie.insert("CATS".to_string());
        let input_guess = vec![&Cell {
            letter: 'C',
            col: 0,
            row: 0,
        }];

        let good = vec![ &Cell {
            letter: 'C',
            row: 0,
            col: 0,
        }, &Cell {
            letter: 'A',
            row: 0,
            col: 1,
        }, &Cell {
            letter: 'T',
            row: 1,
            col: 0,
        }, &Cell {
            letter: 'S',
            row: 1,
            col: 1,
        }];
        let um = UsedMatrix {
            matrix: vec![
                vec![false, false],
                vec![false, false]
            ],
        };

        assert_eq!(strands.solve_helper_recurse(input_guess, &um, &trie), Some(good)) 
    }
    
    #[test]
    fn test_solve_recurse_4() {
        let strands = Strands::new("CAT\nDOG\nEES".to_string());
        let mut trie = Trie::new();
        trie.insert("CAT".to_string());
        trie.insert("DOG".to_string());
        trie.insert("SEE".to_string());
        trie.insert("DOGS".to_string());

        let um = UsedMatrix {
            matrix: vec![
                vec![false, false, false],
                vec![false, false, false],
                vec![false, false, false],
            ],
        };
        let input_guess = vec![&Cell {
            letter: 'D',
            row: 1,
            col: 0,
        }];
        let good = vec![ &Cell {
            letter: 'D',
            row: 1,
            col: 0,
        }, &Cell {
            letter: 'O',
            row: 1,
            col: 1,
        }, &Cell {
            letter: 'G',
            row: 1,
            col: 2,
        }, &Cell {
            letter: 'S',
            row: 2,
            col: 2,
        }];
        assert_eq!(strands.solve_helper_recurse(input_guess, &um, &trie), Some(good));
        
    }

    #[test]
    fn test_solve_recurse_5() {
        let strands = Strands::new("GFE\nHAD\nIBC".to_string());
        // G F E
        // H A D
        // I B C

        let mut trie = Trie::new();
        trie.insert("ABCDEFGHI".to_string());

        let um = UsedMatrix {
            matrix: vec![
                vec![false, false, false],
                vec![false, false, false],
                vec![false, false, false],
            ],
        };
        let input_guess = vec![&Cell {
            letter: 'A',
            row: 1,
            col: 1,
        }];
        let good = vec![ &Cell {
            letter: 'A',
            row: 1,
            col: 1,
        }, &Cell {
            letter: 'B',
            row: 2,
            col: 1,
        }, &Cell {
            letter: 'C',
            row: 2,
            col: 2,
        }, &Cell {
            letter: 'D',
            row: 1,
            col: 2,
        }, &Cell {
            letter: 'E',
            row: 0,
            col: 2,
        }, &Cell {
            letter: 'F',
            row: 0,
            col: 1,
        }, &Cell {
            letter: 'G',
            row: 0,
            col: 0,
        }, &Cell {
            letter: 'H',
            row: 1,
            col: 0,
        }, &Cell {
            letter: 'I',
            row: 2,
            col: 0,
        }];
        assert_eq!(strands.solve_helper_recurse(input_guess, &um, &trie), Some(good));
        
    }

    #[test]
    fn test_solve_helper_recurse_6() {
        let strands = Strands::new("CAT\nSSR".to_string());
        let mut trie = Trie::new();
        trie.insert("CAT".to_string());
        trie.insert("CARS".to_string());
        let input_guess = vec![&Cell {
            letter: 'C',
            col: 0,
            row: 0,
        }];

        let good = vec![ &Cell {
            letter: 'C',
            row: 0,
            col: 0,
        }, &Cell {
            letter: 'A',
            row: 0,
            col: 1,
        }, &Cell {
            letter: 'R',
            row: 1,
            col: 2,
        }, &Cell {
            letter: 'S',
            row: 1,
            col: 1,
        }];
        let um = UsedMatrix {
            matrix: vec![
                vec![false, false, false],
                vec![false, false, false]
            ],
        };

        assert_eq!(strands.solve_helper_recurse(input_guess, &um, &trie), Some(good)) 
    }

    #[test]
    fn test_solve_helper_1() {
        let strands = Strands::new("CA\nTD".to_string());
        let mut trie = Trie::new();
        trie.insert("CAT".to_string());
        let start = (0 as usize, 0 as usize);
        let good =vec![
            vec![ &Cell {
                letter: 'C',
                row: 0,
                col: 0,
            }, &Cell {
                letter: 'A',
                row: 0,
                col: 1,
            }, &Cell {
                letter: 'T',
                row: 1,
                col: 0,
            }]];
        assert_eq!(strands.solve_helper(start, &trie), Some(good));
    }

    #[test]
    fn test_solve_helper_2() {
        let strands = Strands::new("CA\nTS".to_string());
        let mut trie = Trie::new();
        trie.insert("CAT".to_string());
        trie.insert("CATS".to_string());
        let start = (0 as usize, 0 as usize);
        let good = vec![
            vec![ &Cell {
                letter: 'C',
                row: 0,
                col: 0,
            }, &Cell {
                letter: 'A',
                row: 0,
                col: 1,
            }, &Cell {
                letter: 'T',
                row: 1,
                col: 0,
            }, &Cell {
                letter: 'S',
                row: 1,
                col: 1,
            }]];
        assert_eq!(strands.solve_helper(start, &trie), Some(good));
    }

    #[test]
    fn test_solve_helper_3() {
        let strands = Strands::new("CAT\nRSZ".to_string());
        let mut trie = Trie::new();
        trie.insert("CAT".to_string());
        trie.insert("CARS".to_string());
        let start = (0 as usize, 0 as usize);
        let good = vec![
            vec![ &Cell {
                letter: 'C',
                row: 0,
                col: 0,
            }, &Cell {
                letter: 'A',
                row: 0,
                col: 1,
            }, &Cell {
                letter: 'R',
                row: 1,
                col: 0,
            }, &Cell {
                letter: 'S',
                row: 1,
                col: 1,
            }]];
        assert_eq!(strands.solve_helper(start, &trie), Some(good));
    }

    #[test]
    fn test_solve_helper_4() {
        let strands = Strands::new("ACT\nRSZ".to_string());
        let mut trie = Trie::new();
        trie.insert("CAT".to_string());
        trie.insert("CARS".to_string());
        let start = (0 as usize, 0 as usize);
        let good = vec![
            vec![ &Cell {
                letter: 'C',
                row: 0,
                col: 1,
            }, &Cell {
                letter: 'A',
                row: 0,
                col: 0,
            }, &Cell {
                letter: 'R',
                row: 1,
                col: 0,
            }, &Cell {
                letter: 'S',
                row: 1,
                col: 1,
            }]];
        assert_eq!(strands.solve_helper(start, &trie), Some(good));
    }

    #[test]
    fn test_solve_helper_5() {
        let strands = Strands::new("CAT\nDOG\nBEE".to_string());
        // CAT
        // DOG
        // BEE
        let mut trie = Trie::new();
        trie.insert("CAT".to_string());
        trie.insert("DOG".to_string());
        trie.insert("BEE".to_string());
        let good = vec![
            vec![ &Cell {
                letter: 'C',
                row: 0,
                col: 0,
            }, &Cell {
                letter: 'A',
                row: 0,
                col: 1,
            }, &Cell {
                letter: 'T',
                row: 0,
                col: 2,
            }], vec! [ &Cell {
                letter: 'D',
                row: 1,
                col: 0,
            }, &Cell {
                letter: 'O',
                row: 1,
                col: 1,
            }, &Cell {
                letter: 'G',
                row: 1,
                col: 2,
            }], vec![ &Cell {
                letter: 'B',
                row: 2,
                col: 0,
            }, &Cell {
                letter: 'E',
                row: 2,
                col: 1,
            }, &Cell {
                letter: 'E',
                row: 2,
                col: 2,
            }]];
        assert_eq!(strands.solve_helper((0, 0), &trie), Some(good));
    }

    #[test]
    fn test_solve_1() {
        let strands = Strands::new("ACT\nRSZ".to_string());
        let mut trie = Trie::new();
        trie.insert("CAT".to_string());
        trie.insert("CARS".to_string());
        let good = vec![
            vec![ &Cell {
                letter: 'C',
                row: 0,
                col: 1,
            }, &Cell {
                letter: 'A',
                row: 0,
                col: 0,
            }, &Cell {
                letter: 'R',
                row: 1,
                col: 0,
            }, &Cell {
                letter: 'S',
                row: 1,
                col: 1,
            }]];
        assert_eq!(strands.solve(1, &trie), Some(good));
    }

    #[test]
    fn test_solve_2() {
        let strands = Strands::new("CAT\nDOG\nBEE".to_string());
        let mut trie = Trie::new();
        trie.insert("CAT".to_string());
        trie.insert("DOG".to_string());
        trie.insert("BEE".to_string());
        let good = vec![
            vec![ &Cell {
                letter: 'C',
                row: 0,
                col: 0,
            }, &Cell {
                letter: 'A',
                row: 0,
                col: 1,
            }, &Cell {
                letter: 'T',
                row: 0,
                col: 2,
            }], vec! [ &Cell {
                letter: 'D',
                row: 1,
                col: 0,
            }, &Cell {
                letter: 'O',
                row: 1,
                col: 1,
            }, &Cell {
                letter: 'G',
                row: 1,
                col: 2,
            }], vec![ &Cell {
                letter: 'B',
                row: 2,
                col: 0,
            }, &Cell {
                letter: 'E',
                row: 2,
                col: 1,
            }, &Cell {
                letter: 'E',
                row: 2,
                col: 2,
            }]];
        assert_eq!(strands.solve(3, &trie), Some(good));
    }
}