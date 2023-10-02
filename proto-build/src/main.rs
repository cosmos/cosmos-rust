//! Build CosmosSDK/Tendermint/IBC proto files. This build script clones the CosmosSDK version
//! specified in the COSMOS_SDK_REV constant and then uses that to build the required
//! proto files for further compilation. This is based on the proto-compiler code
//! in github.com/informalsystems/ibc-rs

use regex::Regex;
use std::{
    env,
    ffi::{OsStr, OsString},
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

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "v0.46.12";

/// The Cosmos ibc-go commit or tag to be cloned and used to build the proto files
const IBC_REV: &str = "v3.0.0";

/// The wasmd commit or tag to be cloned and used to build the proto files
const WASMD_REV: &str = "v0.29.2";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const COSMOS_SDK_PROTO_DIR: &str = "../cosmos-sdk-proto/src/prost/";
/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../cosmos-sdk-go";
/// Directory where the cosmos/ibc-go submodule is located
const IBC_DIR: &str = "../ibc-go";
/// Directory where the submodule is located
const WASMD_DIR: &str = "../wasmd";
/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

// Patch strings used by `copy_and_patch`

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto`)
const EXCLUDED_PROTO_PACKAGES: &[&str] = &["gogoproto", "google", "tendermint"];

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
    let proto_dir: PathBuf = COSMOS_SDK_PROTO_DIR.parse().unwrap();

    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone()).unwrap();
    }

    let temp_sdk_dir = tmp_build_dir.join("cosmos-sdk");
    let temp_ibc_dir = tmp_build_dir.join("ibc-go");
    let temp_wasmd_dir = tmp_build_dir.join("wasmd");

    fs::create_dir_all(&temp_sdk_dir).unwrap();
    fs::create_dir_all(&temp_ibc_dir).unwrap();
    fs::create_dir_all(&temp_wasmd_dir).unwrap();

    update_submodules();
    output_sdk_version(&temp_sdk_dir);
    output_ibc_version(&temp_ibc_dir);
    output_wasmd_version(&temp_wasmd_dir);
    compile_sdk_protos_and_services(&temp_sdk_dir);
    compile_ibc_protos_and_services(&temp_ibc_dir);
    compile_wasmd_proto_and_services(&temp_wasmd_dir);

    copy_generated_files(&temp_sdk_dir, &proto_dir.join("cosmos-sdk"));
    copy_generated_files(&temp_ibc_dir, &proto_dir.join("ibc-go"));
    copy_generated_files(&temp_wasmd_dir, &proto_dir.join("wasmd"));

    apply_patches(&proto_dir);

    info!("Running rustfmt on prost/tonic-generated code");
    run_rustfmt(&proto_dir);

    if is_github() {
        println!(
            "Rebuild protos with proto-build (cosmos-sdk rev: {} ibc-go rev: {} wasmd rev: {}))",
            COSMOS_SDK_REV, IBC_REV, WASMD_REV
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

fn run_cmd(cmd: impl AsRef<OsStr>, args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    let stdout = if is_quiet() {
        process::Stdio::null()
    } else {
        process::Stdio::inherit()
    };

    let exit_status = process::Command::new(&cmd)
        .args(args)
        .stdout(stdout)
        .status()
        .unwrap_or_else(|e| match e.kind() {
            io::ErrorKind::NotFound => panic!(
                "error running '{:?}': command not found. Is it installed?",
                cmd.as_ref()
            ),
            _ => panic!("error running '{:?}': {:?}", cmd.as_ref(), e),
        });

    if !exit_status.success() {
        match exit_status.code() {
            Some(code) => panic!("{:?} exited with error code: {:?}", cmd.as_ref(), code),
            None => panic!("{:?} exited without error code", cmd.as_ref()),
        }
    }
}

fn run_buf(config: &str, proto_path: impl AsRef<Path>, out_dir: impl AsRef<Path>) {
    run_cmd(
        "buf",
        [
            "generate",
            "--template",
            config,
            "--include-imports",
            "-o",
            &out_dir.as_ref().display().to_string(),
            &proto_path.as_ref().display().to_string(),
        ],
    );
}

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) {
    run_cmd("git", args)
}

fn run_rustfmt(dir: &Path) {
    let mut args = ["--edition", "2021"]
        .iter()
        .map(Into::into)
        .collect::<Vec<OsString>>();

    args.extend(
        WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file() && e.path().extension() == Some(OsStr::new("rs")))
            .map(|e| e.into_path())
            .map(Into::into),
    );

    run_cmd("rustfmt", args);
}

fn update_submodules() {
    info!("Updating cosmos/cosmos-sdk submodule...");
    run_git(["submodule", "update", "--init"]);
    run_git(["-C", COSMOS_SDK_DIR, "fetch"]);
    run_git(["-C", COSMOS_SDK_DIR, "reset", "--hard", COSMOS_SDK_REV]);

    info!("Updating cosmos/ibc-go submodule...");
    run_git(["submodule", "update", "--init"]);
    run_git(["-C", IBC_DIR, "fetch"]);
    run_git(["-C", IBC_DIR, "reset", "--hard", IBC_REV]);

    info!("Updating wasmd submodule...");
    run_git(["submodule", "update", "--init"]);
    run_git(["-C", WASMD_DIR, "fetch"]);
    run_git(["-C", WASMD_DIR, "reset", "--hard", WASMD_REV]);
}

fn output_sdk_version(out_dir: &Path) {
    let path = out_dir.join("COSMOS_SDK_COMMIT");
    fs::write(path, COSMOS_SDK_REV).unwrap();
}

fn output_ibc_version(out_dir: &Path) {
    let path = out_dir.join("IBC_COMMIT");
    fs::write(path, IBC_REV).unwrap();
}

fn output_wasmd_version(out_dir: &Path) {
    let path = out_dir.join("WASMD_COMMIT");
    fs::write(path, WASMD_REV).unwrap();
}

fn compile_sdk_protos_and_services(out_dir: &Path) {
    info!(
        "Compiling cosmos-sdk .proto files to Rust into '{}'...",
        out_dir.display()
    );

    // Compile all of the proto files, along with grpc service clients
    info!("Compiling proto definitions and clients for GRPC services!");
    let proto_path = Path::new(COSMOS_SDK_DIR).join("proto");
    run_buf("buf.sdk.gen.yaml", proto_path, out_dir);
    info!("=> Done!");
}

fn compile_wasmd_proto_and_services(out_dir: &Path) {
    let sdk_dir = Path::new(WASMD_DIR);
    let proto_path = sdk_dir.join("proto");
    let proto_paths = [format!("{}/proto/cosmwasm/wasm", sdk_dir.display())];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    // Compile all proto client for GRPC services
    info!("Compiling wasmd proto clients for GRPC services!");
    run_buf("buf.wasmd.gen.yaml", proto_path, out_dir);
    info!("=> Done!");
}

fn compile_ibc_protos_and_services(out_dir: &Path) {
    info!(
        "Compiling .proto files to Rust into '{}'...",
        out_dir.display()
    );

    let root = env!("CARGO_MANIFEST_DIR");
    let ibc_dir = Path::new(IBC_DIR);

    let proto_includes_paths = [
        format!("{}/../proto", root),
        format!("{}/proto", ibc_dir.display()),
        format!("{}/third_party/proto", ibc_dir.display()),
    ];

    let proto_paths = [
        format!("{}/../proto/definitions/mock", root),
        format!(
            "{}/proto/ibc/applications/interchain_accounts",
            ibc_dir.display()
        ),
        format!("{}/proto/ibc/applications/transfer", ibc_dir.display()),
        format!("{}/proto/ibc/core/channel", ibc_dir.display()),
        format!("{}/proto/ibc/core/client", ibc_dir.display()),
        format!("{}/proto/ibc/core/commitment", ibc_dir.display()),
        format!("{}/proto/ibc/core/connection", ibc_dir.display()),
        format!("{}/proto/ibc/core/port", ibc_dir.display()),
        format!("{}/proto/ibc/core/types", ibc_dir.display()),
        format!("{}/proto/ibc/lightclients/localhost", ibc_dir.display()),
        format!("{}/proto/ibc/lightclients/solomachine", ibc_dir.display()),
        format!("{}/proto/ibc/lightclients/tendermint", ibc_dir.display()),
    ];
    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    // Compile all of the proto files, along with the grpc service clients
    info!("Compiling proto definitions and clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir(out_dir)
        .extern_path(".tendermint", "::tendermint_proto")
        .compile(&protos, &includes)
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
    remove_dir_all(to_dir).unwrap_or_default();
    create_dir_all(to_dir).unwrap();

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
        // Use `tendermint-proto` proto definitions
        ("(super::)+tendermint", "tendermint_proto"),
        // Feature-gate gRPC client modules
        (
            "/// Generated client implementations.",
            "/// Generated client implementations.\n\
             #[cfg(feature = \"grpc\")]\n\
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
        ),
        // Feature-gate gRPC impls which use `tonic::transport`
        (
            "impl(.+)tonic::transport(.+)",
            "#[cfg(feature = \"grpc-transport\")]\n    \
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc-transport\")))]\n    \
             impl${1}tonic::transport${2}",
        ),
        // Feature-gate gRPC server modules
        (
            "/// Generated server implementations.",
            "/// Generated server implementations.\n\
             #[cfg(feature = \"grpc\")]\n\
             #[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
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

    fs::write(dest, &contents)
}

fn patch_file(path: impl AsRef<Path>, pattern: &Regex, replacement: &str) -> io::Result<()> {
    let mut contents = fs::read_to_string(&path)?;
    contents = pattern.replace_all(&contents, replacement).to_string();
    fs::write(path, &contents)
}

/// Fix clashing type names in prost-generated code. See cosmos/cosmos-rust#154.
fn apply_patches(proto_dir: &Path) {
    for (pattern, replacement) in [
        ("enum Validators", "enum Policy"),
        (
            "stake_authorization::Validators",
            "stake_authorization::Policy",
        ),
    ] {
        patch_file(
            &proto_dir.join("cosmos-sdk/cosmos.staking.v1beta1.rs"),
            &Regex::new(pattern).unwrap(),
            replacement,
        )
        .expect("error patching cosmos.staking.v1beta1.rs");
    }
}
