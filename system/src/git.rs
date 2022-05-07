use std::process::Command;
use std::str::FromStr;

use crate::hack;

pub fn log() -> hack::Result<std::collections::HashMap<String, Vec<u64>>> {
    let args = vec![
        "log",
        "--format=format:XX\t%H\t%ct",
        "--name-status",
        "--reverse",
    ];
    let git_log = Command::new("git").args(args).output();
    let stdout_vec = git_log?.stdout;
    let stdout = std::str::from_utf8(&stdout_vec)?;
    let lines = stdout.split("\n");

    let mut files = std::collections::HashMap::<String, Vec<u64>>::new();

    let mut dt: Option<u64> = None;
    for line in lines {
        let columns: Vec<&str> = line.split("\t").collect();
        if columns.len() > 2 && columns[0] == "XX" {
            dt = Some(u64::from_str(columns[2])?);
        } else if columns.len() == 2 {
            let name = columns[1].to_string();
            files.entry(name).or_insert(vec![]).push(dt.unwrap());
        } else if columns.len() == 3 {
            let from = columns[1].to_string();
            let to = columns[2].to_string();
            let past_commits = files.get(&from).unwrap_or(&vec![]).to_vec();
            files.insert(to, past_commits);
        } else {
            dt = None;
        }
    }

    Ok(files)
}
