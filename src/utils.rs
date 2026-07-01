use std::{fs, path::Path};

use anyhow::{Context, Result};

pub fn ensure_snapshot_dir(ss_path: &Path) -> Result<()> {
    fs::create_dir_all(ss_path)
        .with_context(|| format!("failed to create snapshot directory {}", ss_path.display()))
}

pub fn copy_path(src: &Path, dst: &Path) -> Result<()> {
    let metadata =
        fs::metadata(src).with_context(|| format!("failed to read {}", src.display()))?;

    if metadata.is_dir() {
        copy_dir(src, dst)
    } else {
        if let Some(parent) = dst.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create directory {}", parent.display()))?;
        }

        fs::copy(src, dst)
            .with_context(|| format!("failed to copy {} to {}", src.display(), dst.display()))?;

        Ok(())
    }
}

pub fn replace_path(src: &Path, dst: &Path) -> Result<()> {
    fs::metadata(src).with_context(|| format!("failed to read {}", src.display()))?;
    remove_path_if_exists(dst)?;
    copy_path(src, dst)
}

pub fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)
        .with_context(|| format!("failed to create directory {}", dst.display()))?;

    for entry in
        fs::read_dir(src).with_context(|| format!("failed to read directory {}", src.display()))?
    {
        let entry = entry
            .with_context(|| format!("failed to read entry in directory {}", src.display()))?;
        copy_path(&entry.path(), &dst.join(entry.file_name()))?;
    }

    Ok(())
}

fn remove_path_if_exists(path: &Path) -> Result<()> {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(());
    };

    if metadata.is_dir() {
        fs::remove_dir_all(path)
            .with_context(|| format!("failed to remove directory {}", path.display()))?;
    } else {
        fs::remove_file(path)
            .with_context(|| format!("failed to remove file {}", path.display()))?;
    }

    Ok(())
}
