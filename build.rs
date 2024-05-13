use std::{io::Error, process::Command};
fn main() -> Result<(), Error> {
    let git_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()?;

    let git_hash = String::from_utf8(git_hash.stdout).unwrap();

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    let git_branch = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;

    let git_branch = String::from_utf8(git_branch.stdout).unwrap();

    println!("cargo:rustc-env=GIT_BRANCH={}", git_branch);

    Ok(())
}