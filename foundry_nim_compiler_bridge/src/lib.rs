use foundry_compilers::{
    Project, ProjectPathsConfig, artifacts::{ConfigurableContractArtifact}, multi::MultiCompilerSettings
};
use std::collections::BTreeMap;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;

use foundry_compilers::artifacts::output_selection::OutputSelection;

#[unsafe(no_mangle)]
pub extern "C" fn compile_solidity_project(root_path_ptr: *const c_char) -> *mut c_char {
    if root_path_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(root_path_ptr) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let root_path = Path::new(path_str);

    let source_dir = ProjectPathsConfig::find_source_dir(root_path);

    let mut settings = MultiCompilerSettings::default();
    settings.solc.settings.output_selection = OutputSelection::common_output_selection(["evm.bytecode.object".to_string(), "abi".to_string()]);

    let paths = ProjectPathsConfig::builder()
        .root(Path::new(path_str))
        .sources(source_dir)
        .tests(".no-tests")
        .scripts(".no-scripts")
        .build()
        .unwrap();

    let project = Project::builder()
        .paths(paths)
        .no_artifacts()
        .settings(settings)
        .build(Default::default())
        .unwrap();

    match project.compile() {
        Ok(output) => {
            if output.has_compiler_errors() {
                let errors: Vec<String> = output.output().errors.iter().map(|e| e.to_string()).collect();
                return CString::new(serde_json::json!({ "compiler_errors": errors }).to_string()).unwrap().into_raw();
            }

            let artifacts: BTreeMap<String, ConfigurableContractArtifact> = output
                .into_artifacts()
                .filter(|(_id, artifact)| {
                    if let Some(bytes) = &artifact.bytecode {
                        bytes.object.is_non_empty_bytecode()
                    } else {
                        false
                    }
                })
                .map(|(id, artifact)| (id.name, artifact))
                .collect();

            match serde_json::to_string(&artifacts) {
                Ok(json_output) => CString::new(json_output).unwrap().into_raw(),
                Err(e) => {
                    let err_msg = format!("{{\"error\": \"serialization failed: {}\"}}", e);
                    CString::new(err_msg).unwrap().into_raw()
                }
            }
        }
        Err(e) => {
            let err_msg = format!("{{\"error\": \"{}\"}}", e);
            CString::new(err_msg).unwrap().into_raw()
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_rust_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}
