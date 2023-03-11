use std::{env, path::PathBuf};

use color_eyre::{owo_colors::OwoColorize, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};

use crate::{data::Data, lesson::lesson};

mod data;
mod lesson;
mod utils;

fn get_player_data_location() -> PathBuf {
    let home = env::var("HOME")
        .or(env::var("Home"))
        .expect("$HOME isn't set.");

    let mut target = PathBuf::from(home);
    target.push(".Rewfresh.rwf");
    target
}

fn create_player_profile() -> Result<Data> {
    println!("{}", "Not found! Creating. Ready?".purple());
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is your name?")
        .with_initial_text("John")
        .interact_text()?;

    Ok(Data { name, xp: 0 })
}

fn main() -> Result<()> {
    let player_data_location = get_player_data_location().into_boxed_path();
    put!("Welcome to Rewfresh.".purple());
    put!("Loading player information ...".cyan());

    let mut player_data = if player_data_location.exists() {
        Data::load_from_file(&player_data_location)?
    } else {
        let profile = create_player_profile()?;
        profile.save(&player_data_location)?;
        profile
    };

    println!("{} {}", "Welcome".cyan(), player_data.name.bold());

    loop {
        println!(
            "{} {} {}",
            "You current have".purple(),
            player_data.xp.cyan(),
            "XP".green()
        );

        if !Confirm::new()
            .with_prompt("Do want to start a round?")
            .interact()?
        {
            break;
        }

        lesson(&mut player_data)?;
        player_data.save(&player_data_location)?;
    }

    Ok(())
}
