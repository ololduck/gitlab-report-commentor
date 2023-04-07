use std::collections::HashMap;
use std::env;
use std::process::exit;
use log::error;

use reqwest::blocking::Client;
use crate::BAD_CONFIG;

pub fn post_to_merge_request(comment: &str) -> Result<(), failure::Error> {
    let Ok(base_url) = env::var("CI_API_V4_URL") else {
        error!("Could not find CI_API_V4_URL in the environment, which is a default GitLab-CI variable. Am I running on GitLab-CI?");
        exit(BAD_CONFIG);
    };
    let Ok(project_id) =
        env::var("CI_PROJECT_ID") else {
        error!("Could not find CI_PROJECT_ID in the environment, which is a default GitLab-CI variable. Am I running on GitLab-CI?");
        exit(BAD_CONFIG);
    };
    let Ok(merge_request_id) = env::var("CI_MERGE_REQUEST_IID")else {
        error!("Could not find CI_MERGE_REQUEST_IID in the environment, am I running in a merge request pipeline?");
        exit(BAD_CONFIG);
    };
    let Ok(gitlab_token) = env::var("GITLAB_USER_TOKEN") else {
        error!("Could not find GITLAB_USER_TOKEN in the environment. I need it to send what I have to say.");
        exit(BAD_CONFIG);
    };
    let client = Client::new();
    let _resp = client
        .post(format!(
            "{base_url}/projects/{project_id}/merge_requests/{merge_request_id}/notes"
        ))
        .header("PRIVATE-TOKEN", gitlab_token)
        .json(&{
            let mut json = HashMap::new();
            json.insert("body", comment.to_string());
            json
        })
        .send()?;

    Ok(())
}
