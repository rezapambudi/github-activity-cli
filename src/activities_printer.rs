use serde_json::Value;

pub fn print_activities(body: Value) {
    body.as_array()
        .into_iter()
        .flatten()
        .for_each(|event| describe_event(event));
}

fn unwrap_value(filed: &Value) -> &str {
    return filed.as_str().unwrap_or_default();
}

fn describe_event(event: &Value) {
    let repository_name = unwrap_value(&event["repo"]["name"]);
    let created_at = unwrap_value(&event["created_at"]);
    let payload = &event["payload"];

    let activity = match unwrap_value(&event["type"]) {
        "ReleaseEvent" => format!("{}", unwrap_value(&event["payload"]["release"]["name"])),
        "DeleteEvent" => {
            let payload = &event["payload"];
            format!(
                "Delete {} {} on {} repository",
                unwrap_value(&payload["ref_type"]),
                unwrap_value(&payload["ref"]),
                repository_name
            )
        }
        "PullRequestEvent" => {
            let base = &payload["pull_request"]["base"];
            let head = &payload["pull_request"]["head"];
            format!(
                "{} Pull Request from {} to {} on {} repository, URL : {}",
                unwrap_value(&payload["action"]),
                unwrap_value(&head["ref"]),
                unwrap_value(&base["ref"]),
                repository_name,
                unwrap_value(&payload["pull_request"]["url"]),
            )
        }
        "PushEvent" => format!(
            "Push to branch {} on {} repository",
            unwrap_value(&payload["ref"]),
            repository_name
        ),
        "CreateEvent" => format!(
            "create {} {} from {} on {} repository",
            unwrap_value(&payload["ref_type"]),
            unwrap_value(&payload["ref"]),
            unwrap_value(&payload["master_branch"]),
            repository_name
        ),
        _ => format!(
            "Unknwon event {} please raise issue to repos",
            unwrap_value(&event["type"])
        ),
    };

    println!("{created_at} | {activity}")
}
