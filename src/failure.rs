use crate::http::{GitHubClient, UrlBuilder};
use crate::{Action, Job, Runs};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FailureReasonRequest {
    pub owner: String,
    pub repo: String,
    pub actor: Option<String>,
    pub branch: Option<String>,
    pub event: Option<String>,
    pub created: Option<String>,
    pub max_runs: Option<usize>,
    pub max_pages: u64,
    pub per_page: u64,
    pub samples_per_reason: usize,
    pub save_logs_dir: Option<PathBuf>,
    pub fail_on_log_error: bool,
}

impl Default for FailureReasonRequest {
    fn default() -> Self {
        Self {
            owner: String::new(),
            repo: String::new(),
            actor: None,
            branch: None,
            event: None,
            created: None,
            max_runs: Some(100),
            max_pages: 10,
            per_page: 100,
            samples_per_reason: 3,
            save_logs_dir: None,
            fail_on_log_error: false,
        }
    }
}

impl FailureReasonRequest {
    pub fn new(owner: impl Into<String>, repo: impl Into<String>) -> Self {
        Self {
            owner: owner.into(),
            repo: repo.into(),
            ..Default::default()
        }
    }

    fn per_page(&self) -> u64 {
        self.per_page.clamp(1, 100)
    }

    fn max_pages(&self) -> u64 {
        self.max_pages.max(1)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct FailureReasonSample {
    pub workflow: String,
    pub job: String,
    pub step: Option<String>,
    pub run_id: i64,
    pub run_number: u64,
    pub run_attempt: Option<u64>,
    pub job_id: i64,
    pub run_html_url: String,
    pub job_html_url: Option<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub log_path: Option<PathBuf>,
    pub log_fetch_error: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct FailureReasonBucket {
    pub reason: String,
    pub count: usize,
    pub samples: Vec<FailureReasonSample>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct FailureReasonHistogram {
    pub total_failed_runs: usize,
    pub total_failed_jobs: usize,
    pub buckets: Vec<FailureReasonBucket>,
}

pub async fn fetch_failed_workflow_runs(
    request: &FailureReasonRequest,
) -> Result<Vec<Action>, Box<dyn std::error::Error>> {
    let client = GitHubClient::new()?;
    let per_page = request.per_page();
    let mut runs = Vec::new();

    for page in 1..=request.max_pages() {
        let url = UrlBuilder::new(format!(
            "https://api.github.com/repos/{}/{}/actions/runs",
            request.owner, request.repo
        ))
        .param("actor", request.actor.clone())
        .param("branch", request.branch.clone())
        .param("event", request.event.clone())
        .param("created", request.created.clone())
        .required_param("status", "failure")
        .required_param("per_page", per_page)
        .required_param("page", page)
        .build();

        let page_runs: Runs = client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let fetched = page_runs.workflow_runs.len();
        runs.extend(
            page_runs
                .workflow_runs
                .into_iter()
                .filter(|run| run.conclusion.as_deref() == Some("failure")),
        );

        if let Some(max_runs) = request.max_runs {
            if runs.len() >= max_runs {
                break;
            }
        }
        if fetched == 0 || fetched < per_page as usize {
            break;
        }
    }

    runs.sort_by(|left, right| {
        right
            .created_at
            .cmp(&left.created_at)
            .then_with(|| right.updated_at.cmp(&left.updated_at))
            .then_with(|| right.id.cmp(&left.id))
    });
    if let Some(max_runs) = request.max_runs {
        runs.truncate(max_runs);
    }

    Ok(runs)
}

pub async fn fetch_failure_reason_histogram(
    request: FailureReasonRequest,
) -> Result<FailureReasonHistogram, Box<dyn std::error::Error>> {
    let runs = fetch_failed_workflow_runs(&request).await?;
    let mut buckets = BTreeMap::new();
    let mut total_failed_jobs = 0;

    for run in &runs {
        for job in run.fetch_jobs().await?.into_iter().filter(job_failed) {
            total_failed_jobs += 1;
            let sample = failure_reason_sample(&request, run, &job).await?;
            insert_sample(&mut buckets, sample, request.samples_per_reason);
        }
    }

    let mut buckets: Vec<FailureReasonBucket> = buckets.into_values().collect();
    buckets.sort_by(|left, right| {
        right
            .count
            .cmp(&left.count)
            .then_with(|| left.reason.cmp(&right.reason))
    });

    Ok(FailureReasonHistogram {
        total_failed_runs: runs.len(),
        total_failed_jobs,
        buckets,
    })
}

fn job_failed(job: &Job) -> bool {
    job.conclusion.as_deref() == Some("failure") || job.failed_step().is_some()
}

async fn failure_reason_sample(
    request: &FailureReasonRequest,
    run: &Action,
    job: &Job,
) -> Result<(String, FailureReasonSample), Box<dyn std::error::Error>> {
    let workflow = run
        .name
        .clone()
        .or_else(|| job.workflow_name.clone())
        .unwrap_or_else(|| "workflow".to_string());
    let step = job.failed_step().map(|step| step.name.clone());
    let fallback_reason = fallback_failure_reason(job);
    let mut log_path = None;
    let mut log_fetch_error = None;

    let reason = match job.fetch_logs().await {
        Ok(log) => {
            if let Some(dir) = &request.save_logs_dir {
                log_path = Some(save_failure_log(dir, run, job, &log)?);
            }
            extract_failure_reason(&log).unwrap_or(fallback_reason)
        }
        Err(err) => {
            if request.fail_on_log_error {
                return Err(err);
            }
            log_fetch_error = Some(err.to_string());
            fallback_reason
        }
    };

    let sample = FailureReasonSample {
        workflow,
        job: job.name.clone(),
        step,
        run_id: run.id,
        run_number: run.run_number,
        run_attempt: run.run_attempt.or(job.run_attempt),
        job_id: job.id,
        run_html_url: run.html_url.clone(),
        job_html_url: job.html_url.clone(),
        created_at: run.created_at.clone(),
        completed_at: job.completed_at.clone(),
        log_path,
        log_fetch_error,
    };

    Ok((reason, sample))
}

fn insert_sample(
    buckets: &mut BTreeMap<String, FailureReasonBucket>,
    (reason, sample): (String, FailureReasonSample),
    samples_per_reason: usize,
) {
    let bucket = buckets
        .entry(reason.clone())
        .or_insert_with(|| FailureReasonBucket {
            reason,
            count: 0,
            samples: Vec::new(),
        });
    bucket.count += 1;
    if bucket.samples.len() < samples_per_reason {
        bucket.samples.push(sample);
    }
}

fn save_failure_log(
    dir: &Path,
    run: &Action,
    job: &Job,
    log: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    std::fs::create_dir_all(dir)?;
    let path = dir.join(failure_log_filename(run, job));
    std::fs::write(&path, log)?;
    Ok(path)
}

pub fn failure_log_filename(run: &Action, job: &Job) -> String {
    let workflow = run
        .name
        .as_deref()
        .or(job.workflow_name.as_deref())
        .unwrap_or("workflow");
    let attempt = run.run_attempt.or(job.run_attempt).unwrap_or(1);
    format!(
        "run-{}-id-{}-attempt-{}-{}-job-{}-{}.log",
        run.run_number,
        run.id,
        attempt,
        sanitize_filename_component(workflow),
        job.id,
        sanitize_filename_component(&job.name)
    )
}

pub fn sanitize_filename_component(input: &str) -> String {
    let mut out = String::with_capacity(input.len().min(80));
    let mut last_dash = false;

    for ch in input.chars() {
        let mapped = if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' {
            last_dash = false;
            Some(ch.to_ascii_lowercase())
        } else if ch == '-' || ch.is_whitespace() || ch == '/' || ch == ':' {
            if last_dash {
                None
            } else {
                last_dash = true;
                Some('-')
            }
        } else if last_dash {
            None
        } else {
            last_dash = true;
            Some('-')
        };

        if let Some(ch) = mapped {
            out.push(ch);
        }
        if out.len() >= 80 {
            break;
        }
    }

    let trimmed = out.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "unnamed".to_string()
    } else {
        trimmed
    }
}

fn fallback_failure_reason(job: &Job) -> String {
    match job.failed_step() {
        Some(step) => format!("{} failed in step {}", job.name, step.name),
        None => format!("{} failed", job.name),
    }
}

pub fn extract_failure_reason(log: &str) -> Option<String> {
    let mut best_specific = None;
    let mut best_generic = None;

    for (index, line) in log.lines().enumerate() {
        let Some(candidate) = classify_failure_line(line) else {
            continue;
        };
        let slot = if candidate.generic {
            &mut best_generic
        } else {
            &mut best_specific
        };
        let replace = match slot {
            None => true,
            Some((score, current_index, _)) => {
                candidate.score > *score || (candidate.score == *score && index >= *current_index)
            }
        };
        if replace {
            *slot = Some((candidate.score, index, candidate.message));
        }
    }

    best_specific
        .or(best_generic)
        .map(|(_, _, reason)| truncate_reason(&reason, 240))
}

struct FailureCandidate {
    message: String,
    score: i32,
    generic: bool,
}

fn classify_failure_line(raw: &str) -> Option<FailureCandidate> {
    let (line, annotated_error) = clean_log_line(raw)?;
    if line_is_noise(&line) {
        return None;
    }

    let lower = line.to_ascii_lowercase();
    let generic = is_generic_failure_line(&lower);
    let mut score = 0;

    if annotated_error {
        score += 80;
    }
    if lower.starts_with("error:")
        || lower.starts_with("error ")
        || lower.starts_with("error[")
        || lower.starts_with("error:")
    {
        score += 80;
    }
    if lower.contains(" error:") || lower.contains(" error[") || lower.contains(" error ") {
        score += 65;
    }
    if lower.starts_with("fatal:") || lower.contains(" fatal:") {
        score += 75;
    }
    if lower.starts_with("npm err!") || lower.starts_with("yarn error") {
        score += 70;
    }
    if lower.contains("panicked at") || lower.contains("panic") {
        score += 70;
    }
    if lower.contains("assertion") {
        score += 60;
    }
    if lower.contains("exception") || lower.contains("traceback") {
        score += 65;
    }
    if lower.contains("no such file")
        || lower.contains("permission denied")
        || lower.contains("connection refused")
        || lower.contains("segmentation fault")
    {
        score += 65;
    }
    if lower.contains("timed out") || lower.contains("timeout") {
        score += 55;
    }
    if lower.contains("failed") || lower.contains("failure") {
        score += 45;
    }
    if lower.contains("exit code") {
        score += 35;
    }

    if generic {
        score = score.min(40);
    }

    if score == 0 {
        return None;
    }

    Some(FailureCandidate {
        message: line,
        score,
        generic,
    })
}

fn clean_log_line(raw: &str) -> Option<(String, bool)> {
    let stripped = strip_ansi(raw);
    let mut line = strip_leading_timestamp(stripped.trim());
    let mut annotated_error = false;

    while let Some(rest) = line.strip_prefix("##[error]") {
        annotated_error = true;
        line = rest.trim_start();
    }
    while let Some(rest) = line.strip_prefix("Error: ") {
        annotated_error = true;
        line = rest.trim_start();
    }
    if line.starts_with("::error") {
        if let Some(offset) = line.get(2..).and_then(|rest| rest.find("::")) {
            annotated_error = true;
            line = line[offset + 4..].trim_start();
        }
    }

    let collapsed = collapse_whitespace(line);
    if collapsed.is_empty() {
        None
    } else {
        Some((collapsed, annotated_error))
    }
}

fn strip_leading_timestamp(line: &str) -> &str {
    if line.len() < 21 {
        return line;
    }

    let Some(space) = line.find(' ') else {
        return line;
    };
    let maybe_timestamp = &line[..space];
    if maybe_timestamp.len() >= 20
        && maybe_timestamp.as_bytes().get(4) == Some(&b'-')
        && maybe_timestamp.as_bytes().get(7) == Some(&b'-')
        && maybe_timestamp.contains('T')
        && maybe_timestamp.ends_with('Z')
    {
        line[space + 1..].trim_start()
    } else {
        line
    }
}

fn strip_ansi(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            chars.next();
            for code in chars.by_ref() {
                if ('@'..='~').contains(&code) {
                    break;
                }
            }
        } else {
            out.push(ch);
        }
    }

    out
}

fn collapse_whitespace(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut last_space = false;

    for ch in input.chars() {
        if ch.is_whitespace() {
            if !last_space {
                out.push(' ');
                last_space = true;
            }
        } else {
            out.push(ch);
            last_space = false;
        }
    }

    out.trim().to_string()
}

fn line_is_noise(line: &str) -> bool {
    let lower = line.to_ascii_lowercase();
    lower.starts_with("##[group]")
        || lower.starts_with("##[endgroup]")
        || lower.starts_with("run ")
        || lower.starts_with("shell:")
        || lower.starts_with("env:")
        || lower == "build failed"
        || lower == "test failures"
}

fn is_generic_failure_line(lower: &str) -> bool {
    lower.contains("process completed with exit code")
        || lower.contains("the operation was canceled")
        || lower.contains("job failed")
        || lower.contains("completed with exit code")
}

fn truncate_reason(reason: &str, max_chars: usize) -> String {
    let mut chars = reason.chars();
    let mut out: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        out.push_str("...");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_annotated_error_without_timestamp() {
        let log = "\
2026-05-22T00:00:00.0000000Z cargo test
2026-05-22T00:00:01.0000000Z ##[error]database container failed to become healthy
2026-05-22T00:00:02.0000000Z ##[error]Process completed with exit code 1.";

        assert_eq!(
            extract_failure_reason(log).as_deref(),
            Some("database container failed to become healthy")
        );
    }

    #[test]
    fn prefers_specific_failure_over_generic_exit_code() {
        let log = "\
2026-05-22T00:00:00.0000000Z thread 'replication::tests::round_trip' panicked at src/lib.rs:42:9
2026-05-22T00:00:02.0000000Z Error: Process completed with exit code 101.";

        assert_eq!(
            extract_failure_reason(log).as_deref(),
            Some("thread 'replication::tests::round_trip' panicked at src/lib.rs:42:9")
        );
    }

    #[test]
    fn strips_ansi_and_collapses_spaces() {
        let log = "\u{1b}[31merror:\u{1b}[0m   could not compile   crate_name";

        assert_eq!(
            extract_failure_reason(log).as_deref(),
            Some("error: could not compile crate_name")
        );
    }

    #[test]
    fn sanitizes_filename_component() {
        assert_eq!(
            sanitize_filename_component("CI / Rust: nightly (macOS)"),
            "ci-rust-nightly-macos"
        );
    }
}
