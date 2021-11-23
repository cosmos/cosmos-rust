//! Build CosmosSDK/Tendermint/IBC proto files. This build script clones the CosmosSDK version
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

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "v0.44.1";

/// The Tendermint commit or tag to be cloned and used to build the proto files
const TENDERMINT_REV: &str = "v0.34.13";

/// The Cosmos ibc-go commit or tag to be cloned and used to build the proto files
const IBC_REV: &str = "v1.2.0";

/// The wasmd commit or tag to be cloned and used to build the proto files
const WASMD_REV: &str = "v0.17.0";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const COSMOS_SDK_PROTO_DIR: &str = "../cosmos-sdk-proto/src/prost/";
/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../cosmos-sdk-go";
/// Directory where the tendermint submodule is located
const TENDERMINT_DIR: &str = "../tendermint";
/// Directory where the cosmos/ibc-go submodule is located
const IBC_DIR: &str = "../ibc-go";
/// Directory where the submodule is located
const WASMD_DIR: &str = "../wasmd";
/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

// Patch strings used by `copy_and_patch`

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto`)
const EXCLUDED_PROTO_PACKAGES: &[&str] = &["gogoproto", "google"];
/// Regex for locating instances of `tendermint-proto` in prost/tonic build output
const TENDERMINT_PROTO_REGEX: &str = "(super::)+tendermint";
/// Attribute preceeding a Tonic client definition
const TONIC_CLIENT_ATTRIBUTE: &str = "#[doc = r\" Generated client implementations.\"]";
/// Attributes to add to gRPC clients
const GRPC_CLIENT_ATTRIBUTES: &[&str] = &[
    "#[cfg(feature = \"grpc\")]",
    "#[cfg_attr(docsrs, doc(cfg(feature = \"grpc\")))]",
    TONIC_CLIENT_ATTRIBUTE,
];

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

    fs::create_dir(tmp_build_dir.clone()).unwrap();

    update_submodules();
    output_sdk_version(&tmp_build_dir);
    output_tendermint_version(&tmp_build_dir);
    output_ibc_version(&tmp_build_dir);
    output_wasmd_version(&tmp_build_dir);
    compile_sdk_protos_and_services(&tmp_build_dir);
    compile_ibc_protos_and_services(&tmp_build_dir);
    compile_wasmd_protos(&tmp_build_dir);
    compile_wasmd_proto_services(&tmp_build_dir);
    compile_tendermint_protos_and_services(&tmp_build_dir);
    copy_generated_files(&tmp_build_dir, &proto_dir);

    if is_github() {
        println!(
            "Rebuild protos with proto-build (cosmos-sdk rev: {} tendermint rev: {} ibc-go rev: {} wasmd rev: {}))",
            COSMOS_SDK_REV, TENDERMINT_REV, IBC_REV, WASMD_REV
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
    // this is a repo wide command and only needs to
    // be run once.
    run_git(&["submodule", "update", "--init"]);

    info!("Updating cosmos/cosmos-sdk submodule...");
    run_git(&["-C", COSMOS_SDK_DIR, "fetch"]);
    run_git(&["-C", COSMOS_SDK_DIR, "reset", "--hard", COSMOS_SDK_REV]);

    info!("Updating tendermint submodule...");
    run_git(&["-C", TENDERMINT_DIR, "fetch"]);
    run_git(&["-C", TENDERMINT_DIR, "reset", "--hard", TENDERMINT_REV]);

    info!("Updating cosmos/ibc-go submodule...");
    run_git(&["-C", IBC_DIR, "fetch"]);
    run_git(&["-C", IBC_DIR, "reset", "--hard", IBC_REV]);

    info!("Updating wasmd submodule...");
    run_git(&["-C", WASMD_DIR, "fetch"]);
    run_git(&["-C", WASMD_DIR, "reset", "--hard", WASMD_REV]);
}

fn output_sdk_version(out_dir: &Path) {
    let path = out_dir.join("COSMOS_SDK_COMMIT");
    fs::write(path, COSMOS_SDK_REV).unwrap();
}

fn output_tendermint_version(out_dir: &Path) {
    let path = out_dir.join("TENDERMINT_COMMIT");
    fs::write(path, TENDERMINT_REV).unwrap();
}

fn output_ibc_version(out_dir: &Path) {
    let path = out_dir.join("IBC_COMMIT");
    fs::write(path, IBC_REV).unwrap();
}

fn output_wasmd_version(out_dir: &Path) {
    let path = out_dir.join("WASMD_COMMIT");
    fs::write(path, WASMD_REV).unwrap();
}

fn compile_wasmd_protos(out_dir: &Path) {
    let sdk_dir = Path::new(WASMD_DIR);

    info!(
        "Compiling .proto files to Rust into '{}'...",
        out_dir.display()
    );

    let root = env!("CARGO_MANIFEST_DIR");

    // Paths
    let proto_paths = [format!("{}/proto/cosmwasm/wasm", sdk_dir.display())];

    let proto_includes_paths = [
        format!("{}/../proto", root),
        format!("{}/proto", sdk_dir.display()),
        format!("{}/third_party/proto", sdk_dir.display()),
    ];
    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    // Compile all proto files
    let mut config = prost_build::Config::default();
    config.out_dir(out_dir);
    config.extern_path(".tendermint", "crate::tendermint");

    if let Err(e) = config.compile_protos(&protos, &includes) {
        eprintln!("[error] couldn't compile protos: {}", e);
        panic!("protoc failed!");
    }
}

fn compile_sdk_protos_and_services(out_dir: &Path) {
    info!(
        "Compiling cosmos-sdk .proto files to Rust into '{}'...",
        out_dir.display()
    );

    let root = env!("CARGO_MANIFEST_DIR");
    let sdk_dir = Path::new(COSMOS_SDK_DIR);

    let proto_includes_paths = [
        format!("{}/../proto", root),
        format!("{}/proto", sdk_dir.display()),
        format!("{}/third_party/proto", sdk_dir.display()),
    ];

    // Paths
    let proto_paths = [
        format!("{}/../proto/definitions/mock", root),
        format!("{}/proto/cosmos/auth", sdk_dir.display()),
        format!("{}/proto/cosmos/bank", sdk_dir.display()),
        format!("{}/proto/cosmos/base", sdk_dir.display()),
        format!("{}/proto/cosmos/base/tendermint", sdk_dir.display()),
        format!("{}/proto/cosmos/capability", sdk_dir.display()),
        format!("{}/proto/cosmos/crisis", sdk_dir.display()),
        format!("{}/proto/cosmos/crypto", sdk_dir.display()),
        format!("{}/proto/cosmos/distribution", sdk_dir.display()),
        format!("{}/proto/cosmos/evidence", sdk_dir.display()),
        format!("{}/proto/cosmos/genutil", sdk_dir.display()),
        format!("{}/proto/cosmos/gov", sdk_dir.display()),
        format!("{}/proto/cosmos/mint", sdk_dir.display()),
        format!("{}/proto/cosmos/params", sdk_dir.display()),
        format!("{}/proto/cosmos/slashing", sdk_dir.display()),
        format!("{}/proto/cosmos/staking", sdk_dir.display()),
        format!("{}/proto/cosmos/tx", sdk_dir.display()),
        format!("{}/proto/cosmos/upgrade", sdk_dir.display()),
        format!("{}/proto/cosmos/vesting", sdk_dir.display()),
    ];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    // Compile all of the proto files, along with grpc service clients
    info!("Compiling proto definitions and clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .format(true)
        .out_dir(out_dir)
        .extern_path(".tendermint", "crate::tendermint")
        .compile(&protos, &includes)
        .unwrap();

    info!("=> Done!");
}

fn compile_tendermint_protos_and_services(out_dir: &Path) {
    info!(
        "Compiling tendermint .proto files to Rust into '{}'...",
        out_dir.display()
    );

    let tendermint_dir = Path::new(TENDERMINT_DIR);

    let proto_includes_paths = [
        format!("{}/proto", tendermint_dir.display()),
        format!("{}/proto/tendermint", tendermint_dir.display()),
        format!("{}/third_party/proto", tendermint_dir.display()),
    ];

    // paths for only proto generation, these can not be combined
    // because the service generator writes to the same files and will
    // not create struct definitions if there are no services
    // folders here will have all protos in them compiled
    let proto_paths = [
        format!("{}/proto/tendermint/blocksync", tendermint_dir.display()),
        format!("{}/proto/tendermint/consensus", tendermint_dir.display()),
        format!("{}/proto/tendermint/crypto", tendermint_dir.display()),
        format!("{}/proto/tendermint/mempool", tendermint_dir.display()),
        format!("{}/proto/tendermint/p2p", tendermint_dir.display()),
        format!("{}/proto/tendermint/privval", tendermint_dir.display()),
        format!("{}/proto/tendermint/state", tendermint_dir.display()),
        format!("{}/proto/tendermint/statesync", tendermint_dir.display()),
        format!("{}/proto/tendermint/types", tendermint_dir.display()),
        format!("{}/proto/tendermint/version", tendermint_dir.display()),
        format!("{}/proto/tendermint/libs", tendermint_dir.display()),
    ];

    // paths for GRPC service generation, these are strict paths, no other
    // files will be found
    let proto_grpc_paths = [
        format!(
            "{}/proto/tendermint/abci/types.proto",
            tendermint_dir.display()
        ),
        format!(
            "{}/proto/tendermint/rpc/grpc/types.proto",
            tendermint_dir.display()
        ),
    ];
    // these paths will be 'clobbered' that is generated with grpc definitions by the tonic build
    // and then overwritten by the prost build when it generates files of the same name without
    // the service definitions. These files are specifically exempted from clobbering by renaming
    // and then restoring them.
    let proto_grpc_noclobber_paths = ["tendermint.abci.rs", "tendermint.rpc.grpc.rs"];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    collect_protos(&proto_paths, &mut protos);

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    // List available paths for services
    let proto_grpc_paths: Vec<PathBuf> = proto_grpc_paths.iter().map(PathBuf::from).collect();

    let mut config = prost_build::Config::default();
    config.out_dir(out_dir);

    // Compile all of the proto files, along with grpc service clients
    info!("Compiling proto definitions and clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .format(true)
        .out_dir(out_dir)
        .compile(&proto_grpc_paths, &includes)
        .unwrap();

    for i in proto_grpc_noclobber_paths {
        fs::rename(out_dir.join(i), out_dir.join(format!("{}.noclobber", i))).unwrap();
    }

    if let Err(e) = config.compile_protos(&protos, &includes) {
        eprintln!("[error] couldn't compile protos: {}", e);
        panic!("protoc failed!");
    }

    for i in proto_grpc_noclobber_paths {
        fs::rename(out_dir.join(format!("{}.noclobber", i)), out_dir.join(i)).unwrap();
    }

    info!("=> Done!");
}

fn compile_wasmd_proto_services(out_dir: impl AsRef<Path>) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sdk_dir = PathBuf::from(WASMD_DIR);

    let proto_includes_paths = [
        root.join("../proto"),
        sdk_dir.join("proto"),
        sdk_dir.join("third_party/proto"),
    ];

    // List available paths for dependencies
    let includes = proto_includes_paths
        .iter()
        .map(|p| p.as_os_str().to_os_string())
        .collect::<Vec<_>>();

    let proto_services_path = [
        sdk_dir.join("proto/cosmwasm/wasm/v1beta1/query.proto"),
        sdk_dir.join("proto/cosmwasm/wasm/v1beta1/tx.proto"),
    ];

    // List available paths for dependencies
    let services = proto_services_path
        .iter()
        .map(|p| p.as_os_str().to_os_string())
        .collect::<Vec<_>>();

    // Compile all proto client for GRPC services
    info!("Compiling wasmd proto clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .format(true)
        .out_dir(out_dir)
        .compile(&services, &includes)
        .unwrap();

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
        .format(true)
        .out_dir(out_dir)
        .extern_path(".tendermint", "crate::tendermint")
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
    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in EXCLUDED_PROTO_PACKAGES {
        if let Some(filename) = src.as_ref().file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let contents = fs::read_to_string(src)?;

    // `prost-build` output references types from `tendermint-proto` crate
    // relative paths, which we need to munge into `crate::tendermint`
    let contents = Regex::new(TENDERMINT_PROTO_REGEX)
        .unwrap()
        .replace_all(&contents, "crate::tendermint");

    // Patch each service definition with a feature attribute
    let patched_contents =
        contents.replace(TONIC_CLIENT_ATTRIBUTE, &GRPC_CLIENT_ATTRIBUTES.join("\n"));

    fs::write(dest, patched_contents)
}
