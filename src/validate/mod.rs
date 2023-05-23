use std::error::Error;
use std::string;
use crate::cmd::{ValidationArgs, ConventionConfig,  Convention};
use crate::error::GitVentionError;
use crate::git::search::Searcher;
use regex::Regex;


struct ConventionViolation{
    message: String
}

fn check_regex(message: &str, regex: &str) -> Result<Option<ConventionViolation>, GitVentionError>{
    let re = Regex::new(regex).map_err(|e| {GitVentionError(e.to_string())})?;
    if re.is_match(message) {
        return Ok(None);
    }
    Ok(Some(ConventionViolation{message: format!("{} is not VALID", message)}))
}

fn check_max_length(message: &str, max_length: u32) -> Result<Option<ConventionViolation>, GitVentionError> {
    let message_length = message.chars().count();
    if usize::try_from(max_length).map(|max| {message_length > max}).map_err(|e| {GitVentionError(e.to_string())})? {
        return Ok(Some(ConventionViolation{message: format!("{} is not VALID", message)}));
    }
    Ok(None)
}

fn check_leading_whitespace(message: &str) -> Option<ConventionViolation> {
    if message.starts_with(char::is_whitespace) {
        return Some(ConventionViolation{message: format!("{} is not VALID", message)});
    }

    None
}

/* 
fn validator(convetion_type: Convention) -> Option<ConventionViolation> {
    return match convetion_type {
        ConventionType::TitleRegex => validate_message
    }
}*/

pub fn validate<T: Searcher>(search: T, args: &ValidationArgs, convention_config: &ConventionConfig) -> Result<(), Box<dyn Error>> {

    let hashes = search.commit_hashes(&args.target)?;

    for hash in hashes {

        let message = search.commit_by_hash(&hash)?;

        for convention in convention_config.title_conventions() {
            let validation_result = match convention {
                Convention::TitleRegex(regex) => check_regex(&message.message, &regex)?,
                Convention::TitleMaxLength(length) => check_max_length(&message.message, length)?,
                Convention::TitleNoLeadingWhitespace => check_leading_whitespace(&message.message),
            };
            match  validation_result {
                Some(cv) => eprintln!("{}", cv.message),
                None => println!("Commit: `{}` is VALID", message.message),
            };
        }
    }

    Ok(())
}
