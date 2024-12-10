use chrono::{DateTime, FixedOffset};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SimpleUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub login: String,
    pub id: serde_json::Value,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub site_admin: bool,
    pub starred_at: Option<String>,
    pub user_view_type: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Actor {
    pub id: serde_json::Value,
    pub login: String,
    pub display_login: Option<String>,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub avatar_url: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepoStub {
    pub id: u64,
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub id: String,
    pub r#type: Option<String>,
    pub actor: Actor,
    pub repo: RepoStub,
    pub org: Option<Actor>,
    pub payload: serde_json::Value,
    pub public: bool,
    pub created_at: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum EventPayload {}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Label {
    pub id: serde_json::Value,
    pub node_id: Option<String>,
    pub url: Option<String>,
    pub name: String,
    pub color: String,
    pub default: bool,
    pub description: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub href: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct IssueComment {
    pub id: serde_json::Value,
    pub node_id: String,
    pub url: String,
    pub body: Option<String>,
    pub body_text: Option<String>,
    pub body_hmtl: Option<String>,
    pub html_url: String,
    pub user: Option<SimpleUser>,
    pub created_at: String,
    pub updated_at: String,
    pub issue_url: String,
    // TODO(rescrv):  author-association
    pub author_association: String,
    // TODO(rescrv):  integration
    pub performed_via_github_app: Option<serde_json::Value>,
    // TODO(rescrv):  reaction-rollup
    pub reactions: Option<serde_json::Value>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PullRequestHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub r#ref: String,
    pub sha: String,
    pub user: SimpleUser,
    // TODO(rescrv):  repository
    pub repo: serde_json::Value,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PullRequest {
    pub url: String,
    pub id: serde_json::Value,
    pub node_id: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub issue_url: String,
    pub commits_url: String,
    pub review_comments_url: String,
    pub review_comment_url: String,
    pub comments_url: String,
    pub statuses_url: String,
    pub number: u64,
    pub state: String,
    pub locked: bool,
    pub title: String,
    pub user: SimpleUser,
    pub body: Option<String>,
    pub labels: Vec<Label>,
    // TODO(rescrv): nullable-milestone
    pub milestone: serde_json::Value,
    pub active_lock_reason: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub assignee: Option<SimpleUser>,
    pub assignees: Option<Vec<SimpleUser>>,
    pub requested_reviewers: Option<Vec<SimpleUser>>,
    // TODO(rescrv): team-simple
    pub requested_teams: Option<serde_json::Value>,
    pub head: PullRequestHead,
    pub base: PullRequestHead,
    // TODO(rescrv): _links
    pub _links: serde_json::Value,
    // TODO(rescrv): author-association
    pub author_association: String,
    // TODO(rescrv): auto-merge
    pub auto_merge: serde_json::Value,
    pub draft: Option<bool>,
    pub merged: bool,
    pub mergeable: Option<bool>,
    pub rebaseable: Option<bool>,
    pub mergeable_state: String,
    pub merged_by: Option<SimpleUser>,
    pub comments: u64,
    pub review_comments: u64,
    pub maintainer_can_modify: bool,
    pub commits: u64,
    pub additions: u64,
    pub deletions: u64,
    pub changed_files: u64,
}

impl PullRequest {
    pub fn fetch_comments(
        &self,
        since: Option<DateTime<FixedOffset>>,
    ) -> Result<Vec<IssueComment>, std::io::Error> {
        let mut sep = '?';
        let mut url = self.comments_url.clone();
        if let Some(since) = since {
            url = format!("{url}{sep}since={}", since.to_rfc3339());
            sep = '&';
        }
        _ = sep;
        let output = std::process::Command::new("gh")
            .arg("api")
            .arg(&url)
            .output()?;
        Ok(serde_json::from_slice(&output.stdout)?)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NotificationSubject {
    pub title: String,
    pub url: String,
    pub latest_comment_url: Option<String>,
    pub r#type: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Notification {
    pub id: String,
    pub unread: bool,
    pub reason: String,
    pub updated_at: String,
    pub last_read_at: Option<String>,
    pub subject: NotificationSubject,
    // TODO(rescrv):  minimal-repository
    pub repository: serde_json::Value,
    pub url: String,
    pub subscription_url: String,
}

impl Notification {
    pub fn fetch_all(
        all: bool,
        participating: bool,
        since: Option<DateTime<FixedOffset>>,
        before: Option<DateTime<FixedOffset>>,
    ) -> Result<Vec<Notification>, std::io::Error> {
        let mut sep = '?';
        let mut url = "/notifications".to_string();
        if all {
            url.push_str(&format!("{}all=true", sep));
            sep = '&';
        }
        if participating {
            url.push_str(&format!("{}participating=true", sep));
            sep = '&';
        }
        if let Some(since) = since {
            url.push_str(&format!("{}since={}", sep, since.to_rfc3339()));
            sep = '&';
        }
        if let Some(before) = before {
            url.push_str(&format!("{}before={}", sep, before.to_rfc3339()));
            sep = '&';
        }
        _ = sep;
        let output = std::process::Command::new("gh")
            .arg("api")
            .arg(url)
            .output()?;
        let mut notifications: Vec<Notification> = serde_json::from_slice(&output.stdout)?;
        notifications.sort_by_key(|n| n.updated_at.clone());
        Ok(notifications)
    }

    pub fn fetch_pull_request(&self) -> Result<PullRequest, std::io::Error> {
        if self.subject.r#type != "PullRequest" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "not a pull request",
            ));
        }
        let output = std::process::Command::new("gh")
            .arg("api")
            .arg(&self.subject.url)
            .output()?;
        let pull_request: PullRequest = serde_json::from_slice(&output.stdout)?;
        Ok(pull_request)
    }
}
