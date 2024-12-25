//! Mark as read the notifications that correspond to a push event.
//!
//! The github notification interface is a bit weird. It doesn't have a good way to remain
//! subscribed to types of activity.

use chrono::DateTime;

use ghai::Notification;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    for thread in Notification::fetch_all(false, false, None, None)? {
        if thread.subject.r#type != "PullRequest" {
            continue;
        }
        if thread.last_read_at.is_none() {
            continue;
        }
        let pr = thread.fetch_pull_request()?;
        if pr.merged.unwrap_or(false) {
            continue;
        }
        let comments = pr.fetch_comments(
            thread
                .last_read_at
                .as_ref()
                .map(|r| DateTime::parse_from_rfc3339(r).unwrap()),
        )?;
        if !comments.is_empty() {
            continue;
        }
        let _output = std::process::Command::new("gh")
            .arg("api")
            .arg("--method")
            .arg("PATCH")
            .arg("-H")
            .arg("Accept: application/vnd.github+json")
            .arg(format!("/notifications/threads/{}", thread.id))
            .output()?;
    }
    Ok(())
}
