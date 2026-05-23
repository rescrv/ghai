use arrrg::CommandLine;
use ghai::{fetch_failure_reason_histogram, FailureReasonRequest};
use std::path::PathBuf;

#[derive(Debug, Default, Eq, PartialEq, arrrg_derive::CommandLine)]
struct Options {
    #[arrrg(required, "The owner of the repository")]
    owner: String,
    #[arrrg(required, "The name of the repository")]
    repo: String,
    #[arrrg(optional, "Filter to a given actor's actions")]
    actor: Option<String>,
    #[arrrg(optional, "Filter to a given branch")]
    branch: Option<String>,
    #[arrrg(optional, "Filter to a given event")]
    event: Option<String>,
    #[arrrg(optional, "Filter by GitHub created search syntax")]
    created: Option<String>,
    #[arrrg(optional, "Maximum failed workflow runs to inspect")]
    max_runs: Option<usize>,
    #[arrrg(optional, "Maximum pages of failed workflow runs to inspect")]
    max_pages: Option<u64>,
    #[arrrg(optional, "Workflow runs to fetch per page")]
    per_page: Option<u64>,
    #[arrrg(optional, "Samples to print per failure reason")]
    samples: Option<usize>,
    #[arrrg(optional, "Directory in which to save failed job logs")]
    save_logs_dir: Option<String>,
    #[arrrg(flag, "Output the histogram as JSON")]
    json: bool,
    #[arrrg(flag, "Abort if any failed job log cannot be downloaded")]
    fail_on_log_error: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (options, free) =
        Options::from_command_line_relaxed("USAGE: ghai-failure-reasons [options]");
    if !free.is_empty() {
        eprintln!("command takes no arguments");
        std::process::exit(1);
    }

    let mut request = FailureReasonRequest::new(options.owner, options.repo);
    request.actor = options.actor;
    request.branch = options.branch;
    request.event = options.event;
    request.created = options.created;
    request.fail_on_log_error = options.fail_on_log_error;
    if let Some(max_runs) = options.max_runs {
        request.max_runs = Some(max_runs);
    }
    if let Some(max_pages) = options.max_pages {
        request.max_pages = max_pages;
    }
    if let Some(per_page) = options.per_page {
        request.per_page = per_page;
    }
    if let Some(samples) = options.samples {
        request.samples_per_reason = samples;
    }
    if let Some(save_logs_dir) = options.save_logs_dir {
        request.save_logs_dir = Some(PathBuf::from(save_logs_dir));
    }

    let histogram = fetch_failure_reason_histogram(request).await?;
    if options.json {
        println!("{}", serde_json::to_string_pretty(&histogram)?);
    } else {
        print_histogram(&histogram);
    }

    Ok(())
}

fn print_histogram(histogram: &ghai::FailureReasonHistogram) {
    println!(
        "failed runs: {}  failed jobs: {}  reasons: {}",
        histogram.total_failed_runs,
        histogram.total_failed_jobs,
        histogram.buckets.len()
    );
    println!("{:>5}  reason", "count");
    for bucket in &histogram.buckets {
        println!("{:>5}  {}", bucket.count, bucket.reason);
        for sample in &bucket.samples {
            let step = sample
                .step
                .as_ref()
                .map(|step| format!(" step={}", step))
                .unwrap_or_default();
            let log_path = sample
                .log_path
                .as_ref()
                .map(|path| format!(" log={}", path.display()))
                .unwrap_or_default();
            let log_error = sample
                .log_fetch_error
                .as_ref()
                .map(|error| format!(" log_error={}", error))
                .unwrap_or_default();
            println!(
                "       run={} attempt={} workflow={} job={}{}{}{}",
                sample.run_number,
                sample.run_attempt.unwrap_or(1),
                sample.workflow,
                sample.job,
                step,
                log_path,
                log_error
            );
        }
    }
}
