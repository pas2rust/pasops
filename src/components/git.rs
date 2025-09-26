use super::prelude::*;
use std::path::Path;
use tokio::process::Command;

pub async fn git(badge_path: &str, repo_dir: &str) -> MyResult<()> {
    let repo_dir = Path::new(repo_dir);
    let badge_path = Path::new(badge_path);
    let filename = badge_path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("invalid badge file name")?;

    let ls = Command::new("ls")
        .arg("-la")
        .current_dir(repo_dir)
        .output()
        .await?;
    eprintln!(
        "Files in {}:\n{}",
        repo_dir.display(),
        String::from_utf8_lossy(&ls.stdout)
    );

    let cat = Command::new("cat").arg(badge_path).output().await?;
    eprintln!(
        "Content of {}:\n{}",
        badge_path.display(),
        String::from_utf8_lossy(&cat.stdout)
    );

    Command::new("git")
        .args(["config", "user.name", "github-actions[bot]"])
        .current_dir(repo_dir)
        .output()
        .await?;
    Command::new("git")
        .args([
            "config",
            "user.email",
            "41898282+github-actions[bot]@users.noreply.github.com",
        ])
        .current_dir(repo_dir)
        .output()
        .await?;

    let add = Command::new("git")
        .args(["add", "--", filename])
        .current_dir(repo_dir)
        .output()
        .await?;
    if !add.status.success() {
        return Err(format!("git add failed: {}", String::from_utf8_lossy(&add.stderr)).into());
    }

    let msg = format!("chore: Update {filename}");
    let commit = Command::new("git")
        .args(["commit", "-m", &msg, "--allow-empty"])
        .current_dir(repo_dir)
        .output()
        .await?;
    if !commit.status.success() {
        return Err(format!(
            "git commit failed: {}",
            String::from_utf8_lossy(&commit.stderr)
        )
        .into());
    }

    let push = Command::new("git")
        .args(["push"])
        .current_dir(repo_dir)
        .output()
        .await?;
    if !push.status.success() {
        return Err(format!("git push failed: {}", String::from_utf8_lossy(&push.stderr)).into());
    }

    let log = Command::new("git")
        .args(["log", "--oneline", "-3"])
        .current_dir(repo_dir)
        .output()
        .await?;
    eprintln!("Last 3 commits:\n{}", String::from_utf8_lossy(&log.stdout));

    Ok(())
}
