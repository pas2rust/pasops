use super::prelude::*;
use std::path::Path;
use tokio::process::Command;

pub async fn git(args: &Args) -> MyResult<()> {
    let repo_dir = Path::new(&args.destiny).canonicalize()?;
    let src = Path::new(&args.badge_name);
    let filename = src
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("invalid badge file name")?;
    let dest_path = repo_dir.join(filename);

    tokio::fs::copy(&src, &dest_path).await?;
    Command::new("git")
        .args(&["config", "user.name", "github-actions[bot]"])
        .current_dir(&repo_dir)
        .output()
        .await?;
    Command::new("git")
        .args(&[
            "config",
            "user.email",
            "41898282+github-actions[bot]@users.noreply.github.com",
        ])
        .current_dir(&repo_dir)
        .output()
        .await?;

    // git add
    let add = Command::new("git")
        .args(&["add", "--", filename])
        .current_dir(&repo_dir)
        .output()
        .await?;
    if !add.status.success() {
        let err = String::from_utf8_lossy(&add.stderr);
        return Err(format!("git add failed: {}", err).into());
    }

    // git commit --allow-empty
    let msg = format!("chore: Update {}", filename);
    let commit = Command::new("git")
        .args(&["commit", "-m", &msg, "--allow-empty"])
        .current_dir(&repo_dir)
        .output()
        .await?;
    if !commit.status.success() {
        let stderr = String::from_utf8_lossy(&commit.stderr);
        return Err(format!("git commit failed: {}", stderr).into());
    }

    // git push
    let push = Command::new("git")
        .args(&["push"])
        .current_dir(&repo_dir)
        .output()
        .await?;
    if !push.status.success() {
        let stderr = String::from_utf8_lossy(&push.stderr);
        return Err(format!("git push failed: {}", stderr).into());
    }

    Ok(())
}
