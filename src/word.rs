use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Word {
    pub word: String,
    pub chars: HashMap<char, usize>,
}

impl Word {
    pub fn new(word: &str) -> Word {
        let word = word.to_string().to_ascii_uppercase();
        Word {
            chars: count_chars(&word),
            word,
        }
    }

    pub fn random_from_list(words: &[String]) -> Word {
        let idx = fastrand::usize(..words.len());
        Word::new(&words[idx])
    }
}

pub fn count_chars(text: &str) -> HashMap<char, usize> {
    let mut chars = HashMap::new();
    for char in text.chars() {
        match chars.get_mut(&char) {
            Some(count) => *count += 1,
            None => {
                chars.insert(char, 1);
            }
        }
    }
    chars
}
