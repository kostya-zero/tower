use std::fmt::Display;

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub enum Clients {
    Tower,
    bRAC,
    Mefedroniy,
    CRAB,
    clRAC,
    Snowdrop,
}

impl Display for Clients {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Clients::Tower => write!(f, "Tower"),
            Clients::bRAC => write!(f, "bRAC"),
            Clients::Mefedroniy => write!(f, "Mefedroniy"),
            Clients::CRAB => write!(f, "CRAB"),
            Clients::clRAC => write!(f, "clRAC"),
            Clients::Snowdrop => write!(f, "Snowdrop"),
        }
    }
}
