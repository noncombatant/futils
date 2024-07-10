// Copyright 2023 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

pub type LinkCount = u16;
pub type DeviceNumber = i32;
pub type Mode = u16;
pub type BlockSize = i32;

#[derive(Serialize)]
pub struct Status<'a> {
    pub name: &'a str,
    pub file_type: String,
    pub size: i64,
    pub modified_time: String,
    pub user: String,
    pub group: String,
    pub permissions: String,
    pub links: LinkCount,
    pub device: DeviceNumber,
    pub inode: u64,
    pub accessed_time: String,
    pub changed_time: String,
    pub birth_time: String,
    pub mode: Mode,
    pub blocks: i64,
    pub block_size: BlockSize,
}
