use arrrg::CommandLine;

use ghai::{Issue, Pull};

#[derive(Clone, Debug, Eq, PartialEq, arrrg_derive::CommandLine)]
struct Options {
    #[arrrg(optional, "Model to use within ollama")]
    model: String,
}

impl Default for Options {
    fn default() -> Options {
        Self {
            model: "llama3.2:3b".to_string(),
        }
    }
}

async fn summarize_issue(options: &Options, number: String) {
    let output = std::process::Command::new("gh")
                    .arg("issue")
                    .arg("view")
                    .arg("--comments")
                    .arg("--json")
                    .arg("assignees,author,body,closed,closedAt,comments,createdAt,id,labels,milestone,number,projectCards,projectItems,reactionGroups,state,title,updatedAt,url")
                    .arg(number)
                    .output().unwrap();
    let data = String::from_utf8_lossy(&output.stdout);
    let issue: Issue = serde_json::from_str(&data).unwrap();
    let mut prompt = "Summarize this GitHub Issue thread:\n\n".to_string();
    prompt += &issue.title;
    prompt.push('\n');
    prompt += "="
        .chars()
        .cycle()
        .take(issue.title.len())
        .collect::<String>()
        .as_str();
    prompt.push('\n');
    prompt += "Author: ";
    prompt += &issue.author.login;
    prompt.push('\n');
    prompt += "Created-At: ";
    prompt += &issue.created_at;
    prompt.push('\n');
    prompt += &issue.body.unwrap_or_default();
    prompt += "\n\nComments:\n";
    prompt += &issue
        .comments
        .iter()
        .map(|c| {
            let mut comment = c.author.login.clone();
            comment.push_str(" at ");
            comment.push_str(&c.created_at);
            comment.push('\n');
            comment.push_str(&c.body);
            comment.push_str("\n\n");
            comment
        })
        .collect::<String>();
    let g = yammer::GenerateRequest {
        model: options.model.clone(),
        prompt,
        stream: Some(true),
        suffix: "".to_string(),
        raw: None,
        system: None,
        format: None,
        images: None,
        template: None,
        keep_alive: None,
        options: serde_json::json!({}),
    };
    yammer::Request::generate(yammer::RequestOptions::default(), g)
        .unwrap()
        .accumulate(&mut (&mut yammer::FieldWriteAccumulator::new(
            std::io::stdout(),
            "response",
        ),))
        .await
        .unwrap();
    println!();
}

async fn summarize_pr(options: &Options, number: String) {
    let output = std::process::Command::new("gh")
                    .arg("pr")
                    .arg("view")
                    .arg("--comments")
                    .arg("--json")
                    .arg("additions,assignees,author,autoMergeRequest,baseRefName,body,changedFiles,closed,closedAt,comments,commits,createdAt,deletions,files,headRefName,headRefOid,headRepository,headRepositoryOwner,id,isCrossRepository,isDraft,labels,latestReviews,maintainerCanModify,mergeCommit,mergeStateStatus,mergeable,mergedAt,mergedBy,milestone,number,potentialMergeCommit,projectCards,projectItems,reactionGroups,reviewDecision,reviewRequests,reviews,state,statusCheckRollup,title,updatedAt,url")
                    .arg(number)
                    .output().unwrap();
    let data = String::from_utf8_lossy(&output.stdout);
    let pull: Pull = serde_json::from_str(&data).unwrap();
    let mut prompt = "Summarize this GitHub Pull Request:\n\n".to_string();
    prompt += &pull.title;
    prompt.push('\n');
    prompt += "="
        .chars()
        .cycle()
        .take(pull.title.len())
        .collect::<String>()
        .as_str();
    prompt.push('\n');
    prompt += "Author: ";
    prompt += &pull.author.login;
    prompt.push('\n');
    prompt += "Created-At: ";
    prompt += &pull.created_at;
    prompt.push('\n');
    prompt += &pull.body.unwrap_or_default();
    prompt += "\n\nComments:\n";
    prompt += &pull
        .comments
        .iter()
        .map(|c| {
            let mut comment = c.author.login.clone();
            comment.push_str(" at ");
            comment.push_str(&c.created_at);
            comment.push('\n');
            comment.push_str(&c.body);
            comment.push_str("\n\n");
            comment
        })
        .collect::<String>();
    let g = yammer::GenerateRequest {
        model: options.model.clone(),
        prompt,
        stream: Some(true),
        suffix: "".to_string(),
        raw: None,
        system: None,
        format: None,
        images: None,
        template: None,
        keep_alive: None,
        options: serde_json::json!({}),
    };
    yammer::Request::generate(yammer::RequestOptions::default(), g)
        .unwrap()
        .accumulate(&mut (&mut yammer::FieldWriteAccumulator::new(
            std::io::stdout(),
            "response",
        ),))
        .await
        .unwrap();
    println!();
}

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: ghai <command>");
        return;
    }
    let args = args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    match args[1] {
        "summarize" => {
            if args.len() < 3 {
                eprintln!("Usage: ghai summarize issue|pr [options] <number>");
                return;
            }
            if args[2] != "issue" && args[2] != "pr" {
                eprintln!("Unknown type: {}", args[2]);
                return;
            }
            let (options, free) = Options::from_arguments_relaxed(
                "Usage: ghai summarize issue|pr [options] <number>",
                &args[3..],
            );
            for number in free {
                if args[2] == "issue" {
                    summarize_issue(&options, number).await;
                } else {
                    summarize_pr(&options, number).await;
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}
