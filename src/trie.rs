// Quick and dirty trie implementation

#[derive(Debug, PartialEq)]
pub struct Trie{
    root_node: Box<Node>,
}

#[derive(Debug, PartialEq)]
struct Node{
    letter: Option<char>, //root node has no char, so Option necessary
    children: Vec<Box<Node>>,
    word_end: bool,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root_node: Box::new(Node::new(None)) }
    }

    pub fn insert(&mut self, word: String) {
        // given a word, insert it into the trie
        let mut current_node = &mut self.root_node;
        for letter in word.chars() {
            let mut need_new = true;
            let mut index = 0;
            for child in &current_node.children {
                if child.letter.unwrap() == letter {
                    need_new = false;
                    break;
                }
                index += 1;
            }
            if need_new {
                let new_node = Box::new(Node::new(Some(letter)));
                current_node.children.push(new_node);
                current_node = current_node.children.last_mut().unwrap();
            }
            else {
                current_node = &mut current_node.children[index];
            }
        }
        current_node.word_end = true;
    }

    pub fn search(&self, word: &String) -> Option<(String, bool)> {
        //given a word, search for it in the trie
        //if found, returns the match (echos input) and if it's the end of word, otherwise returns None
        let mut current_node = &self.root_node;
        for letter_index in 0..word.len() {
            let mut found = false;
            let mut next_node_ind = 0;
            for child_index in 0..current_node.children.len() {
                if current_node.children[child_index].letter == word.chars().nth(letter_index) {
                    found = true;
                    next_node_ind = child_index;
                    break
                }
            }
            if found {
                current_node = &current_node.children[next_node_ind];
            }
            else {
                return None
            }
        }
        Some((word.to_string(), current_node.word_end))
    }

}

impl Node{
    fn new(letval: Option<char>) -> Self {
        Node {
            letter: letval,
            children: Vec::new(),
            word_end: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, Trie};

    #[test]
    fn test_trie_new() {
        let trie = Trie::new();
        let good = Trie { root_node: {
            Box::new(Node {
                letter: None,
                children: vec![],
                word_end: false,
            })}
        };
        assert_eq!(trie, good);
    }

    #[test]
    fn test_node_new() {
        let node = Node::new(Some('a'));
        let good = Node {
            letter: Some('a'),
            children: vec![],
            word_end: false,
        };
        assert_eq!(node, good);
    }

    #[test]
    fn test_insert_1() {
        let mut trie = Trie::new();
        trie.insert("dog".to_string());
        assert_eq!(trie.root_node.children.len(), 1);
        assert_eq!(trie.root_node.children[0].letter, Some('d'));
        assert_eq!(trie.root_node.children[0].children[0].letter, Some('o'));
        assert_eq!(trie.root_node.children[0].children[0].children[0].letter, Some('g'));
    }

    #[test]
    fn test_insert_2() {
        let mut trie = Trie::new();
        trie.insert("dog".to_string());
        trie.insert("cat".to_string());
        assert_eq!(trie.root_node.children.len(), 2);
        assert_eq!(trie.root_node.children[0].letter, Some('d'));
        assert_eq!(trie.root_node.children[0].children[0].letter, Some('o'));
        assert_eq!(trie.root_node.children[0].children[0].children[0].letter, Some('g'));

        assert_eq!(trie.root_node.children[1].letter, Some('c'));
        assert_eq!(trie.root_node.children[1].children[0].letter, Some('a'));
        assert_eq!(trie.root_node.children[1].children[0].children[0].letter, Some('t'));
    }

    #[test]
    fn test_insert_3() {
        let mut trie = Trie::new();
        trie.insert("dog".to_string());
        trie.insert("dad".to_string());
        assert_eq!(trie.root_node.children.len(), 1);
        assert_eq!(trie.root_node.children[0].letter, Some('d'));
        assert_eq!(trie.root_node.children[0].children[0].letter, Some('o'));
        assert_eq!(trie.root_node.children[0].children[0].children[0].letter, Some('g'));

        assert_eq!(trie.root_node.children[0].letter, Some('d'));
        assert_eq!(trie.root_node.children[0].children[1].letter, Some('a'));
        assert_eq!(trie.root_node.children[0].children[1].children[0].letter, Some('d'));
    }

    #[test]
    fn test_insert_4() {
        let mut trie = Trie::new();
        trie.insert("dog".to_string());
        assert!(!trie.root_node.word_end);
    }

    #[test]
    fn test_insert_5() {
        let mut trie = Trie::new();
        trie.insert("dog".to_string());
        assert!(trie.root_node.children[0].children[0].children[0].word_end);
    }

    #[test]
    fn test_insert_6() {
        let mut trie = Trie::new();
        trie.insert("cat".to_string());
        trie.insert("cats".to_string());
        assert_eq!(trie.root_node.children.len(), 1);
        assert_eq!(trie.root_node.children[0].letter, Some('c'));
        assert_eq!(trie.root_node.children[0].children[0].letter, Some('a'));
        assert_eq!(trie.root_node.children[0].children[0].children[0].letter, Some('t'));
        assert!(trie.root_node.children[0].children[0].children[0].word_end);

        assert_eq!(trie.root_node.children[0].children[0].children[0].children[0].letter, Some('s'));
        assert!(trie.root_node.children[0].children[0].children[0].children[0].word_end);
    }

    #[test]
    fn test_search_1() {
        let mut trie = Trie::new();
        trie.insert("dog".to_string());
        trie.insert("cat".to_string());

        assert_eq!(
            trie.search(&"dog".to_string()),
            Some(("dog".to_string(), true))
        );
        assert_eq!(
            trie.search(&"cat".to_string()),
            Some(("cat".to_string(), true))
        )
    }

    #[test]
    fn test_search_2() {
        let mut trie = Trie::new();
        trie.insert("dog".to_string());
        trie.insert("cat".to_string());

        assert_eq!(
            trie.search(&"asdf".to_string()),
            None
        );
    }

    #[test]
    fn test_search_3() {
        use std::io::BufRead;

        let file_path = std::path::Path::new("./english-words/words_alpha.txt");
        let file = std::fs::File::open(file_path).unwrap();
        let reader = std::io::BufReader::new(file);
    
        let mut trie = Trie::new();
        for line in reader.lines() {
            let line = line.unwrap();
            trie.insert(line);
        }

        assert_eq!(
            trie.search(&"cat".to_string()),
            Some(("cat".to_string(), true))
        )
    }

    #[test]
    fn test_search_4() {
        use std::io::BufRead;

        let file_path = std::path::Path::new("./english-words/words_alpha.txt");
        let file = std::fs::File::open(file_path).unwrap();
        let reader = std::io::BufReader::new(file);
    
        let mut trie = Trie::new();
        for line in reader.lines() {
            let line = line.unwrap();
            trie.insert(line);
        }

        assert_eq!(
            trie.search(&"cat".to_string()),
            Some(("cat".to_string(), true))
        );

        assert_eq!(
            trie.search(&"catastrophe".to_string()),
            Some(("catastrophe".to_string(), true))
        )
    }

    
}