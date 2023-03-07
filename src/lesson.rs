use std::{
    io::{stdout, Write},
    thread,
    time::{Duration, Instant},
};

use color_eyre::{owo_colors::OwoColorize, Result};
use dialoguer::Completion;
use dialoguer::{theme::ColorfulTheme, Input};
use rand::{seq::SliceRandom, thread_rng};
use strsim::levenshtein;

use crate::data::Data;

pub static WORDS: &'static str = include_str!("../assets/wordlist.txt");

pub fn choose_word() -> String {
    let words: Vec<_> = WORDS.split::<&str>("\n").collect();

    let mut rng = thread_rng();
    words.choose(&mut rng).unwrap().to_string()
}

pub fn random_setence(word_count: u8) -> String {
    let mut setence = vec![];

    for _ in 0..word_count {
        setence.push(choose_word());
    }

    setence.join(" ")
}

pub fn diff_percent(a: &str, b: &str) -> usize {
    let len = a.len().max(b.len());
    let diff = levenshtein(a, b);

    (100 * (len - diff)) / len
}

pub fn lesson(data: &mut Data) -> Result<()> {
    let setence = random_setence(4);
    let text = format!(
        "{} {} {}",
        "Your setence is:".magenta(),
        setence.green(),
        "REMEMBER THAT IT WILL DISAPPEAR IN FIVE SECONDS!".red()
    );

    let mut stdout = stdout();
    write!(stdout, "{text}\r")?;
    stdout.flush()?;

    thread::sleep(Duration::from_secs(5));
    write!(stdout, "{}\r", " ".repeat(text.len()))?;
    stdout.flush()?;

    writeln!(stdout)?;

    let wc = WordCompletion::default();

    let started = Instant::now();
    let user_setence: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What was the setence?")
        .completion_with(&wc)
        .validate_with(|input: &String| -> Result<(), String> {
            let percent = diff_percent(&input, &setence);
            if percent >= 92 {
                Ok(())
            } else {
                Err(format!(
                    "Incorrect response! (You must 92% of correctness. And you have: {}%.)",
                    percent
                ))
            }
        })
        .interact_text()?;

    let percent = diff_percent(&user_setence, &setence) as i64;
    let xp: i64 = (percent * started.elapsed().as_millis() as i64
        / started.elapsed().as_secs() as i64)
        % 1000;

    data.xp += xp;

    Ok(())
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
        self.words
            .iter()
            .filter(|option| option.starts_with(input))
            .collect::<Vec<_>>()
            .first()
            .map(ToOwned::to_owned)
            .cloned()
    }
}
