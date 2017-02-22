use slack_api;

pub static TABLE_HEADER: &'static str = "<table><thead><tr><th>Time</th><th>User</th><th>Message</th></tr></thead><tbody>";
pub static TABLE_FOOTER: &'static str = "</tbody></table>";

pub trait BasicHTMLRender {
    fn render(&self) -> Option<String>;
}

fn basic_row(a: &str, b: &str, c: &str) -> String {
    format!("<tr><td><em>{}</em></td> <td><strong>{}</strong></td> \
                              <td>{}</td></tr>",
            a,
            b,
            c)
}

fn or_empty(s: &Option<String>) -> &str {
    s.as_ref().map(|s| s as &str).unwrap_or("")
}

impl BasicHTMLRender for slack_api::Message {
    fn render(&self) -> Option<String> {
        use slack_api::Message::*;
        match self {
            &Standard { ref ts, ref text, ref user, .. } => {
                Some(basic_row(ts, or_empty(user), or_empty(text)))
            }
            &ChannelJoin { ref ts, ref text, ref user, .. } => Some(basic_row(ts, user, text)),
            &FileShare { ref ts, ref user, ref file, .. } => {
                let modtext = format!("<a href=\"{}\">Heres a file!</a>", file.permalink);
                Some(basic_row(ts, user, &modtext))
            }
            &ChannelPurpose { ref ts, ref text, ref user, .. } => Some(basic_row(ts, user, text)),
            &ChannelTopic { ref ts, ref text, ref user, .. } => Some(basic_row(ts, user, text)),
            &PinnedItem { ref ts, ref text, ref user, .. } => Some(basic_row(ts, user, text)),
            // Other formats todo, and will not be rendered.
            _ => {
                warn!("Unparsed message: {:?}", self);
                None
            }
        }
    }
}

impl BasicHTMLRender for slack_api::Channel {
    fn render(&self) -> Option<String> {
        Some(format!("<h2>{}</h2>", self.name.clone()).to_string())
    }
}
