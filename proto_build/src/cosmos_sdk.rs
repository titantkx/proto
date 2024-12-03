//! Note: The following code was largely taken from github.com/cosmos/cosmos-rust/cosmos-sdk-proto and modified for
//! use in Gravity Bridge and Althea-Chain.
//! Some unneded features have been removed, like auto checkout and wasm proto compliation.
//!
//! Build CosmosSDK/Tendermint/IBC proto files. This build script clones the CosmosSDK version
//! specified in the COSMOS_SDK_REV constant and then uses that to build the required
//! proto files for further compilation. This is based on the proto-compiler code
//! in github.com/informalsystems/ibc-rs

use std::path::Path;

use crate::{compile_protos, RegexReplace};

pub struct RootDirs {
    pub cosmos: String,
    pub tendermint: String,
    pub ibc: String,
}

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto` or `cosmos-sdk-proto`)
/// "gogoproto", "google", "cosmos_proto"
pub const EXCLUDED_PROTO_PACKAGES: &[&str] = &[];
/// Regex fixes for super::[super::, ...]cosmos and similarly for tendermint
pub const COSMOS_SDK_PROTO_REGEX: &str = "(super::)+cosmos";
pub const COSMOS_SDK_PROTO_REPLACE: &str = "crate::cosmos";
pub const TENDERMINT_PROTO_REGEX: &str = "(super::)+tendermint";
pub const TENDERMINT_PROTO_REPLACE: &str = "crate::tendermint";

pub const COSMOS_REGEX_REPLACE: RegexReplace =
    RegexReplace::new(COSMOS_SDK_PROTO_REGEX, COSMOS_SDK_PROTO_REPLACE);
pub const TENDERMINT_REGEX_REPLACE: RegexReplace =
    RegexReplace::new(TENDERMINT_PROTO_REGEX, TENDERMINT_PROTO_REPLACE);

/// Initiates the compilation of the cosmos-sdk, tendermint, ibc, and bech32-ibc protos
pub fn cosmos_main(roots: RootDirs, tmp_dir: &str, out_dir: &str) {
    // Make sure that imports are fixed when compiling
    // * old logic use  [COSMOS_REGEX_REPLACE, TENDERMINT_REGEX_REPLACE]
    let regex_replacements = vec![];
    // Note that this order is very important, as any project with the potential to clobber compiled rust files will clobber another project
    // this can easily be avoided by splitting each project into its own directory + crate, but for historical reasons it is being left in
    // the manner cosmos-sdk-proto performs this work (for now)

    compile_ibc_protos_and_services(
        Path::new(&roots.ibc),
        Path::new(tmp_dir),
        Path::new(out_dir),
        &regex_replacements,
    );

    // TODO: Split each project off into its own compilation directory + crate
    compile_sdk_protos_and_services(
        Path::new(&roots.cosmos),
        Path::new(tmp_dir),
        Path::new(out_dir),
        &regex_replacements,
    );
}

fn compile_sdk_protos_and_services(
    root: &Path,
    tmp_path: &Path,
    out_path: &Path,
    regex_replacements: &[RegexReplace],
) {
    info!(
        "Compiling cosmos-sdk .proto files to Rust into '{}'...",
        out_path.display(),
    );

    let proto_path = root.join("proto");

    let proto_include_paths = [];

    let mut additional_replacements = vec![
        RegexReplace::new("enum Validators", "enum Policy"),
        RegexReplace::new(
            "stake_authorization::Validators",
            "stake_authorization::Policy",
        ),
    ];

    // Concatenate the original replacements with the additional ones
    let mut all_replacements = regex_replacements.to_vec();
    all_replacements.append(&mut additional_replacements);

    compile_protos(crate::CompileArgs {
        proto_path: &proto_path,
        proto_include_paths: &proto_include_paths,
        replacements: &all_replacements,
        exclusions: EXCLUDED_PROTO_PACKAGES,
        tmp_path,
        out_path,
        clean_tmp: false,
        clean_out: false,
    });

    info!("=> Done!");
}

fn compile_ibc_protos_and_services(
    root: &Path,
    tmp_path: &Path,
    out_path: &Path,
    regex_replacements: &[RegexReplace],
) {
    info!(
        "Compiling ibc .proto files to Rust into '{}'...",
        out_path.display()
    );

    let proto_path = root.join("proto");

    let proto_include_paths = [];

    compile_protos(crate::CompileArgs {
        proto_path: &proto_path,
        proto_include_paths: &proto_include_paths,
        replacements: regex_replacements,
        exclusions: EXCLUDED_PROTO_PACKAGES,
        tmp_path,
        out_path,
        clean_tmp: true,
        clean_out: true,
    });
}
