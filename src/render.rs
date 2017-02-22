use slack_api;
use std::io::Write;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

pub static TABLE_HEADER: &'static str = "<table><thead><tr><th>Time</th><th>User</th><th>Message</th></tr></thead><tbody>";
pub static TABLE_FOOTER: &'static str = "</tbody></table>";

pub trait BasicHTMLRender {
    fn render(&self) -> Option<String>;
}

impl BasicHTMLRender for slack_api::Message {
    fn render(&self) -> Option<String> {
        use slack_api::Message::*;
        match self.clone() {
            Standard { ts, text, user, .. } => {
                Some(format!("<tr><td><em>{}</em></td> <td><strong>{}</strong></td> \
                              <td>{}</td></tr>",
                             ts,
                             user.unwrap_or("".to_string()).clone(),
                             text.unwrap_or("".to_string()).clone())
                    .to_string())
            }
            ChannelJoin { ts, text, user, .. } => {
                Some(format!("<tr><td><em>{}</em></td> <td><strong>{}</strong></td> \
                              <td>{}</td></tr>",
                             ts,
                             user,
                             text)
                    .to_string())
            }
            FileShare { ts, user, file, .. } => {
                Some(format!("<tr><td><em>{}</em></td> <td><strong>{}</strong></td> \
                              <td><a href=\"{}\">Heres a file!</a></td></tr>",
                             ts,
                             user,
                             file.permalink)
                    .to_string())
            }
            ChannelPurpose { ts, text, user, .. } => {
                Some(format!("<tr><td><em>{}</em></td> <td><strong>{}</strong></td> \
                              <td>{}</td></tr>",
                             ts,
                             user,
                             text)
                    .to_string())
            }
            ChannelTopic { ts, text, user, .. } => {
                Some(format!("<tr><td><em>{}</em></td> <td><strong>{}</strong></td> \
                              <td>{}</td></tr>",
                             ts,
                             user,
                             text)
                    .to_string())
            }
            _ => {
                println_stderr!("{:?}", self);
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
