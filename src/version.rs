use crate::shell::ShellResult;

/// Command line usage help.
pub(crate) const VERSION_HELP_MESSAGE: &str = include_str!("version_help.md");

// TODO: Add version to main_help.md

// TODO: Do all the variables listed in https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-crates

pub(crate) fn version_main(_: &[String]) -> ShellResult {
    println!("Manifest directory: {}", env!("CARGO_MANIFEST_DIR"));
    println!("Authors: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Description: {}", env!("CARGO_PKG_DESCRIPTION"));
    println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
    println!("Name: {}", env!("CARGO_PKG_NAME"));
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Version (major): {}", env!("CARGO_PKG_VERSION_MAJOR"));
    println!("Version (minor): {}", env!("CARGO_PKG_VERSION_MINOR"));
    println!("Version (patch): {}", env!("CARGO_PKG_VERSION_PATCH"));
    println!("Version (pre): {}", env!("CARGO_PKG_VERSION_PRE"));
    Ok(0)
}
