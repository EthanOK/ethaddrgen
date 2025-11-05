use patterns::{Pattern, Patterns, PatternConfig, parse_patterns};
use std::sync::{Arc, Mutex};
use regex::{Regex, RegexBuilder};
use clap::ArgMatches;
use termcolor::BufferWriter;

impl Pattern for Regex {
    fn matches(&self, string: &str) -> bool {
        self.is_match(string)
    }

    fn parse<T: AsRef<str>>(string: T, config: &PatternConfig) -> Result<Self, String> {
        match RegexBuilder::new(string.as_ref())
                  .case_insensitive(config.case_insensitive)
                  .multi_line(false)
                  .dot_matches_new_line(false)
                  .ignore_whitespace(true)
                  .unicode(true)
                  .build() {
            Ok(result) => return Ok(result),
            Err(error) => return Err(format!("Invalid regex: {}", error)),
        }
    }
}

pub struct RegexPatterns {
    vec: Vec<Regex>,
}

impl RegexPatterns {
    pub fn new(buffer_writer: Arc<Mutex<BufferWriter>>, matches: &ArgMatches, case_insensitive: bool) -> RegexPatterns {
        let config = PatternConfig { case_insensitive };
        let vec = parse_patterns(buffer_writer, matches, &config);

        RegexPatterns { vec }
    }
}

impl Patterns for RegexPatterns {
    fn contains(&self, address: &String) -> bool {
        // Linear search
        for pattern in &self.vec {
            if pattern.matches(address) {
                return true;
            }
        }

        return false;
    }

    fn len(&self) -> usize {
        self.vec.len()
    }
}
