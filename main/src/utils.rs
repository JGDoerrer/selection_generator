use std::process::Command;

/// Print the git sha of the last commit
/// the sha is equal to: git rev-parse --short --head
pub fn print_git_info() {
    println!();
    println!("Branch: {}", env!("GIT_BRANCH"));
    println!("SHA: {}", env!("GIT_HASH"));
    println!();
}

/// Print a subset of the information of lscpu.
/// Interesting values are:
/// Model name, Core, Thread, min MHz, max MHZ
pub fn print_lscpu() {
    let lscpu = Command::new("sh")
        .arg("-c")
        .arg("lscpu | grep -E 'Model Name|Core|Thread|min MHz|max MHz' | tr -s ' '")
        .output()
        .unwrap();
    let lscpu = String::from_utf8(lscpu.stdout).unwrap();

    println!("{}", lscpu);
    println!("===============");
}
