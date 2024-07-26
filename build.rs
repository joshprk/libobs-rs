use std::collections::HashSet;
use std::env;
use std::path::PathBuf;

use bindgen::callbacks::ParseCallbacks;
use bindgen::callbacks::MacroParsingBehavior;

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> MacroParsingBehavior {
        if self.0.contains(name) {
            return MacroParsingBehavior::Ignore
        }

        MacroParsingBehavior::Default
    }
}

fn get_ignored_macros() -> Box<IgnoreMacros> {
    let mut ignored = HashSet::new();
    let ignore_list = vec![
        "FE_INVALID",
        "FE_DIVBYZERO",
        "FE_OVERFLOW",
        "FE_UNDERFLOW",
        "FE_INEXACT",
        "FE_TONEAREST",
        "FE_DOWNWARD",
        "FE_UPWARD",
        "FE_TOWARDZERO",
        "FP_NORMAL",
        "FP_SUBNORMAL",
        "FP_ZERO",
        "FP_INFINITE",
        "FP_NAN",
    ];

    for item in ignore_list {
       ignored.insert(item.to_string()); 
    }

    Box::new(IgnoreMacros(ignored))
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=headers");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rustc-link-search=native={}", env!("CARGO_MANIFEST_DIR"));
    println!("cargo:rustc-link-lib=dylib=obs");

    if let Some(path) = env::var("LIBOBS_PATH").ok() {
        println!("cargo:rustc-link-search=native={}", path);
    }
    
    let bindings = bindgen::builder()
        .header("headers/obs.h")
        .blocklist_function("_bindgen_ty_2")
        .parse_callbacks(get_ignored_macros())
        .blocklist_function("_+.*")
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(false)
        .derive_partialeq(true)
        .derive_eq(true)
        .derive_partialord(true)
        .derive_ord(true)
        .layout_tests(false)
        .merge_extern_blocks(true)
        .generate()
        .expect("Error generating bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}