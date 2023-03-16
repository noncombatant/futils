use atty::Stream;
use nix::sys::stat::{stat, FileStat, Mode};
use serde::Serialize;
use std::fs::read_dir;
use std::io::{stdout, Error, Write};
use std::path::Path;
use users::{get_group_by_gid, get_user_by_uid};

use crate::shell::{parse_options, ShellResult};
use crate::time::format_utc_timestamp;
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

fn format_permissions(mode: u16) -> String {
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

#[derive(Serialize, Debug)]
struct Status<'a> {
    name: &'a str,
    size: i64,
    modified_time: String,
    user: String,
    group: String,
    permissions: String,
    links: u16,
    device: i32,
    inode: u64,
    accessed_time: String,
    changed_time: String,
    birth_time: String,
    // TODO: Use the non-permission bits into a `type: String` field.
    mode: u16,
    blocks: i64,
    block_size: i32,
}

impl<'a> Status<'a> {
    fn new(status: &FileStat, name: &'a str) -> Status<'a> {
        Status {
            name,
            size: status.st_size,
            modified_time: format_utc_timestamp(status.st_mtime),
            user: format_uid(status.st_uid),
            group: format_gid(status.st_gid),
            permissions: format_permissions(status.st_mode),
            links: status.st_nlink,
            device: status.st_dev,
            inode: status.st_ino,
            accessed_time: format_utc_timestamp(status.st_atime),
            changed_time: format_utc_timestamp(status.st_ctime),
            birth_time: format_utc_timestamp(status.st_birthtime),
            mode: status.st_mode,
            blocks: status.st_blocks,
            block_size: status.st_blksize,
        }
    }

    // TODO: This should take `output` as a `dyn io::Write`.
    fn write_columns(&self, field_delimiter: &[u8], record_delimiter: &[u8]) -> Result<(), Error> {
        let mut output = stdout();
        output.write_all(self.name.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(format!("{}", self.size).as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.modified_time.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.user.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.group.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.permissions.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(format!("{}", self.links).as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(format!("{}", self.device).as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(format!("{}", self.inode).as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.accessed_time.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.changed_time.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.birth_time.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(format!("{}", self.mode).as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(format!("{}", self.blocks).as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(format!("{}", self.block_size).as_bytes())?;
        output.write_all(record_delimiter)?;
        Ok(())
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

    let mut stdout = stdout();

    let mut status = 0;
    let count = arguments.len();
    if options.json && count != 1 {
        println!("[");
    } else if !options.json {
        // TODO: Document -O and -j
        let headers = vec![
            b"Name".as_slice(),
            b"Size".as_slice(),
            b"Modified".as_slice(),
            b"User".as_slice(),
            b"Group".as_slice(),
            b"Permissions".as_slice(),
            b"Links".as_slice(),
            b"Device".as_slice(),
            b"Inode".as_slice(),
            b"Accessed".as_slice(),
            b"Changed".as_slice(),
            b"Birth".as_slice(),
            b"Mode".as_slice(),
            b"Blocks".as_slice(),
            b"Block Size".as_slice(),
        ];
        stdout.write_all(&headers.join(options.output_field_delimiter.as_slice()))?;
        stdout.write_all(&options.output_record_delimiter)?;
    }
    for (i, pathname) in arguments.iter().enumerate() {
        match stat(pathname.as_str()) {
            Ok(s) => {
                let s = Status::new(&s, pathname);
                if options.json {
                    // TODO: Don't `unwrap` here; handle the error.
                    let json = to_json(&s).unwrap();
                    stdout.write_all(json.as_bytes())?;
                } else {
                    s.write_columns(
                        &options.output_field_delimiter,
                        &options.output_record_delimiter,
                    )?;
                }
                stdout.write_all(if options.json {
                    if i < count - 1 {
                        b","
                    } else {
                        b""
                    }
                } else {
                    &options.output_record_delimiter
                })?;
            }
            Err(e) => {
                eprintln!("{}: {}", pathname, e);
                status += 1
            }
        }
    }
    if options.json && count != 1 {
        println!("]");
    }
    Ok(status)
}
