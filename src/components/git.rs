use super::prelude::*;
use std::path::Path;
use tokio::process::Command;

pub async fn git(args: &Args) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let repo_dir = Path::new(&args.destiny).canonicalize()?;

    if !repo_dir.is_dir() {
        return Err(format!("'{}' is not a directory", repo_dir.display()).into());
    }
    if !repo_dir.join(".git").exists() {
        return Err(format!("'{}' doesn't look like a git repo (no .git)", repo_dir.display()).into());
    }

    let status = Command::new("git")
        .args(&["config", "user.name", "github-actions[bot]"])
        .current_dir(&repo_dir)
        .status()
        .await?;
    if !status.success() {
        return Err(format!("git config user.name failed: {}", status).into());
    }

    let status = Command::new("git")
        .args(&["config", "user.email", "41898282+github-actions[bot]@users.noreply.github.com"])
        .current_dir(&repo_dir)
        .status()
        .await?;
    if !status.success() {
        return Err(format!("git config user.email failed: {}", status).into());
    }

    let status = Command::new("git")
        .args(&["add", &args.badge_name])
        .current_dir(&repo_dir)
        .status()
        .await?;
    if !status.success() {
        return Err(format!("git add failed: {}", status).into());
    }

   
    let msg = format!("chore: Update {}", args.badge_name);
    let status = Command::new("git")
        .args(&["commit", "-m", &msg, "--allow-empty"])
        .current_dir(&repo_dir)
        .status()
        .await?;
    if !status.success() {
        return Err(format!("git commit failed: {}", status).into());
    }

    let status = Command::new("git")
        .args(&["push"])
        .current_dir(&repo_dir)
        .status()
        .await?;
    if !status.success() {
        return Err(format!("git push failed: {}", status).into());
    }

    Ok(())
}
