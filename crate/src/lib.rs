use brick::SourceFile;
use brick_ld::InputModule;
use brick_wasm_backend::{compile, BackendOptions};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct CompileResults {
    binary: Option<Vec<u8>>,
    errors: Option<String>,
}

#[wasm_bindgen]
impl CompileResults {
    #[wasm_bindgen(getter)]
    pub fn binary(&self) -> Option<Vec<u8>> {
        self.binary.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn errors(&self) -> Option<String> {
        self.errors.clone()
    }
}

#[wasm_bindgen]
pub fn compile_source(source: &str) -> CompileResults {
    let sources = &[SourceFile {
        filename: "<web input>",
        module_name: "main",
        contents: source.to_string(),
    }];

    let mut binary = None;
    let mut errors = None;

    match compile(
        sources,
        BackendOptions {
            include_start_marker: true,
            top_level_name: "main",
        },
    ) {
        Ok(module) => {
            match brick_ld::link(&[
                InputModule {
                    name: "main",
                    definition: module.as_slice(),
                    public_exports: true,
                    is_start: true,
                },
                InputModule {
                    name: "brick-runtime",
                    definition: runtime_binary::wasm_runtime(),
                    public_exports: false,
                    is_start: false,
                },
            ]) {
                Ok(module) => {
                    binary = Some(module);
                }
                Err(err) => {
                    errors = Some(format!("{err}"));
                }
            }
        }
        Err(e) => {
            errors = Some(format!("{e}"));
        }
    }

    CompileResults { binary, errors }
}
