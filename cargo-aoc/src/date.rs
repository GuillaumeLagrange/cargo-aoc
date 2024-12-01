use chrono::prelude::*;
use chrono_tz::EST;
use std::fmt;

use crate::Input;

#[derive(Debug, Clone, Copy)]
pub struct AOCDate {
    /// The day of the input to retrieve
    pub day: u32,
    /// The year of the input to retrieve
    pub year: i32,
}

impl AOCDate {
    pub fn new(matches: &Input) -> Self {
        // Get the current date in the EST timezone, which is used by advent of code to
        // release new puzzles.
        let utc_today = Utc::now().naive_utc();
        let today = EST.from_utc_datetime(&utc_today);
        let day: u32 = matches
            .day
            .map(|d| d.0 as u32)
            .unwrap_or_else(|| today.day());

        let year: i32 = matches.year.unwrap_or_else(|| today.year());

        AOCDate { day, year }
    }

    pub fn input_directory(&self) -> String {
        format!("input/{}", self.year)
    }

    pub fn input_filename(&self) -> String {
        format!("{}/day{}.txt", self.input_directory(), self.day)
    }

    /// Get the url to request the input for the date
    pub fn input_request_url(&self) -> String {
        format!(
            "https://adventofcode.com/{}/day/{}/input",
            self.year, self.day
        )
    }

    pub fn puzzle_directory(&self) -> String {
        format!("puzzles/{}", self.year)
    }

    pub fn puzzle_filename(&self) -> String {
        format!("{}/day{}.md", self.puzzle_directory(), self.day)
    }

    /// Get the url to request the puzzle for the date
    pub fn puzzle_request_url(&self) -> String {
        format!("https://adventofcode.com/{}/day/{}", self.year, self.day)
    }
}

impl fmt::Display for AOCDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Day {} of {}", self.day, self.year)
    }
}
