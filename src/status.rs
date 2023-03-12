use atty::Stream;
use nix::sys::stat::{stat, FileStat, Mode};
use serde::Serialize;
use std::fs::read_dir;
use std::path::Path;
use users::{get_group_by_gid, get_user_by_uid};

use crate::shell::{parse_options, ShellResult};
use crate::time::utc_timestamp_to_string;
use crate::util::help;

/// Command line usage help.
pub(crate) const STATUS_HELP_MESSAGE: &str = include_str!("status_help.md");

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
    permissions: String,
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

fn permissions_string(mode: u16) -> String {
    let mode = Mode::from_bits(
        mode & (Mode::S_IRWXU.bits() | Mode::S_IRWXG.bits() | Mode::S_IRWXO.bits()),
    )
    .unwrap();
    let mut bytes = vec![b'-'; 9];
    if mode.contains(Mode::S_IRUSR) {
        bytes[0] = b'r';
    }
    if mode.contains(Mode::S_IWUSR) {
        bytes[1] = b'w';
    }
    if mode.contains(Mode::S_IXUSR) {
        bytes[2] = b'x';
    }
    if mode.contains(Mode::S_IRGRP) {
        bytes[3] = b'r';
    }
    if mode.contains(Mode::S_IWGRP) {
        bytes[4] = b'w';
    }
    if mode.contains(Mode::S_IXGRP) {
        bytes[5] = b'x';
    }
    if mode.contains(Mode::S_IROTH) {
        bytes[6] = b'r';
    }
    if mode.contains(Mode::S_IWOTH) {
        bytes[7] = b'w';
    }
    if mode.contains(Mode::S_IXOTH) {
        bytes[8] = b'x';
    }
    String::from_utf8(bytes).unwrap()
}

impl<'a> Status<'a> {
    fn new(status: &FileStat, name: &'a str) -> Status<'a> {
        Status {
            name,
            device: status.st_dev,
            mode: status.st_mode,
            permissions: permissions_string(status.st_mode),
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

/// Runs the `status` command on `arguments`.
pub(crate) fn status_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(0, STATUS_HELP_MESSAGE);
    }

    let arguments = if arguments.is_empty() {
        // TODO: This crunchy code highlights that we have a type problem
        // — we're doing a lot of work to turn `OsStr`s into `String`s, but
        // ultimately we'll be changing CLI arguments to be always `OsString`.
        // See also the conversion code at the top of `main`.
        read_dir(Path::new("."))?
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into())
            .collect()
    } else {
        Vec::from(arguments)
    };

    let to_json = if atty::is(Stream::Stdout) {
        serde_json::to_string_pretty
    } else {
        serde_json::to_string
    };

    let mut status = 0;
    let count = arguments.len();
    if count != 1 {
        println!("[");
    }
    for (i, pathname) in arguments.iter().enumerate() {
        match stat(pathname.as_str()) {
            Ok(s) => {
                let s = Status::new(&s, pathname);
                let s = to_json(&s).unwrap();
                println!("{}{}", s, if i < count - 1 { "," } else { "" });
            }
            Err(e) => {
                eprintln!("{}: {}", pathname, e);
                status += 1
            }
        }
    }
    if count != 1 {
        println!("]");
    }
    Ok(status)
}
