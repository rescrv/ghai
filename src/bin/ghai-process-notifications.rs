//! Mark as read the notifications that correspond to a push event.
//!
//! The github notification interface is a bit weird. It doesn't have a good way to remain
//! subscribed to types of activity.

use claudius::{Anthropic, ContentBlock, KnownModel, MessageCreateParams, Model};
use policyai::{Manager, Policy, Usage};
use std::io::{self, Write};

use ghai::policy::{get_policy_type, Decision};
use ghai::xml::{build_issue_notification_context, build_pull_request_notification_context};
use ghai::Notification;

async fn generate_summary(
    client: &Anthropic,
    xml_context: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let prompt = format!(
        "Please provide a single-line summary of this GitHub notification. Be concise and descriptive, focusing on what the PR/Issue is about:\n\n{}", 
        xml_context
    );
    let req = MessageCreateParams {
        max_tokens: 128,
        model: Model::Known(KnownModel::ClaudeSonnet40),
        messages: vec![prompt.into()],
        ..Default::default()
    };
    let resp = client.send(req).await?;
    Ok(resp
        .content
        .iter()
        .filter_map(|x| {
            if let ContentBlock::Text(t) = x {
                Some(t.text.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n"))
}

fn confirm_via_ui(summary: &str) -> bool {
    println!("Summary: {}", summary);
    print!("Confirm? (y/N): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    matches!(input.trim().to_lowercase().as_str(), "y" | "yes")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Anthropic::new(None)?;
    let mut manager = Manager::default();
    manager.add(Policy {
        r#type: get_policy_type(),
        prompt: "The notification is for a pull request.".to_string(),
        action: serde_json::json! {{
            "mark_unread": true,
            "mark_read": false,
        }},
    });
    manager.add(Policy {
        r#type: get_policy_type(),
        prompt: "The author of is @sanketkedia.".to_string(),
        action: serde_json::json! {{
            "mark_unread": false,
            "mark_read": true,
            "priority": "low",
        }},
    });
    for thread in Notification::fetch_all::<chrono::FixedOffset>(false, false, None, None)
        .await?
        .into_iter()
    {
        let llm_prompt = match thread.subject.r#type.as_str() {
            "PullRequest" => {
                let pr = thread.fetch_pull_request().await?;
                build_pull_request_notification_context(&thread, &pr)
            }
            "Issue" => {
                let issue = thread.fetch_issue().await?;
                build_issue_notification_context(&thread, &issue)
            }
            x => {
                eprintln!("unhandled notification type: {x}");
                continue;
            }
        };
        let template = MessageCreateParams {
            max_tokens: 3333,
            model: Model::Known(KnownModel::ClaudeSonnet40),
            messages: vec![],
            ..Default::default()
        };
        let mut usage = Usage::default();
        let report = manager
            .apply(&client, template, &llm_prompt, Some(&mut usage))
            .await?;

        let decision: Decision = serde_json::from_value(report.value()).unwrap();
        let summary = generate_summary(&client, &llm_prompt)
            .await
            .unwrap_or_else(|_| "Unable to generate summary".to_string());
        println!("{decision:?}");
        match (decision.mark_unread, decision.mark_read) {
            (true, false) => {
                println!("suggesting to mark this one unread");
                if confirm_via_ui(&summary) {
                    thread.mark_as_unread().await.unwrap();
                }
            }
            (false, true) => {
                println!("suggesting to mark this one read");
                if confirm_via_ui(&summary) {
                    thread.mark_as_read().await.unwrap();
                }
            }
            _ => {
                println!("don't know what to do; skipping");
                println!("Summary: {}", summary);
                println!("URL: {}", thread.subject.url);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    Ok(())
}
