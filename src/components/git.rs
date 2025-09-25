use super::prelude::*;
use std::path::Path;
use tokio::{fs, process::Command};

pub async fn git(args: &Args) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let repo_dir = Path::new(&args.destiny);
    if !repo_dir.exists() {
        return Err(format!("destination '{}' does not exist", args.destiny).into());
    }

    let src = Path::new(&args.badge_name);
    if !src.exists() {
        return Err(format!("source file '{}' not found", src.display()).into());
    }

    let filename = src
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("invalid badge file name")?;

    let dest_path = repo_dir.join(filename);
    if dest_path.exists() {
        fs::remove_file(&dest_path).await?;
    }

    fs::rename(&src, &dest_path).await?;

    let status = Command::new("git")
        .args(&["config", "user.name", "github-actions[bot]"])
        .current_dir(repo_dir)
        .status()
        .await?;
    if !status.success() {
        return Err(format!("git config user.name failed: {}", status).into());
    }

    let status = Command::new("git")
        .args(&[
            "config",
            "user.email",
            "41898282+github-actions[bot]@users.noreply.github.com",
        ])
        .current_dir(repo_dir)
        .status()
        .await?;
    if !status.success() {
        return Err(format!("git config user.email failed: {}", status).into());
    }

    let status = Command::new("git")
        .args(&["add", filename])
        .current_dir(repo_dir)
        .status()
        .await?;
    if !status.success() {
        return Err(format!("git add failed: {}", status).into());
    }

    let msg = format!("chore: Update {}", filename);
    let status = Command::new("git")
        .args(&["commit", "-m", &msg, "--allow-empty"])
        .current_dir(repo_dir)
        .status()
        .await?;
    if !status.success() {
        return Err(format!("git commit failed: {}", status).into());
    }

    let status = Command::new("git")
        .args(&["push"])
        .current_dir(repo_dir)
        .status()
        .await?;
    if !status.success() {
        return Err(format!("git push failed: {}", status).into());
    }

    Ok(())
}
