use crate::message::MessageResponse;
use crate::{clients::Clients, message::Message};
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

lazy_static! {
    pub static ref DATE_REGEX: Regex = Regex::new(r"\[(.*?)\] (.*)").unwrap();
    pub static ref CLIENTS: Vec<(Regex, Clients)> = vec![
        (Regex::new(r"\u{25B2}<(.*?)> (.*)").unwrap(), Clients::Tower),
        (
            Regex::new(r"\u{B9AC}\u{3E70}<(.*?)> (.*)").unwrap(),
            Clients::bRAC
        ),
        (
            Regex::new(r"\u{2550}\u{2550}\u{2550}<(.*?)> (.*)").unwrap(),
            Clients::CRAB
        ),
        (
            Regex::new(r"\u{00B0}\u{0298}<(.*?)> (.*)").unwrap(),
            Clients::Mefedroniy
        ),
        (
            Regex::new(r"\u{0D9E}<(.*?)> (.*)").unwrap(),
            Clients::Snowdrop
        ),
        (Regex::new(r"<(.*?)> (.*)").unwrap(), Clients::clRAC),
    ];
    pub static ref BRACES_REGEX: Regex = Regex::new(r"\{[^}]*\}\s").unwrap();
    pub static ref ANSI_REGEX: Regex =
        Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap();
    pub static ref CONTROL_CHARS_REGEX: Regex = Regex::new(r"[\x00-\x1F\x7F]").unwrap();
    pub static ref AR_MARK_REGEX: Regex = Regex::new(r"\x06!!AR!!<[^>]*>$").unwrap();
}

pub fn format_messages(messages: Vec<Cow<str>>) -> Vec<MessageResponse> {
    let mut formatted_messages: Vec<MessageResponse> = Vec::new();
    for message in messages.iter() {
        let mut new_message = Message {
            content: "".to_string(),
            username: "".to_string(),
            timestamp: None,
            client: "".to_string(),
        };

        let mut new_message_response = MessageResponse {
            message: None,
            raw_string: message.to_string(),
        };
        let cleaned = BRACES_REGEX.replace_all(message.trim(), "");
        let sanitized = sanitize(&cleaned);


        let date_regex = DATE_REGEX.captures(sanitized.as_str());
        if let Some(some_date) = date_regex {
            if let Some(date) = some_date.get(1) {
                new_message.timestamp = Some(date.as_str().to_string());
            }

            if let Some(content) = some_date.get(2) {
                CLIENTS.iter().find_map(|(reg, client)| {
                    reg.captures(content.as_str()).map(|cap| {
                        let username = cap.get(1).map_or("", |m| m.as_str());
                        let text = cap.get(2).map_or("", |m| m.as_str());
                        new_message.content = text.to_string();
                        new_message.username = username.to_string();
                        new_message.client = client.to_string();
                    })
                });
            } else {
                formatted_messages.push(new_message_response);
                continue;
            }
        }

        new_message_response.message = Some(new_message);
        formatted_messages.push(new_message_response);
    }
    formatted_messages
}

pub fn sanitize(text: &str) -> String {
    let removed_ansi = ANSI_REGEX.replace_all(text, "");
    let removed_control = CONTROL_CHARS_REGEX.replace_all(&removed_ansi, "");
    let removed_ar = AR_MARK_REGEX.replace(&removed_control, "");
    removed_ar.into_owned()
}