use lazy_static::lazy_static;
use regex::Regex;

use crate::{clients::Clients, message::Message};

lazy_static! {
    pub static ref DATE_REGEX: Regex = Regex::new(r"\[(.*?)\] (.*)").unwrap();
    pub static ref COLORED_USERNAMES: Vec<(Regex, Clients)> = vec![
        (Regex::new(r"\u{25B2}<(.*?)> (.*)").unwrap(), Clients::Tower),
        (Regex::new(r"\u{B9AC}\u{3E70}<(.*?)> (.*)").unwrap(), Clients::bRAC),
        (Regex::new(r"\u{2550}\u{2550}\u{2550}<(.*?)> (.*)").unwrap(), Clients::CRAB),
        (Regex::new(r"\u{00B0}\u{0298}<(.*?)> (.*)").unwrap(), Clients::Mefedroniy),
        (Regex::new(r"<(.*?)> (.*)").unwrap(), Clients::clRAC),
    ];
    pub static ref BRACES_REGEX: Regex = Regex::new(r"\{[^}]*\}\s").unwrap();
    pub static ref ANSI_REGEX: Regex = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap();
    pub static ref CONTROL_CHARS_REGEX: Regex = Regex::new(r"[\x00-\x1F\x7F]").unwrap();
}

pub fn format_message(message: &str) -> Option<Message> {
    None
}