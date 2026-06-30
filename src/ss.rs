use std::{
    env,
    path::{Path, PathBuf},
};

// sys_path returns the system path for the object in question and ss_path returns the snapshot
// path for the object in question.

pub fn appconfig_sys_path(name: &str) -> PathBuf {
    env::home_dir().unwrap().join(".config").join(name)
}

pub fn appconfig_ss_path(ss_path: &Path, name: &str) -> PathBuf {
    ss_path.join("AppConfig").join(name)
}

pub fn font_sys_path(name: &str) -> PathBuf {
    env::home_dir()
        .unwrap()
        .join(".local")
        .join("share")
        .join("fonts")
        .join(name)
}

pub fn font_ss_path(ss_path: &Path, name: &str) -> PathBuf {
    ss_path.join("Fonts").join(name)
}
