#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub node_id: Option<String>,
    pub avatar_url: String,
    pub gravatar_id: String,
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
    pub user_view_type: Option<String>,
    pub site_admin: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Author {
    pub id: Option<String>,
    pub is_bot: Option<bool>,
    pub login: String,
    pub name: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Team {
    pub description: Option<String>,
    pub html_url: String,
    pub id: u64,
    pub members_url: String,
    pub name: String,
    pub node_id: Option<String>,
    pub notification_setting: String,
    pub parent: Option<String>,
    pub permission: String,
    pub privacy: String,
    pub repositories_url: String,
    pub slug: String,
    pub url: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Label {
    pub id: String,
    pub url: Option<String>,
    pub name: String,
    pub color: String,
    pub default: Option<bool>,
    pub description: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Milestone {
    pub closed_at: Option<String>,
    pub closed_issues: Option<u64>,
    pub created_at: Option<String>,
    pub creator: Option<User>,
    pub description: Option<String>,
    #[serde(rename = "dueOn")]
    pub due_on: Option<String>,
    pub html_url: Option<String>,
    pub id: Option<u64>,
    pub labels_url: Option<String>,
    pub node_id: Option<String>,
    pub number: Option<u64>,
    pub open_issues: Option<u64>,
    pub state: Option<String>,
    pub title: Option<String>,
    pub updated_at: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct PullRequest {
    url: String,
    html_url: String,
    diff_url: String,
    patch_url: String,
    merged_at: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Reactions {
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
pub struct Comment {
    pub id: String,
    pub author: Author,
    #[serde(rename = "authorAssociation")]
    pub author_association: String,
    pub body: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "includesCreatedEdit")]
    pub includes_created_edit: bool,
    #[serde(rename = "isMinimized")]
    pub is_minimized: bool,
    #[serde(rename = "minimizedReason")]
    pub minimized_reason: String,
    #[serde(rename = "reactionGroups")]
    pub reaction_groups: Vec<serde_json::Value>,
    pub url: String,
    #[serde(rename = "viewerDidAuthor")]
    pub viewer_did_author: bool,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Issue {
    pub author: Author,
    pub url: String,
    pub repository_url: Option<String>,
    pub labels_url: Option<String>,
    pub comments_url: Option<String>,
    pub events_url: Option<String>,
    pub html_url: Option<String>,
    pub id: String,
    pub node_id: Option<String>,
    pub number: u64,
    pub title: String,
    pub labels: Vec<Label>,
    pub state: String,
    pub locked: Option<bool>,
    pub assignee: Option<Author>,
    pub assignees: Vec<Author>,
    pub milestone: Option<Milestone>,
    pub comments: Vec<Comment>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub closed: Option<bool>,
    #[serde(rename = "closedAt")]
    pub closed_at: Option<String>,
    #[serde(rename = "closedBy")]
    pub closed_by: Option<Author>,
    #[serde(rename = "authorAssociation")]
    pub author_association: Option<String>,
    pub active_lock_reason: Option<String>,
    pub draft: Option<bool>,
    pub pull_request: Option<PullRequest>,
    pub body: Option<String>,
    pub reactions: Option<Reactions>,
    pub timeline_url: Option<String>,
    pub performed_via_github_app: Option<String>,
    pub state_reason: Option<String>,
    #[serde(rename = "projectCards")]
    pub project_cards: Option<serde_json::Value>,
    #[serde(rename = "projectItems")]
    pub project_items: Option<serde_json::Value>,
    #[serde(rename = "reactionGroups")]
    pub reaction_groups: Vec<serde_json::Value>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct License {
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: Option<String>,
    pub node_id: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Repo {
    pub id: String,
    pub node_id: Option<String>,
    pub name: String,
    pub full_name: Option<String>,
    pub private: Option<bool>,
    pub owner: Option<User>,
    pub html_url: Option<String>,
    pub description: Option<String>,
    pub fork: Option<bool>,
    pub url: Option<String>,
    pub forks_url: Option<String>,
    pub keys_url: Option<String>,
    pub collaborators_url: Option<String>,
    pub teams_url: Option<String>,
    pub hooks_url: Option<String>,
    pub issue_events_url: Option<String>,
    pub events_url: Option<String>,
    pub assignees_url: Option<String>,
    pub branches_url: Option<String>,
    pub tags_url: Option<String>,
    pub blobs_url: Option<String>,
    pub git_tags_url: Option<String>,
    pub git_refs_url: Option<String>,
    pub trees_url: Option<String>,
    pub statuses_url: Option<String>,
    pub languages_url: Option<String>,
    pub stargazers_url: Option<String>,
    pub contributors_url: Option<String>,
    pub subscribers_url: Option<String>,
    pub subscription_url: Option<String>,
    pub commits_url: Option<String>,
    pub git_commits_url: Option<String>,
    pub comments_url: Option<String>,
    pub issue_comment_url: Option<String>,
    pub contents_url: Option<String>,
    pub compare_url: Option<String>,
    pub merges_url: Option<String>,
    pub archive_url: Option<String>,
    pub downloads_url: Option<String>,
    pub issues_url: Option<String>,
    pub pulls_url: Option<String>,
    pub milestones_url: Option<String>,
    pub notifications_url: Option<String>,
    pub labels_url: Option<String>,
    pub releases_url: Option<String>,
    pub deployments_url: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub pushed_at: Option<String>,
    pub git_url: Option<String>,
    pub ssh_url: Option<String>,
    pub clone_url: Option<String>,
    pub svn_url: Option<String>,
    pub homepage: Option<String>,
    pub size: Option<u64>,
    pub stargazers_count: Option<u64>,
    pub watchers_count: Option<u64>,
    pub language: Option<String>,
    pub has_issues: Option<bool>,
    pub has_projects: Option<bool>,
    pub has_downloads: Option<bool>,
    pub has_wiki: Option<bool>,
    pub has_pages: Option<bool>,
    pub has_discussions: Option<bool>,
    pub forks_count: Option<u64>,
    pub mirror_url: Option<String>,
    pub archived: Option<bool>,
    pub disabled: Option<bool>,
    pub open_issues_count: Option<u64>,
    pub license: Option<License>,
    pub allow_forking: Option<bool>,
    pub is_template: Option<bool>,
    pub web_commit_signoff_required: Option<bool>,
    pub topics: Option<Vec<String>>,
    pub visibility: Option<String>,
    pub forks: Option<u64>,
    pub open_issues: Option<u64>,
    pub watchers: Option<u64>,
    pub default_branch: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Commitish {
    #[serde(rename = "authoredDate")]
    pub authored_date: Option<String>,
    #[serde(rename = "committedDate")]
    pub committed_date: Option<String>,
    pub authors: Option<serde_json::Value>,
    #[serde(rename = "messageBody")]
    pub message_body: Option<String>,
    #[serde(rename = "messageHeadline")]
    pub message_headline: Option<String>,
    pub oid: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Href {
    pub href: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct Links {
    comments: Href,
    commits: Href,
    html: Href,
    issue: Href,
    review_comment: Href,
    review_comments: Href,
    #[serde(rename = "self")]
    this: Href,
    statuses: Href,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct AutoMerge {
    pub commit_message: Option<String>,
    pub commit_title: Option<String>,
    pub enabled_by: User,
    pub merge_method: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
// TODO(rescrv): #[serde(deny_unknown_fields)]
pub struct Pull {
    pub author: Author,
    pub url: String,
    pub id: String,
    pub node_id: Option<String>,
    pub html_url: Option<String>,
    pub diff_url: Option<String>,
    pub patch_url: Option<String>,
    pub issue_url: Option<String>,
    pub number: u64,
    pub state: String,
    pub locked: Option<bool>,
    pub title: String,
    pub body: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub closed: Option<bool>,
    #[serde(rename = "closedAt")]
    pub closed_at: Option<String>,
    pub merged_at: Option<String>,
    pub merge_commit_sha: Option<String>,
    pub assignee: Option<User>,
    pub assignees: Vec<User>,
    #[serde(rename = "requestedReviewers")]
    pub requested_reviewers: Option<Vec<User>>,
    #[serde(rename = "requestedTeams")]
    pub requested_teams: Option<Vec<Team>>,
    pub labels: Vec<Label>,
    pub milestone: Option<Milestone>,
    pub draft: Option<bool>,
    pub maintainer_can_modify: Option<bool>,
    pub merged: Option<bool>,
    pub mergeable: Option<String>,
    pub mergeable_state: Option<String>,
    pub merged_by: Option<User>,
    pub rebaseable: Option<bool>,
    pub commits: Vec<Commitish>,
    pub commits_url: Option<String>,
    pub review_comments: Option<u64>,
    pub review_comments_url: Option<String>,
    pub review_comment_url: Option<String>,
    pub comments: Vec<Comment>,
    pub comments_url: Option<String>,
    pub statuses_url: Option<String>,
    pub additions: u64,
    pub deletions: u64,
    #[serde(rename = "changedFiles")]
    pub changed_files: Option<u64>,
    #[serde(rename = "authorAssociation")]
    pub author_association: Option<String>,
    pub auto_merge: Option<AutoMerge>,
    #[serde(rename = "autoMergeRequest")]
    pub auto_merge_request: Option<serde_json::Value>,
    #[serde(rename = "baseRefName")]
    pub base_ref_name: Option<serde_json::Value>,
    pub active_lock_reason: Option<String>,
    pub files: Option<serde_json::Value>,
    #[serde(rename = "headRefName")]
    pub head_ref_name: Option<String>,
    #[serde(rename = "headRefOid")]
    pub head_ref_oid: Option<String>,
    #[serde(rename = "headRepository")]
    pub head_repository: Option<Repo>,
}
