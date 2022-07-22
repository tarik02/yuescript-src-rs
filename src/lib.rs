use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Build {
    out_dir: Option<PathBuf>,
    include_dirs: Vec<PathBuf>,
}

impl Build {
    pub fn new() -> Build {
        Build {
            out_dir: env::var_os("OUT_DIR").map(|s| PathBuf::from(s).join("yuescript-build")),
            include_dirs: vec![],
        }
    }

    pub fn out_dir<P: AsRef<Path>>(&mut self, path: P) -> &mut Build {
        self.out_dir = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn include_dirs(&mut self, include_dirs: Vec<PathBuf>) -> &mut Self {
        self.include_dirs = include_dirs;
        self
    }

    pub fn build(&mut self) {
        let out_dir = self.out_dir.as_ref().expect("OUT_DIR not set");
        let lib_dir = out_dir.join("lib");
        let include_dir = out_dir.join("include");

        let source_dir_base = Path::new(env!("CARGO_MANIFEST_DIR"));
        let source_dir = source_dir_base.join("yuescript/src");

        if lib_dir.exists() {
            fs::remove_dir_all(&lib_dir).unwrap();
        }
        fs::create_dir_all(&lib_dir).unwrap();

        if include_dir.exists() {
            fs::remove_dir_all(&include_dir).unwrap();
        }
        fs::create_dir_all(&include_dir).unwrap();

        cc::Build::new()
            .cpp(true)
            .opt_level(3)
            .include(source_dir_base.join("yuescript/src"))
            .includes(&self.include_dirs)
            .file(source_dir.join("yuescript/ast.cpp"))
            .file(source_dir.join("yuescript/parser.cpp"))
            .file(source_dir.join("yuescript/yue_compiler.cpp"))
            .file(source_dir.join("yuescript/yue_parser.cpp"))
            .file(source_dir.join("yuescript/yuescript.cpp"))
            .flag_if_supported("-std=c++17")
            .out_dir(&lib_dir)
            .compile("yue");

        for f in &[
            "yuescript/ast.hpp",
            "yuescript/parser.hpp",
            "yuescript/yue_compiler.h",
            "yuescript/yue_parser.h",
            "yuescript/yuescript.h",
        ] {
            fs::create_dir_all(include_dir.join(f).parent().unwrap()).unwrap();
            fs::copy(source_dir.join(f), include_dir.join(f)).unwrap();
        }

        println!("cargo:include={}", include_dir.display());
        println!("cargo:lib={}", lib_dir.display());
        println!("cargo:rustc-link-lib=static={}", "yue");
    }
}
