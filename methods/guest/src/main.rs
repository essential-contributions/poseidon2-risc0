use risc0_zkvm::guest::env;

use core::ptr::null_mut;
use risc0_zkvm_platform::syscall::{sys_poseidon2, DIGEST_WORDS};

fn main() {
    // Copied directly from risc0/risc0/zkvm/methods/guest/src/bin/multi_test.rs (v3.0.3)
    let input: &[u32; 8] = &[
        0x10000, 0x30002, 0x50004, 0x70006, 0x90008, 0xB000A, 0xD000C, 0xF000E,
    ];
    let expected: &[u32] = &[
        1749308481, 879447913, 499502012, 1842374203, 1869354733, 71489094, 19273002,
        690566044,
    ];
    let mut actual: [u32; DIGEST_WORDS] = [0u32; 8];
    unsafe {
        sys_poseidon2(null_mut(), input.as_ptr() as *const u8, &mut actual, 1u32);
    }
    assert_eq!(expected, actual);
}
