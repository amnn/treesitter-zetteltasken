use std::path::Path;

fn main() {
    let src_dir = Path::new("src");
    let vendor_dir = Path::new("vendor");

    let mut c_config = cc::Build::new();
    c_config.std("c11").include(src_dir);

    #[cfg(target_env = "msvc")]
    c_config.flag("-utf-8");

    let parser_path = src_dir.join("parser.c");
    c_config.file(&parser_path);
    println!("cargo:rerun-if-changed={}", parser_path.to_str().unwrap());

    // The markdown external scanner is vendored into the project, and included in our own external
    // scanner implementation. Include it as a file to watch for changes, but don't include it as
    // its own source file.
    let mut md_scanner_path = vendor_dir.to_owned();
    md_scanner_path.extend([
        "tree-sitter-markdown",
        "tree-sitter-markdown",
        "src",
        "scanner.c",
    ]);
    println!(
        "cargo:rerun-if-changed={}",
        md_scanner_path.to_str().unwrap()
    );

    let scanner_path = src_dir.join("scanner.c");
    c_config.file(&scanner_path);
    println!("cargo:rerun-if-changed={}", scanner_path.to_str().unwrap());

    c_config.compile("tree-sitter-zetteltasken");
}
