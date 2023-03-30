use std::collections::HashMap;
use std::env;

use reqwest::blocking::Client;

pub fn post_to_merge_request(comment: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("GITLAB_URL")?;
    let project_id = env::var("PROJECT_ID")?;
    if let Ok(merge_request_id) = env::var("MERGE_REQUEST_ID") {
        let client = Client::new();
        let _resp = client
            .post(format!(
                "{base_url}/api/v4/projects/{project_id}/merge_requests/{merge_request_id}/notes"
            ))
            .header("PRIVATE-TOKEN", env::var("GITLAB_USER_TOKEN")?)
            .json(&{
                let mut json = HashMap::new();
                json.insert("body", comment.to_string());
                json
            })
            .send()?;
    }
    Ok(())
}
