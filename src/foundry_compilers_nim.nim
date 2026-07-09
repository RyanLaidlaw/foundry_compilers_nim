import std/[os, json]

const currentDir = currentSourcePath().splitFile.dir

{.link: currentDir / "../../../foundry_compilers_nim/foundry_nim_compiler_bridge/target/release/libfoundry_nim_compiler_bridge.a".}

when defined(macosx):
  {.passL: "-lresolv -lc -framework CoreFoundation -framework Security".}
elif defined(linux):
  {.passL: "-lrt -lpthread -lm -ldl -lc".}

proc compile_solidity_project(rootPath: cstring): cstring {.cdecl, importc: "compile_solidity_project".}
proc free_rust_string(strPtr: cstring) {.cdecl, importc: "free_rust_string".}

proc compileSolidity*(projectPath: string): JsonNode =
  let rawResult = compile_solidity_project(projectPath.cstring)
  if rawResult.isNil:
    return newJObject()
  try:
    result = parseJson($rawResult)
  finally:
    free_rust_string(rawResult)