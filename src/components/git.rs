use super::prelude::*;
use std::process::Stdio;
use tokio::process::Command;

pub async fn setup_git_identity() -> MyResult<()> {
    let status_name = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.name")
        .arg("github-actions[bot]")
        .status()
        .await?;
    
    if !status_name.success() {
        return Err("git config user.name failed".into());
    }

    let status_email = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("user.email")
        .arg("github-actions[bot]@users.noreply.github.com")
        .status()
        .await?;
    
    if !status_email.success() {
        return Err("git config user.email failed".into());
    }

    Ok(())
}

pub async fn git_add_or_update_remote(name: &str, url: &str) -> MyResult<()> {
    let output = Command::new("git")
        .arg("remote")
        .arg("get-url")
        .arg(name)
        .output()
        .await?;
    
    if output.status.success() {
        let status = Command::new("git")
            .arg("remote")
            .arg("set-url")
            .arg(name)
            .arg(url)
            .status()
            .await?;
        if !status.success() {
            return Err(format!("git remote set-url {} failed", name).into());
        }
    } else {
        let status = Command::new("git")
            .arg("remote")
            .arg("add")
            .arg(name)
            .arg(url)
            .status()
            .await?;
        if !status.success() {
            return Err(format!("git remote add {} failed", name).into());
        }
    }

    Ok(())
}

pub async fn git_add(file_name: &str, token: Option<String>) -> MyResult<()> {
    let token = token
        .or_else(|| std::env::var("GITHUB_TOKEN").ok())
        .ok_or("GITHUB_TOKEN not set")?;

    let status = Command::new("git")
        .arg("-c")
        .arg(format!("http.extraHeader=Authorization: Bearer {}", token))
        .arg("add")
        .arg(file_name)
        .status()
        .await?;

    if !status.success() {
        return Err("git add failed".into());
    }

    Ok(())
}

pub async fn git_commit(message: &str, token: Option<String>) -> MyResult<()> {
    let token = token
        .or_else(|| std::env::var("BADGE_TOKEN").ok())
        .ok_or("BADGE_TOKEN not set")?;

    let status = Command::new("git")
        .arg("-c")
        .arg(format!("http.extraHeader=Authorization: Bearer {}", token))
        .arg("commit")
        .arg("-m")
        .arg(message)
        .arg("--allow-empty") // sempre allow-empty
        .status()
        .await?;

    if !status.success() {
        return Err("git commit failed".into());
    }

    Ok(())
}

pub async fn git_push(token: Option<String>, remote: &str, branch: &str) -> MyResult<()> {
    let token = token
        .or_else(|| std::env::var("GITHUB_TOKEN").ok())
        .ok_or("GITHUB_TOKEN not set")?;

    let status = Command::new("git")
        .arg("-c")
        .arg(format!("http.extraHeader=Authorization: Bearer {}", token))
        .arg("push")
        .arg(remote)
        .arg(branch)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;

    if !status.success() {
        return Err(format!("`git push` exited with code {:?}", status.code()).into());
    }

    Ok(())
}