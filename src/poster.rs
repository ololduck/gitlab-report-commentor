use std::collections::HashMap;
use std::env;

use reqwest::blocking::Client;

pub fn post_to_merge_request(comment: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("CI_API_V4_URL").expect("Could not find CI_API_V4_URL in the environment");
    let project_id = env::var("CI_PROJECT_ID").expect("Could not find CI_PROJECT_ID in the environment");
    let merge_request_id = env::var("CI_MERGE_REQUEST_IID").expect("Could not find envvar CI_MERGE_REQUEST_IID, am I running in a merge request pipeline?");
    let client = Client::new();
    let _resp = client
        .post(format!(
            "{base_url}/projects/{project_id}/merge_requests/{merge_request_id}/notes"
        ))
        .header("PRIVATE-TOKEN", env::var("GITLAB_USER_TOKEN")?)
        .json(&{
            let mut json = HashMap::new();
            json.insert("body", comment.to_string());
            json
        })
        .send()?;

    Ok(())
}
