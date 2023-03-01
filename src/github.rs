use std::path::PathBuf;

use base64::{engine::general_purpose, Engine};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;

pub struct Github {
    token: String,
    repo: String,
    branch: String,
}

impl Github {
    pub fn new(token: &str, repo: &str, branch: &str) -> Github {
        Github {
            token: token.to_owned(),
            repo: repo.to_owned(),
            branch: branch.to_owned(),
        }
    }
}

#[derive(Debug)]
enum Content<'a> {
    Path(&'a PathBuf),
    Str(&'a str),
}
#[derive(Debug)]
pub struct NewContent<'c> {
    git_path: String,
    content: Content<'c>,
}

impl<'c> NewContent<'c> {
    pub fn path(git_path: &str, path: &'c PathBuf) -> NewContent<'c> {
        NewContent {
            git_path: git_path.to_owned(),
            content: Content::Path(path),
        }
    }

    pub fn text(git_path: &str, str: &'c str) -> NewContent<'c> {
        NewContent {
            git_path: git_path.to_owned(),
            content: Content::Str(str),
        }
    }
}

#[derive(Deserialize, Debug)]
struct RepoState {
    object: RepoObject,
}

#[derive(Deserialize, Debug)]
struct RepoObject {
    sha: String,
}

impl Github {
    pub async fn commit(
        &self,
        path_name: &str,
        content: &str,
        commit_msg: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let oid = self.get_oid().await?;
        self.add_file(&oid, path_name, content, commit_msg).await
    }

    async fn get_oid(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.github.com/repos/{}/git/ref/heads/{}",
            self.repo, self.branch
        );
        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .header(
                reqwest::header::AUTHORIZATION,
                format!("bearer {}", self.token),
            )
            .header(reqwest::header::USER_AGENT, &self.repo)
            .send()
            .await?
            .text()
            .await?;

        // TODO: panic or pass on the error?
        match serde_json::from_str::<RepoState>(&res) {
            Ok(state) => Ok(state.object.sha),
            Err(_) => panic!("Unexpected JSON from get_oid call: {}", res),
        }
    }

    async fn add_file(
        &self,
        oid: &str,
        path_name: &str,
        content: &str,
        commit_msg: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let payload: String = self.mutation_json(oid, path_name, content, commit_msg);

        let client = reqwest::Client::new();

        let res = client
            .post("https://api.github.com/graphql")
            .header(
                reqwest::header::AUTHORIZATION,
                format!("bearer {}", self.token),
            )
            .header(reqwest::header::USER_AGENT, &self.repo)
            .body(payload)
            .send()
            .await?
            .text()
            .await?;

        let v: Value = serde_json::from_str(&res)?;

        match v.get("errors") {
            Some(_) => panic!("Error when commiting file: {}", res),
            None => Ok(()),
        }
    }

    fn mutation_json(&self, oid: &str, path_name: &str, content: &str, commit_msg: &str) -> String {
        // "The contents of a FileAddition must be encoded using RFC 4648 compliant base64,
        // i.e. correct padding is required and no characters outside the standard alphabet may be used.
        let b64_content = general_purpose::STANDARD.encode(content);

        let payload = json!({
            "query": "mutation ($input: CreateCommitOnBranchInput!) { createCommitOnBranch(input: $input) { commit { url } } }",
            "variables": {
            "input": {
                "branch": {
                    "repositoryNameWithOwner": format!("{}", self.repo),
                    "branchName": format!("{}", self.branch),
                },
                "message": { "headline": format!("{}", commit_msg) },
                "fileChanges": {
                    "additions": [ {
                        "path": format!("{}", path_name),
                        "contents": format!("{}", b64_content),
                    } ]
                },
                "expectedHeadOid": format!("{}", oid)
            }
            }
        });

        payload.to_string()
    }
}
