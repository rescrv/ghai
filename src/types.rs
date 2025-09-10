use crate::http::{GitHubClient, UrlBuilder};
use chrono::{DateTime, TimeZone};

/// Trait for types that can fetch comments
pub trait CommentFetcher: Sync {
    fn comments_url(&self) -> &str;

    fn fetch_comments<Tz: TimeZone>(
        &self,
        since: Option<DateTime<Tz>>,
    ) -> impl std::future::Future<Output = Result<Vec<IssueComment>, Box<dyn std::error::Error>>> + Send
    where
        Tz::Offset: Send,
    {
        async {
            let url = UrlBuilder::new(self.comments_url())
                .param("since", since.map(|s| s.to_rfc3339()))
                .build();

            let client = GitHubClient::new()?;
            let response = client
                .get(&url)
                .send()
                .await?
                .json::<Vec<IssueComment>>()
                .await?;

            Ok(response)
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct IssueDependenciesSummary {
    pub blocked_by: i64,
    pub blocking: i64,
    pub total_blocked_by: i64,
    pub total_blocking: i64,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct IssueFieldValueSingleSelectOption {
    pub id: i64,
    pub name: String,
    pub color: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct IssueFieldValue {
    pub issue_field_id: i64,
    pub node_id: String,
    pub data_type: String,
    pub value: Option<serde_json::Value>,
    pub single_select_option: Option<IssueFieldValueSingleSelectOption>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct IssueType {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub is_enabled: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Commit {
    pub id: String,
    pub tree_id: String,
    pub distinct: bool,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: CommitUser,
    pub committer: CommitUser,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct CommitUser {
    pub name: String,
    pub email: String,
    pub username: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Repository {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub owner: SimpleUser,
    pub html_url: String,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: Option<String>,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub pulls_url: String,
    pub releases_url: String,
    pub ssh_url: Option<String>,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub clone_url: Option<String>,
    pub mirror_url: Option<String>,
    pub hooks_url: String,
    pub svn_url: Option<String>,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub forks_count: Option<u64>,
    pub stargazers_count: Option<u64>,
    pub watchers_count: Option<u64>,
    pub size: Option<u64>,
    pub default_branch: Option<String>,
    pub open_issues_count: Option<u64>,
    pub is_template: Option<bool>,
    pub topics: Option<Vec<String>>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_pages: Option<bool>,
    pub has_downloads: Option<bool>,
    pub has_discussions: Option<bool>,
    pub archived: Option<bool>,
    pub disabled: Option<bool>,
    pub visibility: Option<String>,
    pub pushed_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub permissions: Option<RepositoryPermissions>,
    pub allow_rebase_merge: Option<bool>,
    pub template_repository: Option<serde_json::Value>,
    pub temp_clone_token: Option<String>,
    pub allow_squash_merge: Option<bool>,
    pub allow_auto_merge: Option<bool>,
    pub delete_branch_on_merge: Option<bool>,
    pub allow_merge_commit: Option<bool>,
    pub allow_forking: Option<bool>,
    pub web_commit_signoff_required: Option<bool>,
    pub subscribers_count: Option<u64>,
    pub network_count: Option<u64>,
    pub license: Option<RepositoryLicense>,
    pub forks: Option<u64>,
    pub open_issues: Option<u64>,
    pub watchers: Option<u64>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryPermissions {
    pub admin: bool,
    pub maintain: Option<bool>,
    pub push: bool,
    pub pull: bool,
    pub triage: Option<bool>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryLicense {
    pub key: String,
    pub name: String,
    pub spdx_id: Option<String>,
    pub url: Option<String>,
    pub node_id: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Milestone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: i64,
    pub node_id: String,
    pub number: u64,
    pub title: String,
    pub description: Option<String>,
    pub creator: Option<SimpleUser>,
    pub open_issues: u64,
    pub closed_issues: u64,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub due_on: Option<String>,
    pub closed_at: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct IssuePullRequest {
    pub url: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub merged_at: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Integration {
    pub id: i64,
    pub slug: Option<String>,
    pub node_id: String,
    pub owner: SimpleUser,
    pub name: String,
    pub description: Option<String>,
    pub external_url: String,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub permissions: IntegrationPermissions,
    pub events: Vec<String>,
    pub installations_count: Option<u64>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub webhook_secret: Option<String>,
    pub pem: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrationPermissions {
    pub issues: Option<String>,
    pub checks: Option<String>,
    pub metadata: Option<String>,
    pub contents: Option<String>,
    pub deployments: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorAssociation {
    Owner,
    Member,
    Collaborator,
    Contributor,
    FirstTimeContributor,
    FirstTimer,
    Mannequin,
    None,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReactionRollup {
    pub url: String,
    pub total_count: u64,
    #[serde(rename = "+1")]
    pub plus_one: u64,
    #[serde(rename = "-1")]
    pub minus_one: u64,
    pub laugh: u64,
    pub hooray: u64,
    pub confused: u64,
    pub heart: u64,
    pub rocket: u64,
    pub eyes: u64,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SubIssuesSummary {
    pub completed: u64,
    pub percent_completed: u64,
    pub total: u64,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_link: Link,
    pub html: Link,
    pub issue: Option<Link>,
    pub comments: Option<Link>,
    pub review_comments: Option<Link>,
    pub review_comment: Option<Link>,
    pub commits: Option<Link>,
    pub statuses: Option<Link>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Team {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub html_url: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub privacy: String,
    pub permission: String,
    pub members_url: String,
    pub repositories_url: String,
    pub parent: Option<Box<Team>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AutoMerge {
    pub enabled_by: SimpleUser,
    pub merge_method: String,
    pub commit_title: Option<String>,
    pub commit_message: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct HeadCommit {
    pub id: String,
    pub tree_id: String,
    pub message: String,
    pub timestamp: String,
    pub author: CommitUser,
    pub committer: CommitUser,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SimpleUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub login: String,
    pub id: i64,
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
    pub id: i64,
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
        issue: Box<Issue>,
        comment: Box<IssueComment>,
    },
    Issue {
        action: String,
        issue: Box<Issue>,
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
        commits: Vec<Commit>,
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
        pull_request: Box<PullRequest>,
    },
    PullRequestReview {
        action: String,
        pull_request: Box<PullRequest>,
        review: Box<PullRequestReview>,
    },
    PullRequestComment {
        action: String,
        pull_request: Box<PullRequest>,
        comment: Box<PullRequestReviewComment>,
    },
    Forkee {
        forkee: Box<Repository>,
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

impl Label {
    pub fn name(&self) -> &str {
        match self {
            Self::Detailed { name, .. } => name.as_str(),
            Self::Simple(name) => name.as_str(),
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Link {
    pub href: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Issue {
    pub id: i64,
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
    pub milestone: Option<Milestone>,
    pub locked: bool,
    pub active_lock_reason: Option<String>,
    pub comments: u64,
    pub pull_request: Option<IssuePullRequest>,
    pub closed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub draft: Option<bool>,
    pub closed_by: Option<SimpleUser>,
    pub body_html: Option<String>,
    pub body_text: Option<String>,
    pub timeline_url: Option<String>,
    pub repository: Option<Repository>,
    pub performed_via_github_app: Option<Integration>,
    pub author_association: AuthorAssociation,
    pub reactions: Option<ReactionRollup>,
    pub sub_issues_summary: Option<SubIssuesSummary>,
    pub issue_dependencies_summary: Option<IssueDependenciesSummary>,
    pub issue_field_values: Option<Vec<IssueFieldValue>>,
    pub parent_issue_url: Option<String>,
    #[serde(rename = "type")]
    pub r#type: Option<IssueType>,
}

impl Issue {
    #[allow(clippy::too_many_arguments)]
    pub async fn fetch_user_issues<Tz: TimeZone>(
        filter: Option<String>,
        state: Option<String>,
        labels: Option<String>,
        sort: Option<String>,
        direction: Option<String>,
        since: Option<DateTime<Tz>>,
        per_page: Option<u64>,
        page: Option<u64>,
    ) -> Result<Vec<Issue>, Box<dyn std::error::Error>> {
        let url = UrlBuilder::new("https://api.github.com/issues")
            .param("filter", filter)
            .param("state", state)
            .param("labels", labels)
            .param("sort", sort)
            .param("direction", direction)
            .param("since", since.map(|s| s.to_rfc3339()))
            .param("per_page", per_page)
            .param("page", page)
            .build();

        let client = GitHubClient::new()?;
        let issues: Vec<Issue> = client.get(&url).send().await?.json().await?;

        Ok(issues)
    }
}

impl CommentFetcher for Issue {
    fn comments_url(&self) -> &str {
        &self.comments_url
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct IssueComment {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub body: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub html_url: String,
    pub user: Option<SimpleUser>,
    pub created_at: String,
    pub updated_at: String,
    pub issue_url: String,
    pub author_association: AuthorAssociation,
    pub performed_via_github_app: Option<Integration>,
    pub reactions: Option<ReactionRollup>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReview {
    pub _links: Option<Links>,
    pub author_association: String,
    pub body: Option<String>,
    pub commit_id: String,
    pub html_url: String,
    pub id: i64,
    pub node_id: String,
    pub pull_request_url: String,
    pub state: String,
    pub submitted_at: String,
    pub user: SimpleUser,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PullRequestReviewComment {
    pub id: i64,
    pub node_id: String,
    pub url: String,
    pub body: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub html_url: String,
    pub user: Option<SimpleUser>,
    pub created_at: String,
    pub updated_at: String,
    pub issue_url: Option<String>,
    pub author_association: AuthorAssociation,
    pub performed_via_github_app: Option<Integration>,
    pub reactions: Option<ReactionRollup>,
    pub _links: Option<Links>,
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
    pub repo: Repository,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PullRequest {
    pub url: String,
    pub id: i64,
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
    pub milestone: Option<Milestone>,
    pub active_lock_reason: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub assignee: Option<SimpleUser>,
    pub assignees: Option<Vec<SimpleUser>>,
    pub requested_reviewers: Option<Vec<SimpleUser>>,
    pub requested_teams: Option<Vec<Team>>,
    pub head: PullRequestHead,
    pub base: PullRequestHead,
    pub _links: Links,
    pub author_association: AuthorAssociation,
    pub auto_merge: Option<AutoMerge>,
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

impl CommentFetcher for PullRequest {
    fn comments_url(&self) -> &str {
        &self.comments_url
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
    pub repository: Repository,
    pub url: String,
    pub subscription_url: String,
}

impl Notification {
    pub async fn fetch_all<Tz: chrono::TimeZone>(
        all: bool,
        participating: bool,
        since: Option<DateTime<Tz>>,
        before: Option<DateTime<Tz>>,
    ) -> Result<Vec<Notification>, Box<dyn std::error::Error>> {
        let url = UrlBuilder::new("https://api.github.com/notifications")
            .param("all", if all { Some("true") } else { None })
            .param(
                "participating",
                if participating { Some("true") } else { None },
            )
            .param("since", since.map(|s| s.to_rfc3339()))
            .param("before", before.map(|s| s.to_rfc3339()))
            .build();

        let client = GitHubClient::new()?;
        let mut notifications: Vec<Notification> = client.get(&url).send().await?.json().await?;

        notifications.sort_by_key(|n| n.updated_at.clone());
        Ok(notifications)
    }

    pub async fn fetch_pull_request(&self) -> Result<PullRequest, Box<dyn std::error::Error>> {
        if self.subject.r#type != "PullRequest" {
            return Err("not a pull request".into());
        }

        let client = GitHubClient::new()?;
        let pull_request: PullRequest = client.get(&self.subject.url).send().await?.json().await?;

        Ok(pull_request)
    }

    pub async fn fetch_issue(&self) -> Result<Issue, Box<dyn std::error::Error>> {
        if self.subject.r#type != "Issue" {
            return Err("not an issue".into());
        }

        let client = GitHubClient::new()?;
        let issue: Issue = client.get(&self.subject.url).send().await?.json().await?;

        Ok(issue)
    }

    pub async fn mark_as_read(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = GitHubClient::new()?;
        let url = format!("https://api.github.com/notifications/threads/{}", self.id);

        client
            .request(reqwest::Method::PATCH, &url)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
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
    pub id: i64,
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
    pub pull_requests: Vec<IssuePullRequest>,
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
    pub head_commit: HeadCommit,
    pub repository: Repository,
    pub head_repository: Repository,
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
    pub async fn fetch_all(
        owner: String,
        repo: String,
        actor: Option<String>,
        workflow_run_branch: Option<String>,
        event: Option<String>,
        workflow_run_status: Option<String>,
        per_page: Option<u64>,
        page: Option<u64>,
    ) -> Result<Vec<Action>, Box<dyn std::error::Error>> {
        let url = UrlBuilder::new(format!(
            "https://api.github.com/repos/{owner}/{repo}/actions/runs"
        ))
        .param("actor", actor)
        .param("workflow_run_branch", workflow_run_branch)
        .param("event", event)
        .param("workflow_run_status", workflow_run_status)
        .param("per_page", per_page)
        .param("page", page)
        .build();

        let client = GitHubClient::new()?;
        let mut runs: Runs = client.get(&url).send().await?.json().await?;

        runs.workflow_runs.sort_by_key(|n| n.updated_at.clone());
        Ok(runs.workflow_runs)
    }

    pub async fn fetch_jobs(&self) -> Result<Vec<Job>, Box<dyn std::error::Error>> {
        let client = GitHubClient::new()?;
        let run_jobs: RunJobs = client.get(&self.jobs_url).send().await?.json().await?;

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
    pub id: i64,
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
