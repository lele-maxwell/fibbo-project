use reqwest::Client;
use std::env;
use anyhow:: {Result, Context};

pub async fn post_comment(pr_content: &str) -> Result<(), anyhow::Error> {
    let repo = env::var("GITHUB_REPOSITORY").expect("GITHUB_REPOSITORY not set");
    let pr_number: u64 = env::var("PR_NUMBER")
    .map_err(|_| anyhow::anyhow!("PR_NUMBER variable is not set"))?
    .parse()
    .map_err(|_| anyhow::anyhow!("Failed to parse PR_NUMBER as a number"))?;

      //  .unwrap_or_else(|_| "1".to_string())
       
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let url = format!(
        "https://api.github.com/repos/{}/issues/{}/comments",
        repo, pr_number
    );

    let client = Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", github_token))
        .header("User-Agent", "FibBot")
        .header("Accept", "application/vnd.github.full+json")
        .json(&serde_json::json!({ "body": pr_content }))
        .send()
        .await?;

    if response.status().is_success() {
        println!("✅ Comment posted successfully.");
    } else {
        eprintln!("❌ Failed to post comment: {:?}", response.text().await?);
    }

    Ok(())
}