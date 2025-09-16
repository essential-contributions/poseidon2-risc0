# poseidon2-risc0
A simple test for running the poseidon2 hash function inside the risc0 guest.

This repo follows the template of a new risc0 project exactly exactly.
```
cargo risczero new poseidon2-risc0 --guest-name test_poseidon2_guest
```

The only code modified is in `methods/guest/src/main.rs`, which copies the code found directly from `risc0/risc0/zkvm/methods/guest/src/bin/multi_test.rs` (v3.0.3):
```
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
```

When trying to run `RUSTFLAGS="-C target-cpu=native" cargo run --release`, the following error occurs:
```
set(row: 16784, col: 31, val: 0x00020036) cur: 0x000800d8

thread 'main' panicked at host/src/main.rs:39:10:
called `Result::unwrap()` on an `Err` value: rx len failed

Caused by:
    failed to fill whole buffer
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

When running in debug (`RISC0_DEV_MODE=1 RUST_LOG=info RISC0_INFO=1 cargo run --release`), there is no error:
```
WARNING: proving in dev mode. This will not generate valid, secure proofs.
2025-09-16T22:37:28.100796Z  INFO risc0_zkvm::host::server::exec::executor: execution time: 2.25731ms
2025-09-16T22:37:28.100835Z  INFO risc0_zkvm::host::server::session: number of segments: 1
2025-09-16T22:37:28.100839Z  INFO risc0_zkvm::host::server::session: 32768 total cycles
2025-09-16T22:37:28.100841Z  INFO risc0_zkvm::host::server::session: 3828 user cycles (11.68%)
2025-09-16T22:37:28.100865Z  INFO risc0_zkvm::host::server::session: 19201 paging cycles (58.60%)
2025-09-16T22:37:28.100885Z  INFO risc0_zkvm::host::server::session: 9739 reserved cycles (29.72%)
2025-09-16T22:37:28.100906Z  INFO risc0_zkvm::host::server::session: ecalls
2025-09-16T22:37:28.100926Z  INFO risc0_zkvm::host::server::session:    4 Sha2 calls, 296 cycles, (0.90%)
2025-09-16T22:37:28.100932Z  INFO risc0_zkvm::host::server::session:    5 Read calls, 13 cycles, (0.04%)
2025-09-16T22:37:28.100933Z  INFO risc0_zkvm::host::server::session:    1 Poseidon2 calls, 13 cycles, (0.04%)
2025-09-16T22:37:28.100935Z  INFO risc0_zkvm::host::server::session:    1 Terminate calls, 2 cycles, (0.01%)
2025-09-16T22:37:28.100936Z  INFO risc0_zkvm::host::server::session:    0 Write calls, 0 cycles, (0.00%)
2025-09-16T22:37:28.100937Z  INFO risc0_zkvm::host::server::session:    0 User calls, 0 cycles, (0.00%)
2025-09-16T22:37:28.100939Z  INFO risc0_zkvm::host::server::session:    0 BigInt calls, 0 cycles, (0.00%)
2025-09-16T22:37:28.100940Z  INFO risc0_zkvm::host::server::session: syscalls
2025-09-16T22:37:28.100942Z  INFO risc0_zkvm::host::server::session:    0 Write calls
2025-09-16T22:37:28.100959Z  INFO risc0_zkvm::host::server::session:    0 VerifyIntegrity2 calls
2025-09-16T22:37:28.100963Z  INFO risc0_zkvm::host::server::session:    0 VerifyIntegrity calls
2025-09-16T22:37:28.100964Z  INFO risc0_zkvm::host::server::session:    0 Read calls
2025-09-16T22:37:28.100965Z  INFO risc0_zkvm::host::server::session:    0 ProveKeccak calls
2025-09-16T22:37:28.100966Z  INFO risc0_zkvm::host::server::session:    0 Keccak calls
WARNING: Proving in dev mode does not generate a valid receipt. Receipts generated from this process are invalid and should never be used in production.
```

The only additional dependency added to the guest code is`risc0-zkvm-platform = { version = "2.2.0", features = ["export-syscalls"] }`, which is the exact version that matches the v3.0.3 release.
