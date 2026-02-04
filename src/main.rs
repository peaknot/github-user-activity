use serde::{Deserialize};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    username: String,
    #[command(subcommand)]
    command: Option<EventType>,
}
#[derive(Subcommand)]
enum EventType {
    #[command(alias = "pe")]
    PushEvent,
    #[command(alias = "we")]
    WatchEvent,
    #[command(alias = "ie")]
    IssuesEvent
}
impl EventType {
    fn execute(&self, events: &[Event]) {
        let filter = match self {
            EventType::PushEvent => "PushEvent",
            EventType::WatchEvent => "WatchEvent",
            EventType::IssuesEvent => "IssuesEvent",
        };
        display_event(events, Some(filter));
    }
}
#[derive(Deserialize, Debug,)]
struct Event {
    #[serde(rename = "type")]
    event_type: String,
    repo: Repo,
    payload: Payload,
}
#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
}
#[derive(Deserialize, Debug)]
struct Payload {
    action: Option<String>,
}

fn main() {
    let args = Cli::parse();
    match fetch_events(&args.username) {
        Ok(events) => match args.command {
            Some(cmd) => cmd.execute(&events),
            None => display_event(&events, None),
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn fetch_events(username: &str) -> Result<Vec<Event>, Box<dyn std::error::Error>>  {
    let url = format!("https://api.github.com/users/{username}/events");

    let client = reqwest::blocking::Client::new();
    let response = client.get(&url)
        .header("User-Agent", "github-user-activity-cli")
        .send()?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }
    let events = response.json::<Vec<Event>>()?;

    Ok(events)
}
fn display_event(data: &[Event], filter: Option<&str>) {
    for event in data {
        if let Some(f) = filter {
            if event.event_type != f {
                continue;
            }
        }
        match event.event_type.as_str() {
            "PushEvent" => {
                println!("Pushed to {}", event.repo.name);
            },
            "WatchEvent" => println!("Starred {}", event.repo.name),
            "IssuesEvent" => {
                if let Some(action) = &event.payload.action {
                    println!("{} {}", action, event.repo.name);
                } else {
                    println!("Interacted with {}", event.repo.name);
                };
            },
            _ => (),
        }
    }
}
