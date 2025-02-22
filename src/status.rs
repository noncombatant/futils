// Copyright 2022 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

//! The `futils status` command.

use std::fs::read_dir;
use std::io::{Write, stdout};
use std::path::Path;

use atty::Stream;
use nix::sys::stat::{FileStat, Mode, lstat};
use users::{get_group_by_gid, get_user_by_uid};

use crate::os;
use crate::shell::{EmptyResult, Options, ShellResult, parse_options};
use crate::time::format_utc_timestamp;
use crate::util::{exit_with_result, help};

pub const STATUS_HELP: &str = include_str!("status.md");
pub const STATUS_HELP_VERBOSE: &str = include_str!("status_verbose.md");

fn format_uid(uid: u32) -> String {
    get_user_by_uid(uid).map_or(format!("{uid}"), |s| s.name().to_string_lossy().to_string())
}

fn format_gid(gid: u32) -> String {
    get_group_by_gid(gid).map_or(format!("{gid}"), |s| s.name().to_string_lossy().to_string())
}

const fn get_permissions(mode: os::Mode) -> Option<Mode> {
    Mode::from_bits(
        mode & (Mode::S_IRWXU.bits()
            | Mode::S_IRWXG.bits()
            | Mode::S_IRWXO.bits()
            | Mode::S_ISUID.bits()
            | Mode::S_ISGID.bits()
            | Mode::S_ISVTX.bits()),
    )
}

fn format_permissions(mode: os::Mode) -> String {
    get_permissions(mode).map_or("---------".to_string(), |mode| {
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
        if mode.contains(Mode::S_ISUID) {
            bytes[2] = b'S';
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
        if mode.contains(Mode::S_ISGID) {
            bytes[5] = b'S';
        }
        if mode.contains(Mode::S_IROTH) {
            bytes[6] = b'r';
        }
        if mode.contains(Mode::S_IWOTH) {
            bytes[7] = b'w';
        }
        if mode.contains(Mode::S_IXOTH) {
            bytes[8] = if mode.contains(Mode::S_ISVTX) {
                b'T'
            } else {
                b'x'
            }
        }
        String::from_utf8(bytes).map_or(String::new(), |v| v)
    })
}

fn format_type(mode: os::Mode) -> String {
    // Darwin's `stat`(2) says:
    // #define        S_IFMT   0170000  /* type of file */
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

    static S_IFMT: u16 = 0o0_170_000;
    static S_IFSOCK: u16 = 0o0_140_000;
    static S_IFLNK: u16 = 0o0_120_000;
    static S_IFREG: u16 = 0o0_100_000;
    static S_IFBLK: u16 = 0o0_060_000;
    static S_IFDIR: u16 = 0o0_040_000;
    static S_IFCHR: u16 = 0o0_020_000;
    static S_IFIFO: u16 = 0o0_010_000;
    static S_ISUID: u16 = 0o0_004_000;
    static S_ISGID: u16 = 0o0_002_000;

    // From ls(1):
    //
    // -F    Display a slash (‘/’) immediately after each pathname that is a
    //       directory, an asterisk (‘*’) after each that is executable, an at
    //       sign (‘@’) after each symbolic link, an equals sign (‘=’) after
    //       each socket, a percent sign (‘%’) after each whiteout, and a
    //       vertical bar (‘|’) after each that is a FIFO.

    let permissions = get_permissions(mode);
    let setuid = (mode & S_ISUID as os::Mode) != 0;
    let setgid = (mode & S_ISGID as os::Mode) != 0;
    let mode = mode & S_IFMT as os::Mode;
    let r = if setuid || setgid {
        "!"
    } else if S_IFIFO as os::Mode == mode & S_IFIFO as os::Mode {
        "|"
    } else if S_IFLNK as os::Mode == mode & S_IFLNK as os::Mode {
        "@"
    } else if S_IFBLK as os::Mode == mode & S_IFBLK as os::Mode {
        "b"
    } else if S_IFCHR as os::Mode == mode & S_IFCHR as os::Mode {
        "c"
    } else if S_IFDIR as os::Mode == mode & S_IFDIR as os::Mode {
        "d"
    } else if S_IFREG as os::Mode == mode & S_IFREG as os::Mode {
        permissions.map_or("?", |m| {
            if m.contains(Mode::S_IXUSR) || m.contains(Mode::S_IXGRP) || m.contains(Mode::S_IXOTH) {
                "*"
            } else {
                "-"
            }
        })
    } else if S_IFSOCK as os::Mode == mode & S_IFSOCK as os::Mode {
        "="
    } else {
        "?"
    };
    r.to_string()
}

impl<'a> os::Status<'a> {
    fn new(status: &FileStat, name: &'a str) -> Self {
        os::Status {
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
            #[cfg(target_os = "macos")]
            birth_time: format_utc_timestamp(status.st_birthtime),
            mode: status.st_mode,
            blocks: status.st_blocks,
            block_size: status.st_blksize,
        }
    }

    fn write_columns(&self, output: &mut dyn Write, options: &Options) -> EmptyResult {
        if options.verbose {
            self.write_columns_verbose(output, &options.output_field_delimiter)
        } else {
            self.write_columns_concise(output, &options.output_field_delimiter)
        }
    }

    fn write_columns_concise(&self, output: &mut dyn Write, field_delimiter: &[u8]) -> EmptyResult {
        output.write_all(self.file_type.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.permissions.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.user.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.group.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(format!("{:>9}", self.size).as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.modified_time.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.name.as_bytes())?;
        Ok(())
    }

    fn write_columns_verbose(&self, output: &mut dyn Write, field_delimiter: &[u8]) -> EmptyResult {
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
        output.write_all(format!("{:>3}", self.links).as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(format!("{:>4}", self.device).as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(format!("{:>6}", self.inode).as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.accessed_time.as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.changed_time.as_bytes())?;
        output.write_all(field_delimiter)?;
        #[cfg(target_os = "macos")]
        {
            output.write_all(self.birth_time.as_bytes())?;
            output.write_all(field_delimiter)?;
        }
        output.write_all(format!("{:>9}", self.mode).as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(format!("{}", self.blocks).as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(format!("{}", self.block_size).as_bytes())?;
        output.write_all(field_delimiter)?;
        output.write_all(self.name.as_bytes())?;
        Ok(())
    }

    fn write_json(&self, output: &mut dyn Write, pretty: bool) -> EmptyResult {
        let to_json = if pretty {
            serde_json::to_writer_pretty
        } else {
            serde_json::to_writer
        };
        Ok(to_json(output, &self)?)
    }
}

/// Runs the `status` command on `arguments`.
pub fn status_main(arguments: &[String]) -> ShellResult {
    let (options, arguments) = parse_options(arguments)?;
    if options.help {
        exit_with_result(help(
            0,
            STATUS_HELP,
            true,
            if options.verbose {
                Some(STATUS_HELP_VERBOSE)
            } else {
                None
            },
        ));
    }

    let arguments = if arguments.is_empty() {
        // TODO: This crunchy code highlights that we have a type problem
        // — we're doing a lot of work to turn `OsStr`s into `String`s, but
        // ultimately we'll be changing CLI arguments to be always `OsString`.
        // See also the conversion code at the top of `main`.
        read_dir(Path::new("."))?
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into())
            .filter(|name: &String| options.show_all || !name.starts_with('.'))
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
                b"Name".as_slice(),
            ]
        } else {
            vec![
                b"Type".as_slice(),
                b"Permissions".as_slice(),
                b"User".as_slice(),
                b"Group".as_slice(),
                b"     Size".as_slice(),
                b"Modified           ".as_slice(),
                b"Name".as_slice(),
            ]
        };
        stdout.write_all(&headers.join(options.output_field_delimiter.as_slice()))?;
        stdout.write_all(&options.output_record_delimiter)?;
    }
    for (i, pathname) in arguments.iter().enumerate() {
        match lstat(pathname.as_str()) {
            Ok(s) => {
                let s = os::Status::new(&s, pathname);
                if options.json_output {
                    s.write_json(&mut stdout, atty::is(Stream::Stdout))?;
                } else {
                    s.write_columns(&mut stdout, &options)?;
                }
                stdout.write_all(if options.json_output {
                    if i < count - 1 { b",\n" } else { b"\n" }
                } else {
                    &options.output_record_delimiter
                })?;
            }
            Err(error) => {
                eprintln!("{pathname}: {error}");
                status += 1;
            }
        }
    }
    if options.json_output && count != 1 {
        println!("]");
    }
    Ok(status)
}
