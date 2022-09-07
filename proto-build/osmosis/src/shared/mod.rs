use std::{
    env,
    ffi::OsStr,
    path::PathBuf,
    process,
    sync::atomic::{self, AtomicBool},
};
use walkdir::WalkDir;

/// Suppress log messages
// TODO(tarcieri): use a logger for this
static QUIET: AtomicBool = AtomicBool::new(false);

pub mod log_macro {
    /// Log info to the console (if `QUIET` is disabled)
    // TODO(tarcieri): use a logger for this
    macro_rules! info {
            ($msg:expr) => {
                if !is_quiet() {
                    println!("[info] {}", $msg)
                }
            };
            ($fmt:expr, $($arg:tt)+) => {
                info!(&format!($fmt, $($arg)+))
            };
        }
    pub(crate) use info;
}

pub fn is_quiet() -> bool {
    QUIET.load(atomic::Ordering::Relaxed)
}

pub fn set_quiet() {
    QUIET.store(true, atomic::Ordering::Relaxed);
}

/// Parse `--github` flag passed to `proto-build` on the eponymous GitHub Actions job.
/// Disables `info`-level log messages, instead outputting only a commit message.
pub fn is_github() -> bool {
    env::args().any(|arg| arg == "--github")
}

pub fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = if is_quiet() {
        process::Stdio::null()
    } else {
        process::Stdio::inherit()
    };

    let exit_status = process::Command::new("git")
        .args(args)
        .stdout(stdout)
        .status()
        .expect("git exit status missing");

    if !exit_status.success() {
        panic!("git exited with error code: {:?}", exit_status.code());
    }
}

pub fn run_cmd(cmd: &str, args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = if is_quiet() {
        process::Stdio::null()
    } else {
        process::Stdio::inherit()
    };

    let exit_status = process::Command::new(cmd)
        .args(args)
        .stdout(stdout)
        .status()
        .expect("rm exit status missing");

    if !exit_status.success() {
        panic!("rm exited with error code: {:?}", exit_status.code());
    }
}

/// collect_protos walks every path in `proto_paths` and recursively locates all .proto
/// files in each path's subdirectories, adding the full path of each file to `protos`
///
/// Any errors encountered will cause failure for the path provided to WalkDir::new()
pub fn collect_protos(proto_paths: &[String], protos: &mut Vec<PathBuf>) {
    for proto_path in proto_paths {
        protos.append(
            &mut WalkDir::new(proto_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| {
                    e.file_type().is_file()
                        && e.path().extension().is_some()
                        && e.path().extension().unwrap() == "proto"
                })
                .map(|e| e.into_path())
                .collect(),
        );
    }
}
