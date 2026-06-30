use std::{path::Path, process::Command};

use anyhow::{Context, Result, bail};

pub fn init(repo_path: &Path) -> Result<()> {
    let mut command = Command::new("git");
    command.arg("init").arg(repo_path);

    run_command(command, format!("git init {}", repo_path.display())).map(|_| ())
}

/// Add's all unstaged changes and commits
pub fn commit(repo_path: &Path, msg: &str) -> Result<()> {
    run_git(repo_path, &["add", "-A"])?;
    run_git(repo_path, &["commit", "-m", msg])
}

pub fn push(repo_path: &Path) -> Result<()> {
    run_git(repo_path, &["push"])
}

pub fn has_changes(repo_path: &Path) -> Result<bool> {
    Ok(!git_output(repo_path, &["status", "--porcelain"])?
        .trim()
        .is_empty())
}

/// stashes all unsaged changes, calls git pull, and then does stash pop
pub fn pull(repo_path: &Path) -> Result<()> {
    let should_stash = has_changes(repo_path)?;

    if should_stash {
        run_git(
            repo_path,
            &[
                "stash",
                "push",
                "--include-untracked",
                "--message",
                "syscfg auto-stash before pull",
            ],
        )?;
    }

    run_git(repo_path, &["pull"])?;

    if should_stash {
        run_git(repo_path, &["stash", "pop"])?;
    }

    Ok(())
}

fn run_git(repo_path: &Path, args: &[&str]) -> Result<()> {
    git_output(repo_path, args).map(|_| ())
}

fn git_output(repo_path: &Path, args: &[&str]) -> Result<String> {
    let mut command = Command::new("git");
    command.arg("-C").arg(repo_path).args(args);

    run_command(
        command,
        format!("git -C {} {}", repo_path.display(), args.join(" ")),
    )
}

fn run_command(mut command: Command, description: String) -> Result<String> {
    let output = command
        .output()
        .with_context(|| format!("failed to run {description}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let details = if stderr.trim().is_empty() {
            stdout.trim()
        } else {
            stderr.trim()
        };

        bail!(
            "{} failed with status {}{}{}",
            description,
            output.status,
            if details.is_empty() { "" } else { ": " },
            details
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}
