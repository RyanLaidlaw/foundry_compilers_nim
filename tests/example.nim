import json, os, tables
import foundry_compilers_nim

let sourceDir = currentSourcePath().parentDir()
let path = sourceDir / "test_files"

let artifacts = compileSolidity(path).getFields()

echo artifacts["Counter"]["bytecode"]["object"]
echo artifacts["Bank"]["abi"]