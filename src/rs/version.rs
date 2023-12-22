/* SPDX-FileCopyrightText: © 2023 Decompollaborate */
/* SPDX-License-Identifier: MIT */

// Version should be synced with pyproject.toml, Cargo.toml and src/ipl3checksum/__init__.py
pub static VERSION_MAJOR: i32 = 1;
pub static VERSION_MINOR: i32 = 1;
pub static VERSION_PATCH: i32 = 1;

pub static VERSION_INFO: (i32, i32, i32) = (VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH);

// TODO: figure out a way to construct this string by using VERSION_MAJOR, VERSION_MINOR and VERSION_PATCH (concat! and stringify! didn't work)
pub static VERSION_STR: &str = "1.1.1";

pub static AUTHOR: &str = "Decompollaborate";

#[cfg(feature = "c_bindings")]
mod c_bindings {
    #[no_mangle]
    static ipl3checksum_version_major: i32 = super::VERSION_MAJOR;
    #[no_mangle]
    static ipl3checksum_version_minor: i32 = super::VERSION_MINOR;
    #[no_mangle]
    static ipl3checksum_version_patch: i32 = super::VERSION_PATCH;

    // TODO: construct this from super::VERSION_STR
    #[no_mangle]
    static ipl3checksum_version_str: &[u8] = b"1.1.1\0";

    // TODO: construct this from super::AUTHOR
    #[no_mangle]
    static ipl3checksum_version_author: &[u8] = b"Decompollaborate\0";
}
