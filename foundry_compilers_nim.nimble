version       = "0.1.9"
author        = "Ryan Laidlaw"
description   = "A Nim wrapper around the Rust foundry-compilers crate"
license       = "MIT"
srcDir        = "src"

requires "nim >= 2.2.10"

installDirs = @["src", "foundry_nim_compiler_bridge"]

task buildRust, "Build the Rust FFI bridge":
  exec "cargo build --release --manifest-path " & thisDir() & "/foundry_nim_compiler_bridge/Cargo.toml"

before install:
  buildRustTask()

before build:
  buildRustTask()

before test:
  buildRustTask()