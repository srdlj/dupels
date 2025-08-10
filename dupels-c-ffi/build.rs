use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config = cbindgen::Config::default();
    config.language = cbindgen::Language::C;
    config.style = cbindgen::Style::Both;
    config.cpp_compat = true;
    config.documentation = true;
    config.documentation_style = cbindgen::DocumentationStyle::Doxy;
    
    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        .write_to_file("dupels.h");
}
