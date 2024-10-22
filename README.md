## TL;DR

The working example can be found at:
[./nix-rust-esp-blink](./nix-rust-esp-blink)


## Workflow to get a Rust Hello World on ESP-WROOM-32

```sh
nix develop

## https://docs.esp-rs.org/book/writing-your-own-application/generate-project/index.html
cargo install cargo-generate
cargo generate esp-rs/esp-template  ## todo: cargo generate esp-rs/esp-idf-template cargo
cd nix-rust-esp-blink/  ## or whatever project name we entered

cargo build
cargo run --release
```

Output:
```
   Compiling nix-rust-esp-blink v0.1.0 (…/nixpkgs-esp-dev-rust/nix-rust-esp-blink)
    Finished `release` profile [optimized + debuginfo] target(s) in 1.67s
     Running `espflash flash --monitor target/xtensa-esp32-none-elf/release/nix-rust-esp-blink`
[ INFO ] Serial port: '/dev/ttyUSB0'
[ INFO ] Connecting...
[ INFO ] Using flash stub
Chip type:         esp32 (revision v1.0)
[…]
[00:00:01] [========================================]      16/16      0x10000                                                                                                                  [ INFO ] Flashing has completed!
Commands:
    CTRL+R    Reset chip
    CTRL+C    Exit

rst:0x1 (POWERON_RESET),boot:0x13 (SPI_FAST_FLASH_BOOT)
[…]
I (31) boot: ESP-IDF v5.1-beta1-378-gea5e0ff298-dirt 2nd stage bootloader
[…]
I (157) boot: Loaded app from partition at offset 0x10000
I (157) boot: Disabling RNG early entropy source...
INFO - Hello world!
```


## Based on

* https://github.com/hsel-netsys/nixpkgs-esp-dev-rust/tree/rust
* https://github.com/mirrexagon/nixpkgs-esp-dev
* https://docs.esp-rs.org/book/writing-your-own-application/generate-project/esp-template.html
