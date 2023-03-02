use std::error::Error;
use std::path::PathBuf;

use base64::{engine::general_purpose, Engine};
use serde::Deserialize;
use serde_json::json;
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

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

#[derive(Debug, Clone)]
enum Content {
    Path(PathBuf),
    Str(String),
}
#[derive(Debug, Clone)]
pub struct NewContent {
    git_path: String,
    content: Content,
}

impl NewContent {
    pub fn path(git_path: &str, path: &PathBuf) -> NewContent {
        NewContent {
            git_path: git_path.to_owned(),
            content: Content::Path(path.to_owned()),
        }
    }

    pub fn text(git_path: &str, str: &str) -> NewContent {
        NewContent {
            git_path: git_path.to_owned(),
            content: Content::Str(str.to_owned()),
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
        commit_msg: &str,
        content: &[NewContent],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let oid = self.get_oid().await?;
        self.add_files(&oid, commit_msg, content).await
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

    async fn add_files(
        &self,
        oid: &str,
        commit_msg: &str,
        content: &[NewContent],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let payload: String = self.mutation_json(oid, commit_msg, content).await?;

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

    async fn read_file(path: &PathBuf) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut buf = Vec::new();
        File::open(path).await?.read_to_end(&mut buf).await?;
        Ok(buf)
    }

    async fn to_addition(content: &NewContent) -> Result<Value, Box<dyn Error>> {
        // "The contents of a FileAddition must be encoded using RFC 4648 compliant base64,
        // i.e. correct padding is required and no characters outside the standard alphabet may be used.
        let b64_content = match &content.content {
            Content::Path(p) => general_purpose::STANDARD.encode(Github::read_file(p).await?),
            Content::Str(s) => general_purpose::STANDARD.encode(s),
        };

        Ok(json!({
             "path": format!("{}", content.git_path),
             "contents": format!("{}", b64_content),
        }))
    }

    async fn mutation_json(
        &self,
        oid: &str,
        commit_msg: &str,
        contents: &[NewContent],
    ) -> Result<String, Box<dyn Error>> {
        let mut additions = Vec::new();
        for content in contents {
            let val = Github::to_addition(content).await?;
            additions.push(val);
        }

        // NB: CreateBlob+createTree+CreateCommitOnBranchInput+updateRef may be an alaternative if file size is an issue. 

        let payload = json!({
            "query": "mutation ($input: CreateCommitOnBranchInput!) { createCommitOnBranch(input: $input) { commit { url } } }",
            "variables": {
            "input": {
                "branch": {
                    "repositoryNameWithOwner": format!("{}", self.repo),
                    "branchName": format!("{}", self.branch),
                },
                "message": { "headline": format!("{commit_msg}") },
                "fileChanges": {
                    "additions": additions
                },
                "expectedHeadOid": format!("{oid}")
            }
            }
        });

        Ok(payload.to_string())
    }
}
