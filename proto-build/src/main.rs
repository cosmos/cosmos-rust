//! Build CosmosSDK/Tendermint/IBC proto files. This build script clones the CosmosSDK version
//! specified in the COSMOS_REV constant and then uses that to build the required
//! proto files for further compilation. This is based on the proto-compiler code
//! in github.com/informalsystems/ibc-rs

use git2::{Oid, Reference, Repository};
use std::{fs, fs::copy, fs::create_dir_all, path::Path, process};
use std::{fs::remove_dir_all, path::PathBuf};
use walkdir::WalkDir;

/// The Cosmos commit or tag to be cloned and used to build the proto files
const COSMOS_REV: &str = "v0.40.0-rc3";
/// The Cosmos sdk repo url, must be a publicly available git repo and a proper git clone string
const COSMOS_SDK_REPO: &str = "https://github.com/cosmos/cosmos-sdk";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated proto files go into in this repo
const COSMOS_SDK_PROTO_DIR: &str = "../cosmos-sdk-proto/src/prost/";
/// A temporary branch created in the cloned repo
const TMP_BRANCH: &str = "tmp-branch";
/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";
/// The temporary directory where cosmos-sdk is cloned into to read proto files from
const TMP_REPO_DIR: &str = "/tmp/tmp-cosmos-sdk/";

fn main() {
    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let tmp_repo_dir: PathBuf = TMP_REPO_DIR.parse().unwrap();
    let proto_dir: PathBuf = COSMOS_SDK_PROTO_DIR.parse().unwrap();

    if tmp_repo_dir.exists() {
        fs::remove_dir_all(tmp_repo_dir.clone()).unwrap();
    }
    if tmp_build_dir.exists() {
        fs::remove_dir_all(tmp_build_dir.clone()).unwrap();
    }

    println!("[info] Cloning cosmos/cosmos-sdk repository...");

    let repo = Repository::clone(COSMOS_SDK_REPO, &tmp_repo_dir).unwrap_or_else(|e| {
        println!("[error] Failed to clone the repository: {}", e);
        process::exit(1)
    });
    fs::create_dir(tmp_build_dir.clone()).unwrap();

    println!("[info] Cloned at '{}'", tmp_repo_dir.display());

    if let Ok(oid) = Oid::from_str(COSMOS_REV) {
        checkout_commit(&repo, oid).unwrap_or_else(|e| {
            println!("[error] Failed to checkout commit {}: {}", COSMOS_REV, e);
            process::exit(1)
        });
    } else if let Some(ref tag) = have_tag(&repo, COSMOS_REV) {
        checkout_tag(&repo, tag).unwrap_or_else(|e| {
            println!("[error] Failed to checkout tag {}: {}", COSMOS_REV, e);
            process::exit(1)
        });
    } else {
        panic!(
            "Failed to interpret {} as either a valid git tag or commit hash!",
            COSMOS_REV
        );
    }

    output_sdk_version(&tmp_build_dir);
    compile_protos(&tmp_repo_dir, &tmp_build_dir);
    compile_proto_services(&tmp_repo_dir, &tmp_build_dir);
    copy_generated_files(&tmp_build_dir, &proto_dir)
}

fn have_tag<'a>(repo: &'a Repository, tag_name: &str) -> Option<Reference<'a>> {
    // Find a tag with name `tag_name`
    // refactored from this code, the exact interpretation of flatten and flat_map
    // is hard to intuit.
    // repo.references()
    //     .unwrap()
    //     .into_iter()
    //     .flatten()
    //     .filter(|r| r.is_tag())
    //     .flat_map(|r| r.peel_to_tag())
    //     .find(|t| t.name() == Some(tag_name))
    for git_ref in repo.references().unwrap().into_iter() {
        if let Ok(git_ref) = git_ref {
            if git_ref.name() == Some(&format!("refs/tags/{}", tag_name)) {
                return Some(git_ref);
            }
        }
    }
    None
}

fn checkout_commit(repo: &Repository, oid: Oid) -> Result<(), git2::Error> {
    let commit = repo.find_commit(oid)?;

    // Create a new branch `rev` that points to `commit`
    repo.branch(TMP_BRANCH, &commit, true)?;

    // Checkout the newly created branch
    let treeish = format!("refs/heads/{}", TMP_BRANCH);
    let object = repo.revparse_single(&treeish)?;
    repo.checkout_tree(&object, None)?;
    repo.set_head(&treeish)?;

    Ok(())
}

fn checkout_tag(repo: &Repository, git_ref: &Reference) -> Result<(), git2::Error> {
    let rev = format!("refs/heads/{}", TMP_BRANCH);
    // Checkout the newly created branch
    let obj = repo.revparse_single(&git_ref.name().unwrap())?;

    // Get the commit this tag points to
    let commit = obj.as_commit().unwrap();

    // Create a new branch `tag_name` that points to `commit`
    repo.branch(TMP_BRANCH, &commit, true)?;
    repo.checkout_tree(&obj, None)?;
    repo.set_head(&rev)?;

    println!("[info] Checked out tag {}", TMP_BRANCH);

    Ok(())
}

fn output_sdk_version(out_dir: &Path) {
    let path = out_dir.join("COSMOS_SDK_COMMIT");
    std::fs::write(path, COSMOS_REV).unwrap();
}

fn compile_protos(sdk_dir: &Path, out_dir: &Path) {
    println!(
        "[info] Compiling .proto files to Rust into '{}'...",
        out_dir.display()
    );

    let root = env!("CARGO_MANIFEST_DIR");

    // Paths
    let proto_paths = [
        format!("{}/../proto/definitions/mock", root),
        format!("{}/proto/ibc", sdk_dir.display()),
        format!("{}/proto/cosmos/tx", sdk_dir.display()),
        format!("{}/proto/cosmos/bank", sdk_dir.display()),
        format!("{}/proto/cosmos/base", sdk_dir.display()),
        format!("{}/proto/cosmos/staking", sdk_dir.display()),
    ];

    let proto_includes_paths = [
        format!("{}/../proto", root),
        format!("{}/proto", sdk_dir.display()),
        format!("{}/third_party/proto", sdk_dir.display()),
    ];

    // List available proto files
    let mut protos: Vec<PathBuf> = vec![];
    for proto_path in &proto_paths {
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

    // List available paths for dependencies
    let includes: Vec<PathBuf> = proto_includes_paths.iter().map(PathBuf::from).collect();

    // Compile all proto files
    let mut config = prost_build::Config::default();
    config.out_dir(out_dir);
    config.extern_path(".tendermint", "::tendermint_proto");
    config.compile_protos(&protos, &includes).unwrap();
}

fn compile_proto_services(sdk_dir: impl AsRef<Path>, out_dir: impl AsRef<Path>) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sdk_dir = sdk_dir.as_ref().to_owned();

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
        sdk_dir.join("proto/cosmos/auth/v1beta1/query.proto"),
        sdk_dir.join("proto/cosmos/staking/v1beta1/query.proto"),
        sdk_dir.join("proto/cosmos/bank/v1beta1/query.proto"),
        sdk_dir.join("proto/cosmos/tx/v1beta1/service.proto"),
        sdk_dir.join("proto/cosmos/tx/v1beta1/tx.proto"),
    ];

    // List available paths for dependencies
    let services = proto_services_path
        .iter()
        .map(|p| p.as_os_str().to_os_string())
        .collect::<Vec<_>>();

    // Compile all proto client for GRPC services
    println!("[info ] Compiling proto clients for GRPC services!");
    tonic_build::configure()
        .build_client(true)
        .build_server(false)
        .format(false)
        .out_dir(out_dir)
        .compile(&services, &includes)
        .unwrap();

    println!("[info ] => Done!");
}

fn copy_generated_files(from_dir: &Path, to_dir: &Path) {
    println!(
        "[info ] Copying generated files into '{}'...",
        to_dir.display()
    );

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
            copy(e.path(), format!("{}/{}", to_dir.display(), &filename))
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            println!("[error] Error while copying compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}
