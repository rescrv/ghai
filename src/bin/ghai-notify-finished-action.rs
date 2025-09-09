use std::collections::HashSet;

use arrrg::CommandLine;

use ghai::Action;

#[derive(Debug, Default, Eq, PartialEq, arrrg_derive::CommandLine)]
struct Options {
    #[arrrg(required, "The owner of the repository")]
    owner: String,
    #[arrrg(required, "The name of the repository")]
    repo: String,
    #[arrrg(optional, "Filter to a given actor's actions")]
    actor: Option<String>,
    #[arrrg(optional, "Filter to a given status")]
    status: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (options, free) =
        Options::from_command_line_relaxed("USAGE: ghai-notify-finished-action [options]");
    let mut first = true;
    let mut in_progress = HashSet::new();
    loop {
        let actions = Action::fetch_all(
            options.owner.clone(),
            options.repo.clone(),
            options.actor.clone(),
            None,
            None,
            options.status.clone(),
            None,
            None,
        )
        .await?;
        if first {
            for action in actions {
                if action.status != Some("completed".to_string()) {
                    in_progress.insert(action.id);
                }
            }
            first = false;
        } else {
            for action in actions {
                if action.status == Some("completed".to_string())
                    && in_progress.contains(&action.id)
                {
                    in_progress.remove(&action.id);
                }
            }
        }
        if in_progress.is_empty() {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
    }
    if !free.is_empty() {
        std::process::Command::new(&free[0])
            .args(free[1..].to_vec())
            .spawn()?;
    }
    Ok(())
}
