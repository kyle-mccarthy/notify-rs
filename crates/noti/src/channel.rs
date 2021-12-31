use std::fmt::Display;

mod error;
mod provider;

pub mod email;
pub mod sms;

pub use error::Error;
pub use provider::Provider;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChannelType {
    Email,
    Sms
}

impl Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Email => f.write_str("Email"),
            Self::Sms => f.write_str("SMS"),
        }
    }
}

