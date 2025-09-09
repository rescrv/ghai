use arrrg::CommandLine;
use chrono::DateTime;
use ghai::Issue;

#[derive(Debug, Default, Eq, PartialEq, arrrg_derive::CommandLine)]
struct Options {
    #[arrrg(
        optional,
        "Filter issues (assigned, created, mentioned, subscribed, all)"
    )]
    filter: Option<String>,
    #[arrrg(optional, "Filter by state (open, closed, all)")]
    state: Option<String>,
    #[arrrg(optional, "Filter by labels (comma separated)")]
    labels: Option<String>,
    #[arrrg(optional, "Sort by (created, updated, comments)")]
    sort: Option<String>,
    #[arrrg(optional, "Direction (asc, desc)")]
    direction: Option<String>,
    #[arrrg(optional, "Only issues updated at or after this time (ISO 8601)")]
    since: Option<String>,
    #[arrrg(optional, "Number of results per page (default 30, max 100)")]
    per_page: Option<u64>,
    #[arrrg(optional, "Page number")]
    page: Option<u64>,
    #[arrrg(flag, "Show only issue URLs")]
    urls_only: bool,
    #[arrrg(flag, "Show detailed information")]
    detailed: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (options, free) =
        Options::from_command_line_relaxed("USAGE: ghai-print-my-issues [options]");

    if !free.is_empty() {
        eprintln!("command takes no arguments");
        std::process::exit(1);
    }

    let since = if let Some(since_str) = options.since {
        Some(DateTime::parse_from_rfc3339(&since_str)?)
    } else {
        None
    };

    let issues = Issue::fetch_user_issues(
        options.filter,
        options.state,
        options.labels,
        options.sort,
        options.direction,
        since,
        options.per_page,
        options.page,
    )
    .await?;

    if issues.is_empty() {
        println!("No issues found");
        return Ok(());
    }

    for issue in issues {
        if options.urls_only {
            println!("{}", issue.html_url);
        } else if options.detailed {
            println!("#{}: {}", issue.number, issue.title);
            println!("  URL: {}", issue.html_url);
            println!("  State: {}", issue.state);
            if let Some(user) = &issue.user {
                println!("  Author: {}", user.login);
            }
            println!("  Created: {}", issue.created_at);
            println!("  Updated: {}", issue.updated_at);
            if !issue.labels.is_empty() {
                print!("  Labels: ");
                for (i, label) in issue.labels.iter().enumerate() {
                    if i > 0 {
                        print!(", ");
                    }
                    match label {
                        ghai::Label::Detailed { name, .. } => print!("{}", name),
                        ghai::Label::Simple(name) => print!("{}", name),
                    }
                }
                println!();
            }
            if let Some(body) = &issue.body {
                if !body.is_empty() {
                    let preview = if body.len() > 100 {
                        format!("{}...", &body[..97])
                    } else {
                        body.clone()
                    };
                    println!("  Body: {}", preview.replace('\n', " "));
                }
            }
            println!();
        } else {
            let labels = issue
                .labels
                .iter()
                .map(|label| match label {
                    ghai::Label::Detailed { name, .. } => name.clone(),
                    ghai::Label::Simple(name) => name.clone(),
                })
                .collect::<Vec<_>>()
                .join(",");

            let labels_str = if labels.is_empty() {
                String::new()
            } else {
                format!(" [{}]", labels)
            };

            println!(
                "- #{}: {} ({}){}\n  {}",
                issue.number, issue.title, issue.state, labels_str, issue.html_url
            );
        }
    }

    Ok(())
}
