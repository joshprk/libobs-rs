use std::collections::HashSet;

#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn get_ignored_macros() -> IgnoreMacros {
    let mut ignored = HashSet::new();
    ignored.insert("FE_INVALID".to_string());
    ignored.insert("FE_DIVBYZERO".to_string());
    ignored.insert("FE_OVERFLOW".to_string());
    ignored.insert("FE_UNDERFLOW".to_string());
    ignored.insert("FE_INEXACT".to_string());
    ignored.insert("FE_TONEAREST".to_string());
    ignored.insert("FE_DOWNWARD".to_string());
    ignored.insert("FE_UPWARD".to_string());
    ignored.insert("FE_TOWARDZERO".to_string());
    ignored.insert("FP_NORMAL".to_string());
    ignored.insert("FP_SUBNORMAL".to_string());
    ignored.insert("FP_ZERO".to_string());
    ignored.insert("FP_INFINITE".to_string());
    ignored.insert("FP_NAN".to_string());
    IgnoreMacros(ignored)
}

fn main() {
    println!("cargo:rustc-link-search=native={}", env!("CARGO_MANIFEST_DIR"));
    println!("cargo:rustc-link-lib=obs");
    println!("cargo:rustc-env=LIBOBS_BINDINGS_FILE=bindings.rs");

    bindgen::builder()
        .header("headers/obs.h")
        .blocklist_function("_bindgen_ty_2")
        .parse_callbacks(Box::new(get_ignored_macros()))
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
        .expect("Error generating bindings")
        .write_to_file(&format!("{}/src/bindings.rs", env!("CARGO_MANIFEST_DIR")))
        .expect("Error outputting bindings");
}