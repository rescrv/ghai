use arrrg::CommandLine;
use chrono::DateTime;

use ghai::Issue;

#[derive(Debug, Default, Eq, PartialEq, arrrg_derive::CommandLine)]
struct Options {
    #[arrrg(required, "The organization to fetch issues for")]
    org: String,
    #[arrrg(optional, "What to filter by.")]
    filter: Option<String>,
    #[arrrg(optional, "The state of the issues to fetch")]
    state: Option<String>,
    #[arrrg(optional, "The sort order of the issues")]
    sort: Option<String>,
    #[arrrg(optional, "The direction to sort the issues")]
    since: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (options, _) = Options::from_command_line_relaxed("USAGE: ghai-my-issues [options]");
    let since = options
        .since
        .map(|x| DateTime::parse_from_rfc3339(&x).unwrap());
    for issue in Issue::fetch_all(
        options.org.clone(),
        options.filter.clone(),
        options.state.clone(),
        options.sort.clone(),
        since,
    )? {
        println!("{} {}", issue.html_url, issue.title);
    }
    Ok(())
}
