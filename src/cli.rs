use crate::activities_printer;
use crate::github_client;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    username: String,
}

impl Cli {
    pub fn run_command(&self) {
        let response = match github_client::get_user_activities(self.username.clone())
            .and_then(|res| res.json::<serde_json::Value>())
        {
            Ok(res) => res,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        let body = match response.as_array() {
            Some(body) if !body.is_empty() => body,
            _ => {
                println!(
                    "🫥 {} hasn’t had any activity in the last 30 days.",
                    self.username
                );

                return;
            }
        };

        activities_printer::print_activities(body);
    }
}
