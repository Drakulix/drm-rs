#[cfg(feature = "use_bindgen")]
mod use_bindgen {
    extern crate bindgen;
    extern crate pkg_config;

    use self::bindgen::Builder;
    use std::env::var;
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::Write;

    const MACROS: &'static [&str] = &[
        "DRM_MODE_PROP_SIGNED_RANGE",
        "DRM_MODE_PROP_OBJECT"
    ];

    // Unfortunately the cexpr crate (and as such, bindgen) does not support C
    // functional macros (https://github.com/jethrogb/rust-cexpr/issues/3).
    // Therefore we must create them ourselves.
    fn bind_function_macro(name: &str) -> String {
        let temp_bind = "const unsigned int _".to_string() + name + " = " + name + ";\n";
        let undef = "#undef ".to_string() + name + "\n";
        let new_bind = "const unsigned int ".to_string() + name + " = _" + name + ";\n";

        temp_bind + &undef + &new_bind
    }

    pub fn generate_header() {
        let out_path = String::from(var("OUT_DIR").unwrap());
        let header = out_path.clone() + "/bindings.h";

        let mut f = File::create(header).expect("Could not create header");
        let includes = "#include <drm.h>\n#include <drm_mode.h>\n".to_string();
        f.write(includes.as_bytes()).expect("Could not write header.");

        for m in MACROS {
            f.write(bind_function_macro(m).as_bytes())
                .expect("Could not write header");
        }
    }

    pub fn generate_bindings() {
        let out_path = String::from(var("OUT_DIR").unwrap());
        let header = out_path.clone() + "/bindings.h";

        let pkgconf = pkg_config::Config::new();
        let lib = pkgconf.probe("libdrm").unwrap();

        let mut builder = Builder::default()
            .header(header)
            .ctypes_prefix("libc")
            .emit_builtins()
            .emit_clang_ast()
            .emit_ir()
            .derive_debug(true)
            .derive_default(true);

        for path in lib.include_paths {
            let arg = "-I".to_string() + &path.into_os_string().into_string().unwrap();
            builder = builder.clang_arg(arg)
        }
        let bindings = builder.generate().expect("Unable to generate libdrm bindings");

        let bind_file = PathBuf::from(out_path).join("bindings.rs");

        bindings.write_to_file(bind_file).expect("Could not write bindings");
    }
}

#[cfg(feature = "use_bindgen")]
pub fn main() {
    use_bindgen::generate_header();
    use_bindgen::generate_bindings();
}

#[cfg(not(feature = "use_bindgen"))]
pub fn main() {}

