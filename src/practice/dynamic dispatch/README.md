# For Debug, Use GDB Debugger

GDB is helpful to visualize the assembly code when comaparing multiple states/concepts.

- **How to run it:** Use `cargo test` then type `gdb -tui <test exe file>` look something like `Running src/lib.rs (target/debug/deps/rust_practice-adb0a384fcaee666)` in the terminal

- **Example:** `gdb -tui target/debug/deps/rust_practice-adb0a384fcaee666`. Look different commands at GDB Debugger Website.

- **GDB Setting for Rust** Use following commands for prettier visualization.
  `set demangle-style auto` \
  `set language rust` \
  `set print asm-demangle yes`

## Or Use Default Rust Debugger
