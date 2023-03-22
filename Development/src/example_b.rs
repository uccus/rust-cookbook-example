use std::process::Command;
use semver::{Version, VersionReq};

pub fn test(){
    let output = Command::new("git")
        .arg("--version")    
        .output().unwrap();
    let output = String::from_utf8(output.stdout).unwrap();
    println!("{}", output);
    let version = output.split(" ").last().unwrap();
    println!("{}", version);

    let version = "2.3.1";
    let parsed_version = Version::parse(version).unwrap();
    println!("{}", parsed_version);
}