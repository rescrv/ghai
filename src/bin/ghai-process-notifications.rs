//! Mark as read the notifications that correspond to a push event.
//!
//! The github notification interface is a bit weird. It doesn't have a good way to remain
//! subscribed to types of activity.

use arrrg::CommandLine;
use claudius::{Anthropic, ContentBlock, KnownModel, MessageCreateParams, Model};
use policyai::{Manager, Policy, Usage};
use std::io::{self, Write};

use ghai::parser::parse_lines;
use ghai::policy::{get_policy_type, Decision};
use ghai::xml::{build_issue_notification_context, build_pull_request_notification_context};
use ghai::Notification;

#[derive(Debug, Default, Eq, PartialEq, arrrg_derive::CommandLine)]
struct Options {
    #[arrrg(flag, "Preview decisions without taking action")]
    dry_run: bool,
    #[arrrg(flag, "Minimal output (only essential information)")]
    quiet: bool,
    #[arrrg(flag, "Verbose output with full notification context")]
    detailed: bool,
    #[arrrg(flag, "Output decisions in JSON format")]
    json: bool,
    #[arrrg(flag, "Auto-execute decisions without confirmation prompts")]
    no_confirm: bool,
}

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

fn format_decision(decision: &Decision, opts: &Options) -> String {
    if opts.json {
        serde_json::to_string_pretty(decision).unwrap_or_else(|_| "{\"error\": true}".to_string())
    } else {
        match (decision.mark_unread, decision.mark_read) {
            (true, false) => "⚠ Mark as UNREAD".to_string(),
            (false, true) => "✓ Mark as READ".to_string(),
            _ => "⏭  Skip (no action)".to_string(),
        }
    }
}

fn display_notification_info(thread: &Notification, opts: &Options) {
    if opts.quiet {
        return;
    }

    if opts.detailed {
        println!("📋 Notification Details:");
        println!("   Repository: {}", thread.repository.full_name);
        println!("   Type: {}", thread.subject.r#type);
        println!("   Reason: {}", thread.reason);
        println!("   Title: {}", thread.subject.title);
        println!(
            "   Status: {}",
            if thread.unread { "UNREAD" } else { "READ" }
        );
        println!("   Updated: {}", thread.updated_at);
        if let Some(last_read) = &thread.last_read_at {
            println!("   Last read: {}", last_read);
        }
        println!("   URL: {}", thread.subject.url);
        println!("   ---");
    } else {
        println!(
            "📋 {} #{} in {}",
            thread.subject.r#type,
            thread.subject.title.split('#').next_back().unwrap_or("?"),
            thread.repository.full_name
        );
    }
}

fn confirm_via_ui(summary: &str, action: &str, opts: &Options) -> Result<bool, ()> {
    if opts.no_confirm {
        return Ok(true);
    }

    if !opts.quiet {
        println!("Summary: {}", summary);
    }
    print!("Mark as {}? (y/N/q to quit): ", action);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => Ok(true),
        "q" | "quit" => Err(()),
        _ => Ok(false),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (opts, args) =
        Options::from_command_line("USAGE: ghai-process-notifications [options] [policy-files...]");

    if opts.json && !opts.dry_run {
        eprintln!("❌ Error: --json flag requires --dry-run flag");
        eprintln!("   JSON output is only supported in dry-run mode to preview decisions");
        std::process::exit(1);
    }

    let client = Anthropic::new(None)?;
    let mut manager = Manager::default();

    // Read and concatenate all policy files
    let mut concatenated_content = String::new();
    for file_path in &args {
        match std::fs::read_to_string(file_path) {
            Ok(content) => {
                if !opts.quiet {
                    println!("📂 Loaded policy file: {}", file_path);
                }
                concatenated_content.push_str(&content);
                concatenated_content.push('\n');
            }
            Err(e) => {
                eprintln!("❌ Error reading policy file '{}': {}", file_path, e);
                eprintln!("   Please check that the file exists and is readable.");
                std::process::exit(1);
            }
        }
    }

    // Parse the concatenated content using the parser
    if !concatenated_content.trim().is_empty() {
        let parsed_results = parse_lines(&concatenated_content);
        for result in parsed_results {
            match result {
                Ok((prompt, action_json)) => {
                    // Validate action_json decodes to a Decision
                    match serde_json::from_value::<Decision>(action_json.clone()) {
                        Ok(_) => {
                            let policy = Policy {
                                r#type: get_policy_type(),
                                prompt,
                                action: action_json,
                            };
                            manager.add(policy);
                        }
                        Err(e) => {
                            eprintln!("❌ Error validating policy decision:");
                            eprintln!("   Prompt: {}", prompt);
                            eprintln!("   Error: {}", e);
                            eprintln!("   Please check the decision format in your policy file.");
                            std::process::exit(13);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error parsing policy file:");
                    eprintln!("   Error: {}", e);
                    eprintln!("   Please check the syntax of your policy file.");
                    std::process::exit(13);
                }
            }
        }
    }
    let notifications =
        Notification::fetch_all::<chrono::FixedOffset>(false, false, None, None).await?;

    let total_notifications = notifications.len();

    if !opts.quiet {
        println!("\n📊 Processing {} notifications...\n", total_notifications);
    }

    let mut processed = 0;
    let mut marked_read = 0;
    let mut marked_unread = 0;
    let mut skipped = 0;

    for thread in notifications.into_iter() {
        processed += 1;

        if !opts.quiet && !opts.json {
            println!("🔄 Processing {}/{}", processed, total_notifications);
        }
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
                eprintln!("⚠ Skipping unsupported notification type: {}", x);
                eprintln!(
                    "   Notification: {} in {}",
                    thread.subject.title, thread.repository.full_name
                );
                skipped += 1;
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

        display_notification_info(&thread, &opts);

        let decision: Decision = serde_json::from_value(report.value()).unwrap();
        let summary = generate_summary(&client, &llm_prompt)
            .await
            .unwrap_or_else(|e| {
                if !opts.quiet {
                    eprintln!("⚠ Could not generate summary: {}", e);
                }
                format!("Summary unavailable for: {}", thread.subject.title)
            });

        if !opts.quiet {
            println!("{}", format_decision(&decision, &opts));
        }

        match (decision.mark_unread, decision.mark_read) {
            (true, false) => {
                if !opts.quiet {
                    println!("Suggestion: mark as unread");
                }
                if !opts.dry_run {
                    match confirm_via_ui(&summary, "unread", &opts) {
                        Ok(true) => {
                            thread.mark_as_unread().await.unwrap();
                            marked_unread += 1;
                            if !opts.quiet {
                                println!("✓ Marked as unread");
                            }
                        }
                        Ok(false) => {
                            skipped += 1;
                        }
                        Err(()) => {
                            if !opts.quiet {
                                println!("\n👋 Exiting at user request");
                            }
                            std::process::exit(0);
                        }
                    }
                } else {
                    skipped += 1;
                }
            }
            (false, true) => {
                if !opts.quiet {
                    println!("Suggestion: mark as read");
                }
                if !opts.dry_run {
                    match confirm_via_ui(&summary, "read", &opts) {
                        Ok(true) => {
                            thread.mark_as_read().await.unwrap();
                            marked_read += 1;
                            if !opts.quiet {
                                println!("✓ Marked as read");
                            }
                        }
                        Ok(false) => {
                            skipped += 1;
                        }
                        Err(()) => {
                            if !opts.quiet {
                                println!("\n👋 Exiting at user request");
                            }
                            std::process::exit(0);
                        }
                    }
                } else {
                    skipped += 1;
                }
            }
            _ => {
                skipped += 1;
                if !opts.quiet {
                    println!("No action needed - skipping");
                    println!("Summary: {}", summary);
                    println!("URL: {}", thread.subject.url);
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    if !opts.quiet {
        println!("\n📊 Processing Complete!");
        println!("   Total processed: {}", processed);
        println!("   Marked as read: {}", marked_read);
        println!("   Marked as unread: {}", marked_unread);
        println!("   Skipped: {}", skipped);
    }

    Ok(())
}
