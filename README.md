# Nix Setup for developing Rust on ESP/NodeMCU

This Branch contains a guide and a working example.

<img src="https://raw.githubusercontent.com/NixOS/nixos-artwork/refs/heads/master/logo/nix-snowflake-rainbow.svg" height="60px" align="center"/> +
<img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" height="60px" align="center"/> +
<img src="./nix-rust-esp-blink/diagram.png?raw=true" height="60px" align="center"/>

## TL;DR

The working example can be found at:
[./nix-rust-esp-blink](./nix-rust-esp-blink)


## Existing Tools

Rust <img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" height="20px"/> on
ESP <img src="./nix-rust-esp-blink/diagram.png?raw=true" height="20px"/>
is well documented at [https://docs.esp-rs.org/book](https://docs.esp-rs.org/book/writing-your-own-application/generate-project/esp-template.html).

NIX <img src="https://raw.githubusercontent.com/NixOS/nixos-artwork/refs/heads/master/logo/nix-snowflake-colours.svg" height="20px"/> for
ESP <img src="./nix-rust-esp-blink/diagram.png?raw=true" height="20px"/>
is provided by [https://github.com/mirrexagon/nixpkgs-esp-dev](https://github.com/mirrexagon/nixpkgs-esp-dev).

NIX <img src="https://raw.githubusercontent.com/NixOS/nixos-artwork/refs/heads/master/logo/nix-snowflake-colours.svg" height="20px"/> for
Rust <img src="https://upload.wikimedia.org/wikipedia/commons/d/d5/Rust_programming_language_black_logo.svg" height="20px"/> on
ESP <img src="./nix-rust-esp-blink/diagram.png?raw=true" height="20px"/>
is provided by the [fork from hsel-netsys](https://github.com/hsel-netsys/nixpkgs-esp-dev-rust/tree/rust).


## Workflow to get a Rust Hello World on ESP-WROOM-32

### 1. Edit `buildInputs` and `LD_LIBRARY_PATH`

To later build and use `cargo-generate`, we need `pkgs.openssl`.

So first we [edit `shells/esp32s2-idf-rust.nix`](https://github.com/johannesloetzsch/nixpkgs-esp-dev-rust/compare/321d69c4..686e20be#diff-d581342908253b847eb7c7106ff2e9bb8d9de94e22133166b8106e6ec8db10caR34)

```diff
+    openssl  # required for cargo install cargo-generate
   ];
   shellHook = ''
     # fixes libstdc++ issues and libgl.so issues
-    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [ pkgs.libxml2 pkgs.zlib pkgs.stdenv.cc.cc.lib ]}
+    export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [ pkgs.libxml2 pkgs.zlib pkgs.stdenv.cc.cc.lib pkgs.openssl ]}
```


### 2. Use `esp-idf-esp32` instead of `esp-idf-esp32s2`

It's important to build for the correct version of the [ESP32-xx family](https://en.wikipedia.org/wiki/ESP32#ESP32-xx_family).
The ESP-Board we use in this example is a [`ESP-WROOM-32`](https://en.wikipedia.org/wiki/ESP32#Printed_circuit_boards). It contains the ["original" `ESP32`](https://en.wikipedia.org/wiki/ESP32#ESP32) (without any suffix).

The [original repository of mirrexagon](https://github.com/mirrexagon/nixpkgs-esp-dev) provides support for ESP8266 and ESP32(-C3, -S2, -S3, -C6, -H2).
The [Rust Fork by hsel-netsys](https://github.com/hsel-netsys/nixpkgs-esp-dev-rust) provides a [nix-shell with rust support](shells/esp32s2-idf-rust.nix), but only for ESP32-**S2**. This is no problem, since the original `esp-idf-esp32`-package is still available from the [overlay](./overlay.nix).

So we copy the shell with rust support and substitute the package `esp-idf-esp32s2` with `esp-idf-esp32` in `buildInputs`:

```sh
cp shells/esp32{s2,}-idf-rust.nix
vim shells/esp32-idf-rust.nix
```

```diff
diff shells/esp32{s2,}-idf-rust.nix 
6c6
<     esp-idf-esp32s2
---
>     esp-idf-esp32
```

For convenience we add the new shell in flake.nix as devShell and set it as default:

```diff
-      devShells = {
+      devShells = rec {
+        default = esp32-idf-rust;
+
         esp32-idf = import ./shells/esp32-idf.nix { inherit pkgs; };
+        esp32-idf-rust = import ./shells/esp32-idf-rust.nix { inherit pkgs; };
         esp32s2-idf-rust = import ./shells/esp32s2-idf-rust.nix { inherit pkgs; };
```


### 3. Ready to follow the nix-independent instructions :)

We can now run `nix develop` and follow the instructions from [https://docs.esp-rs.org](https://docs.esp-rs.org/book/writing-your-own-application/generate-project/index.html):

```sh
nix develop

cargo install cargo-generate
cargo generate esp-rs/esp-template
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
