use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

use anyhow::{Context, Result, bail};

use crate::{cli::CliCommand, config::Config};

mod cli;
mod config;
mod git;
mod globals;
mod ss;
mod utils;

fn main() {
    let cli = cli::parse();

    let res = match &cli.command {
        CliCommand::MkSnap { output } => mksnap(output),
        CliCommand::Save { output } => save(output),
        CliCommand::Load { input } => load(input),
    };

    if let Err(e) = res {
        eprintln!("{e}");
        process::exit(1);
    }
}

fn mksnap(output: &Option<PathBuf>) -> Result<()> {
    let ss_path = snapshot_path(output)?;

    utils::ensure_snapshot_dir(&ss_path)?;

    git::init(&ss_path)?;

    let config_path = ss_path.join("SnapshotConfig.json");
    let config = serde_json::to_string_pretty(&Config::default())
        .context("failed to serialize default snapshot config")?;
    fs::write(&config_path, format!("{config}\n"))
        .with_context(|| format!("failed to write config file {}", config_path.display()))?;

    git::commit(&ss_path, "Created Snapshot")?;

    Ok(())
}

fn save(output: &Option<PathBuf>) -> Result<()> {
    let ss_path = snapshot_path(output)?;
    utils::ensure_snapshot_dir(&ss_path)?;
    let config = Config::read(&ss_path)?;

    if config.font != "*" {
        bail!("anything other than * is not currently supported for fonts");
    }

    for name in &config.app_config {
        let src = ss::appconfig_sys_path(name);
        let dst = ss::appconfig_ss_path(&ss_path, name);
        utils::replace_path(&src, &dst)
            .with_context(|| format!("failed to save app config {name}"))?;
    }

    save_fonts(&ss_path)?;

    if git::has_changes(&ss_path)? {
        git::commit(&ss_path, "Saved Snapshot")?;
    }

    Ok(())
}

fn load(input: &Option<PathBuf>) -> Result<()> {
    let ss_path = snapshot_path(input)?;
    utils::ensure_snapshot_dir(&ss_path)?;
    let config = Config::read(&ss_path)?;

    if config.font != "*" {
        bail!("anything other than * is not currently supported for fonts");
    }

    for name in &config.app_config {
        let src = ss::appconfig_ss_path(&ss_path, name);
        let dst = ss::appconfig_sys_path(name);
        utils::copy_path(&src, &dst).unwrap();
        // .with_context(|| format!("failed to load app config {name}"))?;
    }

    load_fonts(&ss_path)?;

    Ok(())
}

fn snapshot_path(path: &Option<PathBuf>) -> Result<PathBuf> {
    match path {
        Some(path) => Ok(path.clone()),
        None => Ok(env::home_dir()
            .context("failed to determine home directory")?
            .join(".syscfg-snapshot")),
    }
}

fn load_fonts(ss_path: &Path) -> Result<()> {
    let fonts_path = ss::font_ss_path(ss_path, "");

    if !fonts_path.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(&fonts_path)
        .with_context(|| format!("failed to read fonts directory {}", fonts_path.display()))?
    {
        let entry = entry.with_context(|| {
            format!(
                "failed to read entry in fonts directory {}",
                fonts_path.display()
            )
        })?;
        let name = entry.file_name();
        let dst = ss::font_sys_path(&name.to_string_lossy());
        utils::copy_path(&entry.path(), &dst).with_context(|| {
            format!(
                "failed to load font {}",
                entry.file_name().to_string_lossy()
            )
        })?;
    }

    Ok(())
}

fn save_fonts(ss_path: &Path) -> Result<()> {
    let src = ss::font_sys_path("");
    let dst = ss::font_ss_path(ss_path, "");

    if src.exists() {
        utils::replace_path(&src, &dst).context("failed to save fonts")?;
    } else {
        fs::create_dir_all(&dst)
            .with_context(|| format!("failed to create fonts directory {}", dst.display()))?;
    }

    Ok(())
}
