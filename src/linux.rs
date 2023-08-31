// Copyright 2023 by [Chris Palmer](https://noncombatant.org)
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

pub(crate) type LinkCount = u64;
pub(crate) type DeviceNumber = u64;
pub(crate) type Mode = u32;
pub(crate) type BlockSize = i64;

#[derive(Serialize)]
pub(crate) struct Status<'a> {
    pub(crate) name: &'a str,
    pub(crate) file_type: String,
    pub(crate) size: i64,
    pub(crate) modified_time: String,
    pub(crate) user: String,
    pub(crate) group: String,
    pub(crate) permissions: String,
    pub(crate) links: LinkCount,
    pub(crate) device: DeviceNumber,
    pub(crate) inode: u64,
    pub(crate) accessed_time: String,
    pub(crate) changed_time: String,
    pub(crate) mode: Mode,
    pub(crate) blocks: i64,
    pub(crate) block_size: BlockSize,
}
