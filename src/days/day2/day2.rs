use anyhow::Result;
use log::{debug, info};
use scan_rules::let_scan;
use scan_rules::scanner::Word;

#[derive(Debug)]
pub struct Entry {
    first: i32,
    second: i32,
    character: char,
    password: &'static str,
}

pub async fn solve() -> Result<()> {
    debug!("Executing Day 2...");

    let mut sled_rental_successes = 0;
    let mut toboggan_corporate_successes = 0;

    let password_entries: Vec<&str> = include_str!("../../../resources/day2.txt")
        .split("\n")
        .collect();

    for entry in password_entries {
        debug!("Processing {}...", entry);
        let_scan!(entry; (let first: i32, "-", let second: i32, let character: char, ": ", let password: Word));

        let entry = Entry {
            first,
            second,
            character,
            password,
        };

        debug!("Password Entry: {:?}", entry);

        match validate_sled_rental_rules(&entry) {
            Ok(()) => sled_rental_successes += 1,
            Err(e) => debug!("{}", e),
        }

        match validate_toboggan_corporate_rules(&entry) {
            Ok(()) => toboggan_corporate_successes += 1,
            Err(e) => debug!("{}", e),
        }
    }

    info!("Part One Answer is {}", sled_rental_successes);
    info!("Part Two Answer is {}", toboggan_corporate_successes);
    Ok(())
}

fn validate_sled_rental_rules(entry: &Entry) -> Result<()> {
    let count = entry.password.matches(entry.character).count();
    debug!(
        "The character {} was found {} times in {}",
        entry.character, count, entry.password
    );

    if count >= entry.first as usize && count <= entry.second as usize {
        return Ok(());
    }

    Err(anyhow::anyhow!(
        "{} breaks the sled rental password rules!",
        entry.password
    ))
}

fn validate_toboggan_corporate_rules(entry: &Entry) -> Result<()> {
    let first_offset = (entry.first - 1) as usize;
    let second_offset = (entry.second - 1) as usize;
    let password_bytes = entry.password.as_bytes();

    if (password_bytes[first_offset] as char == entry.character)
        ^ (password_bytes[second_offset] as char == entry.character)
    {
        return Ok(());
    }

    Err(anyhow::anyhow!(
        "{} breaks the Toboggan Corporate password rules!"
    ))
}
