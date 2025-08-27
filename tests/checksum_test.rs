/* SPDX-FileCopyrightText: © 2025 Decompollaborate */
/* SPDX-License-Identifier: MIT */

use ipl3checksum::{calculate_checksum, CICKind, Ipl3ChecksumError};
use std::fs;

#[test]
fn test_dummy_files() -> Result<(), Ipl3ChecksumError> {
    for path_result in fs::read_dir("tests/dummytests").unwrap() {
        let ipl3_folder = path_result.unwrap();
        let folder_name = ipl3_folder.file_name();

        println!("{:?}", folder_name);

        let kind = CICKind::from_name(folder_name.to_str().unwrap()).unwrap();
        println!("CIC Kind: {:?}", kind);

        for bin_path_result in fs::read_dir(ipl3_folder.path()).unwrap() {
            let bin_path = bin_path_result.unwrap();

            println!("{:?}", bin_path);

            println!("    Reading...");

            let bin_bytes = fs::read(bin_path.path()).unwrap();

            println!("    Calculating checksum...");
            let checksum = calculate_checksum(&bin_bytes, kind).unwrap();
            println!("Used CIC Kind: {:?}", kind);

            println!(
                "    Calculated checksum is: 0x{:08X} 0x{:08X}",
                checksum.0, checksum.1
            );

            println!("    Checking checksum...");
            let bin_checksum_0 = read_u32(&bin_bytes, 0x10);
            let bin_checksum_1 = read_u32(&bin_bytes, 0x14);

            println!(
                "    Expected checksum is: 0x{:08X} 0x{:08X}",
                bin_checksum_0, bin_checksum_1
            );

            assert_eq!(checksum.0, bin_checksum_0);
            assert_eq!(checksum.1, bin_checksum_1);

            println!("    {:?} OK", bin_path);

            println!();
        }

        println!();
    }
    Ok(())
}

fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    assert_eq!(offset % 4, 0);
    assert!(offset + 4 <= bytes.len());

    u32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap())
}
