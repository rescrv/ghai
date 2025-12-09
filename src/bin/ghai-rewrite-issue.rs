//! Rewrite a string into a proper GitHub issue with a title and body.

use std::io::Write;

use futures::StreamExt;

use claudius::{
    Anthropic, ContentBlockDelta, MessageCreateParams, MessageStreamEvent, Model, SystemPrompt,
};

const BASE_SYSTEM_PROMPT: &str = r#"You are a professional technical writer who transforms informal issue descriptions into well-structured GitHub issues.

Given a description of an issue, create a proper GitHub issue with:
1. A clear, concise title that summarizes the issue
2. A professional body that explains the issue in detail

Guidelines:
- Use clear, technical language appropriate for a GitHub issue
- The title should be actionable and specific
- The body should provide context, expected behavior, and any relevant details
- Do not add unnecessary pleasantries or filler text

Respond with JSON in this exact format:
{"title": "the title", "body": "the body"}

Do not include any other text, markdown formatting, or code blocks. Output only the raw JSON object."#;

fn load_system_prompt() -> Result<String, Box<dyn std::error::Error>> {
    let user_instructions = if let Ok(context) = std::env::var("GHAI_CONTEXT") {
        std::fs::read_to_string(&context).unwrap_or_default()
    } else {
        String::new()
    };

    let combined = if user_instructions.trim().is_empty() {
        BASE_SYSTEM_PROMPT.to_string()
    } else {
        format!(
            "{}\n\nAdditional instructions:\n{}",
            BASE_SYSTEM_PROMPT, user_instructions
        )
    };

    Ok(combined)
}

async fn rewrite_issue(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Anthropic::new(None)?;
    let system_prompt = load_system_prompt()?;

    let req = MessageCreateParams {
        max_tokens: 1024,
        model: Model::Custom("claude-haiku-4-5".to_string()),
        messages: vec![input.into()],
        system: Some(SystemPrompt::String(system_prompt)),
        ..Default::default()
    };

    let stream = client.stream(req).await?;

    // Pin the stream so it can be polled
    tokio::pin!(stream);

    while let Some(event) = stream.next().await {
        match event {
            Ok(event) => {
                if let MessageStreamEvent::ContentBlockDelta(ref x) = event {
                    if let ContentBlockDelta::TextDelta(ref text_delta) = x.delta {
                        print!("{}", text_delta.text);
                        let _ = std::io::stdout().flush();
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                return Ok(());
            }
        }
    }

    println!();
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    if input.trim().is_empty() {
        eprintln!("USAGE: ghai-rewrite-issue My issue description");
        std::process::exit(13);
    }
    rewrite_issue(&input).await?;
    Ok(())
}
