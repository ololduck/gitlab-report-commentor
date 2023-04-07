use chrono::NaiveDateTime;
use log::error;
use std::collections::HashMap;
use std::env;
use std::process::exit;

use crate::EXIT_CODE_BAD_CONFIG;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

pub fn post_to_merge_request(comment: &str) -> Result<(), failure::Error> {
    let Ok(base_url) = env::var("CI_API_V4_URL") else {
        error!("Could not find CI_API_V4_URL in the environment, which is a default GitLab-CI variable. Am I running on GitLab-CI?");
        exit(EXIT_CODE_BAD_CONFIG);
    };
    let Ok(project_id) =
        env::var("CI_PROJECT_ID") else {
        error!("Could not find CI_PROJECT_ID in the environment, which is a default GitLab-CI variable. Am I running on GitLab-CI?");
        exit(EXIT_CODE_BAD_CONFIG);
    };
    let Ok(merge_request_id) = env::var("CI_MERGE_REQUEST_IID")else {
        error!("Could not find CI_MERGE_REQUEST_IID in the environment, am I running in a merge request pipeline?");
        exit(EXIT_CODE_BAD_CONFIG);
    };
    let Ok(gitlab_token) = env::var("GITLAB_USER_TOKEN") else {
        error!("Could not find GITLAB_USER_TOKEN in the environment. I need it to send what I have to say.");
        exit(EXIT_CODE_BAD_CONFIG);
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

/// see https://docs.gitlab.com/ee/api/merge_requests.html#get-merge-request-diff-versions
pub struct DiffLineComment {
    pub(crate) path: String,
    pub(crate) line: String,
    pub(crate) body: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MergeRequestVersion {
    id: u64,
    // There should be a better way to store a fixed-length string… I wonder which one.
    head_commit_sha: String,
    base_commit_sha: String,
    start_commit_sha: String,
    created_at: NaiveDateTime,
    merge_request_id: u64,
}

/// see https://docs.gitlab.com/ee/api/merge_requests.html#get-single-merge-request-changes
#[derive(Deserialize, Debug)]
struct MR {
    changes: Vec<MRChanges>,
}

#[derive(Deserialize, Debug)]
struct MRChanges {
    new_path: String,
    deleted_file: bool,
}

fn get_mr_changed_files(
    base_url: &str,
    project_id: &str,
    merge_request_id: &str,
    gitlab_token: &str,
) -> Result<Vec<String>, failure::Error> {
    let client = Client::new();

    let resp = client
        .get(format!(
            "{base_url}/projects/{project_id}/merge_requests/{merge_request_id}/versions"
        ))
        .header("PRIVATE-TOKEN", gitlab_token)
        .send()?;
    let resp: MR = resp.json()?;
    Ok(resp
        .changes
        .iter()
        .filter(|c| !c.deleted_file)
        .map(|change| change.new_path.to_string())
        .collect())
}

pub fn post_to_diff(comments: &[DiffLineComment]) -> Result<(), failure::Error> {
    let Ok(base_url) = env::var("CI_API_V4_URL") else {
        error!("Could not find CI_API_V4_URL in the environment, which is a default GitLab-CI variable. Am I running on GitLab-CI?");
        exit(EXIT_CODE_BAD_CONFIG);
    };
    let Ok(project_id) =
        env::var("CI_PROJECT_ID") else {
        error!("Could not find CI_PROJECT_ID in the environment, which is a default GitLab-CI variable. Am I running on GitLab-CI?");
        exit(EXIT_CODE_BAD_CONFIG);
    };
    let Ok(merge_request_id) = env::var("CI_MERGE_REQUEST_IID")else {
        error!("Could not find CI_MERGE_REQUEST_IID in the environment, am I running in a merge request pipeline?");
        exit(EXIT_CODE_BAD_CONFIG);
    };
    let Ok(gitlab_token) = env::var("GITLAB_USER_TOKEN") else {
        error!("Could not find GITLAB_USER_TOKEN in the environment. I need it to send what I have to say.");
        exit(EXIT_CODE_BAD_CONFIG);
    };
    let mr_changed_paths =
        get_mr_changed_files(&base_url, &project_id, &merge_request_id, &gitlab_token)?;
    let client = Client::new();
    let resp = client
        .get(format!(
            "{base_url}/projects/{project_id}/merge_requests/{merge_request_id}/versions"
        ))
        .header("PRIVATE-TOKEN", &gitlab_token)
        .send()?;
    let resp_object: Vec<MergeRequestVersion> = resp.json()?;
    let last_version = resp_object.first().ok_or(failure::err_msg(format!(
        "Could not find a valid Merge Request version! Here's what I got from gitlab: {:?}",
        &resp_object
    )))?;
    for diff_comment in comments
        .iter()
        .filter(|comment| mr_changed_paths.contains(&comment.path))
    {
        let _resp = client
            .post(format!(
                "{base_url}/projects/{project_id}/merge_requests/{merge_request_id}/versions"
            ))
            .header("PRIVATE-TOKEN", &gitlab_token)
            .form(&{
                let mut formdata = HashMap::new();
                formdata.insert("position[position_type]", "text");
                formdata.insert("position[base_sha]", &last_version.base_commit_sha);
                formdata.insert("position[head_sha]", &last_version.head_commit_sha);
                formdata.insert("position[start_sha]", &last_version.start_commit_sha);
                formdata.insert("position[new_path]", &diff_comment.path);
                formdata.insert("position[old_path]", &diff_comment.path); // this may cause issues if the file has been renamed in this merge request. We would need to parse the git revlog of the file and I don't want to :(
                formdata.insert("position[new_line]", &diff_comment.line);
                formdata.insert("body", &diff_comment.body);
                formdata
            })
            .send()?;
    }
    Ok(())
}
