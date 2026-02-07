use reqwest::{
    blocking::{Client, Response},
    header::USER_AGENT,
};

fn get_full_url(username: String) -> String {
    return format!("https://api.github.com/users/{username}/events");
}

pub fn get_user_activities(username: String) -> Result<Response, reqwest::Error> {
    let client = Client::new();
    let request = client
        .get(get_full_url(username))
        .header(USER_AGENT, "Rust");
    return request.send();
}
