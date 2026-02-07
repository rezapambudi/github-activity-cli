use crate::activities_printer;
use crate::github_client;
use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    username: String,
}

impl Cli {
    pub fn run_command(&self) {
        match github_client::get_user_activities(self.username.clone())
            .and_then(|res| res.json::<serde_json::Value>())
        {
            Ok(res) => activities_printer::print_activities(res),
            Err(err) => println!("{}", err),
        }
    }
}
