extern crate slack_api;
extern crate hyper;
extern crate rayon;

use rayon::prelude::*;

use std::io::Write;
mod render;
use render::BasicHTMLRender;

macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);

fn main() {
    let x = hyper::Client::new();
    let api_key = std::env::var("API_KEY").expect("Set env var 'API_KEY'");
    let y = slack_api::rtm::start(&x, &api_key, None, None);

    procedure(&x, &api_key);

}

#[derive(Debug)]
struct HistoryChannel {
    channel: slack_api::Channel,
    messages: Vec<slack_api::Message>,
}

fn procedure<R: slack_api::SlackWebRequestSender>(client: &R, token: &str)
    where R: std::marker::Sync
{
    let ch_resp = slack_api::channels::list(client, token, Some(true))
        .expect("Failed to get channels");

    let x = ch_resp.channels
        .par_iter()
        .map(|c| {
            println_stderr!("Getting {:?}", c.name);
            HistoryChannel {
                channel: c.clone(),
                messages: get_channel_history(client, token, &c.id, None),
            }
        })
        .collect::<Vec<HistoryChannel>>();

    // println!("{:?}", x);

    for channel in x.iter() {
        println!("{}", channel.channel.render().unwrap());
        println!("{}", render::TABLE_HEADER);
        for msg in channel.messages.iter() {
            if let Some(out) = msg.render() {
                println!("{}", out);
            }
        }
        println!("{}", render::TABLE_FOOTER);
    }
}


fn get_channel_history<R: slack_api::SlackWebRequestSender>(client: &R,
                                                            token: &str,
                                                            channel_id: &str,
                                                            latest: Option<String>)
                                                            -> Vec<slack_api::Message> {
    // TODO: doesn't actually get the full history at all, only the latest chunk.

    let resp = slack_api::channels::history(client, token, channel_id, None, None, None, None);

    if let Ok(response) = resp {
        response.messages.iter().map(|x| x.clone()).collect()
    } else {
        println_stderr!("{:?}", resp);
        Vec::new()
    }
}
