//! The `futils status` command.

use std::fs::read_dir;
use std::io::{stdout, Write};
use std::path::Path;

use atty::Stream;
use nix::sys::stat::{stat, FileStat, Mode};
use serde::Serialize;
use users::{get_group_by_gid, get_user_by_uid};

use crate::shell::{parse_options, Options, ShellError, ShellResult};
use crate::time::format_utc_timestamp;
use crate::util::help;

/// Command line usage help.
pub(crate) const STATUS_HELP: &str = include_str!("status_help.md");

pub(crate) const STATUS_HELP_VERBOSE: &str = include_str!("status_help_verbose.md");

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

fn format_type(mode: u16) -> String {
    // Darwin's `stat`(2) says:
    // #define S_IFMT 0170000           /* type of file */
    // #define        S_IFIFO  0010000  /* named pipe (fifo) */
    // #define        S_IFCHR  0020000  /* character special */
    // #define        S_IFDIR  0040000  /* directory */
    // #define        S_IFBLK  0060000  /* block special */
    // #define        S_IFREG  0100000  /* regular */
    // #define        S_IFLNK  0120000  /* symbolic link */
    // #define        S_IFSOCK 0140000  /* socket */
    // #define        S_IFWHT  0160000  /* whiteout */
    //
    // https://github.com/torvalds/linux/blob/master/include/uapi/linux/stat.h:
    // #define S_IFMT    00170000
    // #define S_IFSOCK   0140000
    // #define S_IFLNK    0120000
    // #define S_IFREG    0100000
    // #define S_IFBLK    0060000
    // #define S_IFDIR    0040000
    // #define S_IFCHR    0020000
    // #define S_IFIFO    0010000
    // #define S_ISUID    0004000
    // #define S_ISGID    0002000
    // #define S_ISVTX    0001000
    //
    // So, close enough for horseshoes (and hand grenades).

    static S_IFMT: u16 = 0o0170000;
    static S_IFSOCK: u16 = 0o0140000;
    static S_IFLNK: u16 = 0o0120000;
    static S_IFREG: u16 = 0o0100000;
    static S_IFBLK: u16 = 0o0060000;
    static S_IFDIR: u16 = 0o0040000;
    static S_IFCHR: u16 = 0o0020000;
    static S_IFIFO: u16 = 0o0010000;
    static S_ISUID: u16 = 0o0004000;
    static S_ISGID: u16 = 0o0002000;

    let mode = mode & S_IFMT;
    let r = if S_ISUID == mode & S_ISUID || S_ISGID == mode & S_ISGID {
        "💣"
    } else if S_IFIFO == mode & S_IFIFO {
        "🚰"
    } else if S_IFBLK == mode & S_IFBLK || S_IFCHR == mode & S_IFCHR {
        "🐧"
    } else if S_IFDIR == mode & S_IFDIR {
        "📁"
    } else if S_IFREG == mode & S_IFREG {
        " "
    } else if S_IFLNK == mode & S_IFLNK {
        "→"
    } else if S_IFSOCK == mode & S_IFSOCK {
        "🧦"
    } else {
        "⁉️"
    };
    r.to_string()
}

#[derive(Serialize)]
struct Status<'a> {
    name: &'a str,
    file_type: String,
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
    mode: u16,
    blocks: i64,
    block_size: i32,
}

impl<'a> Status<'a> {
    fn new(status: &FileStat, name: &'a str) -> Status<'a> {
        Status {
            name,
            file_type: format_type(status.st_mode),
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

    fn write_columns(&self, output: &mut dyn Write, options: &Options) -> Result<(), ShellError> {
        if options.verbose {
            self.write_columns_verbose(output, &options.output_field_delimiter)
        } else {
            self.write_columns_concise(output, &options.output_field_delimiter)
        }
    }

    fn write_columns_concise(
        &self,
        output: &mut dyn Write,
        field_delimiter: &[u8],
    ) -> Result<(), ShellError> {
        output.write_all(self.file_type.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.permissions.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.user.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.group.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(format!("{}", self.size).as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.modified_time.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.name.as_bytes())?;
        output.write_all(field_delimiter)?;
        Ok(())
    }

    fn write_columns_verbose(
        &self,
        output: &mut dyn Write,
        field_delimiter: &[u8],
    ) -> Result<(), ShellError> {
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
        output.write_all(self.file_type.as_bytes())?;
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
        Ok(())
    }

    fn write_json(&self, output: &mut dyn Write, pretty: bool) -> Result<(), ShellError> {
        let to_json = if pretty {
            serde_json::to_writer_pretty
        } else {
            serde_json::to_writer
        };
        Ok(to_json(output, &self)?)
    }
}

/// Runs the `status` command on `arguments`.
pub(crate) fn status_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        help(
            0,
            STATUS_HELP,
            if options.verbose {
                Some(STATUS_HELP_VERBOSE)
            } else {
                None
            },
        );
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

    let mut stdout = stdout();
    let mut status = 0;
    let count = arguments.len();
    if options.json_output && count != 1 {
        println!("[");
    } else if !options.json_output {
        let headers = if options.verbose {
            vec![
                b"Name".as_slice(),
                b"Size".as_slice(),
                b"Modified".as_slice(),
                b"User".as_slice(),
                b"Group".as_slice(),
                b"Type".as_slice(),
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
            ]
        } else {
            vec![
                b"Type".as_slice(),
                b"Permissions".as_slice(),
                b"User".as_slice(),
                b"Group".as_slice(),
                b"Size".as_slice(),
                b"Modified           ".as_slice(),
                b"Name".as_slice(),
            ]
        };
        stdout.write_all(&headers.join(options.output_field_delimiter.as_slice()))?;
        stdout.write_all(&options.output_record_delimiter)?;
    }
    for (i, pathname) in arguments.iter().enumerate() {
        match stat(pathname.as_str()) {
            Ok(s) => {
                let s = Status::new(&s, pathname);
                if options.json_output {
                    s.write_json(&mut stdout, atty::is(Stream::Stdout))?;
                } else {
                    s.write_columns(&mut stdout, &options)?;
                }
                stdout.write_all(if options.json_output {
                    if i < count - 1 {
                        b",\n"
                    } else {
                        b"\n"
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
    if options.json_output && count != 1 {
        println!("]");
    }
    Ok(status)
}
