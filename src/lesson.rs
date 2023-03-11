use std::io::stdout;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::utils::diff_percent;
use crate::utils::WordCompletion;
use crate::{
    data::Data,
    utils::{get_word_amount, random_words},
};
use color_eyre::{owo_colors::OwoColorize, Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;

pub fn lesson(data: &mut Data) -> Result<()> {
    let mut stdout = stdout().lock();
    let words = random_words(get_word_amount(data));
    let fmt_words = words
        .iter()
        .map(|x| x.blue().to_string())
        .collect::<Vec<_>>()
        .join(", ");

    let raw_fmt_words = words
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");

    writeln!(stdout, "{}", "=".repeat(40))?;

    write!(
        stdout,
        "{} {} the following words:\n >: {fmt_words}.",
        "try to".cyan(),
        "remember".purple(),
    )?;

    stdout.flush()?;

    for i in 0..raw_fmt_words.len() + 5 {
        write!(stdout, "{}\r", " ".repeat(i))?;
        thread::sleep(Duration::from_millis(250));
        stdout.flush()?;
    }

    write!(stdout, "Erased. {}", "Good Luck".green())?;
    writeln!(stdout)?;

    let mut un_words = vec![];
    let mut tries = 1;
    let started = Instant::now();

    let completion = WordCompletion::default();
    Input::with_theme(&ColorfulTheme::default())
        .completion_with(&completion)
        .with_prompt("What were the words?")
        .validate_with(|v: &String| -> Result<(), String> {
            let user_words = v.split_whitespace();
            let mut correct_words = vec![];

            for word in user_words {
                for b_word in &words {
                    let diff = diff_percent(word, &b_word);
                    if diff >= 80 && diff < 100 {
                        un_words.push((word.to_string(), b_word.to_string()));
                    } else if diff == 100 {
                        correct_words.push(word.to_string());
                    }
                }
            }

            if correct_words.len() + un_words.len() < words.len() {
                let mut base = "Incorrect words.".to_string();

                if !un_words.is_empty() {
                    base += &format!(
                        ". Fixed words: {}",
                        un_words
                            .iter()
                            .map(|(x, y)| format!("{x} -> {y}"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }

                if !correct_words.is_empty() {
                    base += &format!(". Correct words: {}", correct_words.join(", "));
                }

                tries += 1;
                return Err(base);
            }

            Ok(())
        })
        .interact_text()?;

    if !un_words.is_empty() {
        println!(
            "{}{}",
            "Fixed words: ".green(),
            un_words
                .iter()
                .map(|(x, y)| format!("{x} -> {y}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    let gain_xp = ((raw_fmt_words.len() * words.len() % 100) as i64
        + (get_word_amount(&data) as i64) / tries)
        - started.elapsed().as_secs() as i64
        - tries;

    println!(
        "{}:\n {} {gain_xp}\n {} {:.3?}",
        "Results".cyan(),
        "Gained XP:".yellow(),
        "Finished in:".yellow(),
        started.elapsed()
    );

    data.xp += gain_xp;
    Ok(())
}
