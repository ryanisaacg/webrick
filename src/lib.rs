use brick::SourceFile;
use brick_ld::InputModule;
use brick_wasm_backend::{compile, BackendOptions};

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn compile_source(source: &str) -> Vec<u8> {
    let sources = &[SourceFile {
        filename: "<web input>",
        module_name: "main",
        contents: source.to_string(),
    }];

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
                    return module;
                }
                Err(err) => {
                    println!("internal compiler error: {err}");
                }
            }
        }
        Err(e) => {
            println!("{e}");
        }
    }

    Vec::new()
}
