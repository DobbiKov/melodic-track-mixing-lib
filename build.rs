use bindgen::builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bindings = bindgen::builder()
        .header("./libkeyfinder/src/wrapper.h")
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        .allowlist_function("KeyFinder.*")
        .allowlist_type("KeyFinder.*")
        .allowlist_var("KeyFinder.*")
        //.opaque_type("difference_type")
        //.opaque_type("const_pointer")
        //
        .opaque_type("difference_type")
        .blocklist_type("const_pointer")
        .blocklist_type("type_")
        .blocklist_type("__map_const_pointer")
        // also block the two bad std‐aliases that need generic parameters
        .opaque_type("^std___split_buffer___alloc_rr")
        .opaque_type("^std_deque___map")
        .blocklist_item("std___.*")
        // 2) make sure any types those APIs refer to also get pulled in
        .allowlist_recursively(true)
        .layout_tests(false)
        .generate()?;

    // 2) Turn the generated code into a String
    let mut src = bindings.to_string();

    // 3) Remove the two bad alias lines
    let lines: Vec<_> = src
        .lines()
        .filter(|l| {
            !l.starts_with("pub type std___split_buffer___alloc_rr")
                && !l.starts_with("pub type std_deque___map")
        })
        .collect();
    src = lines.join("\n");

    // 4) Append opaque stub definitions so those names still exist
    src.push_str("\n\n");
    src.push_str(
        "#[repr(C)]\n\
         pub struct std___split_buffer___alloc_rr { _unused: [u8; 0] }\n\n\
         #[repr(C)]\n\
         #[derive(Debug)]\n\
         pub struct std_deque___map { _unused: [u8; 0] }\n",
    );

    // 5) Write the cleaned, stub-augmented bindings back out
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
    std::fs::write(out_path.join("bindings.rs"), &src)?;

    std::fs::write(std::path::Path::new("bindings.rs"), &src)?;
    Ok(())
}
