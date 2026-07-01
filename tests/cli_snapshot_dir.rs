use std::{
    env, fs, io,
    path::{Path, PathBuf},
    process::{self, Command, Output},
    time::{SystemTime, UNIX_EPOCH},
};

struct TempRoot {
    path: PathBuf,
}

impl TempRoot {
    fn new(test_name: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after Unix epoch")
            .as_nanos();

        let path = env::temp_dir().join(format!("syscfg-{test_name}-{}-{unique}", process::id()));

        Self { path }
    }

    fn snapshot_path(&self) -> PathBuf {
        self.path.join("missing-parent").join("snapshot")
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        match fs::remove_dir_all(&self.path) {
            Ok(()) => {}
            Err(err) if err.kind() == io::ErrorKind::NotFound => {}
            Err(err) => panic!(
                "failed to clean temporary test directory {}: {err}",
                self.path.display()
            ),
        }
    }
}

#[test]
fn save_and_load_create_explicit_snapshot_directory_before_reading_config() {
    for case in [
        CliCase {
            subcommand: "save",
            path_flag: "-o",
        },
        CliCase {
            subcommand: "load",
            path_flag: "-i",
        },
    ] {
        let temp = TempRoot::new(case.subcommand);
        let snapshot_path = temp.snapshot_path();

        assert_absent(&snapshot_path);

        let output = run_syscfg(case, &snapshot_path);

        assert!(
            snapshot_path.is_dir(),
            "{} should create missing snapshot directory {} before reading its config; status: {}; stderr: {}",
            case.subcommand,
            snapshot_path.display(),
            output.status,
            String::from_utf8_lossy(&output.stderr),
        );
        assert!(
            !output.status.success(),
            "{} without SnapshotConfig.json should fail instead of accepting an unconfigured snapshot",
            case.subcommand,
        );
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("SnapshotConfig.json"),
            "{} should surface the missing snapshot config after creating the directory; stderr: {}",
            case.subcommand,
            String::from_utf8_lossy(&output.stderr),
        );
    }
}

#[derive(Copy, Clone)]
struct CliCase {
    subcommand: &'static str,
    path_flag: &'static str,
}

fn run_syscfg(case: CliCase, snapshot_path: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_syscfg"))
        .arg(case.subcommand)
        .arg(case.path_flag)
        .arg(snapshot_path)
        .output()
        .unwrap_or_else(|err| panic!("failed to run syscfg {}: {err}", case.subcommand))
}

fn assert_absent(path: &Path) {
    assert!(
        !path.exists(),
        "test setup expected {} not to exist",
        path.display()
    );
}
