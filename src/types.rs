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
#[serde(tag = "type")]
pub struct Event {
    pub r#type: String,
    pub id: String,
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
pub enum EventPayload {
    IssueComment {
        action: String,
        issue: Issue,
        comment: IssueComment,
    },
    Issue {
        action: String,
        issue: Issue,
    },
    Create {
        description: String,
        master_branch: String,
        pusher_type: String,
        r#ref: String,
        ref_type: String,
    },
    PushEvent {
        before: String,
        // TODO(rescrv): commits
        commits: Vec<serde_json::Value>,
        distinct_size: u64,
        head: String,
        push_id: u64,
        r#ref: String,
        repository_id: u64,
        size: u64,
    },
    Delete {
        pusher_type: String,
        r#ref: String,
        ref_type: String,
    },
    PullRequest {
        action: String,
        number: u64,
        pull_request: PullRequest,
    },
    PullRequestReview {
        action: String,
        pull_request: PullRequest,
        review: PullRequestReview,
    },
    PullRequestComment {
        action: String,
        pull_request: PullRequest,
        comment: PullRequestReviewComment,
    },
    Forkee {
        // TODO(rescrv): repository
        forkee: serde_json::Value,
    },
    Action {
        action: String,
    },
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Label {
    Detailed {
        id: serde_json::Value,
        node_id: Option<String>,
        url: Option<String>,
        name: String,
        color: String,
        default: bool,
        description: Option<String>,
    },
    Simple(String),
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub href: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Issue {
    pub id: serde_json::Value,
    pub node_id: String,
    pub url: String,
    pub repository_url: String,
    pub labels_url: String,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub number: u64,
    pub state: String,
    pub state_reason: Option<String>,
    pub title: String,
    pub body: Option<String>,
    pub user: Option<SimpleUser>,
    pub labels: Vec<Label>,
    pub assignee: Option<SimpleUser>,
    pub assignees: Option<Vec<SimpleUser>>,
    // TODO(rescrv): milestone
    pub milestone: Option<serde_json::Value>,
    pub locked: bool,
    pub active_lock_reason: Option<String>,
    pub comments: u64,
    // TODO(rescrv): issue-pull-request
    pub pull_request: Option<serde_json::Value>,
    pub closed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub draft: Option<bool>,
    pub closed_by: Option<SimpleUser>,
    pub body_html: Option<String>,
    pub body_text: Option<String>,
    pub timeline_url: Option<String>,
    // TODO(rescrv): repository
    pub repository: Option<serde_json::Value>,
    // TODO(rescrv): integration
    pub performed_via_github_app: Option<serde_json::Value>,
    // TODO(rescrv): author-association
    pub author_association: serde_json::Value,
    // TODO(rescrv): reaction-rollup
    pub reactions: Option<serde_json::Value>,
    // TODO(rescrv): sub-issues-summary
    pub sub_issues_summary: Option<serde_json::Value>,
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReview {
    // TODO(rescrv):  _links
    pub _links: Option<serde_json::Value>,
    pub author_association: String,
    pub body: Option<String>,
    pub commit_id: String,
    pub html_url: String,
    pub id: serde_json::Value,
    pub node_id: String,
    pub pull_request_url: String,
    pub state: String,
    pub submitted_at: String,
    pub user: SimpleUser,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewComment {
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
    pub issue_url: Option<String>,
    // TODO(rescrv):  author-association
    pub author_association: String,
    // TODO(rescrv):  integration
    pub performed_via_github_app: Option<serde_json::Value>,
    // TODO(rescrv):  reaction-rollup
    pub reactions: Option<serde_json::Value>,
    // TODO(rescrv):  _links
    pub _links: Option<serde_json::Value>,
    pub commit_id: Option<String>,
    pub diff_hunk: Option<String>,
    pub line: Option<u64>,
    pub original_commit_id: Option<String>,
    pub original_line: Option<u64>,
    pub original_position: Option<u64>,
    pub original_start_line: Option<u64>,
    pub path: Option<String>,
    pub position: Option<u64>,
    pub pull_request_review_id: Option<u64>,
    pub pull_request_url: Option<String>,
    pub side: Option<String>,
    pub start_line: Option<u64>,
    pub start_side: Option<String>,
    pub subject_type: Option<String>,
    pub in_reply_to_id: Option<serde_json::Value>,
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
    pub merged: Option<bool>,
    pub mergeable: Option<bool>,
    pub rebaseable: Option<bool>,
    pub mergeable_state: Option<String>,
    pub merged_by: Option<SimpleUser>,
    pub comments: Option<u64>,
    pub review_comments: Option<u64>,
    pub maintainer_can_modify: Option<bool>,
    pub commits: Option<u64>,
    pub additions: Option<u64>,
    pub deletions: Option<u64>,
    pub changed_files: Option<u64>,
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

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReferencedWorkflow {
    path: String,
    sha: String,
    r#ref: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub id: serde_json::Value,
    pub name: Option<String>,
    pub node_id: String,
    pub check_suite_id: Option<u64>,
    pub check_suite_node_id: Option<String>,
    pub head_branch: Option<String>,
    pub head_sha: String,
    pub path: String,
    pub run_number: u64,
    pub run_attempt: Option<u64>,
    pub referenced_workflows: Vec<ReferencedWorkflow>,
    pub event: String,
    pub status: Option<String>,
    pub conclusion: Option<String>,
    pub workflow_id: u64,
    pub url: String,
    pub html_url: String,
    // TODO(rescrv): pull-request-minimal
    pub pull_requests: Vec<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
    pub actor: Option<SimpleUser>,
    pub triggering_actor: Option<SimpleUser>,
    pub run_started_at: Option<String>,
    pub jobs_url: String,
    pub logs_url: String,
    pub check_suite_url: String,
    pub artifacts_url: String,
    pub cancel_url: String,
    pub rerun_url: String,
    pub previous_attempt_url: Option<String>,
    pub workflow_url: String,
    // TODO(rescrv): head-commit
    pub head_commit: serde_json::Value,
    // TODO(rescrv): head-repository
    pub repository: serde_json::Value,
    // TODO(rescrv): head-repository
    pub head_repository: serde_json::Value,
    pub head_repository_id: Option<u64>,
    pub display_title: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Runs {
    pub total_count: u64,
    pub workflow_runs: Vec<Action>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunJobs {
    pub total_count: u64,
    pub jobs: Vec<Job>,
}

impl Action {
    #[allow(clippy::too_many_arguments)]
    pub fn fetch_all(
        owner: String,
        repo: String,
        actor: Option<String>,
        workflow_run_branch: Option<String>,
        event: Option<String>,
        workflow_run_status: Option<String>,
        per_page: Option<u64>,
        page: Option<u64>,
    ) -> Result<Vec<Action>, std::io::Error> {
        let mut sep = '?';
        let mut url = format!("/repos/{owner}/{repo}/actions/runs");
        if let Some(actor) = actor {
            url.push_str(&format!("{sep}actor={actor}"));
            sep = '&';
        }
        if let Some(workflow_run_branch) = workflow_run_branch {
            url.push_str(&format!("{sep}workflow_run_branch={workflow_run_branch}"));
            sep = '&';
        }
        if let Some(event) = event {
            url.push_str(&format!("{sep}event={event}"));
            sep = '&';
        }
        if let Some(workflow_run_status) = workflow_run_status {
            url.push_str(&format!("{sep}workflow_run_status={workflow_run_status}"));
            sep = '&';
        }
        if let Some(per_page) = per_page {
            url.push_str(&format!("{sep}per_page={per_page}"));
            sep = '&';
        }
        if let Some(page) = page {
            url.push_str(&format!("{sep}page={page}"));
            sep = '&';
        }
        _ = sep;
        let output = std::process::Command::new("gh")
            .arg("api")
            .arg(url)
            .output()?;
        let mut runs: Runs = serde_json::from_slice(&output.stdout)?;
        runs.workflow_runs.sort_by_key(|n| n.updated_at.clone());
        Ok(runs.workflow_runs)
    }

    pub fn fetch_jobs(&self) -> Result<Vec<Job>, std::io::Error> {
        let sep = '?';
        let url = self.jobs_url.clone();
        _ = sep;
        let output = std::process::Command::new("gh")
            .arg("api")
            .arg(&url)
            .output()?;
        let run_jobs: RunJobs = serde_json::from_slice(&output.stdout)?;
        Ok(run_jobs.jobs)
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JobStep {
    pub status: String,
    pub conclusion: Option<String>,
    pub name: String,
    pub number: u64,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Job {
    pub id: serde_json::Value,
    pub run_id: u64,
    pub run_url: String,
    pub run_attempt: Option<u64>,
    pub node_id: String,
    pub head_sha: String,
    pub url: String,
    pub html_url: Option<String>,
    pub status: String,
    pub conclusion: Option<String>,
    pub created_at: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub name: String,
    pub steps: Vec<JobStep>,
    pub check_run_url: String,
    pub labels: Vec<String>,
    pub runner_id: Option<u64>,
    pub runner_name: Option<String>,
    pub runner_group_id: Option<u64>,
    pub runner_group_name: Option<String>,
    pub workflow_name: Option<String>,
    pub head_branch: Option<String>,
}
