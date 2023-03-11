use dialoguer::Completion;
use rand::{seq::SliceRandom, thread_rng};
use strsim::levenshtein;

use crate::data::Data;

#[macro_export]
macro_rules! put {
    ($text:expr) => {
        println!("{}", $text)
    };
}

pub static WORDS: &'static str = include_str!("../assets/wordlist.txt");

pub fn choose_word() -> String {
    let words: Vec<_> = WORDS.split::<&str>("\n").collect();

    let mut rng = thread_rng();
    words.choose(&mut rng).unwrap().to_string()
}

pub fn random_words(word_count: u8) -> Vec<String> {
    let mut words = vec![];

    for _ in 0..word_count {
        words.push(choose_word());
    }

    words
}

pub fn diff_percent(a: &str, b: &str) -> usize {
    let len = a.len().max(b.len());
    let diff = levenshtein(a, b);

    (100 * (len - diff)) / len
}

pub fn get_word_amount(data: &Data) -> u8 {
    ((data.xp / 1000) % 10 + 1) as u8
}

pub struct WordCompletion {
    pub words: Vec<String>,
}

impl Default for WordCompletion {
    fn default() -> Self {
        Self {
            words: WORDS.split::<&str>("\n").map(|x| x.to_string()).collect(),
        }
    }
}

impl Completion for WordCompletion {
    fn get(&self, input: &str) -> Option<String> {
        let mut words = self.words.clone();
        words.sort_by_key(|x| x.len());
        words.sort_by_cached_key(|x| levenshtein(x, input));
        words.first().map(ToOwned::to_owned)
    }
}
