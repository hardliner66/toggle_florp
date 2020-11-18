use std::{collections::hash_map::DefaultHasher, hash::Hash, hash::Hasher, str::FromStr};

/// The default identifiers for a random color
pub const DEFAULT_RANDOM_IDENTIFIERS: &[&str] = &["r", "rand", "random"];

/// A simple wrapper for a color
#[derive(Debug)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    /// Generates a new random color
    #[must_use]
    pub fn random() -> Color {
        Color(
            rand::random::<u8>(), // r
            rand::random::<u8>(), // g
            rand::random::<u8>(), // b
        )
    }

    /// Generates a new color from the given r g b values
    #[must_use]
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color(r, g, b)
    }

    /// Generates a new color from the given r g b values
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn from_hash(s: &str) -> Color {
        // hash the string and convert to a color
        let hash = calculate_hash(&s);

        let r = ((hash >> 24) & 0xff) as u8;
        let g = ((hash >> 16) & 0xff) as u8;
        let b = ((hash >> 8) & 0xff) as u8;
        let _a = (hash & 0xff) as u8; // this could possibly used as an alpha value in the future

        Color::new(r, g, b)
    }

    /// If one wants to use this library to parse something from twitch,
    /// this is the method to do just that.
    #[must_use]
    pub fn from_message(s: &str) -> Color {
        let parsed: Result<Color, String> = s.parse();
        match parsed {
            Ok(color) => color,
            Err(_) => {
                if DEFAULT_RANDOM_IDENTIFIERS.contains(&s.trim().to_lowercase().as_ref()) {
                    Color::random()
                } else {
                    Color::from_hash(s)
                }
            }
        }
    }

    /// Returns the red part of the color
    #[must_use]
    pub fn r(&self) -> u8 {
        self.0
    }

    /// Returns the green part of the color
    #[must_use]
    pub fn g(&self) -> u8 {
        self.1
    }

    /// Returns the blue part of the color
    #[must_use]
    pub fn b(&self) -> u8 {
        self.2
    }
}

// parses the string into an u8 or returns the original String as error
fn parse_value(s: &str) -> Result<u8, &str> {
    if let Ok(v) = s.parse::<u8>() {
        Ok(v)
    } else {
        Err(s)
    }
}

#[allow(clippy::cast_possible_truncation)]
fn calculate_hash<T: Hash>(t: &T) -> u32 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish() as u32
}

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        // sanitize input for parsing
        let sanitized = s
            .chars()
            .map(|c| match c {
                '0'..='9' => c,
                _ => ' ',
            })
            .collect::<String>();

        // split and parse
        let ok = sanitized
            .split(' ')
            .map(parse_value)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        if ok.len() >= 3 {
            // if we have 3 valid values, return them as color
            Ok(Color::new(ok[0], ok[1], ok[2]))
        } else {
            Err(format!("\"{}\" is not parsable!", s))
        }
    }
}
