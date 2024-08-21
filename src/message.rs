use std::fmt;

#[derive(Debug)]
pub(crate) enum Message {
    UserJoined(String),
    UserLeft(String),
    Chat { sender: String, content: String },
}

impl Message {
    pub fn user_joined(username: &str) -> Self {
        let content = format!("{} has joined the chat", username);
        Self::UserJoined(content.to_string())
    }

    pub fn user_left(username: &str) -> Self {
        let content = format!("{} has left the chat", username);
        Self::UserLeft(content.to_string())
    }

    pub fn chat(sender: impl Into<String>, content: impl Into<String>) -> Self {
        Self::Chat {
            sender: sender.into(),
            content: content.into(),
        }
    }
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UserJoined(content) => write!(f, "[{}]", content),
            Self::UserLeft(content) => write!(f, "[{} :(]", content),
            Self::Chat { sender, content } => write!(f, "{}: {}", sender, content),
        }
    }
}
