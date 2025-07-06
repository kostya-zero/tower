use crate::message::MessageResponse;
use crate::{clients::Clients, message::Message};
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

lazy_static! {
    pub static ref DATE_REGEX: Regex = Regex::new(r"\[(.*?)\] (.*)").unwrap();
    pub static ref CLIENTS: Vec<(Regex, Clients)> = vec![
        (Regex::new(r"\u{25B2}<(.*?)> (.*)").unwrap(), Clients::Tower),
        (Regex::new(r"\u{B9AC}\u{3E70}<(.*?)> (.*)").unwrap(), Clients::bRAC),
        (Regex::new(r"\u{2550}\u{2550}\u{2550}<(.*?)> (.*)").unwrap(), Clients::CRAB),
        (Regex::new(r"\u{00B0}\u{0298}<(.*?)> (.*)").unwrap(), Clients::Mefedroniy),
        (Regex::new(r"\u{0D9E}<(.*?)> (.*)").unwrap(), Clients::Snowdrop),
        (Regex::new(r"<(.*?)> (.*)").unwrap(), Clients::clRAC),
    ];
    pub static ref BRACES_REGEX: Regex = Regex::new(r"\{[^}]*\}\s").unwrap();
    pub static ref ANSI_REGEX: Regex = Regex::new(r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])").unwrap();
    pub static ref CONTROL_CHARS_REGEX: Regex = Regex::new(r"[\x00-\x1F\x7F]").unwrap();
    // Now with a capturing group for the inner value:
    pub static ref AR_MARK_REGEX: Regex = Regex::new(r"(.*)\x06!!AR!!(.*)").unwrap();
}

pub fn format_messages(messages: Vec<Cow<str>>) -> Vec<MessageResponse> {
    messages
        .into_iter()
        .map(|message| {
            let (message, ar_data) = if let Some(caps) = AR_MARK_REGEX.captures(&message.clone()) {
                (
                    caps.get(1).map(|ar_data| ar_data.as_str().to_string()).unwrap_or(message.to_string()),
                    caps.get(2).map(|ar_data| ar_data.as_str().to_string()),
                )
            } else {
                (message.to_string(), None)
            };

            let cleaned = BRACES_REGEX.replace_all(message.trim(), "");
            let sanitized = sanitize(&cleaned);

            let mut new_message = Message {
                content: String::new(),
                username: String::new(),
                timestamp: None,
                client: String::new(),
                avatar_url: ar_data,
            };

            let mut new_message_response = MessageResponse {
                message: None,
                raw_string: message.to_string(),
            };

            if let Some(date_caps) = DATE_REGEX.captures(&sanitized) {
                new_message.timestamp = date_caps.get(1).map(|m| m.as_str().to_string());

                if let Some(content) = date_caps.get(2) {
                    for (reg, client) in CLIENTS.iter() {
                        if let Some(cap) = reg.captures(content.as_str()) {
                            new_message.username =
                                cap.get(1).map_or("", |m| m.as_str()).to_string();
                            new_message.content = cap.get(2).map_or("", |m| m.as_str()).to_string();
                            new_message.client = client.to_string();
                            break;
                        }
                    }
                } else {
                    return new_message_response;
                }
            } else {
                return new_message_response;
            }

            new_message_response.message = Some(new_message);
            new_message_response
        })
        .collect()
}

pub fn sanitize(text: &str) -> String {
    let removed_ansi = ANSI_REGEX.replace_all(text, "");
    let removed_control = CONTROL_CHARS_REGEX.replace_all(&removed_ansi, "");
    removed_control.into_owned()
}
