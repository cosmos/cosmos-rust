//! Build OSMOSIS proto files. This build script clones the CosmosSDK version
//! specified in the COSMOS_SDK_REV constant and then uses that to build the required
//! proto files for further compilation. This is based on the proto-compiler code
//! in github.com/informalsystems/ibc-rs

use regex::Regex;
use std::{
    env,
    ffi::OsStr,
    fs::{self, create_dir_all, remove_dir_all},
    io,
    path::{Path, PathBuf},
    process,
    sync::atomic::{self, AtomicBool},
};
use walkdir::WalkDir;

/// Suppress log messages
// TODO(tarcieri): use a logger for this
static QUIET: AtomicBool = AtomicBool::new(false);


/// The osmosisd commit or tag to be cloned and used to build the proto files
const OSMOSISD_REV: &str = "v8.0.0";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const OSMOSIS_SDK_PROTO_DIR: &str = "../osmosis-proto/src/prost/";
/// Directory where the submodule is located
const WASMD_DIR: &str = "../wasmd";
/// Directory where the submodule is located
const OSMOSISD_DIR: &str = "../osmosis";
/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

// Patch strings used by `copy_and_patch`

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto`)
const EXCLUDED_PROTO_PACKAGES: &[&str] = &["gogoproto", "google", "tendermint","cosmos"]; 

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

fn main() {
    if is_github() {
        set_quiet();
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let proto_dir: PathBuf = OSMOSIS_SDK_PROTO_DIR.parse().unwrap();

    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone()).unwrap();
    }

    let temp_osmosisd_dir = tmp_build_dir.join("osmosisd");

    fs::create_dir_all(&temp_osmosisd_dir).unwrap();

    update_submodules();
    output_osmosisd_version(&temp_osmosisd_dir);
    compile_osmosisd_protos(&temp_osmosisd_dir);
    compile_osmosisd_proto_services(&temp_osmosisd_dir);

    copy_generated_files(&temp_osmosisd_dir, &proto_dir.join("osmosisd"));

    if is_github() {
        println!(
            "Rebuild protos with proto-build (osmosisd rev: {}))",
            OSMOSISD_REV
        );
    }
}

fn is_quiet() -> bool {
    QUIET.load(atomic::Ordering::Relaxed)
}

fn set_quiet() {
    QUIET.store(true, atomic::Ordering::Relaxed);
}

/// Parse `--github` flag passed to `proto-build` on the eponymous GitHub Actions job.
/// Disables `info`-level log messages, instead outputting only a commit message.
fn is_github() -> bool {
    env::args().any(|arg| arg == "--github")
}

fn run_cmd(cmd: &str,args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
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

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
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

fn update_submodules() {
    info!("Updating osmosisd submodule...");
    run_git(&["submodule", "update", "--init"]);
    run_git(&["-C", OSMOSISD_DIR, "fetch"]);
    run_git(&["-C", OSMOSISD_DIR, "reset", "--hard", OSMOSISD_REV]);
    run_cmd("pwd",&[""]);
    /*
     * osmosis/gamm/v1beta1/tx.proto:9:9: "osmosis.gamm.v1beta1.Msg" is already defined in file "osmosis/gamm/pool-models/balancer/tx.proto".
     * To workaround this issue renaming Msg.
     */
    run_cmd("sed",&["-i",r##"/service Msg {/c\service Msg1 {"##,"../osmosis/proto/osmosis/gamm/pool-models/balancer/tx.proto"]);
    run_cmd("sed",&["-i",r##"/service Msg {/c\service Msg2 {"##,"../osmosis/proto/osmosis/gamm/v1beta1/tx.proto"]);
    //run_cmd("rm",&["../osmosis/proto/osmosis/gamm/pool-models/balancer/tx.proto"]);
}


fn output_osmosisd_version(out_dir: &Path) {
    let path = out_dir.join("OSMOSISD_COMMIT");
    fs::write(path, OSMOSISD_REV).unwrap();
}

fn compile_osmosisd_protos(out_dir: &Path) {
    let sdk_dir = Path::new(OSMOSISD_DIR);
    let wasmd_sdk_dir = Path::new(WASMD_DIR);

    info!(
        "Compiling .proto files to Rust into '{}'...",
        out_dir.display()
    );

    // Paths
    let proto_paths = [format!("{}/proto/osmosis", sdk_dir.display())];

    let proto_includes_paths = [
        format!("{}/proto", sdk_dir.display()),
        format!("{}/proto", wasmd_sdk_dir.display()),
        format!("{}/third_party/proto", wasmd_sdk_dir.display()),
    ];
    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    // Compile all proto files
    let mut config = prost_build::Config::default();
    config.out_dir(out_dir);
    config.extern_path(".tendermint", "::tendermint_proto");
    config.extern_path(".cosmos", "cosmos_sdk_proto::cosmos");

    if let Err(e) = config.compile_protos(&protos, &includes) {
        eprintln!("[error] couldn't compile protos: {}", e);
        panic!("protoc failed!");
    }
}

fn compile_osmosisd_proto_services(out_dir: &Path) {
    let sdk_dir = Path::new(OSMOSISD_DIR);
    let wasmd_sdk_dir = Path::new(WASMD_DIR);

    let proto_includes_paths = [
        format!("{}/proto", sdk_dir.display()),
        format!("{}/proto", wasmd_sdk_dir.display()),
        format!("{}/third_party/proto", wasmd_sdk_dir.display()),
    ];

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    let proto_services_path = [
        PathBuf::from(sdk_dir).join("proto/osmosis/gamm/v1beta1/query.proto"),
        PathBuf::from(sdk_dir).join("proto/osmosis/gamm/v1beta1/tx.proto"),
        PathBuf::from(sdk_dir).join("proto/osmosis/gamm/pool-models/balancer/tx.proto"),
    ];

    // List available paths for dependencies
    let services = proto_services_path
        .iter()
        .map(|p| p.as_os_str().to_os_string())
        .collect::<Vec<_>>();

    // Compile all proto client for GRPC services
    info!("Compiling osmosisd proto clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir(out_dir)
        .compile(&services, &includes)
        .unwrap();

    info!("=> Done!");
}

/// collect_protos walks every path in `proto_paths` and recursively locates all .proto
/// files in each path's subdirectories, adding the full path of each file to `protos`
///
/// Any errors encountered will cause failure for the path provided to WalkDir::new()
fn collect_protos(proto_paths: &[String], protos: &mut Vec<PathBuf>) {
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

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    info!("Copying generated files into '{}'...", to_dir.display());

    // Remove old compiled files
    remove_dir_all(&to_dir).unwrap_or_default();
    create_dir_all(&to_dir).unwrap();

    let mut filenames = Vec::new();

    // Copy new compiled files (prost does not use folder structures)
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            copy_and_patch(e.path(), format!("{}/{}", to_dir.display(), &filename))
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}

fn copy_and_patch(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    /// Regex substitutions to apply to the prost-generated output
    const REPLACEMENTS: &[(&str, &str)] = &[
        ("(super::)+cosmos", "cosmos_sdk_proto::cosmos"),
        // Use `tendermint-proto` proto definitions
        ("(super::)+tendermint", "tendermint_proto"),
        // Feature-gate gRPC client modules
        (
            "/// Generated client implementations.",
            "/// Generated client implementations.\n\
             #[cfg(feature = \"grpc\")]\n\
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
        ),
        // Feature-gate gRPC client impls which use `tonic::transport`
        (
            "impl (.+)Client<tonic::transport::Channel>",
            "#[cfg(feature = \"grpc-transport\")]\n    \
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc-transport\")))]\n    \
             impl ${1}Client<tonic::transport::Channel>",
        ),
    ];

    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in EXCLUDED_PROTO_PACKAGES {
        if let Some(filename) = src.as_ref().file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let mut contents = fs::read_to_string(src)?;

    for &(regex, replacement) in REPLACEMENTS {
        contents = Regex::new(regex)
            .unwrap_or_else(|_| panic!("invalid regex: {}", regex))
            .replace_all(&contents, replacement)
            .to_string();
    }

    fs::write(dest, &*contents)
}
