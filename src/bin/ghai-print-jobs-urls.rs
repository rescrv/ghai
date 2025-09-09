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
    if !free.is_empty() {
        eprintln!("command takes no arguments");
        std::process::exit(1);
    }
    for action in Action::fetch_all(
        options.owner.clone(),
        options.repo.clone(),
        options.actor.clone(),
        None,
        None,
        options.status.clone(),
        None,
        None,
    )
    .await?
    .into_iter()
    {
        println!("{}", action.jobs_url);
    }
    Ok(())
}
