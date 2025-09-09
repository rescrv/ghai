/// XML builder utility for constructing XML documents
pub struct XmlBuilder {
    content: String,
    indent_level: usize,
}

impl XmlBuilder {
    /// Create a new XML builder
    pub fn new() -> Self {
        Self {
            content: String::new(),
            indent_level: 0,
        }
    }

    /// Add a section with nested content
    pub fn section<F>(mut self, name: &str, builder: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        self.add_line(&format!("<{}>", name));
        self.indent_level += 1;
        self = builder(self);
        self.indent_level -= 1;
        self.add_line(&format!("</{}>", name));
        self
    }

    /// Add a field with a value
    pub fn field(mut self, name: &str, value: impl std::fmt::Display) -> Self {
        self.add_line(&format!("<{}>{}</{}>", name, value, name));
        self
    }

    /// Add an optional field (empty if None)
    pub fn optional_field(mut self, name: &str, value: Option<impl std::fmt::Display>) -> Self {
        if let Some(val) = value {
            self.field(name, val)
        } else {
            self.add_line(&format!("<{}></{}>", name, name));
            self
        }
    }

    /// Add a raw line of XML content
    pub fn raw_line(mut self, content: &str) -> Self {
        self.add_line(content);
        self
    }

    /// Build the final XML string
    pub fn build(self) -> String {
        self.content
    }

    fn add_line(&mut self, content: &str) {
        self.content.push_str(&"  ".repeat(self.indent_level));
        self.content.push_str(content);
        self.content.push('\n');
    }
}

impl Default for XmlBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Build notification context XML section
pub fn build_notification_context(
    id: &str,
    reason: &str,
    unread: bool,
    updated_at: &str,
    last_read_at: Option<&str>,
) -> String {
    XmlBuilder::new()
        .section("notification_context", |b| {
            b.field("id", id)
                .field("reason", reason)
                .field("status", if unread { "UNREAD" } else { "READ" })
                .field("last_updated", updated_at)
                .optional_field("last_read", last_read_at)
        })
        .build()
}

/// Build repository context XML section
pub fn build_repository_context(
    full_name: &str,
    owner_login: &str,
    owner_type: &str,
    private: bool,
    description: Option<&str>,
) -> String {
    XmlBuilder::new()
        .section("repository_context", |b| {
            b.field("full_name", full_name)
                .raw_line(&format!(
                    "<owner login=\"{}\" type=\"{}\">{}</owner>",
                    owner_login, owner_type, owner_login
                ))
                .field("private", private)
                .optional_field("description", description)
        })
        .build()
}

/// Build author XML section
pub fn build_author_context(login: &str, name: Option<&str>, association: &str) -> String {
    XmlBuilder::new()
        .section("author", |b| {
            b.field("login", login)
                .optional_field("name", name)
                .field("association", association)
        })
        .build()
}

/// Build reviewers XML section
pub fn build_reviewers_context(requested_reviewers: &[String]) -> String {
    if requested_reviewers.is_empty() {
        return String::from("<reviewers></reviewers>\n");
    }

    XmlBuilder::new()
        .section("reviewers", |mut b| {
            for reviewer in requested_reviewers {
                b = b.field("requested", reviewer);
            }
            b
        })
        .build()
}

/// Build statistics XML section
pub fn build_statistics_context(
    comments: Option<u64>,
    review_comments: Option<u64>,
    commits: Option<u64>,
    additions: Option<u64>,
    deletions: Option<u64>,
    changed_files: Option<u64>,
) -> String {
    XmlBuilder::new()
        .section("statistics", |b| {
            b.optional_field("comments", comments)
                .optional_field("review_comments", review_comments)
                .optional_field("commits", commits)
                .optional_field("additions", additions)
                .optional_field("deletions", deletions)
                .optional_field("changed_files", changed_files)
        })
        .build()
}

/// Build simple statistics XML section for issues
pub fn build_issue_statistics_context(comments: u64) -> String {
    XmlBuilder::new()
        .section("statistics", |b| b.field("comments", comments))
        .build()
}

/// Build dates XML section
pub fn build_dates_context(
    created_at: &str,
    updated_at: &str,
    closed_at: Option<&str>,
    merged_at: Option<&str>,
) -> String {
    XmlBuilder::new()
        .section("dates", |b| {
            b.field("created_at", created_at)
                .field("updated_at", updated_at)
                .optional_field("closed_at", closed_at)
                .optional_field("merged_at", merged_at)
        })
        .build()
}

/// Build labels XML section
pub fn build_labels_context(labels: &[impl std::fmt::Display]) -> String {
    if labels.is_empty() {
        return String::from("<labels></labels>\n");
    }

    XmlBuilder::new()
        .section("labels", |mut b| {
            for label in labels {
                b = b.field("label", label);
            }
            b
        })
        .build()
}

/// Build assignees XML section  
pub fn build_assignees_context(primary: Option<&str>, assignees: &[String]) -> String {
    XmlBuilder::new()
        .section("assignees", |mut b| {
            if let Some(primary) = primary {
                b = b.field("primary", primary);
            }
            for assignee in assignees {
                b = b.field("assignee", assignee);
            }
            b
        })
        .build()
}

/// Build branches XML section for pull requests
pub fn build_branches_context(
    head_ref: &str,
    head_sha: &str,
    base_ref: &str,
    base_sha: &str,
) -> String {
    let head_short_sha = &head_sha[..8.min(head_sha.len())];
    let base_short_sha = &base_sha[..8.min(base_sha.len())];

    XmlBuilder::new()
        .section("branches", |b| {
            b.raw_line(&format!(
                "<head ref=\"{}\" sha=\"{}\">{}</head>",
                head_ref, head_short_sha, head_ref
            ))
            .raw_line(&format!(
                "<base ref=\"{}\" sha=\"{}\">{}</base>",
                base_ref, base_short_sha, base_ref
            ))
        })
        .build()
}

/// Build description XML section with proper escaping and truncation
pub fn build_description_context(body: Option<&str>) -> String {
    match body {
        Some(body) if body.len() > 500 => {
            format!(
                "<description truncated=\"true\">\n{}\n... (truncated) ...\n</description>\n",
                escape_xml(&body[..500])
            )
        }
        Some(body) => {
            format!("<description>\n{}\n</description>\n", escape_xml(body))
        }
        None => "<description>\n</description>\n".to_string(),
    }
}

/// Build pull request context XML section
pub fn build_pull_request_context(
    number: u64,
    title: &str,
    state: &str,
    draft: Option<bool>,
    merged: Option<bool>,
    mergeable: Option<bool>,
    mergeable_state: Option<&str>,
) -> String {
    XmlBuilder::new()
        .section("pull_request", |mut b| {
            b = b
                .field("number", number)
                .field("title", escape_xml(title))
                .field("state", state);

            b = b.section("status", |mut status_b| {
                if let Some(draft) = draft {
                    status_b = status_b.field("draft", draft);
                }
                if let Some(merged) = merged {
                    status_b = status_b.field("merged", merged);
                }
                if let Some(mergeable) = mergeable {
                    status_b = status_b.field("mergeable", mergeable);
                }
                if let Some(mergeable_state) = mergeable_state {
                    status_b = status_b.field("mergeable_state", mergeable_state);
                }
                status_b
            });

            b
        })
        .build()
}

/// Build issue context XML section
pub fn build_issue_context(number: u64, title: &str, state: &str) -> String {
    XmlBuilder::new()
        .section("issue", |b| {
            b.field("number", number)
                .field("title", escape_xml(title))
                .field("state", state)
        })
        .build()
}

/// Build complete Pull Request notification context XML
pub fn build_pull_request_notification_context(
    thread: &crate::Notification,
    pr: &crate::PullRequest,
) -> String {
    let mut context = String::new();

    // <notification_context>
    context.push_str(&build_notification_context(
        &thread.id,
        &thread.reason,
        thread.unread,
        &thread.updated_at,
        thread.last_read_at.as_deref(),
    ));
    context.push('\n');

    // <repository_context>
    context.push_str(&build_repository_context(
        &thread.repository.full_name,
        &thread.repository.owner.login,
        &thread.repository.owner.r#type,
        thread.repository.private,
        thread.repository.description.as_deref(),
    ));
    context.push('\n');

    // <pull_request>
    context.push_str(&build_pull_request_context(
        pr.number,
        &pr.title,
        &pr.state,
        pr.draft,
        pr.merged,
        pr.mergeable,
        pr.mergeable_state.as_deref(),
    ));

    // <author>
    context.push_str(&build_author_context(
        &pr.user.login,
        pr.user.name.as_deref(),
        &format!("{:?}", pr.author_association),
    ));

    // <dates>
    context.push_str(&build_dates_context(
        &pr.created_at,
        &pr.updated_at,
        pr.closed_at.as_deref(),
        pr.merged_at.as_deref(),
    ));

    // <labels>
    let label_names: Vec<String> = pr.labels.iter().map(|l| l.name().to_string()).collect();
    context.push_str(&build_labels_context(&label_names));

    // <assignees>
    let assignee_names: Vec<String> = pr
        .assignees
        .as_ref()
        .map_or(Vec::new(), |a| a.iter().map(|u| u.login.clone()).collect());
    context.push_str(&build_assignees_context(
        pr.assignee.as_ref().map(|a| a.login.as_str()),
        &assignee_names,
    ));

    // <reviewers>
    let reviewer_names: Vec<String> = pr
        .requested_reviewers
        .as_ref()
        .map_or(Vec::new(), |r| r.iter().map(|u| u.login.clone()).collect());
    context.push_str(&build_reviewers_context(&reviewer_names));

    // <statistics>
    context.push_str(&build_statistics_context(
        pr.comments,
        pr.review_comments,
        pr.commits,
        pr.additions,
        pr.deletions,
        pr.changed_files,
    ));

    // <branches>
    context.push_str(&build_branches_context(
        &pr.head.r#ref,
        &pr.head.sha,
        &pr.base.r#ref,
        &pr.base.sha,
    ));

    // <description>
    context.push_str(&build_description_context(pr.body.as_deref()));

    context.push_str("</pull_request>\n\n");
    context
}

/// Build complete Issue notification context XML
pub fn build_issue_notification_context(
    thread: &crate::Notification,
    issue: &crate::Issue,
) -> String {
    let mut context = String::new();

    // <notification_context>
    context.push_str(&build_notification_context(
        &thread.id,
        &thread.reason,
        thread.unread,
        &thread.updated_at,
        thread.last_read_at.as_deref(),
    ));
    context.push('\n');

    // <repository_context>
    context.push_str(&build_repository_context(
        &thread.repository.full_name,
        &thread.repository.owner.login,
        &thread.repository.owner.r#type,
        thread.repository.private,
        thread.repository.description.as_deref(),
    ));
    context.push('\n');

    // <issue>
    context.push_str(&build_issue_context(
        issue.number,
        &issue.title,
        &issue.state,
    ));

    // <author>
    if let Some(user) = &issue.user {
        context.push_str(&build_author_context(
            &user.login,
            user.name.as_deref(),
            &format!("{:?}", issue.author_association),
        ));
    }

    // <dates>
    context.push_str(&build_dates_context(
        &issue.created_at,
        &issue.updated_at,
        issue.closed_at.as_deref(),
        None, // issues don't have merged_at
    ));

    // <labels>
    let label_names: Vec<String> = issue.labels.iter().map(|l| l.name().to_string()).collect();
    context.push_str(&build_labels_context(&label_names));

    // <assignees>
    let assignee_names: Vec<String> = issue
        .assignees
        .as_ref()
        .map_or(Vec::new(), |a| a.iter().map(|u| u.login.clone()).collect());
    context.push_str(&build_assignees_context(
        issue.assignee.as_ref().map(|a| a.login.as_str()),
        &assignee_names,
    ));

    // <statistics>
    context.push_str(&build_issue_statistics_context(issue.comments));

    // <description>
    context.push_str(&build_description_context(issue.body.as_deref()));

    context.push_str("</issue>\n\n");
    context
}

/// Escape XML special characters
fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_builder_simple() {
        let xml = XmlBuilder::new()
            .field("name", "test")
            .field("value", 42)
            .build();
        assert_eq!(xml, "<name>test</name>\n<value>42</value>\n");
    }

    #[test]
    fn xml_builder_with_section() {
        let xml = XmlBuilder::new()
            .section("parent", |b| b.field("child", "value"))
            .build();
        assert_eq!(xml, "<parent>\n  <child>value</child>\n</parent>\n");
    }

    #[test]
    fn xml_builder_optional_fields() {
        let xml = XmlBuilder::new()
            .optional_field("has_value", Some("test"))
            .optional_field("no_value", None::<&str>)
            .build();
        assert_eq!(xml, "<has_value>test</has_value>\n<no_value></no_value>\n");
    }

    #[test]
    fn escape_xml_characters() {
        assert_eq!(escape_xml("test & <tag>"), "test &amp; &lt;tag&gt;");
        assert_eq!(
            escape_xml("quote \"text\" here"),
            "quote &quot;text&quot; here"
        );
    }

    #[test]
    fn build_notification_context_xml() {
        let xml =
            build_notification_context("123", "mention", true, "2023-01-01", Some("2023-01-02"));
        assert!(xml.contains("<notification_context>"));
        assert!(xml.contains("<id>123</id>"));
        assert!(xml.contains("<status>UNREAD</status>"));
        assert!(xml.contains("<last_read>2023-01-02</last_read>"));
    }
}
