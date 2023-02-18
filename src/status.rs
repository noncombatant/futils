use getopt::Opt;
use glob::glob;
use nix::sys::stat::{stat, FileStat};
use serde::Serialize;
use users::{get_group_by_gid, get_user_by_uid};

use crate::time::utc_timestamp_to_string;
use crate::util::{help, ShellResult};

const HELP_MESSAGE: &str = "status - print the status of files

Usage:

    status -h
    status pathname [...]

Prints the metadata for each of the given *pathname*s.

Additional options:

    -h  Print this help message.";

fn format_uid(uid: u32) -> String {
    match get_user_by_uid(uid) {
        Some(s) => format!("{}", s.name().to_string_lossy()),
        None => format!("{}", uid),
    }
}

fn format_gid(gid: u32) -> String {
    match get_group_by_gid(gid) {
        Some(s) => format!("{}", s.name().to_string_lossy()),
        None => format!("{}", gid),
    }
}

#[derive(Serialize, Debug)]
struct Status<'a> {
    name: &'a str,
    device: i32,
    mode: u16,
    links: u16,
    inode: u64,
    user: String,
    group: String,
    accessed_time: String,
    modified_time: String,
    changed_time: String,
    birth_time: String,
    size: i64,
    blocks: i64,
    block_size: i32,
}

impl<'a> Status<'a> {
    fn new(status: &FileStat, name: &'a str) -> Status<'a> {
        Status {
            name,
            device: status.st_dev,
            mode: status.st_mode,
            links: status.st_nlink,
            inode: status.st_ino,
            user: format_uid(status.st_uid),
            group: format_gid(status.st_gid),
            accessed_time: utc_timestamp_to_string(status.st_atime),
            changed_time: utc_timestamp_to_string(status.st_ctime),
            modified_time: utc_timestamp_to_string(status.st_mtime),
            birth_time: utc_timestamp_to_string(status.st_birthtime),
            size: status.st_size,
            blocks: status.st_blocks,
            block_size: status.st_blksize,
        }
    }
}

pub fn status_main(arguments: &[String]) -> ShellResult {
    let mut options = getopt::Parser::new(arguments, "h");
    loop {
        match options.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('h', None) => help(0, HELP_MESSAGE),
                _ => help(-1, HELP_MESSAGE),
            },
        }
    }
    let (_, arguments) = arguments.split_at(options.index());

    let arguments = if arguments.is_empty() {
        // TODO: This should really be `read_dir` instead of `glob`. Also, the
        // horrendousness of this chunk of code highlights that we have a type
        // problem — we're doing a lot of work to turn `OsStr`s into `String`s
        // when maybe everything should stay `OsStr`? Or at least, there's got
        // to be a cleaner way to do all this.
        let paths = glob("*").unwrap();
        paths
            .map(|p| p.unwrap().as_os_str().to_string_lossy().into())
            .collect()
    } else {
        Vec::from(arguments)
    };

    let mut errors = 0;
    println!("[");
    for (i, pathname) in arguments.iter().enumerate() {
        match stat(pathname.as_str()) {
            Ok(status) => {
                let status = Status::new(&status, pathname);
                let status = serde_json::to_string(&status).unwrap();
                println!(
                    "{}{}",
                    status,
                    if i < arguments.len() - 1 { "," } else { "" }
                );
            }
            Err(e) => {
                eprintln!("{}: {}", pathname, e);
                errors += 1
            }
        }
    }
    println!("]");
    Ok(errors)
}
