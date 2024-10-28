use std::{collections::HashSet, env, path::PathBuf};

#[cfg(feature = "debug-tracing")]
use std::{
    fs::{self, File},
    io::Write,
};

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

#[cfg(feature = "debug-tracing")]
pub const KEYWORDS: &'static [&'static str] = &[
    "as",
    "use",
    "extern crate",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "if",
    "if let",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "for",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "impl",
    "ref",
    "return",
    "Self",
    "self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    "abstract",
    "alignof",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "offsetof",
    "override",
    "priv",
    "proc",
    "pure",
    "sizeof",
    "typeof",
    "unsized",
    "virtual",
    "yield",
];

#[cfg(feature = "debug-tracing")]
fn extract_args(function: &str) -> String {
    let function = function.replace("\n", "").replace(" ", " ");
    let start_index = function.find("(").unwrap();
    let end_index = function.rfind(")").unwrap();

    let args = &function[start_index + 1..end_index];
    let mut out = vec![];

    let mut buf = String::new();
    let mut nesting = 0;
    let mut after_colon = false;

    let mut prev = ' ';
    for c in args.chars() {
        if c == ' ' {
            continue;
        }

        if c == ',' && nesting == 0 {
            out.push(buf.clone());
            buf.clear();
            after_colon = false;
            continue;
        }

        if c == '<' && prev != '-' {
            nesting += 1;
        }

        if c == '>' && prev != '-' {
            nesting -= 1;
        }

        if c == ':' {
            after_colon = true;
        }

        if !after_colon && nesting == 0 {
            buf.push(c);
        }

        prev = c;
    }

    if buf.len() > 0 {
        out.push(buf);
    }

    out.join(", ")
}

#[cfg(feature = "debug-tracing")]
fn extract_field_name(field_part: &str) -> String {
    let mut out = String::new();
    for c in field_part.chars() {
        if c.is_alphanumeric() || c == '_' {
            out.push(c);
        } else {
            break;
        }
    }

    out
}

#[cfg(feature = "debug-tracing")]
fn generate_wrapper(bindings: &str) -> String {
    let mut indent = 0;
    let mut in_extern_c = false;

    let mut wrapper = String::new();
    let mut functions_to_wrap = vec![];

    // Only wrap obs functions for now
    let to_wrap_start = "obs_";

    let to_exclude = vec![
        "obs_data_get_json_with_defaults",
        "obs_data_get_json_pretty_with_defaults",
        "obs_fader_db_to_def",
        "obs_encoder_parent_video",
        "obs_encoder_set_group",
        "obs_encoder_group_create",
        "obs_encoder_group_destroy",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect::<HashSet<String>>();

    let lines: Vec<_> = bindings.split("\n").collect();
    for i in 0..lines.len() {
        let line = &lines[i];
        if line.contains("#") {
            continue;
        }

        if line.contains("{") {
            if line.starts_with("extern \"C\" {") && indent == 0 {
                in_extern_c = true;
            }
            indent += 1;
        }

        if line.contains("}") {
            indent -= 1;
            if indent == 0 {
                in_extern_c = false;
            }
        }

        let line = line.trim();
        if line.starts_with("pub") && (indent == 0 || (in_extern_c && indent == 1)) {
            let split = line.split(" ").collect::<Vec<&str>>();
            if split.len() < 3 {
                eprintln!(
                    "Warn - Couldn't process line: {} -  {} {}",
                    line, indent, in_extern_c
                );
                continue;
            }

            let field_name = split
                .iter()
                .find(|x| !KEYWORDS.contains(&x))
                .expect("Couldn't find field name for function");
            let matched_field_name = extract_field_name(field_name);
            if matched_field_name.len() == 0 {
                eprintln!(
                    "Warn - Couldn't process line: {} -  {} {}",
                    line, indent, in_extern_c
                );
                continue;
            }

            if in_extern_c
                && line.starts_with("pub fn")
                && matched_field_name.starts_with(to_wrap_start)
                && !to_exclude.contains(matched_field_name.as_str())
            {
                functions_to_wrap.push((i, matched_field_name))
            }
        }
    }

    for (function_index, name) in functions_to_wrap {
        let mut function = vec![];
        for i in function_index..lines.len() {
            let line = &lines[i];
            function.push(*line);

            if line.contains(";") {
                break;
            }
        }

        let function = function.join("\n").replace(";", "");
        let function = function.trim();

        let args = extract_args(function);

        let un_fn = function.replace("pub fn", "pub unsafe fn");
        wrapper.push_str(&format!(
            r#"
{un_fn} {{
    log::debug!("{{}}", "{name}");
    bindings::{name}({args})
}}
        "#
        ));
    }

    return wrapper;
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // This should be the whole directory, but cargo would have to check the whole directory with a lot of files which takes long
    println!("cargo:rerun-if-changed=headers/wrapper.h");
    println!("cargo:rerun-if-changed=headers/vec4.c");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-env-changed=LIBOBS_PATH");

    println!(
        "cargo:rustc-link-search=native={}",
        env!("CARGO_MANIFEST_DIR")
    );
    println!("cargo:rustc-link-lib=dylib=obs");

    if let Some(path) = env::var("LIBOBS_PATH").ok() {
        println!("cargo:rustc-link-search=native={}", path);
    }

    let bindings = bindgen::builder()
        .header("headers/wrapper.h")
        .clang_arg(format!("-I{}", "headers/obs"))
        .blocklist_function("_bindgen_ty_2")
        .parse_callbacks(Box::new(get_ignored_macros()))
        .blocklist_function("_+.*")
        .blocklist_file(".*Windows.h")
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
    let bindings_path = out_path.join("bindings.rs");

    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");

    #[cfg(feature = "debug-tracing")]
    {
        let bindings =
            fs::read_to_string(&bindings_path).expect("Couldn't read bindings file (somehow?)");
        let mut wrapper_f = File::create(out_path.join("bindings_wrapper.rs"))
            .expect("Couldn't create bindings wrapper file");

        wrapper_f
            .write(generate_wrapper(&bindings.replace("\r", "")).as_bytes())
            .expect("Couldn't write to bindings wrapper file");
    }

    cc::Build::new().file("headers/vec4.c").compile("libvec4.a");
}
