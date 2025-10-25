fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // This should be the whole directory, but cargo would have to check the whole directory with a lot of files which takes long
    println!("cargo:rerun-if-changed=headers/wrapper.h");
    println!("cargo:rerun-if-changed=headers/display_capture.h");
    println!("cargo:rerun-if-changed=headers/game_capture.h");
    println!("cargo:rerun-if-changed=headers/vec4.c");
    println!("cargo:rerun-if-changed=headers/window_capture.h");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-env-changed=LIBOBS_PATH");

    println!(
        "cargo:rustc-link-search=native={}",
        env!("CARGO_MANIFEST_DIR")
    );
    println!("cargo:rustc-link-lib=dylib=obs");

    if let Some(path) = std::env::var("LIBOBS_PATH").ok() {
        println!("cargo:rustc-link-search=native={}", path);
    }

    #[cfg(feature = "generate_bindings")]
    bindings::generate_bindings();
}

#[cfg(feature = "generate_bindings")]
mod bindings {
    use std::{collections::HashSet, path::PathBuf};

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

    pub fn generate_bindings() {
        let bindings = bindgen::builder()
            .header("headers/wrapper.h")
            .clang_arg(format!("-I{}", "headers/obs"))
            .blocklist_function("_bindgen_ty_2")
            .parse_callbacks(Box::new(get_ignored_macros()))
            .blocklist_function("_+.*")
            .blocklist_file(".*Windows\\.h")
            .blocklist_file(".*wchar\\.h")
            .blocklist_function("bwstrdup_n")
            .blocklist_function("bwstrdup")
            .derive_copy(true)
            .derive_debug(true)
            .derive_default(false)
            .derive_partialeq(false)
            .derive_eq(false)
            .derive_partialord(false)
            .derive_ord(false)
            .merge_extern_blocks(true)
            .generate()
            .expect("Error generating bindings");

        let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let bindings_path = out_path.join("bindings.rs");
        let bindings = bindings.to_string();
        let lines = bindings.lines().map(|line| {
            if line.trim().starts_with("#[doc") {
                let start_pos = line.find('"').unwrap() + 1;
                let end_pos = line.rfind('"').unwrap();
                let doc = &line[start_pos..end_pos];
                let doc = doc.replace("[", "\\\\[").replace("]", "\\\\]");

                format!("#[doc = \"{}\"]", doc)
            } else {
                line.to_string()
            }
        });

        let bindings = lines.collect::<Vec<_>>().join("\n");
        std::fs::write(&bindings_path, bindings).expect("Couldn't write bindings!");
    }
}
