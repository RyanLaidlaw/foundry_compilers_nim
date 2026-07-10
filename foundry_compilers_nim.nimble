version       = "0.1.0"
author        = "Ryan Laidlaw"
description   = "A Nim wrapper around the Rust foundry-compilers crate"
license       = "MIT"
srcDir        = "src"
bin           = @["foundry_compilers_nim"]

requires "nim >= 2.0.0"

task buildRust, "Build the Rust FFI bridge":
  exec "cargo build --release --manifest-path foundry_nim_compiler_bridge/Cargo.toml"

before build:
  buildRustTask()

before test:
  buildRustTask()