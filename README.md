# Foundry Compilers: Nim

[![Language](https://img.shields.io/badge/language-nim-yellow?style=flat-square&logo=nim")](https://nim-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square&logo=github")](https://github.com/RyanLaidlaw/foundry_compilers_nim/blob/main/LICENSE)

`foundry_compilers_nim` is an easy to use library that wraps Foundry's compiler crates.

# Installation
Simply run `nimble install foundry_compilers_nim`

# Usage
Pass the root of the Foundry project, and let the `foundry_compilers` crate figure out what to compile.
```Nim
import json, os, tables
import foundry_compilers_nim

let sourceDir = currentSourcePath().parentDir()
let path = sourceDir / "test_files"

let artifacts = compileSolidity(path).getFields()

echo artifacts["Counter"]["bytecode"]["object"]
echo artifacts["Bank"]["abi"]
```

# License
This project uses the [MIT License](https://github.com/RyanLaidlaw/foundry_compilers_nim/blob/main/LICENSE)